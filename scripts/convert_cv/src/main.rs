use pulldown_cmark::{html, Parser};
use serde::Serialize;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Serialize)]
struct Resume {
    body: String,
}

fn convert(markdown: &str) -> Resume {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Resume { body: html_output }
}

fn repo_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.to_path_buf())
}

fn resolve_input(path: &str) -> io::Result<PathBuf> {
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        return Ok(candidate.to_path_buf());
    }

    if fs::metadata(candidate).is_ok() {
        return Ok(candidate.to_path_buf());
    }

    let fallback = repo_root().join(candidate);
    if fs::metadata(&fallback).is_ok() {
        Ok(fallback)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Input file {path} was not found relative to the current directory or repository root (expected at {}).",
                fallback.display()
            ),
        ))
    }
}

fn read_markdown(path: &str) -> io::Result<String> {
    let resolved = resolve_input(path)?;
    fs::read_to_string(resolved)
}

fn main() {
    let path = env::args().nth(1).expect("Usage: convert_cv <input.md>");
    let markdown = read_markdown(&path)
        .unwrap_or_else(|err| panic!("Failed to read '{path}'. {err}", path = path, err = err));
    let resume = convert(&markdown);
    serde_json::to_writer(std::io::stdout(), &resume).expect("write json");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_heading() {
        let res = convert("# Title");
        assert_eq!(res.body.trim(), "<h1>Title</h1>");
    }

    #[test]
    fn reads_markdown_from_repository_root() {
        let markdown = read_markdown("CV.MD").expect("read markdown from repository root");
        assert!(markdown.contains("Alexey"));
    }
}
