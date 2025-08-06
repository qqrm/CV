# Architecture Overview

This document outlines how the resume is generated and how the CI/CD pipeline operates.

## CV Generation Workflow
The `sitegen` tool transforms Markdown and Typst sources into HTML pages and PDF files.

### Dynamic role titles and language switching
Generated HTML pages include a small JavaScript snippet that reads `roles.toml`, injects the role title based on the current URL, and enables language switching.

![CV generation workflow](diagrams/CV_GENERATION.svg)

## CI/CD Pipeline
GitHub Actions automate checks, merging and releases.

![CI/CD pipeline](diagrams/CI_PIPELINE.svg)
