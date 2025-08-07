use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use crate::InlineStartError;

/// Convert an English month name into its number.
///
/// Returns `Some(1)` for January through `Some(12)` for December,
/// or `None` if the name is unknown.
pub fn month_from_en(name: &str) -> Option<u32> {
    match name {
        "January" => Some(1),
        "February" => Some(2),
        "March" => Some(3),
        "April" => Some(4),
        "May" => Some(5),
        "June" => Some(6),
        "July" => Some(7),
        "August" => Some(8),
        "September" => Some(9),
        "October" => Some(10),
        "November" => Some(11),
        "December" => Some(12),
        _ => None,
    }
}

/// Convert a Russian month name into its number.
///
/// Returns `Some(1)` for "Январь" through `Some(12)` for "Декабрь",
/// or `None` if the name is unknown.
pub fn month_from_ru(name: &str) -> Option<u32> {
    match name {
        "Январь" => Some(1),
        "Февраль" => Some(2),
        "Март" => Some(3),
        "Апрель" => Some(4),
        "Май" => Some(5),
        "Июнь" => Some(6),
        "Июль" => Some(7),
        "Август" => Some(8),
        "Сентябрь" => Some(9),
        "Октябрь" => Some(10),
        "Ноябрь" => Some(11),
        "Декабрь" => Some(12),
        _ => None,
    }
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
                    let year: i32 = year_text
                        .parse()
                        .map_err(|_| InlineStartError::InvalidFormat)?;
                    if let Some(month) =
                        month_from_en(month_text).or_else(|| month_from_ru(month_text))
                    {
                        return Ok((year, month));
                    }
                }
            }
        }
    }
    Err(InlineStartError::InvalidFormat)
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
