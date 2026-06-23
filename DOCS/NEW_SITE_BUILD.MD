# WASM Site Build

The repository root hosts the Leptos SPA. Trunk builds the static bundle to `dist/` with `public_url = "/CV/"`.

## Prerequisites
- `rustup target add wasm32-unknown-unknown`
- `cargo install trunk`
- Typst available if you need PDF rebuilds (`cargo install typst-cli`)

## Build website

```bash
trunk build --release --config Trunk.toml
```

## Build canonical PDFs + assemble deploy artifact

```bash
# Build canonical PDFs (CI uses .github/actions/build-pdfs)
for src in typst/en/*.typ typst/ru/*.typ; do
  stem="${src%.typ}"
  if [[ "$stem" == *_dark ]]; then
    typst compile --root . "$src" "${stem}.pdf"
  else
    typst compile --root . "$src" "${stem}_light.pdf"
  fi
done

scripts/assemble_pages_dist.sh
```

The final `dist/` includes:
- `index.html` and the `ru` routed page
- Trunk-generated runtime assets (`.wasm`, `.js`, `.css`)
- 4 canonical PDFs at `dist/` root
- `robots.txt` with `Disallow: /`
