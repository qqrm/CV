use log::{debug, info, warn};
use phf::phf_map;
use std::fmt;
use std::io;

const CV_EN_PATH: &str = "profiles/cv/en/CV.MD";

#[derive(Debug)]
pub enum InlineStartError {
    Io(io::Error),
    Parse,
}

impl fmt::Display for InlineStartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineStartError::Io(_) => write!(f, "failed to read {CV_EN_PATH}"),
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
/// The function expects the English CV Markdown at
/// `profiles/cv/en/CV.MD` and
/// looks for a list item starting with the month and year followed by
/// an en dash or em dash and the word "Present" (English) or
/// "Настоящее время" (Russian).
///
/// Returns a pair `(year, month)` on success.
pub fn read_inline_start() -> Result<(i32, u32), InlineStartError> {
    debug!("Scanning {CV_EN_PATH} for inline start");
    let content = std::fs::read_to_string(CV_EN_PATH)?;
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
    warn!("Failed to parse inline start from {CV_EN_PATH}");
    Err(InlineStartError::Parse)
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
