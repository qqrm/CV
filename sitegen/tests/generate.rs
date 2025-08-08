use std::fs;
use std::path::Path;
use std::process::Command;
use regex::Regex;

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
    let index_ru_actual = fs::read_to_string(dist.join("ru").join("index.html")).expect("read ru/index.html");

    let index_normalized = normalize_en(&index_actual);
    let index_ru_normalized = normalize_ru(&index_ru_actual);

    let fixtures = crate_dir.join("tests").join("fixtures");
    let index_expected = fs::read_to_string(fixtures.join("index.html")).expect("expected index.html");
    let index_ru_expected = fs::read_to_string(fixtures.join("ru").join("index.html")).expect("expected ru/index.html");

    assert_eq!(index_normalized, index_expected);
    assert_eq!(index_ru_normalized, index_ru_expected);

    // Ensure role-specific pages link to the correct PDFs
    let em_page = fs::read_to_string(dist.join("em").join("index.html")).expect("read em/index.html");
    assert!(
        em_page.contains("Belyakov_en_em_typst.pdf"),
        "missing English EM PDF link"
    );
    let em_ru_page = fs::read_to_string(dist.join("ru").join("em").join("index.html")).expect("read ru/em/index.html");
    assert!(
        em_ru_page.contains("Belyakov_ru_em_typst.pdf"),
        "missing Russian EM PDF link"
    );

    fs::remove_dir_all(&dist).expect("failed to remove dist");
}
