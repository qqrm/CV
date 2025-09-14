use pulldown_cmark::{html, Parser};
use serde::Serialize;
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

fn main() {
    let path = env::args().nth(1).expect("Usage: convert_cv <input.md>");
    let markdown = fs::read_to_string(path).expect("read input");
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
}
