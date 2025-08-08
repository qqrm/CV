use chrono::{Datelike, NaiveDate, Utc};
use handlebars::Handlebars;
use log::{info, warn};
use pulldown_cmark::{Options, Parser as CmarkParser, html::push_html};
use serde::Serialize;
use sitegen::parser::{read_inline_start, read_roles};
use sitegen::renderer::{format_duration_en, format_duration_ru};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize)]
struct TemplateData<'a> {
    lang: &'a str,
    title: &'a str,
    name: &'a str,
    prefix: &'a str,
    position_block: &'a str,
    date_str: &'a str,
    avatar_src: &'a str,
    html_body: &'a str,
    pdf_typst_en: &'a str,
    pdf_typst_ru: &'a str,
    roles_js: &'a str,
    link_to_en: Option<&'a str>,
}

fn render_page(data: &TemplateData) -> Result<String, handlebars::RenderError> {
    let hb = Handlebars::new();
    let tmpl = include_str!("../../templates/page.hbs");
    hb.render_template(tmpl, data)
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting site generation");
    const INLINE_START: (i32, u32) = (2024, 3);
    const DEFAULT_ROLE: &str = "";
    let inline_start = match read_inline_start() {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to read inline start: {e}");
            INLINE_START
        }
    };
    let roles = read_roles().expect("failed to read roles");
    // Build base PDFs
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
        info!("Created dist directory");
    }
    fs::copy("content/avatar.jpg", dist_dir.join("avatar.jpg"))?;
    info!("Copied avatar to dist directory");

    for lang in ["en", "ru"] {
        info!("Building PDF for language: {}", lang);
        let mut cmd = Command::new("typst");
        cmd.args([
            "compile",
            "templates/resume.typ",
            &format!("dist/Belyakov_{lang}_typst.pdf"),
            "--input",
            &format!("lang={lang}"),
        ]);
        if !DEFAULT_ROLE.is_empty() {
            cmd.args(["--input", &format!("role={DEFAULT_ROLE}")]);
        }
        cmd.status()?;

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
                ])
                .status()?;
        }
    }
    let roles_js = {
        let pairs: Vec<String> = roles.iter().map(|(k, v)| format!("{k}: '{v}'")).collect();
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
    let position_block = if DEFAULT_ROLE.is_empty() {
        String::new()
    } else {
        format!("<p><strong id='position'>{DEFAULT_ROLE}</strong></p>")
    };
    // Prepare HTML bodies
    let pdf_typst_en = "https://github.com/qqrm/CV/releases/latest/download/Belyakov_en_typst.pdf";
    let pdf_typst_ru = "https://github.com/qqrm/CV/releases/latest/download/Belyakov_ru_typst.pdf";

    let markdown_input = fs::read_to_string("cv.md")?;
    let parser = CmarkParser::new_ext(&markdown_input, Options::all());
    let mut html_body_en = String::new();
    push_html(&mut html_body_en, parser);
    html_body_en = html_body_en.replace("./cv.ru.md", "ru/");
    html_body_en = html_body_en.replace(
        "March 2024 – Present  (1 year)",
        &format!("March 2024 – Present  ({})", duration_en),
    );
    if let Some(end) = html_body_en.find("</h1>") {
        html_body_en = html_body_en[end + 5..].trim_start().to_string();
    }

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

    // Render base pages
    let html_template = render_page(&TemplateData {
        lang: "en",
        title: "Alexey Belyakov - CV",
        name: "Alexey Belyakov",
        prefix: "",
        position_block: &position_block,
        date_str: &date_str,
        avatar_src: "avatar.jpg",
        html_body: &html_body_en,
        pdf_typst_en,
        pdf_typst_ru,
        roles_js: &roles_js,
        link_to_en: None,
    })?;

    let html_template_ru = render_page(&TemplateData {
        lang: "ru",
        title: "Алексей Беляков - Резюме",
        name: "Алексей Беляков",
        prefix: "../",
        position_block: &position_block,
        date_str: &date_str,
        avatar_src: "../avatar.jpg",
        html_body: &html_body_ru,
        pdf_typst_en,
        pdf_typst_ru,
        roles_js: &roles_js,
        link_to_en: Some("../"),
    })?;

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
    info!("Wrote English HTML to dist/index.html");

    let ru_dir = docs_dir.join("ru");
    if !ru_dir.exists() {
        fs::create_dir_all(&ru_dir)?;
    }
    fs::write(ru_dir.join("index.html"), &html_template_ru)?;
    info!("Wrote Russian HTML to dist/ru/index.html");

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
        let position_block_role = if DEFAULT_ROLE.is_empty() {
            "<p><strong id='position'></strong></p>"
        } else {
            &position_block
        };
        let en_role_html = render_page(&TemplateData {
            lang: "en",
            title: "Alexey Belyakov - CV",
            name: "Alexey Belyakov",
            prefix: "",
            position_block: position_block_role,
            date_str: &date_str,
            avatar_src: "avatar.jpg",
            html_body: &html_body_en,
            pdf_typst_en: &pdf_typst_en_role,
            pdf_typst_ru: &pdf_typst_ru_role,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(en_role_dir.join("index.html"), en_role_html)?;

        let ru_role_dir = ru_dir.join(role);
        if !ru_role_dir.exists() {
            fs::create_dir_all(&ru_role_dir)?;
        }
        let cross_link = format!("../../{}/", role);
        let position_block_role = if DEFAULT_ROLE.is_empty() {
            "<p><strong id='position'></strong></p>"
        } else {
            &position_block
        };
        let ru_role_html = render_page(&TemplateData {
            lang: "ru",
            title: "Алексей Беляков - Резюме",
            name: "Алексей Беляков",
            prefix: "../",
            position_block: position_block_role,
            date_str: &date_str,
            avatar_src: "../avatar.jpg",
            html_body: &html_body_ru,
            pdf_typst_en: &pdf_typst_en_role,
            pdf_typst_ru: &pdf_typst_ru_role,
            roles_js: &roles_js,
            link_to_en: Some(&cross_link),
        })?;
        fs::write(ru_role_dir.join("index.html"), ru_role_html)?;
    }
    info!("Site generation completed");
    Ok(())
}
