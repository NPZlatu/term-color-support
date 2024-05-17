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

use atty::{ Stream, is };

use crate::environment::Environment;
use crate::options::{
    OutputStreamOptions,
    has_flag,
    extract_force_color_level_from_env,
    extract_color_level_from_flags,
};

/// Enumeration representing the level of color support.
#[derive(Debug)]
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
#[derive(Debug)]
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
        let stdout_color_support_level: Option<ColorSupportLevel> = determine_stream_color_level(
            OutputStreamOptions::new(Some(is(Stream::Stdout)), None)
        );
        ColorInfo::new(stdout_color_support_level.unwrap_or(ColorSupportLevel::NoColor))
    }

    /// Detects and returns color support information for standard error stream.
    pub fn stderr() -> ColorInfo {
        let stderr_color_support_level: Option<ColorSupportLevel> = determine_stream_color_level(
            OutputStreamOptions::new(Some(is(Stream::Stderr)), None)
        );
        ColorInfo::new(stderr_color_support_level.unwrap_or(ColorSupportLevel::NoColor))
    }
}

/// Determines the color support level for a stream based on the provided options.
pub fn determine_stream_color_level(options: OutputStreamOptions) -> Option<ColorSupportLevel> {
    let force_color_level_from_env = extract_force_color_level_from_env();

    let mut color_level_from_flag: Option<ColorSupportLevel> = Some(ColorSupportLevel::NoColor);

    if force_color_level_from_env.is_none() {
        color_level_from_flag = extract_color_level_from_flags();
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
        if has_flag("color=16m") || has_flag("color=full") || has_flag("color=truecolor") {
            return Some(ColorSupportLevel::TrueColor);
        }
        if has_flag("color=256") {
            return Some(ColorSupportLevel::Colors256);
        }
    }

    if !options.is_tty && force_color.is_none() {
        return Some(ColorSupportLevel::NoColor);
    }

    let environment = Environment::new();
    Some(environment.determine_color_level())
}
