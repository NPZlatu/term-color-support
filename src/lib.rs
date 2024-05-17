//! `term_color_support` is a library for detecting and managing color support in terminal environments.
//!
//! This crate provides modules for managing color support detection and information (`colors`),
//! fetching environment details (`environment`), and extracting color support level from
//! environment variables and command-line flags (`options`).
//!
//! The `ColorSupport` struct is re-exported for convenient access to color support detection
//! functionality.
//!

mod colors;
mod environment;
mod options;

pub use colors::ColorSupport;
