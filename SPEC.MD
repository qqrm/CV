# Repository Specification

## 1. Purpose and Scope
- Maintain canonical CV content in English and Russian Markdown sources.
- Generate publishable artifacts: a Leptos/Trunk website and 12 canonical Typst PDFs.
- Keep generated files reproducible through CI/CD assembly and validation.

## 2. Authoritative Content Sources
- `profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD` — base CV narratives.
- `profiles/rust-developer/*` and `profiles/cto/*` — profile-specific narratives.
- `content/avatar.jpg`, `content/sun.svg`, `content/moon.svg` — shared site assets.

## 3. Templates and Layouts
- Typst entry points under `typst/en` and `typst/ru` generate localized canonical PDFs.
- `templates/resume.typ` provides shared rendering logic for all PDF variants.
- Root Leptos app (`Cargo.toml`, `index.html`, `src/lib.rs`, `Trunk.toml`) generates website assets.

## 4. Rust Tooling
- Root crate builds the Leptos website bundle with Trunk.
- `scripts/convert_cv` is a standalone helper that converts markdown CV data into JSON + HTML.
- `scripts/assemble_pages_dist.sh` assembles the final deploy artifact from Trunk output and canonical PDFs.

## 5. Build and Generation Workflow
1. Run `./repo-setup.sh` to install prerequisites and fonts.
2. Build canonical Typst PDFs (12 files: light/dark for CV, Rust Developer, CTO in EN/RU).
3. Build site bundle: `trunk build --release --config Trunk.toml`.
4. Assemble Pages artifact: `scripts/assemble_pages_dist.sh`.

## 6. Testing and Quality Gates
- Validate canonical PDF generation and signatures in CI.
- Validate assembled `dist/` includes required routes, static assets, runtime bundle files, 12 PDFs, and robots.txt.
- Run `cargo test` inside `scripts/convert_cv` when modifying that helper.

## 7. Repository Layout Summary
```
/
├── src/                  # Leptos application source
├── content/              # Shared media assets (avatar, theme icons)
├── DOCS/                 # Architecture and operations docs
├── scripts/convert_cv/   # Markdown-to-HTML converter CLI (Rust)
├── scripts/              # Build assembly scripts
├── typst/                # Typst sources for EN/RU PDFs
└── profiles/             # Localized CV Markdown sources
```

## 8. Outputs and Deployment Targets
- `dist/index.html` plus route copies for `ru`, `rust-developer`, and `cto`.
- 12 canonical PDFs in `dist/` root and release assets:
  - `Belyakov_en_light.pdf`, `Belyakov_en_dark.pdf`
  - `Belyakov_ru_light.pdf`, `Belyakov_ru_dark.pdf`
  - `Belyakov_rustdev_en_light.pdf`, `Belyakov_rustdev_en_dark.pdf`
  - `Belyakov_rustdev_ru_light.pdf`, `Belyakov_rustdev_ru_dark.pdf`
  - `Belyakov_cto_en_light.pdf`, `Belyakov_cto_en_dark.pdf`
  - `Belyakov_cto_ru_light.pdf`, `Belyakov_cto_ru_dark.pdf`
- `dist/robots.txt` with `Disallow: /`.
