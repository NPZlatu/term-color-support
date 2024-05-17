//! `term_color_support` is a library for detecting and managing color support in terminal environments.
//!
//! This crate provides modules for managing color support detection and information (`colors`),
//! fetching environment details (`environment`), and extracting color support level from
//! environment variables and command-line flags (`options`).
//!
//! The `ColorSupport` struct is re-exported for convenient access to color support detection
//! functionality.
//!
//! # Example
//!
//! ```rust
//! use term_color_support::ColorSupport;
//!
//! fn main() {
//!     // Detect and print color support for stdout
//!     println!("Color support for stdout: {:?}", ColorSupport::stdout());
//!
//!     // Detect and print color support for stderr
//!     println!("Color support for stderr: {:?}", ColorSupport::stderr());
//! }
//! ```

mod colors;
mod environment;
mod options;

pub use colors::ColorSupport;
