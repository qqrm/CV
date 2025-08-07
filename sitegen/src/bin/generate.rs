use chrono::{Datelike, NaiveDate, Utc};
use pulldown_cmark::{Options, Parser as CmarkParser, html::push_html};
use sitegen::{format_duration_en, format_duration_ru, read_inline_start, read_roles};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    const AVATAR_SRC_EN: &str = "avatar.jpg";
    const AVATAR_SRC_RU: &str = "../avatar.jpg";
    const INLINE_START: (i32, u32) = (2024, 3);
    const DEFAULT_ROLE: &str = "Rust Team Lead";
    let inline_start = read_inline_start().unwrap_or(INLINE_START);
    let roles = read_roles();
    // Build base PDFs
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
    }
    fs::copy("content/avatar.jpg", dist_dir.join("avatar.jpg"))?;

    for lang in ["en", "ru"] {
        Command::new("typst")
            .args([
                "compile",
                "templates/resume.typ",
                &format!("dist/Belyakov_{lang}_typst.pdf"),
                "--input",
                &format!("lang={lang}"),
                "--input",
                &format!("role={DEFAULT_ROLE}"),
                "--root",
                ".",
            ])
            .status()?;

        for (slug, title) in &roles {
            Command::new("typst")
                .args([
                    "compile",
                    "templates/resume.typ",
                    &format!("dist/Belyakov_{lang}_{slug}.pdf"),
                    "--input",
                    &format!("lang={lang}"),
                    "--input",
                    &format!("role={title}"),
                    "--root",
                    ".",
                ])
                .status()?;
        }
    }
    let roles_js = {
        let pairs: Vec<String> = roles
            .iter()
            .map(|(k, v)| format!("{k}: '{v}'"))
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
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='icon' href='favicon.svg' type='image/svg+xml'>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<header>\n    <h1>Alexey Belyakov</h1>\n    <p><strong id='position'>Rust Team Lead</strong></p>\n    <p>{}</p>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n{}\n</div>\n<footer>\n    <p><a href='{pdf_typst_en}'>Download PDF (EN)</a></p>\n    <p><a href='{pdf_typst_ru}'>Скачать PDF (RU)</a></p>\n</footer>\n<script>\n    const positions = {roles_js};\n    const seg = window.location.pathname.split('/').filter(Boolean).pop();\n    if (positions[seg]) {{ document.getElementById('position').textContent = positions[seg]; }}\n</script>\n</body>\n</html>\n",
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
        "<!DOCTYPE html>\n<html lang='ru'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Алексей Беляков - Резюме</title>\n    <link rel='icon' href='../favicon.svg' type='image/svg+xml'>\n    <link rel='stylesheet' href='../style.css'>\n</head>\n<body>\n<header>\n    <h1>Алексей Беляков</h1>\n    <p><strong id='position'>Rust Team Lead</strong></p>\n    <p>{}</p>\n</header>\n<div class='content'>\n<img class='avatar' src='{}' alt='Avatar'>\n<p><em><a href='../'>Ссылка на английскую версию</a></em></p>\n{}\n</div>\n<footer>\n    <p><a href='{pdf_typst_en}'>Download PDF (EN)</a></p>\n    <p><a href='{pdf_typst_ru}'>Скачать PDF (RU)</a></p>\n</footer>\n<script>\n    const positions = {roles_js};\n    const seg = window.location.pathname.split('/').filter(Boolean).pop();\n    if (positions[seg]) {{ document.getElementById('position').textContent = positions[seg]; }}\n</script>\n</body>\n</html>\n",
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
    if Path::new("docs/style.css").exists() {
        fs::copy("docs/style.css", docs_dir.join("style.css"))?;
    }
    if Path::new("docs/favicon.svg").exists() {
        fs::copy("docs/favicon.svg", docs_dir.join("favicon.svg"))?;
    }
    fs::write(docs_dir.join("index.html"), &html_template)?;

    let ru_dir = docs_dir.join("ru");
    if !ru_dir.exists() {
        fs::create_dir_all(&ru_dir)?;
    }
    fs::write(ru_dir.join("index.html"), &html_template_ru)?;

    // Generate role-specific copies for both languages
    for role in roles.keys() {
        let pdf_typst_en_role =
            format!("https://github.com/qqrm/CV/releases/latest/download/Belyakov_en_{}_typst.pdf", role);
        let pdf_typst_ru_role =
            format!("https://github.com/qqrm/CV/releases/latest/download/Belyakov_ru_{}_typst.pdf", role);

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

