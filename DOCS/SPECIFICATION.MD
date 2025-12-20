# Specification: CV Repository Architecture & CI/CD Pipeline

---

## 1. Project Overview

This repository implements an automated, multilingual CV generator and publisher with the following goals:

- Maintain a single CV narrative in Markdown (`profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD`).
- Render localized Typst PDFs via a shared Typst template (`templates/resume.typ`).
- Generate a static website backed by the `sitegen` Rust crate, bundling localized landing pages for English and Russian plus downloadable PDFs.
- Keep all build artifacts out of Git; CI/CD regenerates PDFs and HTML on demand.

---

## 2. Repository Structure

```
/
  profiles/
    cv/
      en/CV.MD
      ru/CV_RU.MD
  templates/
    resume.typ
  typst/
    en/Belyakov_en.typ
    en/Belyakov_en_dark.typ
    ru/Belyakov_ru.typ
    ru/Belyakov_ru_dark.typ
  sitegen/
    src/
      bin/generate.rs
      bin/validate.rs
      parser.rs
      renderer.rs
  content/avatar.jpg
  DOCS/
    style.css
    favicon.svg
  dist/
    (Generated PDFs/HTML — ignored by Git)
```

---

## 3. Application Logic

- Parse canonical Markdown CVs (EN/RU) to drive both HTML and PDF outputs.
- Render Typst PDFs for each locale (light + dark themes) using the shared `templates/resume.typ` component. The Typst entry points (`typst/*/Belyakov_*.typ`) simply forward the locale to the template.
- Generate a multilingual static site with `sitegen`:
  - `generate.rs` reads the Markdown sources, converts them to HTML, injects duration metadata, annotates PDF links with light/dark options, and emits localized landing pages under `dist/`.
  - `validate.rs` ensures that the Markdown inputs exist and parse cleanly.
- No separate resume Markdown exists; all artifacts flow from the canonical CV narrative.

---

## 4. Multilanguage Support

- Every artifact (HTML, PDF) is produced in English and Russian.
- Language switching on the website is instant (JS-driven), and all UI/links update accordingly.
- Landing pages present locale-specific navigation without role switching.

---

## 5. Data Handling

- Markdown is the single source of truth.
- Typst PDFs (`typst/en/*.pdf`, `typst/ru/*.pdf`) are generated on demand and copied to `dist/` during site generation.
- HTML pages inside `dist/` are regenerated every time `sitegen generate` runs; they should never be checked into Git.

---

## 6. CI/CD Requirements

### 6.1 On Pull Requests and Push

- **Build Rust application** (`cargo build --release`).
- **Validate input files**: `cargo run --release --bin validate -- validate`.
- **Generate PDFs and HTML**: `cargo run --release --bin generate -- generate`.
- Ensure Typst compilation succeeds for `typst/en/Belyakov_en.typ`, `typst/en/Belyakov_en_dark.typ`, `typst/ru/Belyakov_ru.typ`, and `typst/ru/Belyakov_ru_dark.typ`.
- If any step fails, the pipeline is red and the PR is blocked.

### 6.2 On Release (Merge to main/master)

- Repeat the build, validate, and generate steps above.
- Create a GitHub Release and attach the localized PDFs.
- Deploy `dist/` to GitHub Pages.

### 6.3 Recurring Content Refresh

- Triggered manually when downstream consumers require updated publication timestamps.
- Steps:
  1. Run the validation binary to ensure the Markdown inputs are unchanged.
  2. Regenerate PDFs and HTML (`generate` binary + Typst compiles).
  3. Publish refreshed artifacts to GitHub Pages/Releases without altering the CV narrative.

---

## 7. Example GitHub Actions Workflow

```yaml
name: Build, Test & Release CV

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install Typst CLI
        run: cargo install typst-cli

      - name: Format
        run: cargo fmt --all --check

      - name: Build app
        run: cargo build --release

      - name: Validate inputs
        run: cargo run --release --bin validate -- validate

      - name: Generate PDFs and HTML
        run: cargo run --release --bin generate -- generate

      - name: Compile Typst PDFs
        run: |
          typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en_light.pdf
          typst compile --root . typst/en/Belyakov_en_dark.typ typst/en/Belyakov_en_dark.pdf
          typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru_light.pdf
          typst compile --root . typst/ru/Belyakov_ru_dark.typ typst/ru/Belyakov_ru_dark.pdf

  release:
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    needs: build-and-test
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build and generate artifacts
        run: |
          cargo build --release
          cargo run --release --bin generate -- generate

      - name: Release PDFs
        uses: softprops/action-gh-release@v2
        with:
          files: dist/Belyakov_*.pdf

      - name: Deploy GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          publish_dir: ./dist
          github_token: ${{ secrets.GITHUB_TOKEN }}
```

---

## 8. CLI Interface (Recommended)

- `cargo run --release --bin validate -- validate` — Validate all data sources.
- `cargo run --release --bin generate -- generate` — Generate HTML/PDF bundles.
- Typst CLI — build localized PDFs:
  - `typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en_light.pdf`
  - `typst compile --root . typst/en/Belyakov_en_dark.typ typst/en/Belyakov_en_dark.pdf`
  - `typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru_light.pdf`
  - `typst compile --root . typst/ru/Belyakov_ru_dark.typ typst/ru/Belyakov_ru_dark.pdf`

---

## 9. UX & Links

- **Main pages**: `dist/index.html` and `dist/ru/index.html` display the canonical CV with localized chrome and PDF links.
- All PDF links expose light/dark variants with `data-*` attributes populated by `generate.rs`.
