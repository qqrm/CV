use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const PDF_FILES: [(&str, &str); 12] = [
    ("typst/en", "Belyakov_en_light.pdf"),
    ("typst/en", "Belyakov_en_dark.pdf"),
    ("typst/ru", "Belyakov_ru_light.pdf"),
    ("typst/ru", "Belyakov_ru_dark.pdf"),
    ("typst/en", "Belyakov_rustdev_en_light.pdf"),
    ("typst/en", "Belyakov_rustdev_en_dark.pdf"),
    ("typst/ru", "Belyakov_rustdev_ru_light.pdf"),
    ("typst/ru", "Belyakov_rustdev_ru_dark.pdf"),
    ("typst/en", "Belyakov_cto_en_light.pdf"),
    ("typst/en", "Belyakov_cto_en_dark.pdf"),
    ("typst/ru", "Belyakov_cto_ru_light.pdf"),
    ("typst/ru", "Belyakov_cto_ru_dark.pdf"),
];

#[test]
#[serial_test::serial]
fn generate_creates_sitemap_and_copies_canonical_pdfs() {
    let temp = tempfile::tempdir().expect("tempdir");
    let original_dir = env::current_dir().expect("current dir");
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf();
    env::set_current_dir(temp.path()).expect("set temp dir");

    for (dir, file_name) in PDF_FILES {
        fs::create_dir_all(dir).expect("create typst dir");
        fs::write(Path::new(dir).join(file_name), b"%PDF-1.7\n").expect("write fake pdf");
    }

    let output_dir = temp.path().join("out");
    let status = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            &repo_root.join("sitegen/Cargo.toml").display().to_string(),
            "--bin",
            "generate",
            "--",
            "--out",
            output_dir.to_str().expect("utf8 path"),
        ])
        .status()
        .expect("run generate");

    env::set_current_dir(original_dir).expect("restore cwd");
    assert!(status.success(), "generate command failed");

    for (_, file_name) in PDF_FILES {
        let path = output_dir.join(file_name);
        assert!(path.is_file(), "missing generated pdf: {}", path.display());
        let content = fs::read(path).expect("read output pdf");
        assert!(content.starts_with(b"%PDF"), "invalid generated PDF header");
    }

    let sitemap = fs::read_to_string(output_dir.join("sitemap.txt")).expect("read sitemap");
    let expected = [
        "https://qqrm.github.io/CV/",
        "https://qqrm.github.io/CV/ru/",
        "https://qqrm.github.io/CV/rust-developer/",
        "https://qqrm.github.io/CV/rust-developer/ru/",
        "https://qqrm.github.io/CV/cto/",
        "https://qqrm.github.io/CV/cto/ru/",
    ];

    for route in expected {
        assert!(
            sitemap.contains(route),
            "sitemap is missing route {route}: {sitemap}"
        );
    }
}
