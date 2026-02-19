# Repository Specification

## 1. Purpose and Scope
- Maintain the source of truth for Alexey Belyakov's curriculum vitae in English and Russian.
- Generate publishable artifacts from the canonical CV narrative: localized Typst PDFs and a static multilingual website for English and Russian.
- Provide reusable tooling (Rust + Typst + Leptos) that validates content, renders deploy artifacts, and keeps generated outputs synchronized with the Markdown sources.

## 2. Authoritative Content Sources
- `profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD` — canonical CV narratives in English and Russian.
- `profiles/rust-developer/*` and `profiles/cto/*` — profile-specific narrative sources.
- `/content/avatar.jpg`, `/content/sun.svg`, `/content/moon.svg` — shared visual assets for the website.

## 3. Templates and Layouts
- Typst entry points under `typst/en` and `typst/ru` produce localized PDFs using the shared template.
- `templates/resume.typ` exposes a reusable Typst function that renders the canonical CV and profile variants.
- Root Leptos app (`Cargo.toml`, `index.html`, `src/lib.rs`, `Trunk.toml`) is the authoritative website implementation for GitHub Pages.

## 4. Rust Tooling
- Root crate (`Cargo.toml`) builds the Leptos website bundle with Trunk.
- `sitegen` crate (`sitegen/Cargo.toml`) centralizes parser/renderer logic and deployment asset preparation.
  - `validate` ensures Markdown sources exist and are readable.
  - `generate` validates canonical PDFs, copies them to an output directory, and emits `sitemap.txt` (no HTML generation).
- `scripts/convert_cv` is a standalone Rust helper that converts the Markdown CV into JSON with embedded HTML for integrations.

## 5. Build and Generation Workflow
1. Run `./repo-setup.sh` to install prerequisites, configure the canonical Git remote, and provision fonts for Typst.
2. Compile Typst sources to canonical PDFs (light and dark variants for each profile/locale).
3. Validate repository inputs: `cargo run --manifest-path sitegen/Cargo.toml --bin validate`.
4. Build website bundle: `trunk build --release --config Trunk.toml` (outputs `dist/`).
5. Generate deploy assets: `cargo run --manifest-path sitegen/Cargo.toml --bin generate -- --out dist-assets` (outputs canonical PDFs + `sitemap.txt`).
6. Assemble Pages artifact: route copies of `index.html` + trunk assets + top-level PDFs + `sitemap.txt`.

## 6. Testing and Quality Gates
- Execute `cargo test --manifest-path sitegen/Cargo.toml` before commits to cover parser/renderer and asset generation logic.
- Run `cargo test` inside `scripts/convert_cv` when modifying that helper crate.
- Ensure Typst compilation succeeds for canonical outputs so downloadable PDFs stay in sync.

## 7. Repository Layout Summary
```
/
├── src/                  # Leptos application source
├── content/              # Shared media assets (avatar, theme icons)
├── DOCS/                 # Architecture notes and operational documentation
├── scripts/convert_cv/   # Markdown-to-HTML converter CLI (Rust)
├── sitegen/              # Rust crate with validate/generate binaries
├── typst/                # Typst sources for EN/RU PDFs
└── profiles/             # Localized CV Markdown sources
```

## 8. Outputs and Deployment Targets
- `dist/index.html` plus route copies (`ru`, `rust-developer`, `cto`) — landing pages for all audiences.
- `dist/Belyakov_*.pdf` (12 canonical files) — downloadable PDFs copied from Typst builds.
- `dist/sitemap.txt` — flat list of published URLs for search engines.
