/// Utilities for site generation.
///
/// This crate exposes modules for data parsing and template rendering.
pub mod parser;
pub mod renderer;

use std::fmt;

/// Errors that may occur when reading inline CV start information.
#[derive(Debug)]
pub enum InlineStartError {
    /// Failed to read the `cv.md` file.
    Io(std::io::Error),
    /// The file did not contain a recognizable inline start entry.
    InvalidFormat,
}

impl fmt::Display for InlineStartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineStartError::Io(e) => write!(f, "I/O error: {e}"),
            InlineStartError::InvalidFormat => write!(f, "invalid inline start format"),
        }
    }
}

impl std::error::Error for InlineStartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InlineStartError::Io(e) => Some(e),
            InlineStartError::InvalidFormat => None,
        }
    }
}

pub use parser::{RolesFile, month_from_en, month_from_ru, read_inline_start, read_roles};
pub use renderer::{format_duration_en, format_duration_ru};
