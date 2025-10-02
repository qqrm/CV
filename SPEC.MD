# Repository Specification

## 1. Purpose and Scope
- Maintain the source of truth for Alexey Belyakov's curriculum vitae in English and Russian.
- Generate publishable artifacts: Typst-based PDF resumes, role-specific variants, and a static multilingual website. The flagship CV culminates in an Engineering Manager position that highlights cross-team leadership responsibilities.
- Provide reusable tooling (Rust + Typst) to validate content, build pages, and keep outputs synchronized with the Markdown data.

## 2. Authoritative Content Sources
- `profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD` — primary CV narratives in English and Russian.
- `roles.toml` — maps role slugs (for example, `em`) to localized human-readable titles that drive role-specific site exports.
- `/content/avatar.jpg` plus `/DOCS/style.css` and `/DOCS/favicon.svg` — shared visual assets for PDFs and the static site.

## 3. Templates and Layouts
- Typst packages under `typst/en` and `typst/ru` define the PDF layout (`Belyakov_en.typ`, `Belyakov_em_en.typ`, etc.).
- `templates/resume.typ` exposes a reusable Typst function that renders a localized resume by pulling data from the Markdown files, defaulting to the canonical CV for each language while allowing overrides for both the Markdown source and the primary link label.
- `sitegen/templates/page.hbs` is the Handlebars template that powers every HTML page, providing dark/light theming, localized text, and dynamic role titles.

## 4. Rust Tooling
- `sitegen` crate (`sitegen/Cargo.toml`) centralizes parsing and rendering logic.
  - Library modules:
    - `parser.rs` parses month names, extracts inline employment start dates from `profiles/cv/en/CV.MD`, and loads `roles.toml` with validation.
    - `renderer.rs` formats experience durations for English and Russian output.
  - Binaries:
    - `validate` ensures Markdown files and `roles.toml` exist and are syntactically correct.
    - `generate` converts Markdown into HTML, copies Typst-generated PDFs and assets into `dist/`, creates localized role pages, and emits `sitemap.txt`.
- `scripts/convert_cv` is a standalone Rust helper that converts a Markdown CV into JSON with embedded HTML, enabling integrations with external systems.

## 5. Build and Generation Workflow
1. Run `./repo-setup.sh` once to install prerequisites, configure the canonical Git remote, and provision fonts for Typst.
2. Compile Typst sources to PDFs (light and dark variants for each locale and role):
   - `typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en_light.pdf`
   - `typst compile --root . typst/en/Belyakov_en_dark.typ typst/en/Belyakov_en_dark.pdf`
   - `typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru_light.pdf`
   - `typst compile --root . typst/ru/Belyakov_ru_dark.typ typst/ru/Belyakov_ru_dark.pdf`
   - `typst compile --root . typst/en/Belyakov_em_en.typ typst/en/Belyakov_em_en_light.pdf`
   - `typst compile --root . typst/en/Belyakov_em_en_dark.typ typst/en/Belyakov_em_en_dark.pdf`
   - `typst compile --root . typst/ru/Belyakov_em_ru.typ typst/ru/Belyakov_em_ru_light.pdf`
   - `typst compile --root . typst/ru/Belyakov_em_ru_dark.typ typst/ru/Belyakov_em_ru_dark.pdf`
3. Validate repository inputs: `cargo run --manifest-path sitegen/Cargo.toml --bin validate`.
4. Generate the website bundle: `cargo run --manifest-path sitegen/Cargo.toml --bin generate` (outputs HTML, PDFs, and `sitemap.txt` under `dist/`).
5. Publish artifacts manually or through CI by uploading `dist/` to GitHub Pages and attaching PDFs to releases.

## 6. Testing and Quality Gates
- Execute `cargo test --manifest-path sitegen/Cargo.toml` before commits to cover parser/renderer logic.
- Run `cargo test` inside `scripts/convert_cv` when modifying that helper crate.
- Ensure Typst compilation succeeds for both languages and the Engineering Manager variant across light and dark themes to keep downloadable PDFs in sync.

## 7. Repository Layout Summary
```
/
├── content/              # Shared media assets (avatar, theme icons)
├── DOCS/                 # Architecture notes, CSS, and favicon for the site bundle
├── scripts/convert_cv/   # Markdown-to-HTML converter CLI (Rust)
├── sitegen/              # Main Rust crate with generate/validate binaries and templates
├── TASKS/, BACKLOG/      # Planning and roadmap documents
├── typst/                # Typst sources for EN/RU PDFs
└── profiles/             # Localized CV Markdown sources
    └── cv/
        ├── en/          # English narratives (CV.MD)
        └── ru/          # Russian narratives (CV_RU.MD)
```

## 8. Outputs and Deployment Targets
- `dist/index.html` and `dist/ru/index.html` — landing pages for English and Russian audiences.
- `dist/em/index.html` and `dist/em/ru/index.html` — Engineering Manager landing pages with localized PDFs sourced from the canonical CV content.
- `dist/*.pdf` — canonical PDF resumes copied from Typst builds, including the Engineering Manager variation.
- `dist/sitemap.txt` — flat list of published URLs for search engines.
