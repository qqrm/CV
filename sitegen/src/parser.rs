use crate::{InlineStartError, RolesError};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::sync::LazyLock;

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

/// Default role mappings used when `roles.toml` is missing or does not
/// define any roles.
pub fn default_roles() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("tl".to_string(), "Team Lead".to_string()),
        ("tech".to_string(), "Tech Lead".to_string()),
    ])
}

#[derive(Deserialize)]
pub struct RolesFile {
    /// Mapping from role slug to human readable title.
    #[serde(default = "default_roles")]
    pub roles: BTreeMap<String, String>,
}

/// Read role definitions from `roles.toml` if present.
///
/// Returns a map of role slugs to titles. When the file is missing, a
/// small default set is returned instead. Invalid entries produce an
/// error containing the offending slug.
pub fn read_roles() -> Result<BTreeMap<String, String>, RolesError> {
    let text = match fs::read_to_string("roles.toml") {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(default_roles()),
        Err(e) => return Err(RolesError::Io(e)),
    };
    let RolesFile { roles } = toml::from_str::<RolesFile>(&text).map_err(RolesError::Parse)?;
    for (slug, title) in &roles {
        if title.trim().is_empty() {
            return Err(RolesError::EmptyTitle(slug.clone()));
        }
    }
    Ok(roles)
}
