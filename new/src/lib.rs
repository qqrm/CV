use leptos::*;
use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::wasm_bindgen;

const CV_MARKDOWN: &str = include_str!("../../profiles/cv/en/CV.MD");
const SUMMARY: &str = "Engineering leader inspired by Rust. I build strong teams and shape a delivery culture grounded in leading DevOps practices so product, engineering, and compliance move in sync and produce measurable business impact.";

fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn body_without_title() -> String {
    let lines: Vec<&str> = CV_MARKDOWN.lines().collect();
    let trimmed = lines
        .into_iter()
        .skip(1)
        .skip_while(|line| line.trim().is_empty())
        .collect::<Vec<_>>();
    trimmed.join("\n")
}

#[component]
pub fn App() -> impl IntoView {
    let rendered_cv = render_markdown(&body_without_title());

    view! {
        <main class="page">
            <header class="hero">
                <p class="eyebrow">"Alexey Belyakov — Engineering Manager"</p>
                <h1>"Curriculum Vitae"</h1>
                <p class="lede">{SUMMARY}</p>
                <div class="actions">
                    <a class="button" href="https://qqrm.github.io/CV/">"Open classic CV"</a>
                    <a class="button secondary" href="https://github.com/qqrm/CV/releases/latest/download/Belyakov_en_light.pdf">"Download PDF"</a>
                </div>
            </header>

            <section class="content">
                <article class="cv" inner_html=rendered_cv></article>
            </section>
        </main>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| view! { <App/> });
}
