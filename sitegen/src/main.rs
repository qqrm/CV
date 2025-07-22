use pulldown_cmark::{html::push_html, Options, Parser};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown_input = fs::read_to_string("README.md")?;
    let parser = Parser::new_ext(&markdown_input, Options::all());
    let mut html_body = String::new();
    push_html(&mut html_body, parser);
    html_body = html_body.replace("./latex/", "latex/");
    html_body = html_body.replace("./README_ru.md", "README_ru.md");

    let html_template = format!(
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<header>\n    <h1>Alexey Belyakov</h1>\n</header>\n<div class='content'>\n{}\n</div>\n<footer>\n    <p><a href='latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n</footer>\n</body>\n</html>\n",
        html_body
    );

    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    fs::write(docs_dir.join("index.html"), html_template)?;

    // Copy additional assets for GitHub Pages
    fs::create_dir_all(docs_dir.join("latex/en"))?;
    fs::create_dir_all(docs_dir.join("latex/ru"))?;
    fs::copy("latex/en/Belyakov_en.pdf", docs_dir.join("latex/en/Belyakov_en.pdf"))?;
    fs::copy("latex/ru/Belyakov_ru.pdf", docs_dir.join("latex/ru/Belyakov_ru.pdf"))?;
    fs::copy("README_ru.md", docs_dir.join("README_ru.md"))?;
    Ok(())
}
