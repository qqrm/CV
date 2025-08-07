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
