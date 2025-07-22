use pulldown_cmark::{Options, Parser, html::push_html};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate English version
    let markdown_input = fs::read_to_string("README.md")?;
    let parser = Parser::new_ext(&markdown_input, Options::all());
    let mut html_body = String::new();
    push_html(&mut html_body, parser);
    html_body = html_body.replace("./latex/", "latex/");
    html_body = html_body.replace("./README_ru.md", "ru/");

    let html_template = format!(
        "<!DOCTYPE html>\n<html lang='en'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Alexey Belyakov - CV</title>\n    <link rel='stylesheet' href='style.css'>\n</head>\n<body>\n<div class='content'>\n{}\n</div>\n<footer>\n    <p><a href='latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n</footer>\n</body>\n</html>\n",
        html_body
    );

    // Generate Russian version
    let markdown_ru = fs::read_to_string("README_ru.md")?;
    let parser_ru = Parser::new_ext(&markdown_ru, Options::all());
    let mut html_body_ru = String::new();
    push_html(&mut html_body_ru, parser_ru);
    html_body_ru = html_body_ru.replace("./latex/", "../latex/");

    let html_template_ru = format!(
        "<!DOCTYPE html>\n<html lang='ru'>\n<head>\n    <meta charset='UTF-8'>\n    <title>Алексей Беляков - Резюме</title>\n    <link rel='stylesheet' href='../style.css'>\n</head>\n<body>\n<div class='content'>\n{}\n</div>\n<footer>\n    <p><a href='../latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>\n    <p><a href='../latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>\n</footer>\n</body>\n</html>\n",
        html_body_ru
    );

    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    fs::write(docs_dir.join("index.html"), html_template)?;

    let ru_dir = docs_dir.join("ru");
    if !ru_dir.exists() {
        fs::create_dir_all(&ru_dir)?;
    }
    fs::write(ru_dir.join("index.html"), html_template_ru)?;

    Ok(())
}
