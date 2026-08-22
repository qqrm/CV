# Specification: CV Repository Architecture & CI/CD Pipeline

## 1. Project Overview

This repository automates multilingual CV publication:

- Markdown profiles in `profiles/cv/**` are authoritative content.
- Typst builds 4 canonical PDF outputs.
- Leptos + Trunk builds the static website.
- CI/CD assembles and validates deploy artifacts without committing generated output.

## 2. Repository Structure

```
/
  Cargo.toml              # Leptos web app
  Trunk.toml
  index.html
  src/lib.rs
  profiles/cv/
  templates/
  typst/
  scripts/assemble_pages_dist.sh
  content/
```

## 3. Application Logic

- Leptos (root crate) is the authoritative website implementation for GitHub Pages.
- Typst compiles exactly 4 canonical PDFs (light/dark × 1 profile × 2 locales).
- `scripts/assemble_pages_dist.sh` creates route entry points, copies canonical PDFs to `dist/`, and validates `dist/robots.txt`.

## 4. Multilanguage and Routing

GitHub Pages routes are fixed:

- `/CV/`
- `/CV/ru/`

## 5. Data Handling

- Markdown is the source of truth.
- Typst outputs canonical PDFs to `typst/en` and `typst/ru`.
- Final Pages artifact includes Trunk bundle assets, route index files, top-level canonical PDFs, and `robots.txt`.

## 6. CI/CD Requirements

### 6.1 Pull Requests

- Build canonical PDFs with `.github/actions/build-pdfs`.
- Build Leptos with `trunk build --release --config Trunk.toml`.
- Assemble `dist/` via `scripts/assemble_pages_dist.sh`.
- Validate artifact structure locally in CI (no production network checks).

### 6.2 Main Branch Deploy

- Build canonical PDFs.
- Build Leptos bundle with Trunk.
- Assemble `dist/` with routes, PDFs, and robots.txt.
- Publish Pages artifact.
- Serve the 4 canonical PDFs from the Pages artifact.

### 6.3 Post-deploy Verification

- Run bounded `curl` retries for `/CV/` and `/CV/ru/`.
- Verify a Pages-hosted canonical PDF returns `200` with PDF content type.

## 7. CLI Interface

- `trunk build --release --config Trunk.toml`
- `scripts/assemble_pages_dist.sh`
- Typst compilation via `.github/actions/build-pdfs` (canonical naming only)
