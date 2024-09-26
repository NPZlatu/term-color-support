//! Module for managing color support detection and information.
//!
//! This module provides functionality for fetching the color support level based on various
//! environment variables, operating system details, and terminal configurations.
//!
//! The `Environment` struct represents the environment details, including terminal type, color
//! terminal, TeamCity version, Continuous Integration platform, operating system release, terminal
//! program, and terminal program version. It also provides methods for determining the color
//! support level based on the environment.
//!
//! The `ColorSupportLevel` enum defines different levels of color support, including `NoColor`,
//! `Basic`, `Colors256`, and `TrueColor`.
//!
//! The `ColorInfo` struct holds information about the color support level, including whether it
//! has basic color support, 256-color support, and 16-million color (TrueColor) support.
//!
//! The `ColorSupport` struct provides methods for fetching color support information for stdout
//! and stderr streams.
//!
//! The `determine_stream_color_level` function determines the color support level for a given
//! output stream based on the provided options.
//!
//! Additionally, the `options` module provides utility functions for extracting color level
//! information from environment variables and command-line flags.
//!

use crate::colors::ColorSupportLevel;
use os_info;
use regex::Regex;

/// Struct representing the environment details.
pub struct Environment {
    /// Terminal type.
    pub term: String,
    /// Color terminal.
    pub colorterm: Option<String>,
    /// TeamCity version.
    pub teamcity_version: Option<String>,
    /// Continuous Integration platform.
    pub ci: Option<String>,
    /// Operating System release.
    pub os_release: String,
    /// Terminal program.
    pub term_program: Option<String>,
    /// Terminal program version.
    pub term_program_version: String,
}

impl Environment {
    pub fn new(
        term: Option<String>,
        colorterm: Option<String>,
        teamcity_version: Option<String>,
        ci: Option<String>,
        os_release: Option<String>,
        term_program: Option<String>,
        term_program_version: Option<String>,
    ) -> Self {
        let binding = os_info::get();
        let os_release = os_release.unwrap_or_else(|| binding.version().to_string());

        Self {
            term: term
                .unwrap_or_else(|| std::env::var("TERM").unwrap_or_else(|_| String::from(""))),
            colorterm: colorterm.or_else(|| std::env::var("COLORTERM").ok()),
            teamcity_version: teamcity_version.or_else(|| std::env::var("TEAMCITY_VERSION").ok()),
            ci: ci.or_else(|| std::env::var("CI").ok()),
            os_release,
            term_program: term_program.or_else(|| std::env::var("TERM_PROGRAM").ok()),
            term_program_version: term_program_version.unwrap_or_else(|| {
                std::env::var("TERM_PROGRAM_VERSION").unwrap_or_else(|_| String::from(""))
            }),
        }
    }

    pub fn default() -> Self {
        Self::new(None, None, None, None, None, None, None)
    }

    /// Gets the parts of the OS release version.
    fn get_os_release_parts(&self) -> Vec<u32> {
        self.os_release
            .split('.')
            .map(|part| part.parse().unwrap_or(0))
            .collect()
    }

    /// Gets the major version of the terminal program.
    fn get_term_program_version_major(&self) -> Option<u32> {
        if let Some(major_version) = self.term_program_version.split('.').next() {
            if let Ok(major) = major_version.parse::<u32>() {
                return Some(major);
            }
        }
        None
    }

    /// Determines the color support level based on the environment.
    pub fn determine_color_level(&self) -> ColorSupportLevel {
        if self.term == "dumb" {
            return ColorSupportLevel::NoColor;
        }

        if cfg!(windows) {
            let release_parts = self.get_os_release_parts();
            if release_parts[0] >= 10 && release_parts[2] >= 10_586 {
                return if release_parts[2] >= 14_931 {
                    ColorSupportLevel::TrueColor
                } else {
                    ColorSupportLevel::Colors256
                };
            }
            return ColorSupportLevel::Basic;
        }

        if let Some(ci) = &self.ci {
            if ci == "TF_BUILD" && std::env::var("AGENT_NAME").is_ok() {
                return ColorSupportLevel::Basic;
            }
            return ColorSupportLevel::NoColor;
        }

        let regex = Regex::new(r#"^(9\.(0*[1-9]\d*|0+)|\d{2,}\.)"#).unwrap();

        // Check if teamcity_version exists and matches the regex pattern
        if let Some(teamcity_version) = &self.teamcity_version {
            if regex.is_match(teamcity_version) {
                return ColorSupportLevel::Basic;
            } else {
                return ColorSupportLevel::NoColor;
            }
        }

        if let Some(colorterm) = &self.colorterm {
            if colorterm == "truecolor" {
                return ColorSupportLevel::TrueColor;
            }
        }

        if self.term == "xterm-kitty" {
            return ColorSupportLevel::TrueColor;
        }

        if let Some(term_program) = &self.term_program {
            if let Some(version_major) = self.get_term_program_version_major() {
                match term_program.as_str() {
                    "iTerm.app" => {
                        return if version_major >= 3 {
                            ColorSupportLevel::TrueColor
                        } else {
                            ColorSupportLevel::Colors256
                        };
                    }
                    "Apple_Terminal" => {
                        return ColorSupportLevel::Colors256;
                    }
                    _ => {}
                }
            }
        }

        if self.term.ends_with("-256color") {
            return ColorSupportLevel::Colors256;
        }

        if self.term.starts_with("screen")
            || self.term.starts_with("xterm")
            || self.term.starts_with("vt100")
            || self.term.starts_with("vt220")
            || self.term.starts_with("rxvt")
            || self.term.contains("color")
            || self.term.contains("ansi")
            || self.term.contains("cygwin")
            || self.term.contains("linux")
        {
            return ColorSupportLevel::Basic;
        }

        if let Some(colorterm) = &self.colorterm {
            if !colorterm.is_empty() {
                return ColorSupportLevel::Basic;
            }
        }

        ColorSupportLevel::NoColor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test determine_color_level() under various conditions
    #[test]
    fn test_determine_color_level() {
        // Test when term is "dumb"
        let mut environment_dumb = Environment::default();
        environment_dumb.term = String::from("dumb");
        assert_eq!(
            environment_dumb.determine_color_level(),
            ColorSupportLevel::NoColor
        );

        // Test when term is "xterm-kitty"
        let mut environment_xterm_kitty = Environment::default();
        environment_xterm_kitty.term = String::from("xterm-kitty");
        assert_eq!(
            environment_xterm_kitty.determine_color_level(),
            ColorSupportLevel::TrueColor
        );

        // Test when term starts with "vt100"
        let environment_vt100 = Environment::new(
            Some(String::from("vt100-color")),
            Some(String::from("")),
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(
            environment_vt100.determine_color_level(),
            ColorSupportLevel::Basic
        );

        // Test when term is "screen" and colorterm is "truecolor"
        let environment_screen_truecolor = Environment::new(
            Some(String::from("screen")),
            Some(String::from("truecolor")),
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(
            environment_screen_truecolor.determine_color_level(),
            ColorSupportLevel::TrueColor
        );

        // Test when term is "linux"
        let mut environment_linux = Environment::new(
            Some(String::from("linux")),
            Some(String::from("")),
            None,
            None,
            None,
            None,
            None,
        );
        environment_linux.term = String::from("linux");
        assert_eq!(
            environment_linux.determine_color_level(),
            ColorSupportLevel::Basic
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_color_level() {
        // Test when release_parts[0] < 10
        let mut environment = Environment::default();
        environment.os_release = String::from("9.0.0");
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Basic
        );

        // Test when release_parts[0] >= 10 and release_parts[2] < 10_586
        environment.os_release = String::from("10.0.0");
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Basic
        );

        // Test when release_parts[0] >= 10, release_parts[2] >= 10_586, and release_parts[2] < 14_931
        environment.os_release = String::from("10.0.10585");
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Colors256
        );

        // Test when release_parts[0] >= 10 and release_parts[2] >= 14_931
        environment.os_release = String::from("10.0.14931");
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::TrueColor
        );
    }

    #[test]
    fn test_teamcity_version() {
        // Test when teamcity_version starts with "9."
        let mut environment = Environment::default();
        environment.teamcity_version = Some(String::from("9.1"));
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Basic
        );

        // Test when teamcity_version starts with a numeric character
        let mut environment = Environment::default();
        environment.teamcity_version = Some(String::from("10.0"));
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Basic
        );

        // Test when teamcity_version does not meet the conditions
        let mut environment = Environment::default();
        environment.teamcity_version = Some(String::from("8.0"));
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::NoColor
        );
    }

    #[test]
    fn test_get_term_program_version_major() {
        let mut environment = Environment::default();
        environment.term_program_version = String::from("3.2.1");
        assert_eq!(environment.get_term_program_version_major(), Some(3));

        let mut environment = Environment::default();
        environment.colorterm = Some(String::from(""));
        environment.term_program = Some(String::from("Apple_Terminal"));
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Colors256
        );

        let mut environment: Environment = Environment::default();
        environment.term_program_version = String::from("3.2.1");
        environment.colorterm = Some(String::from(""));
        environment.term_program = Some(String::from("iTerm.app"));
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::TrueColor
        );

        let mut environment: Environment = Environment::default();
        environment.colorterm = Some(String::from(""));
        environment.term_program_version = String::from("2.2.1");
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Colors256
        );
    }

    #[test]
    fn test_determine_color_level_basic() {
        let mut environment = Environment::default();
        environment.colorterm = Some(String::from(""));
        environment.term = String::from("rxvt");
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Basic
        );
    }

    #[test]
    fn test_ci_tf_build() {
        // Save original environment variables
        let original_ci = std::env::var("CI").ok();
        let original_agent_name = std::env::var("AGENT_NAME").ok();

        // Mock the environment variables for testing
        std::env::set_var("CI", "TF_BUILD");
        std::env::set_var("AGENT_NAME", "mock_agent");

        // Create an Environment instance
        let environment = Environment::default();

        // Assert that the determine_color_level method returns ColorSupportLevel::Basic
        assert_eq!(
            environment.determine_color_level(),
            ColorSupportLevel::Basic
        );

        // Reset the environment variables back to their original values
        match original_ci {
            Some(val) => std::env::set_var("CI", val),
            None => std::env::remove_var("CI"),
        }
        match original_agent_name {
            Some(val) => std::env::set_var("AGENT_NAME", val),
            None => std::env::remove_var("AGENT_NAME"),
        }
    }

    #[test]
    fn test_get_os_release_parts_valid() {
        let mut environment = Environment::default();
        environment.os_release = String::from("10.2.3");
        assert_eq!(environment.get_os_release_parts(), vec![10, 2, 3]);
    }

    #[test]
    fn test_get_os_release_parts_invalid() {
        let mut environment = Environment::default();
        environment.os_release = String::from("10.a.3");
        assert_eq!(environment.get_os_release_parts(), vec![10, 0, 3]);
    }

    #[test]
    fn test_get_os_release_parts_empty() {
        let mut environment = Environment::default();
        environment.os_release = String::from("");
        assert_eq!(environment.get_os_release_parts(), vec![0]);
    }
}
