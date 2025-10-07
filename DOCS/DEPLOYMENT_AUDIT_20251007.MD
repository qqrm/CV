# Deployment Audit 2025-10-07

## Summary
- Latest `origin/main` commit: `63bee3a4c0be09426cc7c411455caa59381eb5e6` (`Snapshot: 8415187306557e24e3c55f6589b70fd1465819bf`).
- Latest GitHub Pages build commit: `014eae5bba1e9178293d37725a642a88371dd508` (`refactor: simplify English duration formatting`).
- Pages deployment completed successfully on 2025-08-21 but is not aligned with the current `origin/main` snapshot.

## Verification Details

### GitHub Pages Build Status
- `gh api repos/qqrm/CV/pages` reports that GitHub Pages serves `https://qqrm.github.io/CV/` from the `main` branch root.
- `gh api repos/qqrm/CV/pages/builds/latest` confirms the latest build (ID `760263714`) succeeded, publishing commit `014eae5bba1e9178293d37725a642a88371dd508`.

### Landing Page (English)
- `https://qqrm.github.io/CV/` responds with HTTP 200 and `lang="en"` markup.
- The page exposes the Russian locale switcher (`Link to Russian version`) and two PDF anchors with theme-specific data attributes.
- Theme toggle updates the root `data-theme` attribute from `light` → `dark` → `light` when activated twice in a Playwright session.

### Russian Locale
- Navigating to `https://qqrm.github.io/CV/ru/` returns HTTP 200 with `lang="ru"` markup.
- Locale switcher returns to the English page, and the localized PDF anchor provides light/dark variants through the same data attributes.

### PDF Assets
- HEAD requests for `Belyakov_en_light.pdf`, `Belyakov_en_dark.pdf`, `Belyakov_ru_light.pdf`, and `Belyakov_ru_dark.pdf` all return HTTP 200.

## Observations and Follow-Up
- The production site content matches expectations (both locales, theme toggle, and PDF availability operate correctly).
- The deployed commit (`014eae5bba1e9178293d37725a642a88371dd508`) predates the latest `origin/main` snapshot (`63bee3a4c0be09426cc7c411455caa59381eb5e6`). Trigger a new Pages build or rerun the publishing workflow to align deployment with `origin/main`.
