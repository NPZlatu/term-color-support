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

use os_info;
use crate::colors::ColorSupportLevel;

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
    /// Creates a new `Environment` instance.
    pub fn new() -> Self {
        let binding = os_info::get();
        let os_release = binding.version();

        Self {
            term: std::env::var("TERM").unwrap_or_else(|_| String::from("")),
            colorterm: std::env::var("COLORTERM").ok(),
            teamcity_version: std::env::var("TEAMCITY_VERSION").ok(),
            ci: std::env::var("CI").ok(),
            os_release: os_release.to_string(),
            term_program: std::env::var("TERM_PROGRAM").ok(),
            term_program_version: std::env
                ::var("TERM_PROGRAM_VERSION")
                .unwrap_or_else(|_| String::from("")),
        }
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
        self.term_program_version
            .split('.')
            .next()
            .and_then(|major| major.parse().ok())
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

        if let Some(teamcity_version) = &self.teamcity_version {
            if teamcity_version.starts_with("9.") || teamcity_version.starts_with(char::is_numeric) {
                return ColorSupportLevel::Basic;
            }
            return ColorSupportLevel::NoColor;
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

        if
            self.term.starts_with("screen") ||
            self.term.starts_with("xterm") ||
            self.term.starts_with("vt100") ||
            self.term.starts_with("vt220") ||
            self.term.starts_with("rxvt") ||
            self.term.contains("color") ||
            self.term.contains("ansi") ||
            self.term.contains("cygwin") ||
            self.term.contains("linux")
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
