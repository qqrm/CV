# Task: Analyze Static Site Build Pipeline

## Description
The GitHub Pages site sometimes serves outdated content. After removing LaTeX files from the repository they still appear on the deployed site.
Investigate the existing build pipeline and update it so that generated pages are always consistent with the repository contents.

## Related files
- .github/workflows/release.yml
- sitegen/src/*
- DOCS/

## Acceptance criteria
- The workflow reliably rebuilds the site on each push to `main`.
- Stale files are removed from GitHub Pages during deployment.
- Documentation describes the build steps.

## Status
- [ ] Planned
- [ ] In progress
- [ ] Done
