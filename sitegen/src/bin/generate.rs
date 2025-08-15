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
    let base_url = "https://qqrm.github.io/CV/";
    let mut sitemap_urls = vec![base_url.to_string(), format!("{base_url}ru/")];
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
        info!("Created dist directory");
    }
    fs::copy("content/avatar.jpg", dist_dir.join("avatar.jpg"))?;
    info!("Copied avatar to dist directory");

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
    let pdf_typst_en = "https://github.com/qqrm/CV/releases/latest/download/Belyakov_en.pdf";
    let pdf_typst_ru = "https://github.com/qqrm/CV/releases/latest/download/Belyakov_ru.pdf";

    let markdown_input = fs::read_to_string("CV.MD")?;
    let parser = CmarkParser::new_ext(&markdown_input, Options::all());
    let mut html_body_en = String::new();
    push_html(&mut html_body_en, parser);
    html_body_en = html_body_en.replace("./CV_RU.MD", "ru/");
    html_body_en = html_body_en.replace(
        "March 2024 – Present  (1 year)",
        &format!("March 2024 – Present  ({})", duration_en),
    );
    if let Some(end) = html_body_en.find("</h1>") {
        html_body_en = html_body_en[end + 5..].trim_start().to_string();
    }

    let markdown_ru = fs::read_to_string("CV_RU.MD")?;
    let parser_ru = CmarkParser::new_ext(&markdown_ru, Options::all());
    let mut html_body_ru = String::new();
    push_html(&mut html_body_ru, parser_ru);
    html_body_ru = html_body_ru.replace("./CV.MD", "../");
    html_body_ru = html_body_ru.replace(
        "март 2024 – настоящее время (около 1 года)",
        &format!("март 2024 – настоящее время ({})", duration_ru),
    );
    if let Some(end) = html_body_ru.find("</h1>") {
        html_body_ru = html_body_ru[end + 5..].trim_start().to_string();
    }

    // Prepare Product Manager resume bodies
    let markdown_resume_en = fs::read_to_string("CV_PM.MD")?;
    let parser_resume_en = CmarkParser::new_ext(&markdown_resume_en, Options::all());
    let mut html_resume_en = String::new();
    push_html(&mut html_resume_en, parser_resume_en);
    html_resume_en = html_resume_en.replace("./CV_PM_RU.MD", "ru/");
    if let Some(end) = html_resume_en.find("</h1>") {
        html_resume_en = html_resume_en[end + 5..].trim_start().to_string();
    }

    let markdown_resume_ru = fs::read_to_string("CV_PM_RU.MD")?;
    let parser_resume_ru = CmarkParser::new_ext(&markdown_resume_ru, Options::all());
    let mut html_resume_ru = String::new();
    push_html(&mut html_resume_ru, parser_resume_ru);
    html_resume_ru = html_resume_ru.replace("./CV_PM.MD", "../");
    if let Some(end) = html_resume_ru.find("</h1>") {
        html_resume_ru = html_resume_ru[end + 5..].trim_start().to_string();
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
        link_to_en: None,
    })?;

    let docs_dir = Path::new("dist");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    if Path::new("DOCS/style.css").exists() {
        fs::copy("DOCS/style.css", docs_dir.join("style.css"))?;
    }
    if Path::new("DOCS/favicon.svg").exists() {
        fs::copy("DOCS/favicon.svg", docs_dir.join("favicon.svg"))?;
    }
    fs::write(docs_dir.join("index.html"), &html_template)?;
    info!("Wrote English HTML to dist/index.html");

    let ru_root_dir = docs_dir.join("ru");
    if !ru_root_dir.exists() {
        fs::create_dir_all(&ru_root_dir)?;
    }
    fs::write(ru_root_dir.join("index.html"), &html_template_ru)?;
    info!("Wrote Russian HTML to dist/ru/index.html");

    // Generate role-specific copies for both languages
    for role in roles.keys() {
        sitemap_urls.push(format!("{base_url}{role}/"));
        sitemap_urls.push(format!("{base_url}{role}/ru/"));
        let pdf_typst_en_role = format!(
            "https://github.com/qqrm/CV/releases/latest/download/Belyakov_en_{}.pdf",
            role
        );
        let pdf_typst_ru_role = format!(
            "https://github.com/qqrm/CV/releases/latest/download/Belyakov_ru_{}.pdf",
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
            prefix: "../",
            position_block: position_block_role,
            date_str: &date_str,
            avatar_src: "../avatar.jpg",
            html_body: &html_body_en,
            pdf_typst_en: &pdf_typst_en_role,
            pdf_typst_ru: &pdf_typst_ru_role,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(en_role_dir.join("index.html"), en_role_html)?;

        let ru_role_dir = en_role_dir.join("ru");
        if !ru_role_dir.exists() {
            fs::create_dir_all(&ru_role_dir)?;
        }
        let position_block_role = if DEFAULT_ROLE.is_empty() {
            "<p><strong id='position'></strong></p>"
        } else {
            &position_block
        };
        let ru_role_html = render_page(&TemplateData {
            lang: "ru",
            title: "Алексей Беляков - Резюме",
            name: "Алексей Беляков",
            prefix: "../../",
            position_block: position_block_role,
            date_str: &date_str,
            avatar_src: "../../avatar.jpg",
            html_body: &html_body_ru,
            pdf_typst_en: &pdf_typst_en_role,
            pdf_typst_ru: &pdf_typst_ru_role,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(ru_role_dir.join("index.html"), ru_role_html)?;
    }

    // Generate Product Manager resume pages
    sitemap_urls.push(format!("{base_url}resume/pm/"));
    sitemap_urls.push(format!("{base_url}resume/pm/ru/"));

    let resume_dir = docs_dir.join("resume/pm");
    if !resume_dir.exists() {
        fs::create_dir_all(&resume_dir)?;
    }
    let resume_ru_dir = resume_dir.join("ru");
    if !resume_ru_dir.exists() {
        fs::create_dir_all(&resume_ru_dir)?;
    }
    let resume_position = "<p><strong>Product Manager</strong></p>";
    let resume_en_html = render_page(&TemplateData {
        lang: "en",
        title: "Alexey Belyakov - Product Manager Resume",
        name: "Alexey Belyakov",
        prefix: "../../",
        position_block: resume_position,
        date_str: &date_str,
        avatar_src: "../../avatar.jpg",
        html_body: &html_resume_en,
        pdf_typst_en: "https://github.com/qqrm/CV/releases/latest/download/Belyakov_pm_full_en.pdf",
        pdf_typst_ru: "https://github.com/qqrm/CV/releases/latest/download/Belyakov_pm_full_ru.pdf",
        roles_js: &roles_js,
        link_to_en: None,
    })?;
    fs::write(resume_dir.join("index.html"), resume_en_html)?;

    let resume_ru_html = render_page(&TemplateData {
        lang: "ru",
        title: "Алексей Беляков - Резюме продакт-менеджера",
        name: "Алексей Беляков",
        prefix: "../../../",
        position_block: resume_position,
        date_str: &date_str,
        avatar_src: "../../../avatar.jpg",
        html_body: &html_resume_ru,
        pdf_typst_en: "https://github.com/qqrm/CV/releases/latest/download/Belyakov_pm_full_en.pdf",
        pdf_typst_ru: "https://github.com/qqrm/CV/releases/latest/download/Belyakov_pm_full_ru.pdf",
        roles_js: &roles_js,
        link_to_en: None,
    })?;
    fs::write(resume_ru_dir.join("index.html"), resume_ru_html)?;
    let sitemap_content = sitemap_urls.join("\n") + "\n";
    fs::write(docs_dir.join("sitemap.txt"), sitemap_content)?;
    info!("Wrote sitemap to dist/sitemap.txt");
    info!("Site generation completed");
    Ok(())
}
