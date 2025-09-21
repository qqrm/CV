use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;
use sitegen::{format_duration_en, format_duration_ru, read_inline_start};
use toml::Value;

const PDF_THEMES: &[&str] = &["light", "dark"];

fn normalize_en(content: &str) -> String {
    let date_re = Regex::new(r"<p>\d{4}-\d{2}-\d{2}</p>").unwrap();
    let dur_re = Regex::new(r"([A-Za-z]+ \d{4} – Present)\s+\([^)]*\)").unwrap();
    let tmp = date_re.replace(content, "<p>DATE</p>");
    let tmp = dur_re.replace(&tmp, "$1 (DURATION)");
    tmp.to_string()
}

fn normalize_ru(content: &str) -> String {
    let date_re = Regex::new(r"<p>\d{4}-\d{2}-\d{2}</p>").unwrap();
    let dur_re =
        Regex::new(r"(\p{L}+ \d{4} – (?:настоящее время|Настоящее время|Present))\s*\([^)]*\)")
            .unwrap();
    let tmp = date_re.replace(content, "<p>DATE</p>");
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

fn assert_theme_link(content: &str, light: &str, dark: &str) {
    let pattern = format!(
        r#"<a[^>]*\bhref=\"{}\"[^>]*data-light-href=\"{}\"[^>]*data-dark-href=\"{}\"[^>]*>"#,
        regex::escape(light),
        regex::escape(light),
        regex::escape(dark)
    );
    let re = Regex::new(&pattern).expect("invalid resume link pattern");
    assert!(
        re.is_match(content),
        "missing themed resume link for light '{}' and dark '{}'",
        light,
        dark
    );
    let dark_pattern = format!(r#"<a[^>]*\s+href=\"{}\""#, regex::escape(dark));
    let dark_re = Regex::new(&dark_pattern).expect("invalid dark href pattern");
    assert!(
        !dark_re.is_match(content),
        "found dark href '{}' instead of themed link",
        dark
    );
}

#[test]
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

    assert_theme_link(
        &index_actual,
        "Belyakov_en_light.pdf",
        "Belyakov_en_dark.pdf",
    );
    assert_theme_link(
        &index_actual,
        "Belyakov_ru_light.pdf",
        "Belyakov_ru_dark.pdf",
    );
    assert_theme_link(
        &index_ru_actual,
        "../Belyakov_ru_light.pdf",
        "../Belyakov_ru_dark.pdf",
    );
    assert_theme_link(
        &index_ru_actual,
        "../Belyakov_en_light.pdf",
        "../Belyakov_en_dark.pdf",
    );

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

    let english_fragment = format!("{} – Present", start_date.format("%B %Y"));
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
            format!("{month_name} {start_year} – настоящее время"),
            format!("{month_name} {start_year} – Настоящее время"),
            format!("{month_title} {start_year} – настоящее время"),
            format!("{month_title} {start_year} – Настоящее время"),
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

    let index_normalized = normalize_en(&index_actual);
    let index_ru_normalized = normalize_ru(&index_ru_actual);

    let fixtures = crate_dir.join("tests").join("fixtures");
    let index_expected =
        fs::read_to_string(fixtures.join("index.html")).expect("expected index.html");
    let index_ru_expected =
        fs::read_to_string(fixtures.join("ru").join("index.html")).expect("expected ru/index.html");

    assert_eq!(index_normalized, index_expected);
    assert_eq!(index_ru_normalized, index_ru_expected);

    // Load role slugs and verify role-specific pages
    let roles_toml = fs::read_to_string(project_root.join("roles.toml")).expect("read roles.toml");
    let roles: Value = toml::from_str(&roles_toml).expect("parse roles.toml");
    let roles = roles
        .get("roles")
        .and_then(Value::as_table)
        .expect("roles table");

    for slug in roles.keys() {
        for theme in PDF_THEMES {
            assert!(
                dist.join(format!("Belyakov_{}_en_{}.pdf", slug, theme))
                    .exists(),
                "missing dist/Belyakov_{}_en_{}.pdf",
                slug,
                theme
            );
            assert!(
                dist.join(format!("Belyakov_{}_ru_{}.pdf", slug, theme))
                    .exists(),
                "missing dist/Belyakov_{}_ru_{}.pdf",
                slug,
                theme
            );
        }
        let role_dir = dist.join(slug);
        let en_path = role_dir.join("index.html");
        assert!(en_path.exists(), "missing {}/index.html", slug);
        let en_page = fs::read_to_string(&en_path).expect("read role index");
        assert_theme_link(
            &en_page,
            &format!("../Belyakov_{}_en_light.pdf", slug),
            &format!("../Belyakov_{}_en_dark.pdf", slug),
        );
        assert_theme_link(
            &en_page,
            &format!("../Belyakov_{}_ru_light.pdf", slug),
            &format!("../Belyakov_{}_ru_dark.pdf", slug),
        );

        let ru_path = role_dir.join("ru").join("index.html");
        assert!(ru_path.exists(), "missing {}/ru/index.html", slug);
        let ru_page = fs::read_to_string(&ru_path).expect("read role ru index");
        assert_theme_link(
            &ru_page,
            &format!("../../Belyakov_{}_ru_light.pdf", slug),
            &format!("../../Belyakov_{}_ru_dark.pdf", slug),
        );
        assert_theme_link(
            &ru_page,
            &format!("../../Belyakov_{}_en_light.pdf", slug),
            &format!("../../Belyakov_{}_en_dark.pdf", slug),
        );
    }

    for slug in roles.keys() {
        let resume_dir = dist.join("resume").join(slug);
        let en_path = resume_dir.join("index.html");
        assert!(en_path.exists(), "missing resume/{}/index.html", slug);
        let en_page =
            fs::read_to_string(&en_path).unwrap_or_else(|_| panic!("read resume {slug} index"));
        assert_theme_link(
            &en_page,
            &format!("../../Belyakov_{}_en_light.pdf", slug),
            &format!("../../Belyakov_{}_en_dark.pdf", slug),
        );
        assert_theme_link(
            &en_page,
            &format!("../../Belyakov_{}_ru_light.pdf", slug),
            &format!("../../Belyakov_{}_ru_dark.pdf", slug),
        );

        let ru_path = resume_dir.join("ru").join("index.html");
        assert!(ru_path.exists(), "missing resume/{}/ru/index.html", slug);
        let ru_page =
            fs::read_to_string(&ru_path).unwrap_or_else(|_| panic!("read resume {slug} ru index"));
        assert_theme_link(
            &ru_page,
            &format!("../../../Belyakov_{}_ru_light.pdf", slug),
            &format!("../../../Belyakov_{}_ru_dark.pdf", slug),
        );
        assert_theme_link(
            &ru_page,
            &format!("../../../Belyakov_{}_en_light.pdf", slug),
            &format!("../../../Belyakov_{}_en_dark.pdf", slug),
        );
    }

    fs::remove_dir_all(&dist).expect("failed to remove dist");
}
