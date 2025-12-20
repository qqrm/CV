# Repository Specification

## 1. Purpose and Scope
- Maintain the source of truth for Alexey Belyakov's curriculum vitae in English and Russian.
- Generate publishable artifacts from the canonical CV narrative: localized Typst PDFs and a static multilingual website for English and Russian.
- Provide reusable tooling (Rust + Typst) that validates content, renders HTML/PDF outputs, and keeps generated artifacts synchronized with the Markdown sources.

## 2. Authoritative Content Sources
- `profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD` — canonical CV narratives in English and Russian.
- `/content/avatar.jpg` plus `/DOCS/style.css` and `/DOCS/favicon.svg` — shared visual assets for PDFs and the static site.

## 3. Templates and Layouts
- Typst entry points under `typst/en` and `typst/ru` (`Belyakov_en.typ`, `Belyakov_en_dark.typ`, `Belyakov_ru.typ`, `Belyakov_ru_dark.typ`) produce localized PDFs using the shared template.
- `templates/resume.typ` exposes a reusable Typst function that renders the canonical CV without role-specific variants.
- `sitegen/templates/page.hbs` is the Handlebars template that powers every HTML page, providing localized UI text and theming hooks.

## 4. Rust Tooling
- `sitegen` crate (`sitegen/Cargo.toml`) centralizes parsing and rendering logic.
  - Library modules:
    - `parser.rs` parses month names and extracts inline employment start dates from `profiles/cv/en/CV.MD`.
    - `renderer.rs` formats experience durations for English and Russian output.
  - Binaries:
    - `validate` ensures Markdown sources exist and are readable.
    - `generate` converts Markdown into HTML, copies Typst-generated PDFs and assets into `dist/`, and emits `sitemap.txt`.
- `scripts/convert_cv` is a standalone Rust helper that converts the Markdown CV into JSON with embedded HTML for integrations.

## 5. Build and Generation Workflow
1. Run `./repo-setup.sh` to install prerequisites, configure the canonical Git remote, and provision fonts for Typst.
2. Compile Typst sources to PDFs (light and dark variants for each locale):
   - `typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en_light.pdf`
   - `typst compile --root . typst/en/Belyakov_en_dark.typ typst/en/Belyakov_en_dark.pdf`
   - `typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru_light.pdf`
   - `typst compile --root . typst/ru/Belyakov_ru_dark.typ typst/ru/Belyakov_ru_dark.pdf`
3. Validate repository inputs: `cargo run --manifest-path sitegen/Cargo.toml --bin validate`.
4. Generate the website bundle: `cargo run --manifest-path sitegen/Cargo.toml --bin generate` (outputs HTML, PDFs, and `sitemap.txt` under `dist/`).
5. Publish artifacts manually or through CI by uploading `dist/` to GitHub Pages and attaching PDFs to releases.

## 6. Testing and Quality Gates
- Execute `cargo test --manifest-path sitegen/Cargo.toml` before commits to cover parser/renderer logic.
- Run `cargo test` inside `scripts/convert_cv` when modifying that helper crate.
- Ensure Typst compilation succeeds for both locales (light and dark variants) so downloadable PDFs stay in sync.

## 7. Repository Layout Summary
```
/
├── content/              # Shared media assets (avatar, theme icons)
├── DOCS/                 # Architecture notes, CSS, and favicon for the site bundle
├── scripts/convert_cv/   # Markdown-to-HTML converter CLI (Rust)
├── sitegen/              # Main Rust crate with generate/validate binaries and templates
├── typst/                # Typst sources for EN/RU PDFs
└── profiles/             # Localized CV Markdown sources
    ├── cv/
    │   ├── en/           # English narrative (CV.MD)
    │   └── ru/           # Russian narrative (CV_RU.MD)
    └── …                 # Additional localized material as needed
```

## 8. Outputs and Deployment Targets
- `dist/index.html` and `dist/ru/index.html` — landing pages for English and Russian audiences.
- `dist/Belyakov_en_*.pdf`, `dist/Belyakov_ru_*.pdf` — canonical PDF resumes copied from Typst builds.
- `dist/sitemap.txt` — flat list of published URLs for search engines.
