# Architecture Overview

This document outlines how the CV artifacts are generated and deployed.

## CV Generation Workflow

- Markdown in `profiles/cv/**` is the canonical narrative source.
- Typst entry points in `typst/en` and `typst/ru` compile to 4 canonical PDFs.
- Leptos (root crate) renders the website; Trunk bundles it for GitHub Pages at `/CV/`.
- A shared assembly script prepares the final Pages artifact by adding route index files, canonical PDFs, and `robots.txt`.

### Canonical PDF filenames

- `Belyakov_en_light.pdf`
- `Belyakov_en_dark.pdf`
- `Belyakov_ru_light.pdf`
- `Belyakov_ru_dark.pdf`

### Website routes

The deployed Pages routes are stable:

- `/CV/`
- `/CV/ru/`

![CV generation workflow](diagrams/CV_GENERATION.svg)

## CI/CD Pipeline

GitHub Actions runs:

- PR CI: builds canonical PDFs + Trunk site, assembles and validates the same `dist/` structure used in deployment.
- Release deploy: builds Trunk and canonical PDFs, assembles Pages artifact, and deploys Pages with PDF assets included.
- Post-deploy verify: `curl` checks for public routes and representative PDF URLs.

![CI/CD pipeline](diagrams/CI_PIPELINE.svg)
