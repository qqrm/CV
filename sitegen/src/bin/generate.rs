use chrono::{Datelike, NaiveDate, Utc};
use handlebars::Handlebars;
use html_escape::{decode_html_entities, encode_double_quoted_attribute, encode_safe};
use log::{info, warn};
use pulldown_cmark::{Options, Parser as CmarkParser, html::push_html};
use regex::Regex;
use serde::Serialize;
use sitegen::parser::read_inline_start;
use sitegen::renderer::{format_duration_en, format_duration_ru};
use std::error::Error;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;

const PDF_BASE_URL: &str = "https://qqrm.github.io/CV/";
const GITHUB_URL: &str = "https://github.com/qqrm";
const TELEGRAM_URL: &str = "https://leqqrm.t.me";
const EMAIL_URL: &str = "mailto:qqrm@vivaldi.net";
const LINKEDIN_URL: &str = "https://www.linkedin.com/in/qqrm/";
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
    footer_text: &'a str,
    avatar_src: &'a str,
    html_body: &'a str,
    header_actions: &'a str,
    lang_toggle_label: &'a str,
    lang_toggle_target: &'a str,
    lang_toggle_current: &'a str,
    lang_toggle_other: &'a str,
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

struct LocaleConfig<'a> {
    lang: &'a str,
    title: &'a str,
    name: &'a str,
    markdown_path: &'a str,
    output_dir: &'a str,
    prefix: &'a str,
    avatar_src: &'a str,
    body_link_from: &'a str,
    body_link_to: &'a str,
    header_pdf_prefix: &'a str,
    lang_toggle_label: &'a str,
    lang_toggle_target: &'a str,
    lang_toggle_current: &'a str,
    lang_toggle_other: &'a str,
}

struct ProfileConfig<'a> {
    sitemap_path: &'a str,
    pdf_name_prefix: &'a str,
    en: LocaleConfig<'a>,
    ru: LocaleConfig<'a>,
}

fn build_header_actions(pdf_prefix: &str, is_ru: bool) -> String {
    let download_label = if is_ru { "Скачать" } else { "Download" };
    let email_label = if is_ru { "Почта" } else { "Email" };
    format!(
        "<nav class=\"header-actions\">\
<a class=\"action\" href=\"{pdf_prefix}_light.pdf\" data-light-href=\"{pdf_prefix}_light.pdf\" data-dark-href=\"{pdf_prefix}_dark.pdf\" data-light-label=\"{download_label}\" data-dark-label=\"{download_label}\">{download_label}</a>\
<a class=\"action\" href=\"{GITHUB_URL}\" rel=\"noopener\">GitHub</a>\
<a class=\"action\" href=\"{EMAIL_URL}\">{email_label}</a>\
<a class=\"action\" href=\"{TELEGRAM_URL}\" rel=\"noopener\">Telegram</a>\
<a class=\"action\" href=\"{LINKEDIN_URL}\" rel=\"noopener\">LinkedIn</a>\
</nav>"
    )
}

fn render_markdown_html(
    locale: &LocaleConfig,
    duration_data: &Option<(String, String, String, Vec<String>)>,
) -> Result<String, Box<dyn Error>> {
    let markdown = fs::read_to_string(locale.markdown_path)?;
    let parser = CmarkParser::new_ext(&markdown, Options::all());
    let mut html_body = String::new();
    push_html(&mut html_body, parser);

    html_body = html_body.replace(locale.body_link_from, locale.body_link_to);

    for theme in THEME_VARIANTS {
        let base_en_pdf = format!("{PDF_BASE_URL}Belyakov_en_{theme}.pdf");
        let base_ru_pdf = format!("{PDF_BASE_URL}Belyakov_ru_{theme}.pdf");
        let rust_en_pdf = format!("{PDF_BASE_URL}Belyakov_rustdev_en_{theme}.pdf");
        let rust_ru_pdf = format!("{PDF_BASE_URL}Belyakov_rustdev_ru_{theme}.pdf");
        let cto_en_pdf = format!("{PDF_BASE_URL}Belyakov_cto_en_{theme}.pdf");
        let cto_ru_pdf = format!("{PDF_BASE_URL}Belyakov_cto_ru_{theme}.pdf");

        html_body = html_body.replace(
            &base_en_pdf,
            &format!("{}Belyakov_en_{theme}.pdf", locale.prefix),
        );
        html_body = html_body.replace(
            &base_ru_pdf,
            &format!("{}Belyakov_ru_{theme}.pdf", locale.prefix),
        );
        html_body = html_body.replace(
            &rust_en_pdf,
            &format!("{}Belyakov_rustdev_en_{theme}.pdf", locale.prefix),
        );
        html_body = html_body.replace(
            &rust_ru_pdf,
            &format!("{}Belyakov_rustdev_ru_{theme}.pdf", locale.prefix),
        );
        html_body = html_body.replace(
            &cto_en_pdf,
            &format!("{}Belyakov_cto_en_{theme}.pdf", locale.prefix),
        );
        html_body = html_body.replace(
            &cto_ru_pdf,
            &format!("{}Belyakov_cto_ru_{theme}.pdf", locale.prefix),
        );
    }

    if let Some((english_fragment, duration_en, duration_ru, ru_fragments)) = duration_data {
        if locale.lang == "en" {
            if !inject_duration(&mut html_body, english_fragment, duration_en) {
                warn!(
                    "English inline duration fragment '{english_fragment}' not found in generated HTML"
                );
            }
        } else {
            let mut injected_ru = false;
            for fragment in ru_fragments {
                if inject_duration(&mut html_body, fragment, duration_ru) {
                    injected_ru = true;
                    break;
                }
            }
            if !injected_ru {
                warn!("Russian inline duration fragment not found in generated HTML");
            }
        }
    }

    if let Some(end) = html_body.find("</h1>") {
        html_body = html_body[end + 5..].trim_start().to_string();
    }
    annotate_resume_links(&mut html_body);
    Ok(html_body)
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting site generation");

    let inline_start = match read_inline_start() {
        Ok(value) => Some(value),
        Err(e) => {
            warn!("Failed to read inline start: {e}");
            None
        }
    };

    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
    }

    fs::copy("content/avatar.jpg", dist_dir.join("avatar.jpg"))?;
    if Path::new("DOCS/style.css").exists() {
        fs::copy("DOCS/style.css", dist_dir.join("style.css"))?;
    }
    if Path::new("DOCS/favicon.svg").exists() {
        fs::copy("DOCS/favicon.svg", dist_dir.join("favicon.svg"))?;
    }

    let today = Utc::now().date_naive();
    let date_str = today.format("%Y-%m-%d").to_string();
    let footer_text_en = format!("Last updated: {}", date_str);
    let footer_text_ru = format!("Последнее редактирование: {}", date_str);

    let duration_data = inline_start.and_then(|(year, month)| {
        let start_date = NaiveDate::from_ymd_opt(year, month, 1)?;
        let total_months = (today.year() - start_date.year()) * 12
            + (today.month() as i32 - start_date.month() as i32);
        let duration_en = format_duration_en(total_months);
        let duration_ru = format_duration_ru(total_months);
        let english_fragment = format!("{} - Present", start_date.format("%B %Y"));
        let mut ru_fragments = vec![english_fragment.clone()];
        if let Some(month_name) = russian_month_name(month) {
            let month_title = capitalize_first(month_name);
            ru_fragments.extend([
                format!("{month_name} {year} - настоящее время"),
                format!("{month_name} {year} - Настоящее время"),
                format!("{month_title} {year} - настоящее время"),
                format!("{month_title} {year} - Настоящее время"),
            ]);
        }
        Some((english_fragment, duration_en, duration_ru, ru_fragments))
    });

    let profiles = [
        ProfileConfig {
            sitemap_path: "",
            pdf_name_prefix: "Belyakov",
            en: LocaleConfig {
                lang: "en",
                title: "Alexey Belyakov - CV",
                name: "Alexey Belyakov",
                markdown_path: "profiles/cv/en/CV.MD",
                output_dir: "",
                prefix: "",
                avatar_src: "avatar.jpg",
                body_link_from: "../ru/CV_RU.MD",
                body_link_to: "ru/",
                header_pdf_prefix: "Belyakov_en",
                lang_toggle_label: "Switch to Russian version",
                lang_toggle_target: "ru/",
                lang_toggle_current: "EN",
                lang_toggle_other: "RU",
            },
            ru: LocaleConfig {
                lang: "ru",
                title: "Алексей Беляков - Резюме",
                name: "Алексей Беляков",
                markdown_path: "profiles/cv/ru/CV_RU.MD",
                output_dir: "ru",
                prefix: "../",
                avatar_src: "../avatar.jpg",
                body_link_from: "../en/CV.MD",
                body_link_to: "../",
                header_pdf_prefix: "../Belyakov_ru",
                lang_toggle_label: "Переключить на английскую версию",
                lang_toggle_target: "../",
                lang_toggle_current: "RU",
                lang_toggle_other: "EN",
            },
        },
        ProfileConfig {
            sitemap_path: "rust-developer/",
            pdf_name_prefix: "Belyakov_rustdev",
            en: LocaleConfig {
                lang: "en",
                title: "Alexey Belyakov - Rust Developer CV",
                name: "Alexey Belyakov",
                markdown_path: "profiles/rust-developer/en/CV.MD",
                output_dir: "rust-developer",
                prefix: "../",
                avatar_src: "../avatar.jpg",
                body_link_from: "../ru/CV_RU.MD",
                body_link_to: "ru/",
                header_pdf_prefix: "../Belyakov_rustdev_en",
                lang_toggle_label: "Switch to Russian version",
                lang_toggle_target: "ru/",
                lang_toggle_current: "EN",
                lang_toggle_other: "RU",
            },
            ru: LocaleConfig {
                lang: "ru",
                title: "Алексей Беляков - Rust Developer Резюме",
                name: "Алексей Беляков",
                markdown_path: "profiles/rust-developer/ru/CV_RU.MD",
                output_dir: "rust-developer/ru",
                prefix: "../../",
                avatar_src: "../../avatar.jpg",
                body_link_from: "../en/CV.MD",
                body_link_to: "../",
                header_pdf_prefix: "../../Belyakov_rustdev_ru",
                lang_toggle_label: "Переключить на английскую версию",
                lang_toggle_target: "../",
                lang_toggle_current: "RU",
                lang_toggle_other: "EN",
            },
        },
        ProfileConfig {
            sitemap_path: "cto/",
            pdf_name_prefix: "Belyakov_cto",
            en: LocaleConfig {
                lang: "en",
                title: "Alexey Belyakov - CTO CV",
                name: "Alexey Belyakov",
                markdown_path: "profiles/cto/en/CV.MD",
                output_dir: "cto",
                prefix: "../",
                avatar_src: "../avatar.jpg",
                body_link_from: "../ru/CV_RU.MD",
                body_link_to: "ru/",
                header_pdf_prefix: "../Belyakov_cto_en",
                lang_toggle_label: "Switch to Russian version",
                lang_toggle_target: "ru/",
                lang_toggle_current: "EN",
                lang_toggle_other: "RU",
            },
            ru: LocaleConfig {
                lang: "ru",
                title: "Алексей Беляков - CTO Резюме",
                name: "Алексей Беляков",
                markdown_path: "profiles/cto/ru/CV_RU.MD",
                output_dir: "cto/ru",
                prefix: "../../",
                avatar_src: "../../avatar.jpg",
                body_link_from: "../en/CV.MD",
                body_link_to: "../",
                header_pdf_prefix: "../../Belyakov_cto_ru",
                lang_toggle_label: "Переключить на английскую версию",
                lang_toggle_target: "../",
                lang_toggle_current: "RU",
                lang_toggle_other: "EN",
            },
        },
    ];

    let mut sitemap_urls = vec![PDF_BASE_URL.to_string(), format!("{PDF_BASE_URL}ru/")];

    for profile in &profiles {
        for locale in [&profile.en, &profile.ru] {
            let html_body = render_markdown_html(locale, &duration_data)?;
            let header_actions =
                build_header_actions(locale.header_pdf_prefix, locale.lang == "ru");
            let footer_text = if locale.lang == "ru" {
                &footer_text_ru
            } else {
                &footer_text_en
            };
            let page = render_page(&TemplateData {
                lang: locale.lang,
                title: locale.title,
                name: locale.name,
                prefix: locale.prefix,
                footer_text,
                avatar_src: locale.avatar_src,
                html_body: &html_body,
                header_actions: &header_actions,
                lang_toggle_label: locale.lang_toggle_label,
                lang_toggle_target: locale.lang_toggle_target,
                lang_toggle_current: locale.lang_toggle_current,
                lang_toggle_other: locale.lang_toggle_other,
            })?;

            let out_dir = dist_dir.join(locale.output_dir);
            fs::create_dir_all(&out_dir)?;
            fs::write(out_dir.join("index.html"), page)?;
        }

        if !profile.sitemap_path.is_empty() {
            sitemap_urls.push(format!("{PDF_BASE_URL}{}", profile.sitemap_path));
            sitemap_urls.push(format!("{PDF_BASE_URL}{}ru/", profile.sitemap_path));
        }

        for theme in THEME_VARIANTS {
            let en_name = format!("{}_en_{theme}.pdf", profile.pdf_name_prefix);
            let ru_name = format!("{}_ru_{theme}.pdf", profile.pdf_name_prefix);
            let en_src = Path::new("typst/en").join(&en_name);
            let ru_src = Path::new("typst/ru").join(&ru_name);
            let en_dst = dist_dir.join(&en_name);
            let ru_dst = dist_dir.join(&ru_name);
            compile_and_copy_pdf(&en_src, &en_dst)?;
            compile_and_copy_pdf(&ru_src, &ru_dst)?;
        }
    }

    let sitemap_content = sitemap_urls.join(
        "
",
    ) + "
";
    fs::write(dist_dir.join("sitemap.txt"), sitemap_content)?;
    info!("Site generation completed");
    Ok(())
}
