# GitHub Pages Deployment from a Separate Branch

This guide explains how to publish the Leptos/Trunk bundle from a branch that is different from the default branch.

## Choose the Pages source branch
1. Open **Settings → Pages** in GitHub.
2. In **Source**, select the branch that hosts built assets (for example `gh-pages`) and choose folder `/` unless using `/docs` intentionally.
3. Save. GitHub Pages will publish only from that branch.

## Manually publish from a feature branch

```bash
# Build website bundle from repository root
trunk build --release --config Trunk.toml

# Build canonical PDFs (or run the same canonical naming commands used in CI)
for src in typst/en/*.typ typst/ru/*.typ; do
  stem="${src%.typ}"
  if [[ "$stem" == *_dark ]]; then
    typst compile --root . "$src" "${stem}.pdf"
  else
    typst compile --root . "$src" "${stem}_light.pdf"
  fi
done

# Assemble deploy artifact (routes + PDFs + robots)
scripts/assemble_pages_dist.sh

# Prepare a worktree for Pages branch
# If the branch does not exist remotely yet, create it once with: git switch --orphan gh-pages
# Otherwise, fetch the latest copy
git fetch origin gh-pages:gh-pages

git worktree add /tmp/gh-pages gh-pages
rsync -av --delete dist/ /tmp/gh-pages/
(cd /tmp/gh-pages && git add . && git commit -m "Publish site" && git push origin gh-pages)

# Clean up
rm -rf /tmp/gh-pages
```

Notes:
- Keep only built assets on the Pages branch.
- Re-run build + assembly whenever source content changes.
- If using `CNAME`, keep it in the Pages branch root.
