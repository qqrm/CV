use lopdf::Document;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use url::Url;

const BASE_URL: &str = "https://qqrm.github.io/CV/";
const LOG_FILE: &str = "site_check.log";

fn log_line(message: &str) {
    println!("{}", message);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        let _ = writeln!(file, "{}", message);
    }
}

fn check_page(client: &Client, url: &Url, errors: &mut Vec<String>) {
    match client.get(url.clone()).send() {
        Ok(resp) => {
            let status = resp.status();
            if status.is_client_error() || status.is_server_error() {
                let msg = format!("{} returned {}", url, status);
                errors.push(msg.clone());
                log_line(&format!("ERROR {}: {}", status, url));
            } else {
                log_line(&format!("OK {}: {}", status, url));
            }
        }
        Err(e) => {
            let msg = format!("{} exception {}", url, e);
            errors.push(msg.clone());
            log_line(&format!("ERROR exception: {} - {}", url, e));
        }
    }
}

fn probe_variant(client: &Client, url: &Url, errors: &mut Vec<String>) {
    let alt = if url.path().ends_with('/') {
        let trimmed = url.path().trim_end_matches('/');
        if trimmed.is_empty() {
            return;
        }
        let mut u = url.clone();
        u.set_path(trimmed);
        u
    } else {
        let mut u = url.clone();
        let new_path = format!("{}/", url.path());
        u.set_path(&new_path);
        u
    };
    match client.get(alt.clone()).send() {
        Ok(resp) => {
            let status = resp.status();
            if status.is_client_error() || status.is_server_error() {
                let msg = format!("{} returned {}", alt, status);
                errors.push(msg.clone());
                log_line(&format!("ERROR {}: {}", status, alt));
            } else {
                log_line(&format!("OK {}: {}", status, alt));
            }
        }
        Err(e) => {
            let msg = format!("{} exception {}", alt, e);
            errors.push(msg.clone());
            log_line(&format!("ERROR exception: {} - {}", alt, e));
        }
    }
}

fn check_pdf(client: &Client, url: &Url, pdf_status: &mut Vec<String>) {
    match client.get(url.clone()).send() {
        Ok(resp) => {
            let status = resp.status();
            if status.is_client_error() || status.is_server_error() {
                let msg = format!("ERROR {}: {}", status, url);
                pdf_status.push(msg.clone());
                log_line(&format!("PDF ERROR {}: {}", status, url));
                return;
            }
            if let Some(ct) = resp
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
            {
                if !ct.contains("application/pdf") {
                    let msg = format!("ERROR not pdf: {}", url);
                    pdf_status.push(msg.clone());
                    log_line(&format!("PDF ERROR not pdf: {}", url));
                    return;
                }
            } else {
                let msg = format!("ERROR no content-type: {}", url);
                pdf_status.push(msg.clone());
                log_line(&format!("PDF ERROR no content-type: {}", url));
                return;
            }
            match resp.bytes() {
                Ok(bytes) => match Document::load_mem(&bytes) {
                    Ok(doc) => {
                        if doc.get_pages().is_empty() {
                            let msg = format!("ERROR empty pdf: {}", url);
                            pdf_status.push(msg.clone());
                            log_line(&format!("PDF ERROR empty pdf: {}", url));
                        } else {
                            pdf_status.push(format!("OK: {}", url));
                            log_line(&format!("PDF OK: {}", url));
                        }
                    }
                    Err(e) => {
                        let msg = format!("ERROR parse {}: {}", url, e);
                        pdf_status.push(msg.clone());
                        log_line(&format!("PDF ERROR parse: {} - {}", url, e));
                    }
                },
                Err(e) => {
                    let msg = format!("ERROR bytes {}: {}", url, e);
                    pdf_status.push(msg.clone());
                    log_line(&format!("PDF ERROR bytes: {} - {}", url, e));
                }
            }
        }
        Err(e) => {
            let msg = format!("ERROR exception {}: {}", e, url);
            pdf_status.push(msg.clone());
            log_line(&format!("PDF ERROR exception: {} - {}", url, e));
        }
    }
}

fn main() -> std::process::ExitCode {
    let mut args = std::env::args();
    args.next(); // skip binary name
    let mut sitemap_path =
        std::env::var("SITEMAP_PATH").unwrap_or_else(|_| "dist/sitemap.txt".to_string());
    while let Some(arg) = args.next() {
        if arg == "--sitemap" {
            if let Some(path) = args.next() {
                sitemap_path = path;
            }
        }
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("client");

    let base = Url::parse(BASE_URL).expect("base url");
    let mut errors = Vec::new();
    let mut pdf_status = Vec::new();

    let content = match std::fs::read_to_string(&sitemap_path) {
        Ok(c) => c,
        Err(e) => {
            log_line(&format!("ERROR reading sitemap {}: {}", sitemap_path, e));
            return std::process::ExitCode::from(1);
        }
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let url = match Url::parse(line) {
            Ok(u) => u,
            Err(_) => match base.join(line) {
                Ok(u) => u,
                Err(e) => {
                    errors.push(format!("invalid url {}: {}", line, e));
                    continue;
                }
            },
        };
        if url.path().to_lowercase().ends_with(".pdf") {
            check_pdf(&client, &url, &mut pdf_status);
        } else {
            check_page(&client, &url, &mut errors);
            probe_variant(&client, &url, &mut errors);
        }
    }

    log_line("--- PDF status ---");
    for line in &pdf_status {
        log_line(line);
    }

    if errors.is_empty() {
        log_line("Site check completed successfully");
        std::process::ExitCode::SUCCESS
    } else {
        log_line("Site check completed with errors");
        for e in &errors {
            log_line(e);
        }
        std::process::ExitCode::from(1)
    }
}
