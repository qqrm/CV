# Architecture Overview

This document outlines how the resume is generated and how the CI/CD pipeline operates.

## CV Generation Workflow
The `sitegen` tool transforms Markdown and Typst sources into HTML pages and PDF files.

```mermaid
flowchart TD
    A[Markdown sources] --> B[sitegen]
    B --> C[HTML output]
    B --> D[Typst PDFs]
    C --> E[docs/ folder]
    D --> E
```

## CI/CD Pipeline
GitHub Actions automate checks, merging and releases.

```mermaid
flowchart TD
    P[Pull request] --> Q[PR Checks]
    Q -->|success| M[Auto Merge]
    Q -->|failure| P
    M --> R[Release]
    R --> S[Publish site and PDFs]
```
