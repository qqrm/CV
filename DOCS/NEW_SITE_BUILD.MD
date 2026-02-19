# WebAssembly Site Build

The repository root hosts the Leptos-based, client-rendered CV. Use Trunk to produce a static bundle under `dist/` so it can be published to GitHub Pages at `/CV/`.

## Prerequisites
- Install the WebAssembly target: `rustup target add wasm32-unknown-unknown`.
- Install Trunk: `cargo install trunk`.

## Build steps
From the repository root run:

```bash
trunk build --release --config Trunk.toml
```

The output appears in `dist/` with asset URLs scoped to `/CV/` via `public_url` in `Trunk.toml`.
