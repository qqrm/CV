#!/usr/bin/env bash
set -euo pipefail

# Install Typst CLI if it is not already available.
if ! command -v typst >/dev/null 2>&1; then
  if ! command -v curl >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y curl
  fi
  tmpdir="$(mktemp -d)"
  curl -L "https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz" -o "$tmpdir/typst.tar.xz"
  tar -xf "$tmpdir/typst.tar.xz" -C "$tmpdir"
  sudo mv "$tmpdir/typst-x86_64-unknown-linux-musl/typst" /usr/local/bin/typst
  rm -rf "$tmpdir"
fi
