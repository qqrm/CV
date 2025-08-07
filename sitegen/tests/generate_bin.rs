use chrono::{Datelike, NaiveDate, Utc};
use sitegen::{format_duration_en, format_duration_ru};
use std::{fs, path::PathBuf, process::Command};

struct DistCleanup(PathBuf);
impl Drop for DistCleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn generates_expected_dist() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let dist = root.join("dist");
    if dist.exists() {
        fs::remove_dir_all(&dist).unwrap();
    }
    let _guard = DistCleanup(dist.clone());

    let status = Command::new("cargo")
        .args(["run", "--manifest-path", "sitegen/Cargo.toml", "--bin", "generate"])
        .current_dir(&root)
        .status()
        .expect("failed to run generator");
    assert!(status.success());

    let start = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
    let today = Utc::now().date_naive();
    let total_months = (today.year() - start.year()) * 12 + (today.month() as i32 - start.month() as i32);
    let date_str = today.format("%Y-%m-%d").to_string();
    let duration_en = format_duration_en(total_months);
    let duration_ru = format_duration_ru(total_months);

    let fixtures = root.join("sitegen/tests/fixtures");
    let expected_en = fs::read_to_string(fixtures.join("dist/index.html"))
        .unwrap()
        .replace("{{DATE}}", &date_str)
        .replace("{{DURATION_EN}}", &duration_en);
    let actual_en = fs::read_to_string(dist.join("index.html")).unwrap();
    assert_eq!(actual_en, expected_en);

    let expected_ru = fs::read_to_string(fixtures.join("dist/ru/index.html"))
        .unwrap()
        .replace("{{DATE}}", &date_str)
        .replace("{{DURATION_RU}}", &duration_ru);
    let actual_ru = fs::read_to_string(dist.join("ru/index.html")).unwrap();
    assert_eq!(actual_ru, expected_ru);

    let roles = ["tl", "em", "hod", "tech"];
    for role in roles {
        let actual_role_en = fs::read_to_string(dist.join(role).join("index.html")).unwrap();
        let expected_role_en = expected_en
            .replace("Belyakov_en_typst.pdf", &format!("Belyakov_en_{role}_typst.pdf"))
            .replace("Belyakov_ru_typst.pdf", &format!("Belyakov_ru_{role}_typst.pdf"));
        assert_eq!(actual_role_en, expected_role_en);

        let actual_role_ru = fs::read_to_string(dist.join("ru").join(role).join("index.html")).unwrap();
        let expected_role_ru = expected_ru
            .replace("Belyakov_en_typst.pdf", &format!("Belyakov_en_{role}_typst.pdf"))
            .replace("Belyakov_ru_typst.pdf", &format!("Belyakov_ru_{role}_typst.pdf"));
        assert_eq!(actual_role_ru, expected_role_ru);
    }

    assert_eq!(
        fs::read(root.join("content/avatar.jpg")).unwrap(),
        fs::read(dist.join("avatar.jpg")).unwrap()
    );
    assert_eq!(
        fs::read(root.join("docs/style.css")).unwrap(),
        fs::read(dist.join("style.css")).unwrap()
    );
    assert_eq!(
        fs::read(root.join("docs/favicon.svg")).unwrap(),
        fs::read(dist.join("favicon.svg")).unwrap()
    );
}
