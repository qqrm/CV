use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::sync::LazyLock;
use std::{fmt, io};

/// Errors that can occur while parsing the CV start marker.
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

static EN_MONTHS: LazyLock<BTreeMap<&'static str, u32>> = LazyLock::new(|| {
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

static RU_MONTHS: LazyLock<BTreeMap<&'static str, u32>> = LazyLock::new(|| {
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
    EN_MONTHS.get(name).copied()
}

/// Convert a Russian month name into its number.
///
/// Returns `Some(1)` for "Январь" through `Some(12)` for "Декабрь",
/// or `None` if the name is unknown.
pub fn month_from_ru(name: &str) -> Option<u32> {
    RU_MONTHS.get(name).copied()
}

/// Read the starting month and year of the most recent CV entry.
///
/// The function expects a `cv.md` file in the current directory and
/// looks for a list item starting with the month and year followed by
/// an en dash or em dash and the word "Present" (English) or
/// "Настоящее время" (Russian).
///
/// Returns a pair `(year, month)` on success.
pub fn read_inline_start() -> Result<(i32, u32), InlineStartError> {
    let content = std::fs::read_to_string("cv.md").map_err(InlineStartError::Io)?;
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
                    let year: i32 = year_text.parse().map_err(|_| InlineStartError::Parse)?;
                    if let Some(month) =
                        month_from_en(month_text).or_else(|| month_from_ru(month_text))
                    {
                        return Ok((year, month));
                    }
                }
            }
        }
    }
    Err(InlineStartError::Parse)
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
