use chrono::{Datelike, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use pulldown_cmark::{Options, Parser as CmarkParser, html::push_html};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check Markdown and TOML for consistency
    Validate,
    /// Generate PDFs and HTML into /dist
    Generate,
}

fn month_from_en(name: &str) -> Option<u32> {
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

fn month_from_ru(name: &str) -> Option<u32> {
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

fn read_inline_start() -> Option<(i32, u32)> {
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

#[derive(Deserialize)]
struct RolesFile {
    roles: BTreeMap<String, String>,
}

fn read_roles() -> BTreeMap<String, String> {
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

fn validate() -> Result<(), Box<dyn std::error::Error>> {
    fs::read_to_string("cv.md")?;
    fs::read_to_string("cv.ru.md")?;
    let content = fs::read_to_string("roles.toml")?;
    toml::from_str::<RolesFile>(&content)?;
    println!("Validation successful");
    Ok(())
}

fn generate() -> Result<(), Box<dyn std::error::Error>> {
    const AVATAR_SRC_EN: &str = "avatar.jpg";
    const AVATAR_SRC_RU: &str = "../avatar.jpg";
    const INLINE_START: (i32, u32) = (2024, 3);
    let inline_start = read_inline_start().unwrap_or(INLINE_START);
    let roles = read_roles();
    // Build base PDFs
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
    }
    Command::new("typst")
        .args([
            "compile",
            "typst/en/Belyakov_en.typ",
            "dist/Belyakov_en.pdf",
        ])
        .status()?;
    Command::new("typst")
        .args([
            "compile",
            "typst/ru/Belyakov_ru.typ",
            "dist/Belyakov_ru.pdf",
        ])
        .status()?;
    let template_en = fs::read_to_string("typst/en/Belyakov_en.typ")?;
    let template_ru = fs::read_to_string("typst/ru/Belyakov_ru.typ")?;
    for (slug, role) in &roles {
        let temp_en = format!("dist/tmp_en_{slug}.typ");
        fs::write(&temp_en, template_en.replace("Rust Team Lead", role))?;
        Command::new("typst")
            .args(["compile", &temp_en, &format!("dist/Belyakov_en_{slug}.pdf")])
            .status()?;
        fs::remove_file(&temp_en)?;
        let temp_ru = format!("dist/tmp_ru_{slug}.typ");
        fs::write(&temp_ru, template_ru.replace("Rust Team Lead", role))?;
        Command::new("typst")
            .args(["compile", &temp_ru, &format!("dist/Belyakov_ru_{slug}.pdf")])
            .status()?;
        fs::remove_file(&temp_ru)?;
    }
    let roles_js = {
        let pairs: Vec<String> = roles
            .iter()
            .map(|(k, v)| format!("{}: '{}'", k, v))
            .collect();
        format!("{{ {} }}", pairs.join(", "))
    };
    let start_date =
        NaiveDate::from_ymd_opt(inline_start.0, inline_start.1, 1).expect("Invalid start date");
    let today = Utc::now().date_naive();
    let total_months = (today.year() - start_date.year()) * 12
        + (today.month() as i32 - start_date.month() as i32);
    let duration_en = format_duration_en(total_months);
    let duration_ru = format_duration_ru(total_months);
    let date_str = today.format("%Y-%m-%d").to_string();
    // Generate English version
    let pdf_typst_en = "https://github.com/qqrm/CV/releases/latest/download/Belyakov_en_typst.pdf";
    let pdf_typst_ru = "https://github.com/qqrm/CV/releases/latest/download/Belyakov_ru_typst.pdf";

    let markdown_input = fs::read_to_string("cv.md")?;
    let parser = CmarkParser::new_ext(&markdown_input, Options::all());
    let mut html_body = String::new();
    push_html(&mut html_body, parser);
    html_body = html_body.replace("./cv.ru.md", "ru/");
    html_body = html_body.replace(
        "March 2024 – Present  (1 year)",
        &format!("March 2024 – Present  ({})", duration_en),
    );
    if let Some(end) = html_body.find("</h1>") {
        html_body = html_body[end + 5..].trim_start().to_string();
    }

    let html_template = format!(
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<header>\n    <h1>Alexey Belyakov</h1>\n    <p><strong id='position'>Rust Team Lead</strong></p>\n    <p>{}</p>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n{}\n</div>\n<footer>\n    <p><a href='{pdf_typst_en}'>Download PDF (EN)</a></p>\n    <p><a href='{pdf_typst_ru}'>Скачать PDF (RU)</a></p>\n</footer>\n<script>\n    const positions = {roles_js};\n    const seg = window.location.pathname.split('/').filter(Boolean).pop();\n    if (positions[seg]) {{ document.getElementById('position').textContent = positions[seg]; }}\n</script>\n</body>\n</html>\n",
        date_str,
        AVATAR_SRC_EN,
        html_body,
        pdf_typst_en = pdf_typst_en,
        pdf_typst_ru = pdf_typst_ru,
    );

    // Generate Russian version
    let markdown_ru = fs::read_to_string("cv.ru.md")?;
    let parser_ru = CmarkParser::new_ext(&markdown_ru, Options::all());
    let mut html_body_ru = String::new();
    push_html(&mut html_body_ru, parser_ru);
    html_body_ru = html_body_ru.replace(
        "март 2024 – настоящее время (около 1 года)",
        &format!("март 2024 – настоящее время ({})", duration_ru),
    );
    if let Some(end) = html_body_ru.find("</h1>") {
        html_body_ru = html_body_ru[end + 5..].trim_start().to_string();
    }

    let html_template_ru = format!(
        "<!DOCTYPE html>\n<html lang='ru'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Алексей Беляков - Резюме</title>\n    <link rel='stylesheet' href='../style.css'>\n</head>\n<body>\n<header>\n    <h1>Алексей Беляков</h1>\n    <p><strong id='position'>Rust Team Lead</strong></p>\n    <p>{}</p>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n<p><em><a href='../'>Ссылка на английскую версию</a></em></p>\n{}\n</div>\n<footer>\n    <p><a href='{pdf_typst_en}'>Download PDF (EN)</a></p>\n    <p><a href='{pdf_typst_ru}'>Скачать PDF (RU)</a></p>\n</footer>\n<script>\n    const positions = {roles_js};\n    const seg = window.location.pathname.split('/').filter(Boolean).pop();\n    if (positions[seg]) {{ document.getElementById('position').textContent = positions[seg]; }}\n</script>\n</body>\n</html>\n",
        date_str,
        AVATAR_SRC_RU,
        html_body_ru,
        pdf_typst_en = pdf_typst_en,
        pdf_typst_ru = pdf_typst_ru,
    );

    let docs_dir = Path::new("dist");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    fs::copy("content/avatar.jpg", docs_dir.join("avatar.jpg"))?;
    if Path::new("docs/style.css").exists() {
        fs::copy("docs/style.css", docs_dir.join("style.css"))?;
    }
    fs::write(docs_dir.join("index.html"), &html_template)?;

    let ru_dir = docs_dir.join("ru");
    if !ru_dir.exists() {
        fs::create_dir_all(&ru_dir)?;
    }
    fs::write(ru_dir.join("index.html"), &html_template_ru)?;

    // Generate role-specific copies for both languages
    for role in roles.keys() {
        let pdf_typst_en_role = format!(
            "https://github.com/qqrm/CV/releases/latest/download/Belyakov_en_{}_typst.pdf",
            role
        );
        let pdf_typst_ru_role = format!(
            "https://github.com/qqrm/CV/releases/latest/download/Belyakov_ru_{}_typst.pdf",
            role
        );

        let en_role_dir = docs_dir.join(role);
        if !en_role_dir.exists() {
            fs::create_dir_all(&en_role_dir)?;
        }
        let role_template_en = html_template
            .replace(pdf_typst_en, &pdf_typst_en_role)
            .replace(pdf_typst_ru, &pdf_typst_ru_role);
        fs::write(en_role_dir.join("index.html"), role_template_en)?;

        let ru_role_dir = ru_dir.join(role);
        if !ru_role_dir.exists() {
            fs::create_dir_all(&ru_role_dir)?;
        }
        let role_template_ru = html_template_ru
            .replace(pdf_typst_en, &pdf_typst_en_role)
            .replace(pdf_typst_ru, &pdf_typst_ru_role);
        fs::write(ru_role_dir.join("index.html"), role_template_ru)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Validate => validate(),
        Commands::Generate => generate(),
    }
}
