# New WebAssembly Site Build

The `new/` directory hosts the Leptos-based, client-rendered CV. Use Trunk to produce a static bundle under `new/dist` so it can be published to GitHub Pages at `/CV/`.

## Prerequisites
- Install the WebAssembly target: `rustup target add wasm32-unknown-unknown`.
- Install Trunk: `cargo install trunk`.

## Build steps
From the repository root run:

```bash
cd new
trunk build --release
```

The output appears in `new/dist/` with asset URLs scoped to `/CV/` for default hosting.
