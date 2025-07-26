use pulldown_cmark::{html::push_html, Options, Parser};
use std::fs;
use std::path::Path;
use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;

fn format_duration_en(total_months: i32) -> String {
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

fn format_duration_ru(total_months: i32) -> String {
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

fn month_from_english(month: &str) -> Option<u32> {
    match month {
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

fn read_inline_start() -> Option<(i32, u32)> {
    let content = fs::read_to_string("README.md").ok()?;
    let re = Regex::new(r"\*([A-Za-z]+)\s+(\d{4})\s+–\s+Present").ok()?;
    let caps = re.captures(&content)?;
    let month = month_from_english(&caps[1])?;
    let year: i32 = caps[2].parse().ok()?;
    Some((year, month))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const AVATAR_SRC_EN: &str = "avatar.jpg";
    const AVATAR_SRC_RU: &str = "../avatar.jpg";
    const INLINE_START: (i32, u32) = (2024, 3);

    let inline_start = read_inline_start().unwrap_or(INLINE_START);
    let start_date = NaiveDate::from_ymd_opt(inline_start.0, inline_start.1, 1)
        .expect("Invalid start date");
    let today = Utc::now().date_naive();
    let total_months = (today.year() - start_date.year()) * 12
        + (today.month() as i32 - start_date.month() as i32);
    let duration_en = format_duration_en(total_months);
    let duration_ru = format_duration_ru(total_months);
    let date_str = today.format("%Y-%m-%d").to_string();
    // Generate English version
    let markdown_input = fs::read_to_string("README.md")?;
    let parser = Parser::new_ext(&markdown_input, Options::all());
    let mut html_body = String::new();
    push_html(&mut html_body, parser);
    html_body = html_body.replace("./latex/", "latex/");
    html_body = html_body.replace("./README_ru.md", "ru/");
    html_body = html_body.replace(
        "March 2024 – Present  (1 year)",
        &format!("March 2024 – Present  ({})", duration_en),
    );
    if let Some(end) = html_body.find("</h1>") {
        html_body = html_body[end + 5..].trim_start().to_string();
    }

    let html_template = format!(
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<header>\n    <h1>Alexey Belyakov</h1>\n    <p><strong>Rust Team Lead</strong></p>\n    <p>{}</p>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n{}\n</div>\n<footer>\n    <p><a href='latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='typst/en/Belyakov_en.pdf'>Download PDF (Typst EN)</a></p>\n    <p><a href='latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n    <p><a href='typst/ru/Belyakov_ru.pdf'>Скачать PDF (Typst RU)</a></p>\n</footer>\n</body>\n</html>\n",
        date_str, AVATAR_SRC_EN, html_body
    );

    // Generate Russian version
    let markdown_ru = fs::read_to_string("README_ru.md")?;
    let parser_ru = Parser::new_ext(&markdown_ru, Options::all());
    let mut html_body_ru = String::new();
    push_html(&mut html_body_ru, parser_ru);
    html_body_ru = html_body_ru.replace("./latex/", "../latex/");
    html_body_ru = html_body_ru.replace(
        "март 2024 – настоящее время (около 1 года)",
        &format!("март 2024 – настоящее время ({})", duration_ru),
    );
    if let Some(end) = html_body_ru.find("</h1>") {
        html_body_ru = html_body_ru[end + 5..].trim_start().to_string();
    }

    let html_template_ru = format!(
        "<!DOCTYPE html>\n<html lang='ru'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Алексей Беляков - Резюме</title>\n    <link rel='stylesheet' href='../style.css'>\n</head>\n<body>\n<header>\n    <h1>Алексей Беляков</h1>\n    <p><strong>Rust Team Lead</strong></p>\n    <p>{}</p>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n<p><em><a href='../'>Ссылка на английскую версию</a></em><br /><em><a href='../latex/ru/Belyakov_ru.pdf'>Ссылка на PDF</a></em><br /><em><a href='../latex/en/Belyakov_en.pdf'>Ссылка на английский PDF</a></em></p>\n{}\n</div>\n<footer>\n    <p><a href='../latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='../typst/en/Belyakov_en.pdf'>Download PDF (Typst EN)</a></p>\n    <p><a href='../latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n    <p><a href='../typst/ru/Belyakov_ru.pdf'>Скачать PDF (Typst RU)</a></p>\n</footer>\n</body>\n</html>\n",
        date_str, AVATAR_SRC_RU, html_body_ru
    );

    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    fs::copy("content/avatar.jpg", docs_dir.join("avatar.jpg"))?;
    fs::write(docs_dir.join("index.html"), html_template)?;

    let ru_dir = docs_dir.join("ru");
    if !ru_dir.exists() {
        fs::create_dir_all(&ru_dir)?;
    }
    fs::write(ru_dir.join("index.html"), html_template_ru)?;

    Ok(())
}
