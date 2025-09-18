# Repository Specification

## 1. Purpose and Scope
- Maintain the source of truth for Alexey Belyakov's curriculum vitae in English and Russian.
- Generate publishable artifacts: Typst-based PDF resumes, role-specific variants, and a static multilingual website. The flagship CV culminates in an Engineering Manager position that highlights cross-team leadership responsibilities.
- Provide reusable tooling (Rust + Typst) to validate content, build pages, and keep outputs synchronized with the Markdown data.

## 2. Authoritative Content Sources
- `profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD` — primary CV narratives in English and Russian.
- `profiles/cv/en/CV_EM.MD`, `profiles/cv/ru/CV_EM_RU.MD` — Engineering Manager spotlight content for both languages.
- `profiles/resume/en/RESUME_EM.MD`, `profiles/resume/ru/RESUME_EM_RU.MD` — condensed Engineering Manager resume highlights consumed by Typst and site generation.
- `roles.toml` — maps the Engineering Manager slug to a human-readable title that powers the role-specific site and resume exports.
- `/content/avatar.jpg` plus `/DOCS/style.css` and `/DOCS/favicon.svg` — shared visual assets for PDFs and the static site.

## 3. Templates and Layouts
- Typst packages under `typst/en` and `typst/ru` define the PDF layout (`Belyakov_en.typ`, `Belyakov_em_en.typ`, etc.).
- `templates/resume.typ` exposes a reusable Typst function that renders a localized resume by pulling data from the Markdown files.
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
2. Compile Typst sources to PDFs:
   - `typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf`
   - `typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf`
   - Compile the Engineering Manager template as needed (e.g., `Belyakov_em_en.typ`).
3. Validate repository inputs: `cargo run --manifest-path sitegen/Cargo.toml --bin validate`.
4. Generate the website bundle: `cargo run --manifest-path sitegen/Cargo.toml --bin generate` (outputs HTML, PDFs, and `sitemap.txt` under `dist/`).
5. Publish artifacts manually or through CI by uploading `dist/` to GitHub Pages and attaching PDFs to releases.

## 6. Testing and Quality Gates
- Execute `cargo test --manifest-path sitegen/Cargo.toml` before commits to cover parser/renderer logic.
- Run `cargo test` inside `scripts/convert_cv` when modifying that helper crate.
- Ensure Typst compilation succeeds for both languages and the Engineering Manager variant to keep downloadable PDFs in sync.

## 7. Repository Layout Summary
```
/
├── content/              # Shared media assets (avatar, theme icons)
├── DOCS/                 # Architecture notes, CSS, and favicon for the site bundle
├── scripts/convert_cv/   # Markdown-to-HTML converter CLI (Rust)
├── sitegen/              # Main Rust crate with generate/validate binaries and templates
├── TASKS/, BACKLOG/      # Planning and roadmap documents
├── typst/                # Typst sources for EN/RU PDFs
└── profiles/             # Localized CV and resume Markdown sources
    ├── cv/
    │   ├── en/           # English narratives (CV.MD, CV_EM.MD)
    │   └── ru/           # Russian narratives (CV_RU.MD, CV_EM_RU.MD)
    └── resume/
        ├── en/          # Engineering Manager resume highlights (RESUME_EM.MD)
        └── ru/          # Russian resume highlights (RESUME_EM_RU.MD)
```

## 8. Outputs and Deployment Targets
- `dist/index.html` and `dist/ru/index.html` — landing pages for English and Russian audiences.
- `dist/em/index.html` and `dist/em/ru/index.html` — Engineering Manager landing pages with localized PDFs.
- `dist/resume/em/` and `dist/resume/em/ru/` — localized resume landing pages for the Engineering Manager profile.
- `dist/*.pdf` — canonical PDF resumes copied from Typst builds, including the Engineering Manager variation.
- `dist/sitemap.txt` — flat list of published URLs for search engines.
