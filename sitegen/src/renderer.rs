/// Format a duration in months into a human readable English string.
///
/// The result uses singular and plural forms, e.g. "1 year 2 months".
pub fn format_duration_en(total_months: i32) -> String {
    let years = total_months / 12;
    let months = total_months % 12;
    let mut parts = Vec::new();
    if years > 0 {
        parts.push(match years {
            1 => "1 year".to_string(),
            _ => format!("{years} years"),
        });
    }
    if months > 0 {
        parts.push(match months {
            1 => "1 month".to_string(),
            _ => format!("{months} months"),
        });
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
            2..=4 => "года",
            _ => "лет",
        };
        parts.push(format!("{} {}", years, year_word));
    }
    if months > 0 {
        let month_word = match months {
            1 => "месяц",
            2..=4 => "месяца",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_en() {
        assert_eq!(format_duration_en(0), "0 months");
        assert_eq!(format_duration_en(1), "1 month");
        assert_eq!(format_duration_en(2), "2 months");
        assert_eq!(format_duration_en(12), "1 year");
        assert_eq!(format_duration_en(24), "2 years");
        assert_eq!(format_duration_en(14), "1 year 2 months");
    }

    #[test]
    fn test_format_duration_ru() {
        assert_eq!(format_duration_ru(0), "0 месяцев");
        assert_eq!(format_duration_ru(1), "1 месяц");
        assert_eq!(format_duration_ru(2), "2 месяца");
        assert_eq!(format_duration_ru(5), "5 месяцев");
        assert_eq!(format_duration_ru(12), "1 год");
        assert_eq!(format_duration_ru(24), "2 года");
        assert_eq!(format_duration_ru(60), "5 лет");
    }
}
