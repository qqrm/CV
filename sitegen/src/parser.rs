use log::{debug, info, warn};
use phf::phf_map;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum InlineStartError {
    Io(io::Error),
    Parse,
}

impl fmt::Display for InlineStartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineStartError::Io(_) => write!(f, "failed to read CV.MD"),
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

static EN_MONTHS: phf::Map<&'static str, u32> = phf_map! {
    "January" => 1,
    "February" => 2,
    "March" => 3,
    "April" => 4,
    "May" => 5,
    "June" => 6,
    "July" => 7,
    "August" => 8,
    "September" => 9,
    "October" => 10,
    "November" => 11,
    "December" => 12,
};

static RU_MONTHS: phf::Map<&'static str, u32> = phf_map! {
    "Январь" => 1,
    "Февраль" => 2,
    "Март" => 3,
    "Апрель" => 4,
    "Май" => 5,
    "Июнь" => 6,
    "Июль" => 7,
    "Август" => 8,
    "Сентябрь" => 9,
    "Октябрь" => 10,
    "Ноябрь" => 11,
    "Декабрь" => 12,
};

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
/// The function expects a `CV.MD` file in the current directory and
/// looks for a list item starting with the month and year followed by
/// an en dash or em dash and the word "Present" (English) or
/// "Настоящее время" (Russian).
///
/// Returns a pair `(year, month)` on success.
pub fn read_inline_start() -> Result<(i32, u32), InlineStartError> {
    debug!("Scanning CV.MD for inline start");
    let content = std::fs::read_to_string("CV.MD")?;
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
                let parts: Vec<&str> = month_str.split_whitespace().collect();
                if parts.len() == 2 {
                    let (month_text, year_text) = (parts[0], parts[1]);
                    let year: i32 = year_text.parse().map_err(|_| InlineStartError::Parse)?;
                    if let Some(month) =
                        month_from_en(month_text).or_else(|| month_from_ru(month_text))
                    {
                        info!("Inline start parsed: {month_text} {year}");
                        return Ok((year, month));
                    }
                }
            }
        }
    }
    warn!("Failed to parse inline start from CV.MD");
    Err(InlineStartError::Parse)
}

#[derive(Deserialize)]
pub struct RolesFile {
    /// Mapping from role slug to human readable title.
    #[serde(default)]
    pub roles: BTreeMap<String, String>,
}

fn default_roles() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("tl".to_string(), "Team Lead".to_string()),
        ("tech".to_string(), "Tech Lead".to_string()),
    ])
}

#[derive(Debug)]
pub enum RolesError {
    Io(io::Error),
    Parse(toml::de::Error),
    EmptyTitle { slug: String },
}

impl fmt::Display for RolesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RolesError::Io(_) => write!(f, "failed to read roles.toml"),
            RolesError::Parse(_) => write!(f, "could not parse roles.toml"),
            RolesError::EmptyTitle { slug } => {
                write!(f, "role '{slug}' has empty title")
            }
        }
    }
}

impl std::error::Error for RolesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RolesError::Io(err) => Some(err),
            RolesError::Parse(err) => Some(err),
            RolesError::EmptyTitle { .. } => None,
        }
    }
}

impl From<io::Error> for RolesError {
    fn from(err: io::Error) -> Self {
        RolesError::Io(err)
    }
}

/// Read role definitions from `roles.toml` if present.
///
/// Returns a map of role slugs to titles. Missing files fall back to a
/// default set. Invalid entries produce descriptive errors.
pub fn read_roles() -> Result<BTreeMap<String, String>, RolesError> {
    debug!("Loading roles from roles.toml");
    let defaults = default_roles();

    let content = match fs::read_to_string("roles.toml") {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            warn!("roles.toml not found, using defaults");
            return Ok(defaults);
        }
        Err(e) => return Err(RolesError::Io(e)),
    };

    let parsed: RolesFile = toml::from_str(&content).map_err(RolesError::Parse)?;

    for (slug, title) in &parsed.roles {
        if title.trim().is_empty() {
            return Err(RolesError::EmptyTitle { slug: slug.clone() });
        }
    }

    let mut roles = defaults;
    roles.extend(parsed.roles);
    info!("Loaded {} roles", roles.len());
    Ok(roles)
}

#[cfg(test)]
mod tests {
    use super::{month_from_en, month_from_ru};

    #[test]
    fn parses_english_months() {
        let months = [
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
        ];
        for (name, number) in months {
            assert_eq!(month_from_en(name), Some(number));
        }
    }

    #[test]
    fn parses_russian_months() {
        let months = [
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
        ];
        for (name, number) in months {
            assert_eq!(month_from_ru(name), Some(number));
        }
    }

    #[test]
    fn unknown_months_return_none() {
        assert_eq!(month_from_en("Smarch"), None);
        assert_eq!(month_from_ru("Смарч"), None);
    }
}
