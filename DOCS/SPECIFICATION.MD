# Specification: CV Repository Architecture & CI/CD Pipeline

## 1. Project Overview

This repository implements an automated, multilingual CV generator and publisher with the following goals:

- Maintain canonical CV narratives in Markdown (`profiles/...`).
- Render localized Typst PDFs via a shared Typst template (`templates/resume.typ`).
- Build the static website from the root Leptos app with Trunk.
- Keep all build artifacts out of Git; CI/CD regenerates PDFs and website assets on demand.

## 2. Repository Structure

```
/
  Cargo.toml              # Leptos web app
  Trunk.toml
  index.html
  src/lib.rs
  profiles/
  templates/
  typst/
  sitegen/
    src/bin/generate.rs   # Copies canonical PDFs + writes sitemap.txt
    src/bin/validate.rs
  content/
```

## 3. Application Logic

- Leptos (root crate) is the authoritative website implementation for GitHub Pages.
- Typst compiles 12 canonical PDFs (light/dark for each profile and language).
- `sitegen generate` no longer renders HTML: it validates PDF signatures, copies canonical PDFs, and creates `sitemap.txt`.
- `sitegen validate` ensures required Markdown inputs exist.

## 4. Multilanguage and Routing

GitHub Pages routes are fixed and preserved:

- `/CV/`
- `/CV/ru/`
- `/CV/rust-developer/`
- `/CV/rust-developer/ru/`
- `/CV/cto/`
- `/CV/cto/ru/`

The deploy workflow creates route directories and copies `dist/index.html` into each route entry point.

## 5. Data Handling

- Markdown is the source of truth.
- Typst outputs canonical PDFs under `typst/en` and `typst/ru`.
- The final Pages artifact contains the Trunk bundle, route entrypoints, top-level canonical PDFs, and `sitemap.txt`.

## 6. CI/CD Requirements

### 6.1 Pull Requests

- Build and test `sitegen`.
- Build canonical PDFs via the shared composite action.
- Build Leptos with Trunk from repo root.
- Assemble a production-like `dist/` and verify required files, routes, and bundle markers (`/CV/`, release PDF URL prefix).
- No production network checks in PR CI.

### 6.2 Main Branch Deploy

- Build sitegen binary and canonical PDFs.
- Build Leptos bundle from root with Trunk.
- Assemble `dist/` using route copies + `sitegen` assets (`*.pdf`, `sitemap.txt`).
- Publish Pages artifact and create a release attaching `sitegen_bin` and canonical PDFs.

### 6.3 Post-deploy Verification

- Run bounded retry checks with `curl` against production routes and representative PDF/release URLs.

## 7. CLI Interface

- `cargo run --manifest-path sitegen/Cargo.toml --bin validate`
- `cargo run --manifest-path sitegen/Cargo.toml --bin generate -- --out <DIR>`
- `trunk build --release --config Trunk.toml`
