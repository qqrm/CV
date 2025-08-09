use lopdf::Document;
use pdf_extract::extract_text_from_mem;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use url::Url;

const BASE_URL: &str = "https://qqrm.github.io/CV/";
const LOG_FILE: &str = "site_check.log";
const EN_NAME: &str = "Alexey Leonidovich Belyakov";
const RU_NAME: &str = "Алексей Леонидович Беляков";

fn log_line(message: &str) {
    println!("{}", message);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        let _ = writeln!(file, "{}", message);
    }
}

fn parse_companies(path: &Path) -> HashSet<String> {
    let content = match read_to_string(path) {
        Ok(c) => c,
        Err(_) => return HashSet::new(),
    };
    let re = Regex::new(r"^### .* @ ([^|\n]+)").unwrap();
    content
        .lines()
        .filter_map(|line| re.captures(line).map(|cap| cap[1].trim().to_string()))
        .collect()
}

fn check_page(
    client: &Client,
    url: &Url,
    base: &Url,
    errors: &mut Vec<String>,
) -> (Vec<Url>, Vec<Url>) {
    match client.get(url.clone()).send() {
        Ok(resp) => {
            let status = resp.status();
            if status.is_client_error() || status.is_server_error() {
                let msg = format!("{} returned {}", url, status);
                errors.push(msg.clone());
                log_line(&format!("ERROR {}: {}", status, url));
                return (Vec::new(), Vec::new());
            }
            log_line(&format!("OK {}: {}", status, url));
            let body = match resp.text() {
                Ok(b) => b,
                Err(e) => {
                    let msg = format!("{} text error {}", url, e);
                    errors.push(msg.clone());
                    log_line(&format!("ERROR text: {} - {}", url, e));
                    return (Vec::new(), Vec::new());
                }
            };
            let document = Html::parse_document(&body);
            let selector = Selector::parse("a[href]").unwrap();
            let mut pages = Vec::new();
            let mut pdfs = Vec::new();
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    let href = href.split('#').next().unwrap_or("");
                    if href.is_empty() {
                        continue;
                    }
                    if let Ok(full) = url.join(href) {
                        if full.domain() != base.domain() {
                            continue;
                        }
                        if !full.path().starts_with(base.path()) {
                            continue;
                        }
                        if full.as_str() == url.as_str() {
                            continue;
                        }
                        if full.path().to_lowercase().ends_with(".pdf") {
                            pdfs.push(full);
                        } else {
                            pages.push(full);
                        }
                    }
                }
            }
            (pages, pdfs)
        }
        Err(e) => {
            let msg = format!("{} exception {}", url, e);
            errors.push(msg.clone());
            log_line(&format!("ERROR exception: {} - {}", url, e));
            (Vec::new(), Vec::new())
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

fn check_pdf(
    client: &Client,
    url: &Url,
    pdf_status: &mut Vec<String>,
    en_name: &str,
    en_companies: &HashSet<String>,
    ru_name: &str,
    ru_companies: &HashSet<String>,
) {
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
                            match extract_text_from_mem(&bytes) {
                                Ok(text) => {
                                    let (name, companies) = if url.as_str().contains("ru") {
                                        (ru_name, ru_companies)
                                    } else {
                                        (en_name, en_companies)
                                    };
                                    let mut missing = Vec::new();
                                    if !text.contains(name) {
                                        missing.push(format!("name {}", name));
                                    }
                                    for comp in companies {
                                        if !text.contains(comp) {
                                            missing.push(format!("company {}", comp));
                                        }
                                    }
                                    if missing.is_empty() {
                                        pdf_status.push(format!("OK: {}", url));
                                        log_line(&format!("PDF OK: {}", url));
                                    } else {
                                        for item in missing {
                                            let msg = format!("ERROR missing {}: {}", item, url);
                                            pdf_status.push(msg.clone());
                                            log_line(&format!(
                                                "PDF ERROR missing {}: {}",
                                                item, url
                                            ));
                                        }
                                    }
                                }
                                Err(e) => {
                                    let msg = format!("ERROR text {}: {}", url, e);
                                    pdf_status.push(msg.clone());
                                    log_line(&format!("PDF ERROR text: {} - {}", url, e));
                                }
                            }
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
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("client");

    let base = Url::parse(BASE_URL).expect("base url");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root");
    let en_companies = parse_companies(&repo_root.join("CV.MD"));
    let ru_companies = parse_companies(&repo_root.join("CV_RU.MD"));
    let mut visited: HashSet<Url> = HashSet::new();
    let mut errors = Vec::new();
    let mut pdf_status = Vec::new();
    let mut queue: VecDeque<Url> = VecDeque::new();
    queue.push_back(base.clone());

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        let (links, pdfs) = check_page(&client, &current, &base, &mut errors);
        probe_variant(&client, &current, &mut errors);
        for link in links {
            if !visited.contains(&link) {
                queue.push_back(link);
            }
        }
        for pdf in pdfs {
            check_pdf(
                &client,
                &pdf,
                &mut pdf_status,
                EN_NAME,
                &en_companies,
                RU_NAME,
                &ru_companies,
            );
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
