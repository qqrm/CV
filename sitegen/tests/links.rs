use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use scraper::{Html, Selector};
use walkdir::WalkDir;

fn is_external(link: &str) -> bool {
    link.starts_with("http://") || link.starts_with("https://")
}

fn is_ignorable(link: &str) -> bool {
    link.starts_with('#') || link.starts_with("mailto:") || link.starts_with("tel:")
}

#[test]
fn all_links_are_valid() {
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
    let selector = Selector::parse("a").unwrap();
    let mut errors = Vec::new();

    for entry in WalkDir::new(&dist).into_iter().filter_map(Result::ok) {
        if entry.path().extension().and_then(|s| s.to_str()) != Some("html") {
            continue;
        }
        let html = fs::read_to_string(entry.path()).expect("read html");
        let doc = Html::parse_document(&html);
        for el in doc.select(&selector) {
            if let Some(href) = el.value().attr("href") {
                if is_ignorable(href) {
                    continue;
                }
                if is_external(href) {
                    match ureq::head(href).call().or_else(|_| ureq::get(href).call()) {
                        Ok(resp) if resp.status() < 400 => {}
                        Ok(resp) => errors.push(format!("{} -> {}", href, resp.status())),
                        Err(e) => {
                            let msg = e.to_string();
                            if !msg.contains("Network is unreachable")
                                && !msg.contains("failed to lookup address information")
                            {
                                errors.push(format!("{} -> {}", href, msg));
                            }
                        }
                    }
                } else {
                    let path = resolve(entry.path(), href);
                    if !path.exists() {
                        errors.push(format!(
                            "{} references missing {}",
                            entry.path().display(),
                            href
                        ));
                    }
                }
            }
        }
    }

    if !errors.is_empty() {
        panic!("Broken links:\n{}", errors.join("\n"));
    }

    fs::remove_dir_all(&dist).expect("remove dist");
}

fn resolve(base: &Path, link: &str) -> PathBuf {
    let mut parent = base.parent().unwrap().to_path_buf();
    let mut target = Path::new(link);
    while target.starts_with("..") {
        parent.pop();
        target = target.strip_prefix("..").unwrap();
    }
    parent.join(target)
}
