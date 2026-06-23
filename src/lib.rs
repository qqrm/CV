use leptos::{mount::mount_to_body, prelude::*};
use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::wasm_bindgen;

const CV_MARKDOWN_EN: &str = include_str!("../profiles/cv/en/CV.MD");
const CV_MARKDOWN_RU: &str = include_str!("../profiles/cv/ru/CV_RU.MD");
// GitHub Pages serves this app from /CV/, so the avatar URL needs the base prefix.
const AVATAR_SRC: &str = "/CV/avatar.jpg";
const MOON_SRC: &str = "/CV/moon.svg";
const SUN_SRC: &str = "/CV/sun.svg";
const THEME_STORAGE_KEY: &str = "cv-theme";

struct ContactLabels {
    github: &'static str,
    email: &'static str,
    telegram: &'static str,
    linkedin: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Language {
    En,
    Ru,
}

impl Language {
    fn markdown(self) -> &'static str {
        match self {
            Self::En => CV_MARKDOWN_EN,
            Self::Ru => CV_MARKDOWN_RU,
        }
    }

    fn toggle(self) -> Self {
        match self {
            Self::En => Self::Ru,
            Self::Ru => Self::En,
        }
    }

    fn target_path(self) -> &'static str {
        match self {
            Self::En => "/CV/",
            Self::Ru => "/CV/ru/",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::En => "Alexey Belyakov",
            Self::Ru => "Алексей Беляков",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::En => "CTO / Head of Engineering / Head of Delivery",
            Self::Ru => "CTO / Head of Engineering / Head of Delivery",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::En => {
                "Rust/C++ • backend/platform/systems engineering • AI agents in software development"
            }
            Self::Ru => "Rust/C++ • backend/platform/systems engineering • AI-агенты в разработке",
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
                email: "Email",
                telegram: "Telegram",
                linkedin: "LinkedIn",
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    fn from_storage_value(value: &str) -> Option<Self> {
        match value {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
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
    const TELEGRAM: &'static str = "https://t.me/leqqrm";
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

fn body_without_frontmatter(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.starts_with("## "))
        .unwrap_or(0);
    lines[start..].join("\n")
}

fn route_from_pathname(pathname: &str) -> Language {
    let normalized = pathname.trim_end_matches('/');
    if normalized.ends_with("/ru") {
        Language::Ru
    } else {
        Language::En
    }
}

fn initial_language() -> Language {
    let pathname = window()
        .location()
        .pathname()
        .unwrap_or_else(|_| String::from("/CV/"));
    route_from_pathname(&pathname)
}

fn initial_theme() -> Theme {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(THEME_STORAGE_KEY).ok().flatten())
        .and_then(|value| Theme::from_storage_value(&value))
        .unwrap_or(Theme::Light)
}

fn persist_theme(theme: Theme) {
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.set_item(THEME_STORAGE_KEY, theme.as_attr());
    }
}

fn pdf_href(language: Language, theme: Theme) -> String {
    format!(
        "/CV/Belyakov_{}_{}.pdf",
        language.pdf_prefix(),
        theme.as_attr()
    )
}

#[component]
pub fn App() -> impl IntoView {
    let start_language = initial_language();
    let start_theme = initial_theme();
    let (language, set_language) = signal(start_language);
    let (theme, set_theme) = signal(start_theme);

    let rendered_cv =
        Memo::new(move |_| render_markdown(&body_without_frontmatter(language.get().markdown())));

    Effect::new(move || {
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("data-theme", theme.get().as_attr());
        }
    });

    let pdf_href = move || pdf_href(language.get(), theme.get());

    let contact_labels = move || language.get().contact_labels();

    view! {
        <div class="floating-controls">
            <button
                class="theme-toggle"
                aria-label=move || theme.get().toggle_label(language.get())
                on:click=move |_| {
                    let next_theme = theme.get().toggle();
                    set_theme.set(next_theme);
                    persist_theme(next_theme);
                }
            >
                {move || match theme.get() {
                    Theme::Light => view! { <img src=MOON_SRC alt="Moon icon"/> },
                    Theme::Dark => view! { <img src=SUN_SRC alt="Sun icon"/> },
                }}
            </button>

            <button
                class="language-toggle"
                aria-label=move || match language.get() {
                    Language::En => "Переключить на русский",
                    Language::Ru => "Switch to English",
                }
                on:click=move |_| {
                    let next_language = language.get().toggle();
                    set_language.set(next_language);
                    let _ = window().location().set_href(next_language.target_path());
                }
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
                <p class="role-title">{move || language.get().title()}</p>
                <p class="hero-subtitle">{move || language.get().subtitle()}</p>
                <div class="avatar-wrapper">
                    <img class="avatar" src=AVATAR_SRC alt="Alexey Belyakov" />
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
                <article class="cv" inner_html=move || rendered_cv.get()></article>
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

#[cfg(test)]
mod tests {
    use super::{
        AVATAR_SRC, Language, MOON_SRC, SUN_SRC, Theme, body_without_frontmatter,
        pdf_href, route_from_pathname,
    };

    #[test]
    fn route_detection_handles_supported_paths() {
        assert_eq!(route_from_pathname("/CV/"), Language::En);
        assert_eq!(route_from_pathname("/CV/ru/"), Language::Ru);
    }

    #[test]
    fn body_without_frontmatter_removes_header_title_and_subtitle() {
        let body = body_without_frontmatter(
            "# Name\n\n**Title**\nSubtitle\n\n## Summary\nBody\n\n## Experience\nMore",
        );

        assert_eq!(body, "## Summary\nBody\n\n## Experience\nMore");
    }

    #[test]
    fn avatar_src_is_absolute_for_nested_routes() {
        assert_eq!(AVATAR_SRC, "/CV/avatar.jpg");
    }

    #[test]
    fn theme_icon_srcs_are_absolute_for_nested_routes() {
        assert_eq!(MOON_SRC, "/CV/moon.svg");
        assert_eq!(SUN_SRC, "/CV/sun.svg");
        assert!(MOON_SRC.starts_with("/CV/"));
        assert!(SUN_SRC.starts_with("/CV/"));
    }

    #[test]
    fn theme_parses_persisted_values() {
        assert_eq!(Theme::from_storage_value("light"), Some(Theme::Light));
        assert_eq!(Theme::from_storage_value("dark"), Some(Theme::Dark));
        assert_eq!(Theme::from_storage_value(""), None);
        assert_eq!(Theme::from_storage_value("system"), None);
    }

    #[test]
    fn pdf_href_matches_language_and_theme() {
        assert_eq!(
            pdf_href(Language::En, Theme::Light),
            "/CV/Belyakov_en_light.pdf"
        );
        assert_eq!(
            pdf_href(Language::En, Theme::Dark),
            "/CV/Belyakov_en_dark.pdf"
        );
        assert_eq!(
            pdf_href(Language::Ru, Theme::Light),
            "/CV/Belyakov_ru_light.pdf"
        );
        assert_eq!(
            pdf_href(Language::Ru, Theme::Dark),
            "/CV/Belyakov_ru_dark.pdf"
        );
    }
}
