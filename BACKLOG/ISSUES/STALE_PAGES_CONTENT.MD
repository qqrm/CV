# Issue: Stale files served on GitHub Pages

## Description
The GitHub Pages deployment sometimes retains removed files, causing outdated content such as deleted LaTeX artifacts to remain accessible.

## Steps
- Investigate the release workflow for missing cleanup steps.
- Adjust deployment to remove stale files before publishing.
- Document the updated build and deployment process.

## Acceptance Criteria
- Deployments rebuild the site on each push to `main` without leftover files.
- Obsolete files are purged from GitHub Pages during deployment.
- Documentation reflects the final workflow.
