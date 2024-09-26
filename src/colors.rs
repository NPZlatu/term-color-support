//! Module for managing color support detection and information.
//!
//! This module provides functionality to detect the color support level of the terminal,
//! determine color support for standard output and standard error streams, and create
//! `ColorInfo` structs representing the color support information.
//!
//! # Note
//!
//! This module relies on the `atty` crate for detecting whether standard output and
//! standard error streams are connected to a terminal.
//!

use std::io::{stdout, IsTerminal};

/// The module provides functionality to detect and manage color support information for terminal output
/// streams.
///
/// Arguments:
///
/// * `options`: The `options` parameter in the `determine_stream_color_level` function is of type
/// `OutputStreamOptions`. It contains information about the stream, such as whether it is a TTY
/// (terminal) and any sniffed flags related to color support. The function uses this information to
/// determine the color support
///
/// Returns:
///
/// The module provides functionality to detect the color support level of the terminal, determine color
/// support for standard output and standard error streams, and create `ColorInfo` structs representing
/// the color support information. It also includes unit tests for the module's functions.
use crate::environment::Environment;
use crate::options::{
    extract_color_level_from_flags, extract_force_color_level_from_env, has_flag,
    OutputStreamOptions,
};

/// Enumeration representing the level of color support.
#[derive(Debug, PartialEq)]
pub enum ColorSupportLevel {
    /// No color support.
    NoColor,
    /// Basic color support.
    Basic,
    /// Support for 256 colors.
    Colors256,
    /// True color support.
    TrueColor,
}

impl ColorSupportLevel {
    /// Converts a u32 value to a ColorSupportLevel.
    pub fn from_u32(level: u32) -> Option<ColorSupportLevel> {
        match level {
            0 => Some(ColorSupportLevel::NoColor),
            1 => Some(ColorSupportLevel::Basic),
            2 => Some(ColorSupportLevel::Colors256),
            3 => Some(ColorSupportLevel::TrueColor),
            _ => None,
        }
    }
}

/// Struct representing color support information.
#[derive(Debug, PartialEq)]
pub struct ColorInfo {
    /// The color support level.
    pub level: ColorSupportLevel,
    /// Indicates if basic color support is available.
    pub has_basic: bool,
    /// Indicates if 256-color support is available.
    pub has_256: bool,
    /// Indicates if true color support (16 million colors) is available.
    pub has_16m: bool,
}

impl ColorInfo {
    /// Creates a new ColorInfo instance based on the provided color support level.
    pub fn new(level: ColorSupportLevel) -> Self {
        let (has_basic, has_256, has_16m) = match level {
            ColorSupportLevel::NoColor => (false, false, false),
            ColorSupportLevel::Basic => (true, false, false),
            ColorSupportLevel::Colors256 => (true, true, false),
            ColorSupportLevel::TrueColor => (true, true, true),
        };

        ColorInfo {
            level,
            has_basic,
            has_256,
            has_16m,
        }
    }
}

/// Struct representing color support for standard output and standard error streams.
#[derive(Debug)]
pub struct ColorSupport {
    /// Color support information for standard output stream.
    pub stdout: ColorInfo,
    /// Color support information for standard error stream.
    pub stderr: ColorInfo,
}

impl ColorSupport {
    /// Detects and returns color support information for standard output stream.
    pub fn stdout() -> ColorInfo {
        let is_tty = stdout().is_terminal();
        let stdout_color_support_level: Option<ColorSupportLevel> =
            determine_stream_color_level(OutputStreamOptions::new(Some(is_tty), None));
        ColorInfo::new(stdout_color_support_level.unwrap_or(ColorSupportLevel::NoColor))
    }

    /// Detects and returns color support information for standard error stream.
    pub fn stderr() -> ColorInfo {
        let is_tty = stdout().is_terminal();
        let stderr_color_support_level: Option<ColorSupportLevel> =
            determine_stream_color_level(OutputStreamOptions::new(Some(is_tty), None));
        ColorInfo::new(stderr_color_support_level.unwrap_or(ColorSupportLevel::NoColor))
    }
}

/// Determines the color support level for a stream based on the provided options.
pub fn determine_stream_color_level(options: OutputStreamOptions) -> Option<ColorSupportLevel> {
    let args = std::env::args().collect::<Vec<String>>();

    let force_color_level_from_env = extract_force_color_level_from_env();

    let mut color_level_from_flag: Option<ColorSupportLevel> = Some(ColorSupportLevel::NoColor);

    if force_color_level_from_env.is_none() {
        color_level_from_flag = extract_color_level_from_flags(&args);
    }

    let force_color = if options.sniff_flags == true {
        color_level_from_flag
    } else {
        force_color_level_from_env
    };

    if force_color.is_some() {
        return force_color;
    }

    if options.sniff_flags {
        if has_flag("color=16m", &args)
            || has_flag("color=full", &args)
            || has_flag("color=truecolor", &args)
        {
            return Some(ColorSupportLevel::TrueColor);
        }
        if has_flag("color=256", &args) {
            return Some(ColorSupportLevel::Colors256);
        }
    }

    if !options.is_tty && force_color.is_none() {
        return Some(ColorSupportLevel::NoColor);
    }

    let environment = Environment::default();
    Some(environment.determine_color_level())
}

/// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_support_level_from_u32() {
        assert_eq!(
            ColorSupportLevel::from_u32(0),
            Some(ColorSupportLevel::NoColor)
        );
        assert_eq!(
            ColorSupportLevel::from_u32(1),
            Some(ColorSupportLevel::Basic)
        );
        assert_eq!(
            ColorSupportLevel::from_u32(2),
            Some(ColorSupportLevel::Colors256)
        );
        assert_eq!(
            ColorSupportLevel::from_u32(3),
            Some(ColorSupportLevel::TrueColor)
        );
        assert_eq!(ColorSupportLevel::from_u32(4), None);
    }

    #[test]
    fn test_color_info_new() {
        let color_info = ColorInfo::new(ColorSupportLevel::Basic);
        assert_eq!(color_info.level, ColorSupportLevel::Basic);
        assert_eq!(color_info.has_basic, true);
        assert_eq!(color_info.has_256, false);
        assert_eq!(color_info.has_16m, false);
    }

    #[test]
    fn test_color_support_stderr() {
        // As we don't have control over the actual terminal, we'll just test if the function runs without error
        let _ = ColorSupport::stderr();
    }

    #[test]
    fn test_determine_stream_color_level() {
        // As we don't have control over the actual terminal, we'll just test if the function runs without error
        let _ = determine_stream_color_level(OutputStreamOptions::new(Some(false), None));
    }

    /// Tests the detection of color support for standard output stream.
    #[test]
    fn test_color_support_stdout() {
        // As we don't have control over the actual terminal, we'll just test if the function runs without error
        let _ = ColorSupport::stdout();
    }

    /// Tests if ColorSupportLevel enum variants are comparable for equality.
    #[test]
    fn test_color_support_level_equality() {
        assert_eq!(ColorSupportLevel::NoColor, ColorSupportLevel::NoColor);
        assert_eq!(ColorSupportLevel::Basic, ColorSupportLevel::Basic);
        assert_eq!(ColorSupportLevel::Colors256, ColorSupportLevel::Colors256);
        assert_eq!(ColorSupportLevel::TrueColor, ColorSupportLevel::TrueColor);
    }

    /// Tests the equality of ColorInfo instances.
    #[test]
    fn test_color_info_equality() {
        let color_info1 = ColorInfo::new(ColorSupportLevel::Basic);
        let color_info2 = ColorInfo::new(ColorSupportLevel::Basic);
        assert_eq!(color_info1, color_info2);
    }

    /// Tests if ColorInfo instances with different color support levels are not equal.
    #[test]
    fn test_color_info_inequality() {
        let color_info1 = ColorInfo::new(ColorSupportLevel::Basic);
        let color_info2 = ColorInfo::new(ColorSupportLevel::TrueColor);
        assert_ne!(color_info1, color_info2);
    }
}
