use std::fs;
use std::path::Path;
use std::process::Command;

use regex::Regex;
use toml::Value;

fn normalize_en(content: &str) -> String {
    let date_re = Regex::new(r"<p>\d{4}-\d{2}-\d{2}</p>").unwrap();
    let dur_re = Regex::new(r"March 2024 – Present  \([^)]*\)").unwrap();
    let tmp = date_re.replace(content, "<p>DATE</p>");
    let tmp = dur_re.replace(&tmp, "March 2024 – Present  (DURATION)");
    tmp.to_string()
}

fn normalize_ru(content: &str) -> String {
    let date_re = Regex::new(r"<p>\d{4}-\d{2}-\d{2}</p>").unwrap();
    let dur_re = Regex::new(r"март 2024 – настоящее время \([^)]*\)").unwrap();
    let tmp = date_re.replace(content, "<p>DATE</p>");
    let tmp = dur_re.replace(&tmp, "март 2024 – настоящее время (DURATION)");
    tmp.to_string()
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
        let role_dir = dist.join(slug);
        let en_path = role_dir.join("index.html");
        assert!(en_path.exists(), "missing {}/index.html", slug);
        let en_page = fs::read_to_string(&en_path).expect("read role index");
        assert!(
            en_page.contains(&format!("Belyakov_en_{}_typst.pdf", slug)),
            "missing English {} PDF link",
            slug
        );

        let ru_path = role_dir.join("ru").join("index.html");
        assert!(ru_path.exists(), "missing {}/ru/index.html", slug);
        let ru_page = fs::read_to_string(&ru_path).expect("read role ru index");
        assert!(
            ru_page.contains(&format!("Belyakov_ru_{}_typst.pdf", slug)),
            "missing Russian {} PDF link",
            slug
        );
    }

    fs::remove_dir_all(&dist).expect("failed to remove dist");
}
