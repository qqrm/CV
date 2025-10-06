use chrono::{Datelike, NaiveDate, Utc};
use handlebars::Handlebars;
use html_escape::{decode_html_entities, encode_double_quoted_attribute, encode_safe};
use log::{info, warn};
use pulldown_cmark::{Options, Parser as CmarkParser, html::push_html};
use regex::Regex;
use serde::Serialize;
use sitegen::parser::{read_inline_start, read_roles};
use sitegen::renderer::{format_duration_en, format_duration_ru};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;

const PDF_BASE_URL: &str = "https://qqrm.github.io/CV/";
const THEME_VARIANTS: &[&str] = &["light", "dark"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PdfVariant {
    Light,
    Dark,
}

impl PdfVariant {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct AnchorMatch {
    prefix: String,
    variant: PdfVariant,
    attrs_before_href: String,
    attrs_after_href: String,
    wrap_prefix: Option<String>,
    wrap_suffix: Option<String>,
    trailing_break: Option<String>,
    label_text: String,
    range: Range<usize>,
}

#[derive(Clone, Default)]
struct VariantInfo {
    light: Option<AnchorMatch>,
    dark: Option<AnchorMatch>,
}

impl VariantInfo {
    fn assign(&mut self, anchor: AnchorMatch) {
        match anchor.variant {
            PdfVariant::Light => self.light = Some(anchor),
            PdfVariant::Dark => self.dark = Some(anchor),
        }
    }

    fn has_variant(&self, variant: PdfVariant) -> bool {
        match variant {
            PdfVariant::Light => self.light.is_some(),
            PdfVariant::Dark => self.dark.is_some(),
        }
    }

    fn label_for(&self, variant: PdfVariant) -> String {
        match variant {
            PdfVariant::Light => self
                .light
                .as_ref()
                .map(|a| a.label_text.clone())
                .or_else(|| self.dark.as_ref().map(|a| a.label_text.clone()))
                .unwrap_or_else(|| String::from("Light PDF")),
            PdfVariant::Dark => self
                .dark
                .as_ref()
                .map(|a| a.label_text.clone())
                .or_else(|| self.light.as_ref().map(|a| a.label_text.clone()))
                .unwrap_or_else(|| String::from("Dark PDF")),
        }
    }

    fn href_for(&self, prefix: &str, variant: PdfVariant) -> Option<String> {
        match variant {
            PdfVariant::Light => self.light.as_ref().map(|_| format!("{prefix}_light.pdf")),
            PdfVariant::Dark => self.dark.as_ref().map(|_| format!("{prefix}_dark.pdf")),
        }
    }
}

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
    roles_js_ru: &'a str,
    link_to_en: Option<&'a str>,
}

fn typst_source_for(pdf: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let file_name = pdf
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("Invalid PDF file name: {}", pdf.display()))?;

    if let Some(prefix) = file_name.strip_suffix("_light.pdf") {
        Ok(pdf.with_file_name(format!("{prefix}.typ")))
    } else if let Some(prefix) = file_name.strip_suffix(".pdf") {
        Ok(pdf.with_file_name(format!("{prefix}.typ")))
    } else {
        Err(format!("Unsupported PDF name: {file_name}").into())
    }
}

fn compile_pdf(pdf: &Path) -> Result<(), Box<dyn Error>> {
    let typ_path = typst_source_for(pdf)?;
    if !typ_path.exists() {
        return Err(format!("Typst source not found: {}", typ_path.display()).into());
    }

    let status = Command::new("typst")
        .args(["compile", "--root", "."])
        .arg(&typ_path)
        .arg(pdf)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to compile {} from {}",
            pdf.display(),
            typ_path.display()
        )
        .into())
    }
}

fn compile_and_copy_pdf(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    compile_pdf(src)?;
    fs::copy(src, dst)?;
    Ok(())
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

fn annotate_resume_links(html: &mut String) {
    let anchor_re = Regex::new(
        r#"(?xs)
            (?P<wrap_prefix><em[^>]*>\s*)?
            <a
                (?P<before>[^>]*?)
                href="(?P<prefix>[^"]*?)_(?P<variant>light|dark)\.pdf"
                (?P<after>[^>]*)
            >
                (?P<label>.*?)
            </a>
            (?P<wrap_suffix>\s*</em>)?
            (?P<trailing>\s*\\)?
        "#,
    )
    .expect("invalid resume anchor regex");

    let mut matches: Vec<AnchorMatch> = anchor_re
        .captures_iter(html)
        .filter_map(|caps| {
            let m = caps.get(0)?;
            let prefix = caps.name("prefix")?.as_str().to_string();
            let variant = PdfVariant::from_str(caps.name("variant")?.as_str())?;
            let label_raw = caps.name("label").map(|m| m.as_str()).unwrap_or_default();
            let label_text = decode_html_entities(label_raw.trim()).into_owned();

            Some(AnchorMatch {
                prefix,
                variant,
                attrs_before_href: caps
                    .name("before")
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
                attrs_after_href: caps
                    .name("after")
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
                wrap_prefix: caps.name("wrap_prefix").map(|m| m.as_str().to_string()),
                wrap_suffix: caps.name("wrap_suffix").map(|m| m.as_str().to_string()),
                trailing_break: caps.name("trailing").map(|m| m.as_str().to_string()),
                label_text,
                range: m.start()..m.end(),
            })
        })
        .collect();

    if matches.is_empty() {
        return;
    }

    matches.sort_by_key(|m| m.range.start);

    let mut replacements: Vec<(Range<usize>, String)> = Vec::new();
    let mut index = 0;

    while index < matches.len() {
        let prefix = matches[index].prefix.clone();
        let mut group: Vec<AnchorMatch> = Vec::new();

        group.push(matches[index].clone());
        index += 1;

        while index < matches.len() && matches[index].prefix == prefix {
            group.push(matches[index].clone());
            index += 1;
        }

        let mut info = VariantInfo::default();
        for anchor in group.iter().cloned() {
            info.assign(anchor);
        }

        let primary_variant = if info.has_variant(PdfVariant::Light) {
            PdfVariant::Light
        } else if info.has_variant(PdfVariant::Dark) {
            PdfVariant::Dark
        } else {
            continue;
        };

        let primary_anchor = match primary_variant {
            PdfVariant::Light => info.light.as_ref().cloned(),
            PdfVariant::Dark => info.dark.as_ref().cloned(),
        };

        let Some(primary_anchor) = primary_anchor else {
            continue;
        };

        let light_label = info.label_for(PdfVariant::Light);
        let dark_label = info.label_for(PdfVariant::Dark);
        let light_href = info.href_for(&prefix, PdfVariant::Light);
        let dark_href = info.href_for(&prefix, PdfVariant::Dark);

        let (initial_href, initial_label) = match primary_variant {
            PdfVariant::Light => (
                light_href.clone().or_else(|| dark_href.clone()),
                light_label.clone(),
            ),
            PdfVariant::Dark => (
                dark_href.clone().or_else(|| light_href.clone()),
                dark_label.clone(),
            ),
        };

        let Some(initial_href) = initial_href else {
            continue;
        };

        let mut new_anchor = String::new();
        if let Some(prefix_markup) = &primary_anchor.wrap_prefix {
            new_anchor.push_str(prefix_markup);
        }

        new_anchor.push_str("<a");
        if !primary_anchor.attrs_before_href.is_empty() {
            new_anchor.push_str(&primary_anchor.attrs_before_href);
            if !primary_anchor
                .attrs_before_href
                .chars()
                .last()
                .map(|c| c.is_whitespace())
                .unwrap_or(false)
            {
                new_anchor.push(' ');
            }
        } else {
            new_anchor.push(' ');
        }
        new_anchor.push_str("href=\"");
        new_anchor.push_str(&encode_double_quoted_attribute(&initial_href));
        new_anchor.push('"');
        new_anchor.push_str(&primary_anchor.attrs_after_href);

        if let Some(light_href) = &light_href {
            new_anchor.push_str(" data-light-href=\"");
            new_anchor.push_str(&encode_double_quoted_attribute(light_href));
            new_anchor.push('"');
        }
        if let Some(dark_href) = &dark_href {
            new_anchor.push_str(" data-dark-href=\"");
            new_anchor.push_str(&encode_double_quoted_attribute(dark_href));
            new_anchor.push('"');
        }

        new_anchor.push_str(" data-light-label=\"");
        new_anchor.push_str(&encode_double_quoted_attribute(&light_label));
        new_anchor.push('"');
        new_anchor.push_str(" data-dark-label=\"");
        new_anchor.push_str(&encode_double_quoted_attribute(&dark_label));
        new_anchor.push('"');
        new_anchor.push('>');
        new_anchor.push_str(&encode_safe(&initial_label));
        new_anchor.push_str("</a>");

        if let Some(suffix_markup) = &primary_anchor.wrap_suffix {
            new_anchor.push_str(suffix_markup);
        }
        if let Some(trailing) = &primary_anchor.trailing_break {
            new_anchor.push_str(trailing);
        }

        replacements.push((primary_anchor.range.clone(), new_anchor));

        for anchor in group {
            if anchor.range.start != primary_anchor.range.start {
                replacements.push((anchor.range, String::new()));
            }
        }
    }

    replacements.sort_by(|a, b| b.0.start.cmp(&a.0.start));
    for (range, replacement) in replacements {
        html.replace_range(range, &replacement);
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

fn role_title_ru_genitive(slug: &str) -> String {
    match slug {
        "em" => "руководителя разработки",
        _ => slug,
    }
    .to_string()
}

fn role_title_ru_nominative(slug: &str) -> String {
    match slug {
        "em" => "Старший Rust-инженер",
        _ => slug,
    }
    .to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting site generation");
    const INLINE_START: (i32, u32) = (2024, 3);
    const DEFAULT_ROLE: Option<&str> = None;
    let inline_start = match read_inline_start() {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to read inline start: {e}");
            INLINE_START
        }
    };
    let roles = read_roles().expect("failed to read roles");
    let base_url = PDF_BASE_URL;
    let mut sitemap_urls = vec![base_url.to_string(), format!("{base_url}ru/")];
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
        info!("Created dist directory");
    }
    fs::copy("content/avatar.jpg", dist_dir.join("avatar.jpg"))?;
    info!("Copied avatar to dist directory");

    let roles_js_en =
        serde_json::to_string(&roles).expect("failed to serialize English roles to JSON");
    let roles_js_ru = {
        let ru_roles: BTreeMap<String, String> = roles
            .keys()
            .map(|slug| (slug.clone(), role_title_ru_nominative(slug)))
            .collect();
        serde_json::to_string(&ru_roles).expect("failed to serialize Russian roles to JSON")
    };
    let start_date =
        NaiveDate::from_ymd_opt(inline_start.0, inline_start.1, 1).expect("Invalid start date");
    let today = Utc::now().date_naive();
    let total_months = (today.year() - start_date.year()) * 12
        + (today.month() as i32 - start_date.month() as i32);
    let duration_en = format_duration_en(total_months);
    let duration_ru = format_duration_ru(total_months);
    let date_str = today.format("%Y-%m-%d").to_string();
    let position_block = DEFAULT_ROLE
        .map(|role| format!("<p><strong id='position'>{role}</strong></p>"))
        .unwrap_or_default();
    // Prepare HTML bodies
    let markdown_input = fs::read_to_string("profiles/cv/en/CV.MD")?;
    let parser = CmarkParser::new_ext(&markdown_input, Options::all());
    let mut html_body_en = String::new();
    push_html(&mut html_body_en, parser);
    html_body_en = html_body_en.replace("../ru/CV_RU.MD", "ru/");
    for theme in THEME_VARIANTS {
        let en_pdf = format!("{PDF_BASE_URL}Belyakov_en_{theme}.pdf");
        let ru_pdf = format!("{PDF_BASE_URL}Belyakov_ru_{theme}.pdf");
        let en_local = format!("Belyakov_en_{theme}.pdf");
        let ru_local = format!("Belyakov_ru_{theme}.pdf");
        html_body_en = html_body_en.replace(&en_pdf, &en_local);
        html_body_en = html_body_en.replace(&ru_pdf, &ru_local);
    }
    let english_fragment = format!("{} – Present", start_date.format("%B %Y"));
    if !inject_duration(&mut html_body_en, &english_fragment, &duration_en) {
        warn!("English inline duration fragment '{english_fragment}' not found in generated HTML");
    }
    if let Some(end) = html_body_en.find("</h1>") {
        html_body_en = html_body_en[end + 5..].trim_start().to_string();
    }
    annotate_resume_links(&mut html_body_en);

    let markdown_ru = fs::read_to_string("profiles/cv/ru/CV_RU.MD")?;
    let parser_ru = CmarkParser::new_ext(&markdown_ru, Options::all());
    let mut html_body_ru = String::new();
    push_html(&mut html_body_ru, parser_ru);
    html_body_ru = html_body_ru.replace("../en/CV.MD", "../");
    for theme in THEME_VARIANTS {
        let ru_pdf = format!("{PDF_BASE_URL}Belyakov_ru_{theme}.pdf");
        let en_pdf = format!("{PDF_BASE_URL}Belyakov_en_{theme}.pdf");
        let ru_local = format!("../Belyakov_ru_{theme}.pdf");
        let en_local = format!("../Belyakov_en_{theme}.pdf");
        html_body_ru = html_body_ru.replace(&ru_pdf, &ru_local);
        html_body_ru = html_body_ru.replace(&en_pdf, &en_local);
    }
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
    annotate_resume_links(&mut html_body_ru);

    let footer_links_en = extract_first_paragraph(&html_body_en);
    let footer_links_ru = extract_first_paragraph(&html_body_ru);

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
        roles_js: &roles_js_en,
        roles_js_ru: &roles_js_ru,
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
        roles_js: &roles_js_en,
        roles_js_ru: &roles_js_ru,
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
    for theme in THEME_VARIANTS {
        let en_name = format!("Belyakov_en_{}.pdf", theme);
        let ru_name = format!("Belyakov_ru_{}.pdf", theme);
        let en_src = Path::new("typst/en").join(&en_name);
        let ru_src = Path::new("typst/ru").join(&ru_name);
        let en_dst = docs_dir.join(&en_name);
        let ru_dst = docs_dir.join(&ru_name);
        compile_and_copy_pdf(&en_src, &en_dst)?;
        compile_and_copy_pdf(&ru_src, &ru_dst)?;
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
    for (role, title) in &roles {
        sitemap_urls.push(format!("{base_url}{role}/"));
        sitemap_urls.push(format!("{base_url}{role}/ru/"));

        let en_role_dir = docs_dir.join(role);
        if !en_role_dir.exists() {
            fs::create_dir_all(&en_role_dir)?;
        }
        let mut html_body_en_role = html_body_en.clone();
        for theme in THEME_VARIANTS {
            let base_en = format!("Belyakov_en_{}.pdf", theme);
            let base_ru = format!("Belyakov_ru_{}.pdf", theme);
            let rel_en = format!("../Belyakov_en_{}.pdf", theme);
            let rel_ru = format!("../Belyakov_ru_{}.pdf", theme);
            html_body_en_role = html_body_en_role.replace(&base_en, &rel_en);
            html_body_en_role = html_body_en_role.replace(&base_ru, &rel_ru);
        }
        annotate_resume_links(&mut html_body_en_role);
        let footer_links_en_role = extract_first_paragraph(&html_body_en_role);
        let position_block_role = DEFAULT_ROLE
            .map(|_| position_block.clone())
            .unwrap_or_else(|| format!("<p><strong id='position'>{title}</strong></p>"));
        let en_role_html = render_page(&TemplateData {
            lang: "en",
            title: "Alexey Belyakov - CV",
            name: "Alexey Belyakov",
            prefix: "../",
            position_block: &position_block_role,
            date_str: &date_str,
            avatar_src: "../avatar.jpg",
            html_body: &html_body_en_role,
            footer_links: &footer_links_en_role,
            roles_js: &roles_js_en,
            roles_js_ru: &roles_js_ru,
            link_to_en: None,
        })?;
        fs::write(en_role_dir.join("index.html"), en_role_html)?;

        let ru_role_dir = en_role_dir.join("ru");
        if !ru_role_dir.exists() {
            fs::create_dir_all(&ru_role_dir)?;
        }
        let mut html_body_ru_role = html_body_ru.clone();
        for theme in THEME_VARIANTS {
            let base_ru = format!("../Belyakov_ru_{}.pdf", theme);
            let base_en = format!("../Belyakov_en_{}.pdf", theme);
            let rel_ru = format!("../../Belyakov_ru_{}.pdf", theme);
            let rel_en = format!("../../Belyakov_en_{}.pdf", theme);
            html_body_ru_role = html_body_ru_role.replace(&base_ru, &rel_ru);
            html_body_ru_role = html_body_ru_role.replace(&base_en, &rel_en);
        }
        annotate_resume_links(&mut html_body_ru_role);
        let footer_links_ru_role = extract_first_paragraph(&html_body_ru_role);
        let role_title_ru = role_title_ru_genitive(role);
        let position_block_role = DEFAULT_ROLE
            .map(|_| position_block.clone())
            .unwrap_or_else(|| format!("<p><strong id='position'>{}</strong></p>", role_title_ru));
        let ru_role_html = render_page(&TemplateData {
            lang: "ru",
            title: "Алексей Беляков - Резюме",
            name: "Алексей Беляков",
            prefix: "../../",
            position_block: &position_block_role,
            date_str: &date_str,
            avatar_src: "../../avatar.jpg",
            html_body: &html_body_ru_role,
            footer_links: &footer_links_ru_role,
            roles_js: &roles_js_en,
            roles_js_ru: &roles_js_ru,
            link_to_en: None,
        })?;
        fs::write(ru_role_dir.join("index.html"), ru_role_html)?;
    }
    let sitemap_content = sitemap_urls.join("\n") + "\n";
    fs::write(docs_dir.join("sitemap.txt"), sitemap_content)?;
    info!("Wrote sitemap to dist/sitemap.txt");
    info!("Site generation completed");
    Ok(())
}
