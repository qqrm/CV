# GitHub Pages Deployment from a Separate Branch

This guide explains how to publish the static bundle (including the `new/` WebAssembly site served at `/CV/new/`) from a branch that is different from the default branch.

## Choose the Pages source branch
1. Open **Settings → Pages** in the repository on GitHub.
2. In **Source**, select the branch that will host the built assets (e.g., `gh-pages`) and set the folder to `/` unless you intentionally keep assets under `/docs`.
3. Save the configuration. GitHub Pages will pull content only from this branch going forward.

## Manually publish from a feature branch
Use a dedicated branch such as `gh-pages` to avoid committing build artifacts to the main development branches.

```bash
# Build the static bundle (includes the `/CV/new/` assets described in DOCS/NEW_SITE_BUILD.MD)
cd new
trunk build --release
cd ..

# Prepare a worktree for the Pages branch
# If the branch does not exist remotely yet, create it once with: git switch --orphan gh-pages
# Otherwise, fetch the latest copy
git fetch origin gh-pages:gh-pages

git worktree add /tmp/gh-pages gh-pages
rsync -av --delete new/dist/ /tmp/gh-pages/
(cd /tmp/gh-pages && git add . && git commit -m "Publish site" && git push origin gh-pages)

# Clean up the temporary worktree
rm -rf /tmp/gh-pages
```

Notes:
- Keep only the built assets on the Pages branch. Store source files on regular development branches.
- Re-run the build and repeat the copy whenever the static site changes.
- If you use a custom domain or `CNAME`, ensure it lives on the Pages branch root alongside the built files.
