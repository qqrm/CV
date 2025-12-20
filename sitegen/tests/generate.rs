use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;
use scraper::{Html, Selector};
use sitegen::{format_duration_en, format_duration_ru, read_inline_start};

const PDF_THEMES: &[&str] = &["light", "dark"];

fn normalize_en(content: &str) -> String {
    let date_re = Regex::new(
        r#"(?m)^(?P<indent>\s*)<p(?: class=['"]updated-at['"])?>(?:Last updated:\s*)?(?:\d{4}-\d{2}-\d{2})</p>"#,
    )
    .unwrap();
    let dur_re = Regex::new(r"([A-Za-z]+ \d{4} - Present)\s+\([^)]*\)").unwrap();
    let tmp = date_re.replace(content, "${indent}<p>DATE</p>");
    let tmp = dur_re.replace(&tmp, "$1 (DURATION)");
    tmp.to_string()
}

fn normalize_ru(content: &str) -> String {
    let date_re = Regex::new(
        r#"(?m)^(?P<indent>\s*)<p(?: class=['"]updated-at['"])?>(?:Последнее редактирование:\s*)?(?:\d{4}-\d{2}-\d{2})</p>"#,
    )
    .unwrap();
    let dur_re =
        Regex::new(r"(\p{L}+ \d{4} - (?:настоящее время|Настоящее время|Present))\s*\([^)]*\)")
            .unwrap();
    let tmp = date_re.replace(content, "${indent}<p>DATE</p>");
    let tmp = dur_re.replace(&tmp, "$1 (DURATION)");
    tmp.to_string()
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

fn assert_pdf_link_attrs(html: &str, light: &str, dark: &str) {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("a[data-light-href], a[data-dark-href]").expect("valid selector");
    let anchors: Vec<_> = document
        .select(&selector)
        .filter(|node| {
            let value = node.value();
            value.attr("data-light-href") == Some(light)
                && value.attr("data-dark-href") == Some(dark)
        })
        .collect();

    assert!(
        !anchors.is_empty(),
        "expected at least one anchor with light='{light}' and dark='{dark}'"
    );

    for anchor in anchors {
        let element = anchor.value();

        assert_eq!(
            element.attr("data-light-href"),
            Some(light),
            "missing data-light-href for {light}"
        );
        assert_eq!(
            element.attr("data-dark-href"),
            Some(dark),
            "missing data-dark-href for {dark}"
        );

        let light_label = element
            .attr("data-light-label")
            .unwrap_or_else(|| panic!("missing data-light-label for {light}"));
        let dark_label = element
            .attr("data-dark-label")
            .unwrap_or_else(|| panic!("missing data-dark-label for {dark}"));

        let href = element.attr("href").unwrap_or("");
        let text = anchor.text().collect::<String>();
        let text = text.trim();

        assert_eq!(
            href, light,
            "anchor should default to light variant for {light}/{dark}"
        );
        assert_eq!(
            text, light_label,
            "anchor text should match light label for {light}/{dark}"
        );
        assert_eq!(
            text, dark_label,
            "dark label should match anchor text for {light}/{dark}"
        );
    }
}

fn assert_no_pdf_link_attrs(html: &str, light: &str, dark: &str) {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("a[data-light-href], a[data-dark-href]").expect("valid selector");
    let has_match = document.select(&selector).any(|node| {
        let element = node.value();
        element.attr("data-light-href") == Some(light)
            && element.attr("data-dark-href") == Some(dark)
    });

    assert!(
        !has_match,
        "unexpected anchor with light='{light}' and dark='{dark}'"
    );
}

#[test]
#[serial_test::serial]
fn generates_expected_dist() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project_root = crate_dir.parent().expect("project root");

    let status = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            "sitegen/Cargo.toml",
            "--bin",
            "generate",
        ])
        .current_dir(project_root)
        .status()
        .expect("failed to run generate");
    assert!(status.success(), "generate command failed");

    let dist = project_root.join("dist");
    let index_actual = fs::read_to_string(dist.join("index.html")).expect("read index.html");
    let index_ru_actual =
        fs::read_to_string(dist.join("ru").join("index.html")).expect("read ru/index.html");

    for theme in PDF_THEMES {
        assert!(
            dist.join(format!("Belyakov_en_{}.pdf", theme)).exists(),
            "missing dist/Belyakov_en_{}.pdf",
            theme
        );
        assert!(
            dist.join(format!("Belyakov_ru_{}.pdf", theme)).exists(),
            "missing dist/Belyakov_ru_{}.pdf",
            theme
        );
    }

    let original_dir = env::current_dir().expect("current dir");
    env::set_current_dir(project_root).expect("set project root");
    let (start_year, start_month) = read_inline_start().expect("read inline start");
    env::set_current_dir(original_dir).expect("restore current dir");
    let start_date =
        NaiveDate::from_ymd_opt(start_year, start_month, 1).expect("valid inline start date");
    let today = Utc::now().date_naive();
    let total_months = (today.year() - start_date.year()) * 12
        + (today.month() as i32 - start_date.month() as i32);
    let expected_duration_en = format_duration_en(total_months);
    let expected_duration_ru = format_duration_ru(total_months);

    let english_fragment = format!("{} - Present", start_date.format("%B %Y"));
    let english_duration_re = Regex::new(&format!(
        "{}\\s*\\(([^)]*)\\)",
        regex::escape(&english_fragment)
    ))
    .unwrap();
    let mut en_matches = english_duration_re.captures_iter(&index_actual);
    let en_caps = en_matches
        .next()
        .expect("English duration fragment not found");
    assert!(
        en_matches.next().is_none(),
        "English inline duration fragment matched more than once"
    );
    let actual_duration_en = en_caps.get(1).map(|m| m.as_str()).unwrap();
    assert_eq!(actual_duration_en, expected_duration_en);

    let mut ru_fragments = vec![english_fragment.clone()];
    if let Some(month_name) = russian_month_name(start_month) {
        let month_title = capitalize_first(month_name);
        ru_fragments.extend([
            format!("{month_name} {start_year} - настоящее время"),
            format!("{month_name} {start_year} - Настоящее время"),
            format!("{month_title} {start_year} - настоящее время"),
            format!("{month_title} {start_year} - Настоящее время"),
        ]);
    }
    let mut ru_duration: Option<String> = None;
    for fragment in ru_fragments {
        let re = Regex::new(&format!("{}\\s*\\(([^)]*)\\)", regex::escape(&fragment))).unwrap();
        let mut matches = re.captures_iter(&index_ru_actual);
        if let Some(caps) = matches.next() {
            assert!(
                matches.next().is_none(),
                "Russian inline duration fragment '{fragment}' matched more than once"
            );
            ru_duration = Some(caps.get(1).unwrap().as_str().to_string());
            break;
        }
    }
    let actual_duration_ru = ru_duration.expect("Russian duration fragment not found");
    assert_eq!(actual_duration_ru, expected_duration_ru);

    assert!(
        index_actual.contains("Last updated:"),
        "English page is missing the last updated label"
    );
    assert!(
        index_ru_actual.contains("Последнее редактирование:"),
        "Russian page is missing the last updated label"
    );

    let index_normalized = normalize_en(&index_actual);
    let index_ru_normalized = normalize_ru(&index_ru_actual);

    let fixtures = crate_dir.join("tests").join("fixtures");
    let index_expected =
        fs::read_to_string(fixtures.join("index.html")).expect("expected index.html");
    let index_ru_expected =
        fs::read_to_string(fixtures.join("ru").join("index.html")).expect("expected ru/index.html");

    assert_eq!(index_normalized, index_expected);
    assert_eq!(index_ru_normalized, index_ru_expected);

    assert_pdf_link_attrs(
        &index_actual,
        "Belyakov_en_light.pdf",
        "Belyakov_en_dark.pdf",
    );
    assert_no_pdf_link_attrs(
        &index_actual,
        "Belyakov_ru_light.pdf",
        "Belyakov_ru_dark.pdf",
    );
    assert_pdf_link_attrs(
        &index_ru_actual,
        "../Belyakov_ru_light.pdf",
        "../Belyakov_ru_dark.pdf",
    );
    assert_no_pdf_link_attrs(
        &index_ru_actual,
        "../Belyakov_en_light.pdf",
        "../Belyakov_en_dark.pdf",
    );

    let button_selector = Selector::parse("button#language-toggle").expect("valid selector");
    let en_doc = Html::parse_document(&index_actual);
    let en_button = en_doc
        .select(&button_selector)
        .next()
        .expect("expected language toggle on English page");
    let en_value = en_button.value();
    assert_eq!(en_value.attr("data-target"), Some("ru/"));
    assert_eq!(
        en_value.attr("aria-label"),
        Some("Switch to Russian version"),
        "unexpected aria-label on English language toggle"
    );
    let option_selector = Selector::parse("span.lang-option").expect("valid option selector");
    let current_selector =
        Selector::parse("span.lang-option.current").expect("valid current selector");

    let en_options: Vec<String> = en_button
        .select(&option_selector)
        .map(|span| span.text().collect::<String>().trim().to_string())
        .collect();
    assert_eq!(en_options, vec!["EN".to_string(), "RU".to_string()]);

    let en_current = en_button
        .select(&current_selector)
        .next()
        .expect("expected current language span on English page");
    let en_current_label = en_current.text().collect::<String>().trim().to_string();
    assert_eq!(en_current_label, "EN");

    let ru_doc = Html::parse_document(&index_ru_actual);
    let ru_button = ru_doc
        .select(&button_selector)
        .next()
        .expect("expected language toggle on Russian page");
    let ru_value = ru_button.value();
    assert_eq!(ru_value.attr("data-target"), Some("../"));
    assert_eq!(
        ru_value.attr("aria-label"),
        Some("Переключить на английскую версию"),
        "unexpected aria-label on Russian language toggle"
    );

    let ru_options: Vec<String> = ru_button
        .select(&option_selector)
        .map(|span| span.text().collect::<String>().trim().to_string())
        .collect();
    assert_eq!(ru_options, vec!["RU".to_string(), "EN".to_string()]);

    let ru_current = ru_button
        .select(&current_selector)
        .next()
        .expect("expected current language span on Russian page");
    let ru_current_label = ru_current.text().collect::<String>().trim().to_string();
    assert_eq!(ru_current_label, "RU");

    fs::remove_dir_all(&dist).expect("failed to remove dist");
}
