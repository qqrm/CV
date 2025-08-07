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

#[derive(Debug)]
pub enum RolesError {
    Io(io::Error),
    Parse(toml::de::Error),
    EmptyTitle(String),
}

impl fmt::Display for RolesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RolesError::Io(_) => write!(f, "failed to read roles.toml"),
            RolesError::Parse(_) => write!(f, "failed to parse roles.toml"),
            RolesError::EmptyTitle(slug) => write!(f, "role '{slug}' is missing a title"),
        }
    }
}

impl std::error::Error for RolesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RolesError::Io(err) => Some(err),
            RolesError::Parse(err) => Some(err),
            RolesError::EmptyTitle(_) => None,
        }
    }
}

pub use parser::{
    RolesFile, default_roles, month_from_en, month_from_ru, read_inline_start, read_roles,
};
pub use renderer::{format_duration_en, format_duration_ru};
