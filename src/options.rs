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
pub fn has_flag(flag: &str) -> bool {
    let args: Vec<String> = std::env::args().collect();

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
pub fn extract_color_level_from_flags() -> Option<ColorSupportLevel> {
    if
        has_flag("no-color") ||
        has_flag("no-colors") ||
        has_flag("color=false") ||
        has_flag("color=never")
    {
        Some(ColorSupportLevel::NoColor)
    } else if
        has_flag("color") ||
        has_flag("colors") ||
        has_flag("color=true") ||
        has_flag("color=always")
    {
        Some(ColorSupportLevel::Basic)
    } else {
        None
    }
}
