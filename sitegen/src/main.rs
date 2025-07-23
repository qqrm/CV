use pulldown_cmark::{html::push_html, Options, Parser};
use std::fs;
use std::path::Path;
use chrono::{Datelike, NaiveDate, Utc};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const AVATAR_SRC_EN: &str = "avatar.jpg";
    const AVATAR_SRC_RU: &str = "../avatar.jpg";
    const INLINE_START: (i32, u32) = (2024, 3);

    let start_date = NaiveDate::from_ymd_opt(INLINE_START.0, INLINE_START.1, 1)
        .expect("Invalid start date");
    let today = Utc::now().date_naive();
    let total_months = (today.year() - start_date.year()) * 12
        + (today.month() as i32 - start_date.month() as i32);
    let duration_en = format_duration_en(total_months);
    let duration_ru = format_duration_ru(total_months);
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
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<header>\n    <h1>Alexey Belyakov</h1>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n{}\n</div>\n<footer>\n    <p><a href='latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n</footer>\n</body>\n</html>\n",
        AVATAR_SRC_EN, html_body
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
        "<!DOCTYPE html>\n<html lang='ru'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Алексей Беляков - Резюме</title>\n    <link rel='stylesheet' href='../style.css'>\n</head>\n<body>\n<header>\n    <h1>Алексей Беляков</h1>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n{}\n</div>\n<footer>\n    <p><a href='../latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='../latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n</footer>\n</body>\n</html>\n",
        AVATAR_SRC_RU, html_body_ru
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
