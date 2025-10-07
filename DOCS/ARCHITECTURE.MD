# Architecture Overview

This document outlines how the resume is generated and how the CI/CD pipeline operates.

## CV Generation Workflow
The `sitegen` tool transforms Markdown and Typst sources into HTML pages and PDF files.

### Module structure
The `sitegen` crate separates responsibilities between two modules:

- `parser` converts raw files into structured data. It exposes helpers such as `month_from_en`, `month_from_ru`, `read_inline_start`, and the `InlineStartError` type.
- `renderer` produces template-friendly strings, for example `format_duration_en` and `format_duration_ru`.

These modules are independent and are re-exported by the crate root so the binaries can rely on a concise API without cross-module coupling. Binary entry points like `generate` and `validate` build on these modules to produce the final artifacts.

### Language switching
Generated HTML pages use lightweight JavaScript to persist the theme preference, annotate PDF download links for light/dark variants, and provide locale-aware navigation.

![CV generation workflow](diagrams/CV_GENERATION.svg)

## CI/CD Pipeline
GitHub Actions automate checks, merging and releases.

![CI/CD pipeline](diagrams/CI_PIPELINE.svg)
