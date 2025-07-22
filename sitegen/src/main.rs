use pulldown_cmark::{Options, Parser, html::push_html};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown_input = fs::read_to_string("README.md")?;
    let parser = Parser::new_ext(&markdown_input, Options::all());
    let mut html_body = String::new();
    push_html(&mut html_body, parser);

    let html_template = format!(
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<header>\n    <h1>Alexey Belyakov</h1>\n</header>\n<div class='content'>\n{}\n</div>\n<footer>\n    <p><a href='latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n</footer>\n</body>\n</html>\n",
        html_body
    );

    fs::write("index.html", html_template)?;
    Ok(())
}
