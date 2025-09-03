#!/usr/bin/env bash
set -euo pipefail

# Install fonts required for Typst templates.
if ! fc-list | grep -qi "Latin Modern Roman"; then
  sudo apt-get update
  sudo apt-get install -y fonts-lmodern
fi
