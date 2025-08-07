/// Utilities for site generation.
///
/// This crate exposes modules for data parsing and template rendering.
pub mod parser;
pub mod renderer;

use std::{fmt, io};

#[derive(Debug)]
pub enum InlineStartError {
    Io(io::Error),
    Parse,
}

impl fmt::Display for InlineStartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineStartError::Io(_) => write!(f, "failed to read cv.md"),
            InlineStartError::Parse => write!(f, "could not parse inline start"),
        }
    }
}

impl std::error::Error for InlineStartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InlineStartError::Io(err) => Some(err),
            InlineStartError::Parse => None,
        }
    }
}

impl From<io::Error> for InlineStartError {
    fn from(err: io::Error) -> Self {
        InlineStartError::Io(err)
    }
}

pub use parser::{RolesFile, month_from_en, month_from_ru, read_inline_start, read_roles};
pub use renderer::{format_duration_en, format_duration_ru};
