/// Utilities for site generation.
///
/// This module provides helpers for month name parsing,
/// extracting the start date of the most recent CV entry,
/// formatting durations and reading role definitions.
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::sync::LazyLock;

/// English month lookup table.
static MONTHS_EN: LazyLock<BTreeMap<&'static str, u32>> = LazyLock::new(|| {
    BTreeMap::from([
        ("January", 1),
        ("February", 2),
        ("March", 3),
        ("April", 4),
        ("May", 5),
        ("June", 6),
        ("July", 7),
        ("August", 8),
        ("September", 9),
        ("October", 10),
        ("November", 11),
        ("December", 12),
    ])
});

/// Russian month lookup table.
static MONTHS_RU: LazyLock<BTreeMap<&'static str, u32>> = LazyLock::new(|| {
    BTreeMap::from([
        ("Январь", 1),
        ("Февраль", 2),
        ("Март", 3),
        ("Апрель", 4),
        ("Май", 5),
        ("Июнь", 6),
        ("Июль", 7),
        ("Август", 8),
        ("Сентябрь", 9),
        ("Октябрь", 10),
        ("Ноябрь", 11),
        ("Декабрь", 12),
    ])
});

/// Convert an English month name into its number.
///
/// Returns `Some(1)` for January through `Some(12)` for December,
/// or `None` if the name is unknown.
pub fn month_from_en(name: &str) -> Option<u32> {
    MONTHS_EN.get(name).copied()
}

/// Convert a Russian month name into its number.
///
/// Returns `Some(1)` for "Январь" through `Some(12)` for "Декабрь",
/// or `None` if the name is unknown.
pub fn month_from_ru(name: &str) -> Option<u32> {
    MONTHS_RU.get(name).copied()
}

/// Read the starting month and year of the most recent CV entry.
///
/// The function expects a `cv.md` file in the current directory and
/// looks for a list item starting with the month and year followed by
/// an en dash or em dash and the word "Present" (English) or
/// "Настоящее время" (Russian).
///
/// Returns a pair `(year, month)` on success.
pub fn read_inline_start() -> Option<(i32, u32)> {
    let content = std::fs::read_to_string("cv.md").ok()?;
    for line in content.lines() {
        if let Some((month_str, year_str)) = line
            .trim()
            .strip_prefix('*')
            .and_then(|s| s.split_once('–'))
            .or_else(|| {
                line.trim()
                    .strip_prefix('*')
                    .and_then(|s| s.split_once('—'))
            })
        {
            let year_str = year_str.trim();
            if year_str.starts_with("Present") || year_str.starts_with("Настоящее время")
            {
                let parts: Vec<&str> = month_str.trim().split_whitespace().collect();
                if parts.len() == 2 {
                    let (month_text, year_text) = (parts[0], parts[1]);
                    let year: i32 = year_text.parse().ok()?;
                    if let Some(month) =
                        month_from_en(month_text).or_else(|| month_from_ru(month_text))
                    {
                        return Some((year, month));
                    }
                }
            }
        }
    }
    None
}

/// Format a duration in months into a human readable English string.
///
/// The result uses singular and plural forms, e.g. "1 year 2 months".
pub fn format_duration_en(total_months: i32) -> String {
    let years = total_months / 12;
    let months = total_months % 12;
    let mut parts = Vec::new();
    if years > 0 {
        if years == 1 {
            parts.push("1 year".to_string());
        } else {
            parts.push(format!("{} years", years));
        }
    }
    if months > 0 {
        if months == 1 {
            parts.push("1 month".to_string());
        } else {
            parts.push(format!("{} months", months));
        }
    }
    if parts.is_empty() {
        "0 months".to_string()
    } else {
        parts.join(" ")
    }
}

/// Format a duration in months into a human readable Russian string.
///
/// The result uses correct declensions, e.g. "1 год 2 месяца".
pub fn format_duration_ru(total_months: i32) -> String {
    let years = total_months / 12;
    let months = total_months % 12;
    let mut parts = Vec::new();
    if years > 0 {
        let year_word = match years {
            1 => "год",
            2 | 3 | 4 => "года",
            _ => "лет",
        };
        parts.push(format!("{} {}", years, year_word));
    }
    if months > 0 {
        let month_word = match months {
            1 => "месяц",
            2 | 3 | 4 => "месяца",
            _ => "месяцев",
        };
        parts.push(format!("{} {}", months, month_word));
    }
    if parts.is_empty() {
        "0 месяцев".to_string()
    } else {
        parts.join(" ")
    }
}

#[derive(Deserialize)]
pub struct RolesFile {
    /// Mapping from role slug to human readable title.
    pub roles: BTreeMap<String, String>,
}

/// Read role definitions from `roles.toml` if present.
///
/// Returns a map of role slugs to titles. When the file is missing or
/// invalid, a small default set is returned instead.
pub fn read_roles() -> BTreeMap<String, String> {
    fs::read_to_string("roles.toml")
        .ok()
        .and_then(|text| toml::from_str::<RolesFile>(&text).ok())
        .map(|r| r.roles)
        .unwrap_or_else(|| {
            BTreeMap::from([
                ("tl".to_string(), "Team Lead".to_string()),
                ("tech".to_string(), "Tech Lead".to_string()),
            ])
        })
}
