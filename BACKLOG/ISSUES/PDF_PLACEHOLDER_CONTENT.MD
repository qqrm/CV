# Issue: PDFs contain placeholders and raw Markdown

## Description
The English and Russian PDF versions of the CV served via GitHub Releases display unresolved placeholders such as `{name}` and render raw Markdown syntax (e.g., `*[Link to PDF]*`). This suggests that the PDF generation pipeline is not correctly processing templates or formatting.

## Steps
- Inspect the PDF generation workflow for missing template variable replacements.
- Ensure Markdown or Typst content is fully rendered before PDF export.
- Regenerate the PDFs and update the release artifacts.

## Acceptance Criteria
- PDF downloads no longer contain unresolved placeholders.
- Markdown formatting is properly rendered in the PDF output.
- Updated PDFs are published to GitHub Releases.
