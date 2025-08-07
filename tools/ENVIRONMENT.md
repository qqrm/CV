# 🛠️ Rust-based Tooling Environment

Our team prefers Rust-native tools wherever possible, both for speed and hackability.
This toolset is provided as standard for every new team member.

## Core CLI Tools
- [cargo-make](https://github.com/sagiegurari/cargo-make) — Task runner / orchestrator
- [cargo-watch](https://github.com/watchexec/cargo-watch) — Watches file changes and runs tasks
- [cargo-edit](https://github.com/killercup/cargo-edit) — CLI for editing Cargo.toml
- [cargo-nextest](https://nexte.st/) — Parallel Rust test runner
- [cargo-audit](https://github.com/rustsec/rustsec) — Security audits for dependencies
- [proptest](https://github.com/proptest-rs/proptest) — Property-based testing
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) — Fuzz testing for binaries
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) — Code coverage
- [mdBook](https://github.com/rust-lang/mdBook) — Docs generator
- [typst](https://typst.app/) — PDF/report generator
- [zola](https://www.getzola.org/) — Static site generator for docs
- [svgbob](https://github.com/ivanceras/svgbob) — ASCII-to-SVG diagrams
- [gitui](https://github.com/extrawurst/gitui) — Fast terminal git UI
- [delta](https://github.com/dandavison/delta) — Modern git diff viewer
- [helix](https://helix-editor.com/) — Terminal editor
- [zellij](https://zellij.dev/) — Terminal workspace/tmux alternative
- [fd](https://github.com/sharkdp/fd) — Fast alternative to find
- [bat](https://github.com/sharkdp/bat) — Cat clone with syntax highlighting
- [ripgrep](https://github.com/BurntSushi/ripgrep) — Super fast code search

## Install Everything with cargo-binstall
```bash
cargo install cargo-make cargo-watch cargo-edit cargo-nextest cargo-audit proptest cargo-fuzz cargo-tarpaulin mdbook zola svgbob gitui delta helix fd bat ripgrep
```


Builds and tests run exclusively in GitHub CI; no local Makefile is provided.
