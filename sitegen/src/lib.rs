/// Utilities for site generation.
///
/// This crate exposes modules for data parsing and template rendering.
pub mod parser;
pub mod renderer;

pub use parser::{
    InlineStartError, RolesFile, month_from_en, month_from_ru, read_inline_start, read_roles,
};
pub use renderer::{format_duration_en, format_duration_ru};
