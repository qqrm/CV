# CV Repository Specification

## Goals

- Maintain a single public CV narrative in English and Russian.
- Generate publishable artifacts: a Leptos/Trunk website and 4 canonical Typst PDFs.
- Keep generated assets out of the repository; CI builds and publishes them.

## Source Layout

- `profiles/cv/en/CV.MD`, `profiles/cv/ru/CV_RU.MD` — canonical CV narratives.
- `typst/en`, `typst/ru` — Typst entry points for light/dark PDF outputs.
- `src/lib.rs` — Leptos website that renders the Markdown CV.
- `scripts/assemble_pages_dist.sh` — production-like Pages artifact assembly.

## Artifact Contract

CI must:

- Build the Leptos site with Trunk.
- Build exactly 4 canonical PDFs.
- Assemble `dist/` with `/CV/` and `/CV/ru/` route entry points.
- Validate assembled `dist/` includes required static assets, runtime bundle files, 4 PDFs, and `robots.txt`.

## Canonical Routes

- `/CV/`
- `/CV/ru/`

## Canonical PDFs

- `Belyakov_en_light.pdf`, `Belyakov_en_dark.pdf`
- `Belyakov_ru_light.pdf`, `Belyakov_ru_dark.pdf`
