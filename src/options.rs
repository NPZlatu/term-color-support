//! Module for managing output stream options and color level extraction from environment and flags.
//!
//! This module provides functionality for managing output stream options, extracting color support
//! level information from environment variables and command-line flags.
//!
//! The `OutputStreamOptions` struct represents the options for output streams, including whether
//! the stream is a TTY and whether to sniff flags.
//!
//! The `has_flag` function checks whether a given command-line flag is present.
//!
//! The `extract_force_color_level_from_env` function extracts the color support level from the
//! `FORCE_COLOR` environment variable.
//!
//! The `extract_color_level_from_flags` function extracts the color support level from command-line
//! flags such as `--color` or `--no-color`.

use crate::colors::ColorSupportLevel;

/// Struct representing the options for output streams.

pub struct OutputStreamOptions {
    /// Specifies whether the output stream is a TTY.
    pub is_tty: bool,
    /// Specifies whether to sniff flags.
    pub sniff_flags: bool,
}

impl OutputStreamOptions {
    /// Creates a new `OutputStreamOptions` instance with optional parameters.
    pub fn new(is_tty: Option<bool>, sniff_flags: Option<bool>) -> Self {
        OutputStreamOptions {
            is_tty: is_tty.unwrap_or(false),
            sniff_flags: sniff_flags.unwrap_or(true),
        }
    }
}

/// Checks whether a given command-line flag is present.
pub fn has_flag(flag: &str, args: &Vec<String>) -> bool {
    let flag_without_dashes = flag.trim_start_matches('-');

    args.iter().any(|arg| {
        let normalized_arg = arg.trim_start_matches('-').to_lowercase();
        normalized_arg == flag_without_dashes
    })
}

/// Extracts the color support level from the `FORCE_COLOR` environment variable.
pub fn extract_force_color_level_from_env() -> Option<ColorSupportLevel> {
    if let Ok(force_color) = std::env::var("FORCE_COLOR") {
        if force_color == "true" {
            return Some(ColorSupportLevel::Basic);
        }
        if force_color == "false" {
            return Some(ColorSupportLevel::NoColor);
        }
        if force_color.is_empty() {
            return Some(ColorSupportLevel::Basic);
        }
        if let Ok(level) = force_color.parse::<u32>() {
            return ColorSupportLevel::from_u32(level);
        }
    }
    None
}

/// Extracts the color support level from command-line flags.
pub fn extract_color_level_from_flags(args: &Vec<String>) -> Option<ColorSupportLevel> {
    if has_flag("no-color", &args)
        || has_flag("no-colors", &args)
        || has_flag("color=false", &args)
        || has_flag("color=never", &args)
    {
        Some(ColorSupportLevel::NoColor)
    } else if has_flag("color", &args)
        || has_flag("colors", &args)
        || has_flag("color=true", &args)
        || has_flag("color=always", &args)
    {
        Some(ColorSupportLevel::Basic)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_output_stream_options_default() {
        let options = OutputStreamOptions::new(None, None);
        assert_eq!(options.is_tty, false);
        assert_eq!(options.sniff_flags, true);
    }

    #[test]
    fn test_new_output_stream_options_custom() {
        let options = OutputStreamOptions::new(Some(true), Some(false));
        assert_eq!(options.is_tty, true);
        assert_eq!(options.sniff_flags, false);
    }

    #[test]
    fn test_has_flag_present_no_args() {
        let arguments = vec![];
        let flag_present = has_flag("--color", &arguments);
        assert!(!flag_present);
    }

    #[test]
    fn test_has_flag_with_existing_flag() {
        let args = vec![
            String::from("-verbose"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("-verbose", &args));
    }

    #[test]
    fn test_has_flag_with_existing_flag_case_insensitive() {
        let args = vec![
            String::from("-Verbose"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("-verbose", &args));
    }

    #[test]
    fn test_has_flag_without_dashes() {
        let args = vec![
            String::from("verbose"),
            String::from("output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("-verbose", &args));
    }

    #[test]
    fn test_has_flag_with_multiple_args() {
        let args = vec![
            String::from("-verbose"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("-output", &args));
    }

    #[test]
    fn test_has_flag_with_non_existing_flag() {
        let args = vec![
            String::from("-debug"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(!has_flag("-verbose", &args));
    }

    #[test]
    fn test_has_flag_with_empty_args() {
        let args: Vec<String> = vec![];
        assert!(!has_flag("-verbose", &args));
    }

    #[test]
    fn test_has_flag_with_empty_flag() {
        let args = vec![
            String::from("-verbose"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(!has_flag("", &args));
    }

    #[test]
    fn test_has_flag_with_no_dash_in_flag() {
        let args = vec![
            String::from("verbose"),
            String::from("output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("verbose", &args));
    }

    #[test]
    fn test_has_flag_with_no_dash_in_flag_but_with_dash_in_args() {
        let args = vec![
            String::from("-verbose"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("verbose", &args));
    }

    #[test]
    fn test_has_flag_with_no_dash_in_flag_but_with_double_dash_in_args() {
        let args = vec![
            String::from("--verbose"),
            String::from("--output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("verbose", &args));
    }

    #[test]
    fn test_has_flag_with_double_dash_in_flag() {
        let args = vec![
            String::from("-verbose"),
            String::from("-output"),
            String::from("file.txt"),
        ];
        assert!(has_flag("--verbose", &args));
    }

    #[test]
    fn test_extract_force_color_level_from_env_true() {
        temp_env::with_var("FORCE_COLOR", Some("true"), || {
            assert_eq!(
                extract_force_color_level_from_env(),
                Some(ColorSupportLevel::Basic)
            );
        });
    }

    #[test]
    fn test_extract_force_color_level_from_env_false() {
        temp_env::with_var("FORCE_COLOR", Some("false"), || {
            assert_eq!(
                extract_force_color_level_from_env(),
                Some(ColorSupportLevel::NoColor)
            );
        });
    }

    #[test]
    fn test_extract_force_color_level_from_env_empty() {
        temp_env::with_var("FORCE_COLOR", Some(String::from("")), || {
            assert_eq!(
                extract_force_color_level_from_env(),
                Some(ColorSupportLevel::Basic)
            );
        });
    }

    #[test]
    fn test_extract_force_color_level_from_env_valid_integer() {
        temp_env::with_var("FORCE_COLOR", Some("2"), || {
            assert_eq!(
                extract_force_color_level_from_env(),
                Some(ColorSupportLevel::Colors256)
            );
        });
    }

    #[test]
    fn test_extract_force_color_level_from_env_invalid_integer() {
        temp_env::with_var("FORCE_COLOR", Some("not_an_integer"), || {
            assert_eq!(extract_force_color_level_from_env(), None);
        });
    }

    #[test]
    fn test_extract_color_level_from_flags_no_color_flags() {
        let args = vec![String::from("program_name"), String::from("--no-color")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::NoColor)
        );
    }

    #[test]
    fn test_extract_color_level_from_flags_color_false_flags() {
        let args = vec![String::from("program_name"), String::from("--color=false")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::NoColor)
        );
    }

    #[test]
    fn test_extract_color_level_from_flags_color_never_flags() {
        let args = vec![String::from("program_name"), String::from("--color=never")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::NoColor)
        );
    }

    // Test cases for flags indicating basic color support.
    #[test]
    fn test_extract_color_level_from_flags_color_flags() {
        let args = vec![String::from("program_name"), String::from("--color")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::Basic)
        );
    }

    #[test]
    fn test_extract_color_level_from_flags_colors_flags() {
        let args = vec![String::from("program_name"), String::from("--colors")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::Basic)
        );
    }

    #[test]
    fn test_extract_color_level_from_flags_color_true_flags() {
        let args = vec![String::from("program_name"), String::from("--color=true")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::Basic)
        );
    }

    #[test]
    fn test_extract_color_level_from_flags_color_always_flags() {
        let args = vec![String::from("program_name"), String::from("--color=always")];
        assert_eq!(
            extract_color_level_from_flags(&args),
            Some(ColorSupportLevel::Basic)
        );
    }

    // Test case when no relevant flags are present.
    #[test]
    fn test_extract_color_level_from_flags_no_flags() {
        let args = vec![String::from("program_name")];
        assert_eq!(extract_color_level_from_flags(&args), None);
    }
}
