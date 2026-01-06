use leptos::*;
use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::wasm_bindgen;

const CV_MARKDOWN_EN: &str = include_str!("../../profiles/cv/en/CV.MD");
const CV_MARKDOWN_RU: &str = include_str!("../../profiles/cv/ru/CV_RU.MD");

struct ContactLabels {
    github: &'static str,
    email: &'static str,
    telegram: &'static str,
    linkedin: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Language {
    En,
    Ru,
}

impl Language {
    fn toggle(self) -> Self {
        match self {
            Self::En => Self::Ru,
            Self::Ru => Self::En,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::En => "Alexey Belyakov",
            Self::Ru => "Алексей Беляков",
        }
    }

    fn download_label(self) -> &'static str {
        match self {
            Self::En => "Download PDF",
            Self::Ru => "Скачать PDF",
        }
    }

    fn pdf_prefix(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ru => "ru",
        }
    }

    fn contact_labels(self) -> ContactLabels {
        match self {
            Self::En => ContactLabels {
                github: "GitHub",
                email: "Email",
                telegram: "Telegram",
                linkedin: "LinkedIn",
            },
            Self::Ru => ContactLabels {
                github: "GitHub",
                email: "Почта",
                telegram: "Telegram",
                linkedin: "LinkedIn",
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    fn as_attr(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn pdf_suffix(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn toggle_label(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::Light, Language::En) => "Switch to dark theme",
            (Self::Light, Language::Ru) => "Переключить на тёмную тему",
            (Self::Dark, Language::En) => "Switch to light theme",
            (Self::Dark, Language::Ru) => "Переключить на светлую тему",
        }
    }
}

struct ContactUrls;

impl ContactUrls {
    const GITHUB: &'static str = "https://github.com/qqrm";
    const EMAIL: &'static str = "mailto:qqrm@vivaldi.net";
    const TELEGRAM: &'static str = "https://leqqrm.t.me";
    const LINKEDIN: &'static str = "https://www.linkedin.com/in/qqrm/";
}

fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn body_without_title(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let trimmed = lines
        .into_iter()
        .skip(1)
        .skip_while(|line| line.trim().is_empty())
        .collect::<Vec<_>>();
    trimmed.join("\n")
}

#[component]
pub fn App() -> impl IntoView {
    let (language, set_language) = create_signal(Language::En);
    let (theme, set_theme) = create_signal(Theme::Dark);

    let rendered_cv = create_memo(move |_| {
        let markdown = match language.get() {
            Language::En => CV_MARKDOWN_EN,
            Language::Ru => CV_MARKDOWN_RU,
        };

        render_markdown(&body_without_title(markdown))
    });

    create_effect(move |_| {
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("data-theme", theme.get().as_attr());
        }
    });

    let pdf_href = move || {
        format!(
            "https://github.com/qqrm/CV/releases/latest/download/Belyakov_{}_{}.pdf",
            language.get().pdf_prefix(),
            theme.get().pdf_suffix()
        )
    };

    let contact_labels = move || language.get().contact_labels();

    view! {
        <div class="floating-controls">
            <button
                class="theme-toggle"
                aria-label=move || theme.get().toggle_label(language.get())
                on:click=move |_| set_theme.update(|current| *current = current.toggle())
            >
                {move || match theme.get() {
                    Theme::Light => view! { <img src="moon.svg" alt="Moon icon"/> },
                    Theme::Dark => view! { <img src="sun.svg" alt="Sun icon"/> },
                }}
            </button>

            <button
                class="language-toggle"
                aria-label=move || match language.get() {
                    Language::En => "Переключить на русский",
                    Language::Ru => "Switch to English",
                }
                on:click=move |_| set_language.update(|current| *current = current.toggle())
            >
                <span class=move || format!("lang-option {}", if language.get() == Language::En { "current" } else { "" })>
                    "EN"
                </span>
                <span class="lang-separator">"/"</span>
                <span class=move || format!("lang-option {}", if language.get() == Language::Ru { "current" } else { "" })>
                    "RU"
                </span>
            </button>
        </div>

        <main class="page">
            <header class="hero">
                <h1>{move || language.get().name()}</h1>
                <div class="avatar-wrapper">
                    <img class="avatar" src="avatar.png" alt="Alexey Belyakov" />
                </div>
                <div class="contact-actions">
                    <a class="pill" href=ContactUrls::GITHUB target="_blank" rel="noopener">{move || contact_labels().github}</a>
                    <a class="pill" href=ContactUrls::EMAIL>{move || contact_labels().email}</a>
                    <a class="pill" href=ContactUrls::TELEGRAM target="_blank" rel="noopener">{move || contact_labels().telegram}</a>
                    <a class="pill" href=ContactUrls::LINKEDIN target="_blank" rel="noopener">{move || contact_labels().linkedin}</a>
                </div>
                <div class="actions">
                    <a class="button" href=pdf_href>{move || language.get().download_label()}</a>
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
