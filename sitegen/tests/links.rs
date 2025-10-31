use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use scraper::{Html, Selector};
use ureq::tls::{RootCerts, TlsConfig};
use ureq::Agent;
use walkdir::WalkDir;

fn is_external(link: &str) -> bool {
    link.starts_with("http://") || link.starts_with("https://")
}

fn is_ignorable(link: &str) -> bool {
    link.starts_with('#') || link.starts_with("mailto:") || link.starts_with("tel:")
}

fn should_check_external(link: &str) -> bool {
    link.starts_with("https://github.com/qqrm/CV/releases/")
        || link.starts_with("https://qqrm.github.io/CV/")
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

    let agent = Agent::config_builder()
        .tls_config(
            TlsConfig::builder()
                .root_certs(RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .new_agent();

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
                    if should_check_external(href) {
                        match agent
                            .head(href)
                            .call()
                            .or_else(|_| agent.get(href).call())
                        {
                            Ok(resp) if resp.status().is_success() => {}
                            Ok(resp) => errors.push(format!(
                                "{} -> HTTP {}",
                                href,
                                resp.status().as_u16()
                            )),
                            Err(e) => {
                                let msg = e.to_string();
                                if !msg.contains("Network is unreachable")
                                    && !msg.contains("failed to lookup address information")
                                {
                                    errors.push(format!("{} -> {}", href, msg));
                                }
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
