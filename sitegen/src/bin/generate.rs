use chrono::{Datelike, NaiveDate, Utc};
use handlebars::Handlebars;
use log::{info, warn};
use pulldown_cmark::{Options, Parser as CmarkParser, html::push_html};
use regex::Regex;
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
    footer_links: &'a str,
    roles_js: &'a str,
    link_to_en: Option<&'a str>,
}

struct ResumeContent {
    html_en: String,
    html_ru: String,
    footer_en: String,
    footer_ru: String,
}

fn inject_duration(html: &mut String, fragment: &str, duration: &str) -> bool {
    let expected = format!("{fragment} ({duration})");
    if html.contains(&expected) {
        return true;
    }
    let escaped = regex::escape(fragment);
    let re = Regex::new(&format!(r"{}(?:\s*\([^)]*\))?", escaped)).expect("invalid duration regex");
    if re.is_match(html) {
        *html = re.replace(html, expected.as_str()).into_owned();
        true
    } else {
        false
    }
}

fn russian_month_name(month: u32) -> Option<&'static str> {
    const RU_MONTHS: [&str; 12] = [
        "январь",
        "февраль",
        "март",
        "апрель",
        "май",
        "июнь",
        "июль",
        "август",
        "сентябрь",
        "октябрь",
        "ноябрь",
        "декабрь",
    ];
    if (1..=12).contains(&month) {
        Some(RU_MONTHS[(month - 1) as usize])
    } else {
        None
    }
}

fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn render_page(data: &TemplateData) -> Result<String, handlebars::RenderError> {
    let hb = Handlebars::new();
    let tmpl = include_str!("../../templates/page.hbs");
    hb.render_template(tmpl, data)
}

fn extract_first_paragraph(html: &str) -> String {
    html.find("</p>")
        .map(|idx| html[..idx + 4].to_string())
        .unwrap_or_default()
}

fn role_title_ru(slug: &str) -> String {
    match slug {
        "tl" => "тимлида",
        "em" => "инженерного менеджера",
        "hod" => "руководителя разработки",
        "pm" => "продакт-менеджера",
        "tech" => "технического лидера",
        _ => slug,
    }
    .to_string()
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
    let position_block = match DEFAULT_ROLE {
        "" => String::new(),
        role => format!("<p><strong id='position'>{role}</strong></p>"),
    };
    // Prepare HTML bodies
    let pdf_typst_en = "Belyakov_en.pdf";
    let pdf_typst_ru = "Belyakov_ru.pdf";
    let pdf_typst_en_ru = "../Belyakov_en.pdf";
    let pdf_typst_ru_ru = "../Belyakov_ru.pdf";

    let markdown_input = fs::read_to_string("profiles/cv/en/CV.MD")?;
    let parser = CmarkParser::new_ext(&markdown_input, Options::all());
    let mut html_body_en = String::new();
    push_html(&mut html_body_en, parser);
    html_body_en = html_body_en.replace("../ru/CV_RU.MD", "ru/");
    html_body_en = html_body_en.replace("https://qqrm.github.io/CV/Belyakov_en.pdf", pdf_typst_en);
    html_body_en = html_body_en.replace("https://qqrm.github.io/CV/Belyakov_ru.pdf", pdf_typst_ru);
    let english_fragment = format!("{} – Present", start_date.format("%B %Y"));
    if !inject_duration(&mut html_body_en, &english_fragment, &duration_en) {
        warn!("English inline duration fragment '{english_fragment}' not found in generated HTML");
    }
    if let Some(end) = html_body_en.find("</h1>") {
        html_body_en = html_body_en[end + 5..].trim_start().to_string();
    }

    let markdown_ru = fs::read_to_string("profiles/cv/ru/CV_RU.MD")?;
    let parser_ru = CmarkParser::new_ext(&markdown_ru, Options::all());
    let mut html_body_ru = String::new();
    push_html(&mut html_body_ru, parser_ru);
    html_body_ru = html_body_ru.replace("../en/CV.MD", "../");
    html_body_ru =
        html_body_ru.replace("https://qqrm.github.io/CV/Belyakov_ru.pdf", pdf_typst_ru_ru);
    html_body_ru =
        html_body_ru.replace("https://qqrm.github.io/CV/Belyakov_en.pdf", pdf_typst_en_ru);
    let mut ru_fragments = Vec::new();
    ru_fragments.push(english_fragment.clone());
    if let Some(month_name) = russian_month_name(inline_start.1) {
        let month_lower = month_name;
        let month_title = capitalize_first(month_lower);
        let year = inline_start.0;
        ru_fragments.extend([
            format!("{month_lower} {year} – настоящее время"),
            format!("{month_lower} {year} – Настоящее время"),
            format!("{month_title} {year} – настоящее время"),
            format!("{month_title} {year} – Настоящее время"),
        ]);
    }
    let mut injected_ru = false;
    for fragment in ru_fragments {
        if inject_duration(&mut html_body_ru, &fragment, &duration_ru) {
            injected_ru = true;
            break;
        }
    }
    if !injected_ru {
        warn!("Russian inline duration fragment not found in generated HTML");
    }
    if let Some(end) = html_body_ru.find("</h1>") {
        html_body_ru = html_body_ru[end + 5..].trim_start().to_string();
    }

    let footer_links_en = extract_first_paragraph(&html_body_en);
    let footer_links_ru = extract_first_paragraph(&html_body_ru);

    // Prepare role-specific resume bodies
    let resume_specs = [
        (
            "tl",
            "profiles/resume/en/RESUME_TL.MD",
            "profiles/resume/ru/RESUME_TL_RU.MD",
        ),
        (
            "em",
            "profiles/resume/en/RESUME_EM.MD",
            "profiles/resume/ru/RESUME_EM_RU.MD",
        ),
        (
            "hod",
            "profiles/resume/en/RESUME_HOD.MD",
            "profiles/resume/ru/RESUME_HOD_RU.MD",
        ),
        (
            "pm",
            "profiles/resume/en/RESUME_PM.MD",
            "profiles/resume/ru/RESUME_PM_RU.MD",
        ),
        (
            "tech",
            "profiles/resume/en/RESUME_TECH.MD",
            "profiles/resume/ru/RESUME_TECH_RU.MD",
        ),
    ];
    let mut resume_contents: Vec<(String, ResumeContent)> = Vec::new();
    for (slug, en_md, ru_md) in resume_specs {
        let slug_upper = slug.to_ascii_uppercase();
        let markdown_resume_en = fs::read_to_string(en_md)?;
        let parser_resume_en = CmarkParser::new_ext(&markdown_resume_en, Options::all());
        let mut html_resume_en = String::new();
        push_html(&mut html_resume_en, parser_resume_en);
        let link_to_ru = format!("../ru/RESUME_{}_RU.MD", slug_upper);
        html_resume_en = html_resume_en.replace(&link_to_ru, "ru/");
        if let Some(end) = html_resume_en.find("</h1>") {
            html_resume_en = html_resume_en[end + 5..].trim_start().to_string();
        }

        let markdown_resume_ru = fs::read_to_string(ru_md)?;
        let parser_resume_ru = CmarkParser::new_ext(&markdown_resume_ru, Options::all());
        let mut html_resume_ru = String::new();
        push_html(&mut html_resume_ru, parser_resume_ru);
        let link_to_en = format!("../en/RESUME_{}.MD", slug_upper);
        html_resume_ru = html_resume_ru.replace(&link_to_en, "../");
        if let Some(end) = html_resume_ru.find("</h1>") {
            html_resume_ru = html_resume_ru[end + 5..].trim_start().to_string();
        }

        let footer_links_resume_en = extract_first_paragraph(&html_resume_en);
        let footer_links_resume_ru = extract_first_paragraph(&html_resume_ru);
        resume_contents.push((
            slug.to_string(),
            ResumeContent {
                html_en: html_resume_en,
                html_ru: html_resume_ru,
                footer_en: footer_links_resume_en,
                footer_ru: footer_links_resume_ru,
            },
        ));
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
        footer_links: &footer_links_en,
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
        footer_links: &footer_links_ru,
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
    let base_en = docs_dir.join("Belyakov_en.pdf");
    if Path::new("typst/en/Belyakov_en.pdf").exists() {
        fs::copy("typst/en/Belyakov_en.pdf", &base_en)?;
    } else {
        fs::File::create(&base_en)?;
    }
    let base_ru = docs_dir.join("Belyakov_ru.pdf");
    if Path::new("typst/ru/Belyakov_ru.pdf").exists() {
        fs::copy("typst/ru/Belyakov_ru.pdf", &base_ru)?;
    } else {
        fs::File::create(&base_ru)?;
    }
    if Path::new("typst/en/Belyakov_pm_en.pdf").exists() {
        fs::copy(
            "typst/en/Belyakov_pm_en.pdf",
            docs_dir.join("Belyakov_pm_en.pdf"),
        )?;
    }
    if Path::new("typst/ru/Belyakov_pm_ru.pdf").exists() {
        fs::copy(
            "typst/ru/Belyakov_pm_ru.pdf",
            docs_dir.join("Belyakov_pm_ru.pdf"),
        )?;
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

        let src_en = Path::new("typst/en").join(format!("Belyakov_{}_en.pdf", role));
        let src_ru = Path::new("typst/ru").join(format!("Belyakov_{}_ru.pdf", role));
        let dst_en = docs_dir.join(format!("Belyakov_{}_en.pdf", role));
        let dst_ru = docs_dir.join(format!("Belyakov_{}_ru.pdf", role));
        if src_en.exists() {
            fs::copy(&src_en, &dst_en)?;
        } else {
            fs::copy(docs_dir.join("Belyakov_en.pdf"), &dst_en)?;
        }
        if src_ru.exists() {
            fs::copy(&src_ru, &dst_ru)?;
        } else {
            fs::copy(docs_dir.join("Belyakov_ru.pdf"), &dst_ru)?;
        }
        let pdf_typst_en_role = format!("../Belyakov_{}_en.pdf", role);
        let pdf_typst_ru_role = format!("../Belyakov_{}_ru.pdf", role);
        let pdf_typst_en_role_ru = format!("../../Belyakov_{}_en.pdf", role);
        let pdf_typst_ru_role_ru = format!("../../Belyakov_{}_ru.pdf", role);

        let en_role_dir = docs_dir.join(role);
        if !en_role_dir.exists() {
            fs::create_dir_all(&en_role_dir)?;
        }
        let position_block_role = match DEFAULT_ROLE {
            "" => "<p><strong id='position'></strong></p>",
            _ => &position_block,
        };
        let html_body_en_role = html_body_en
            .replace("Belyakov_en.pdf", &pdf_typst_en_role)
            .replace("Belyakov_ru.pdf", &pdf_typst_ru_role);
        let footer_links_en_role = extract_first_paragraph(&html_body_en_role);
        let en_role_html = render_page(&TemplateData {
            lang: "en",
            title: "Alexey Belyakov - CV",
            name: "Alexey Belyakov",
            prefix: "../",
            position_block: position_block_role,
            date_str: &date_str,
            avatar_src: "../avatar.jpg",
            html_body: &html_body_en_role,
            footer_links: &footer_links_en_role,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(en_role_dir.join("index.html"), en_role_html)?;

        let ru_role_dir = en_role_dir.join("ru");
        if !ru_role_dir.exists() {
            fs::create_dir_all(&ru_role_dir)?;
        }
        let position_block_role = match DEFAULT_ROLE {
            "" => "<p><strong id='position'></strong></p>",
            _ => &position_block,
        };
        let html_body_ru_role = html_body_ru
            .replace("../Belyakov_ru.pdf", &pdf_typst_ru_role_ru)
            .replace("../Belyakov_en.pdf", &pdf_typst_en_role_ru);
        let footer_links_ru_role = extract_first_paragraph(&html_body_ru_role);
        let ru_role_html = render_page(&TemplateData {
            lang: "ru",
            title: "Алексей Беляков - Резюме",
            name: "Алексей Беляков",
            prefix: "../../",
            position_block: position_block_role,
            date_str: &date_str,
            avatar_src: "../../avatar.jpg",
            html_body: &html_body_ru_role,
            footer_links: &footer_links_ru_role,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(ru_role_dir.join("index.html"), ru_role_html)?;
    }

    for (slug, content) in &resume_contents {
        sitemap_urls.push(format!("{base_url}resume/{slug}/"));
        sitemap_urls.push(format!("{base_url}resume/{slug}/ru/"));

        let resume_dir = docs_dir.join("resume").join(slug);
        if !resume_dir.exists() {
            fs::create_dir_all(&resume_dir)?;
        }
        let resume_ru_dir = resume_dir.join("ru");
        if !resume_ru_dir.exists() {
            fs::create_dir_all(&resume_ru_dir)?;
        }
        let role_name = roles
            .get(slug.as_str())
            .map(|s| s.as_str())
            .unwrap_or(slug.as_str());
        let resume_position_en = format!("<p><strong>{role_name}</strong></p>");
        let resume_title_en = format!("Alexey Belyakov - {role_name} Resume");
        let resume_en_html = render_page(&TemplateData {
            lang: "en",
            title: &resume_title_en,
            name: "Alexey Belyakov",
            prefix: "../../",
            position_block: &resume_position_en,
            date_str: &date_str,
            avatar_src: "../../avatar.jpg",
            html_body: &content.html_en,
            footer_links: &content.footer_en,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(resume_dir.join("index.html"), resume_en_html)?;

        let role_title_ru = role_title_ru(slug);
        let resume_position_ru = format!("<p><strong>{role_title_ru}</strong></p>");
        let resume_title_ru = format!("Алексей Беляков - Резюме {role_title_ru}");
        let resume_ru_html = render_page(&TemplateData {
            lang: "ru",
            title: &resume_title_ru,
            name: "Алексей Беляков",
            prefix: "../../../",
            position_block: &resume_position_ru,
            date_str: &date_str,
            avatar_src: "../../../avatar.jpg",
            html_body: &content.html_ru,
            footer_links: &content.footer_ru,
            roles_js: &roles_js,
            link_to_en: None,
        })?;
        fs::write(resume_ru_dir.join("index.html"), resume_ru_html)?;
    }
    let sitemap_content = sitemap_urls.join("\n") + "\n";
    fs::write(docs_dir.join("sitemap.txt"), sitemap_content)?;
    info!("Wrote sitemap to dist/sitemap.txt");
    info!("Site generation completed");
    Ok(())
}
