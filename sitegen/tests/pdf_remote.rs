use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use ureq::Agent;
use ureq::Error;
use ureq::http::header::CONTENT_TYPE;
use ureq::tls::{RootCerts, TlsConfig};
use walkdir::WalkDir;

const BASE_URLS: [&str; 1] = ["https://qqrm.github.io/CV/"];
const MIN_PDF_SIZE_BYTES: usize = 1024;

struct DistGuard {
    path: PathBuf,
}

impl DistGuard {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for DistGuard {
    fn drop(&mut self) {
        if self.path.exists()
            && let Err(err) = fs::remove_dir_all(&self.path)
        {
            eprintln!(
                "failed to remove {} during cleanup: {}",
                self.path.display(),
                err
            );
        }
    }
}

#[test]
#[serial_test::serial]
#[ignore = "requires already deployed PDFs and must run only post-deploy"]
fn deployed_pdfs_are_real() {
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
    let _guard = DistGuard::new(dist.clone());

    let mut pdfs = Vec::new();
    for entry in WalkDir::new(&dist).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("pdf"))
                .unwrap_or(false)
        {
            let relative = entry
                .path()
                .strip_prefix(&dist)
                .expect("pdf to be inside dist")
                .to_string_lossy()
                .replace('\\', "/");
            pdfs.push(relative);
        }
    }

    pdfs.sort();
    pdfs.dedup();

    assert!(!pdfs.is_empty(), "no PDFs generated");

    let mut errors = Vec::new();

    let agent = Agent::config_builder()
        .tls_config(
            TlsConfig::builder()
                .root_certs(RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .new_agent();

    for relative in &pdfs {
        let mut attempts = Vec::new();
        let mut verified = false;

        for base in BASE_URLS {
            let url = format!("{base}{relative}");
            match agent.get(&url).call() {
                Ok(resp) => {
                    let status = resp.status();
                    if !status.is_success() {
                        attempts.push(format!("{url} -> HTTP {}", status.as_u16()));
                        continue;
                    }

                    let content_type = resp
                        .headers()
                        .get(CONTENT_TYPE)
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_owned)
                        .unwrap_or_default();
                    if !content_type.contains("application/pdf") {
                        attempts.push(format!("{url} -> unexpected content-type '{content_type}'"));
                        continue;
                    }

                    let mut reader = resp.into_body().into_reader();
                    let mut header = [0_u8; 5];
                    if let Err(err) = reader.read_exact(&mut header) {
                        attempts.push(format!("{url} -> failed to read header: {err}"));
                        continue;
                    }
                    if &header != b"%PDF-" {
                        attempts.push(format!(
                            "{url} -> invalid PDF signature: {}",
                            String::from_utf8_lossy(&header)
                        ));
                        continue;
                    }

                    let mut total_bytes = header.len();
                    let mut buffer = [0_u8; 4096];
                    loop {
                        match reader.read(&mut buffer) {
                            Ok(0) => break,
                            Ok(n) => {
                                total_bytes += n;
                            }
                            Err(err) => {
                                attempts.push(format!("{url} -> read error after header: {err}"));
                                break;
                            }
                        }
                    }

                    if total_bytes < MIN_PDF_SIZE_BYTES {
                        attempts.push(format!(
                            "{url} -> unexpectedly small PDF ({} bytes)",
                            total_bytes
                        ));
                        continue;
                    }

                    verified = true;
                    break;
                }
                Err(Error::StatusCode(code)) => {
                    attempts.push(format!("{url} -> HTTP {code}"));
                }
                Err(Error::Io(err)) => {
                    let message = err.to_string();
                    if message.contains("Network is unreachable")
                        || message.contains("failed to lookup address information")
                    {
                        eprintln!("Skipping {url} due to network issue: {message}");
                    } else {
                        attempts.push(format!("{url} -> IO error: {message}"));
                    }
                }
                Err(Error::Tls(message)) => {
                    eprintln!("Skipping {url} due to TLS issue: {message}");
                }
                Err(other) => {
                    attempts.push(format!("{url} -> error: {other}"));
                }
            }
        }

        if !verified {
            errors.extend(attempts);
        }
    }

    if !errors.is_empty() {
        panic!("Deployed PDF verification failed:\n{}", errors.join("\n"));
    }
}
