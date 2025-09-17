#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Configure git remotes so local operations mirror the upstream repository.
REPO_ROOT="${SCRIPT_DIR}"
ORIGIN_URL="https://github.com/qqrm/CV.git"
if ! git -C "${REPO_ROOT}" remote get-url origin >/dev/null 2>&1; then
  git -C "${REPO_ROOT}" remote add origin "${ORIGIN_URL}"
elif [[ "$(git -C "${REPO_ROOT}" remote get-url origin)" != "${ORIGIN_URL}" ]]; then
  git -C "${REPO_ROOT}" remote set-url origin "${ORIGIN_URL}"
fi

# Install fonts required for Typst templates.
if ! fc-list | grep -qi "Latin Modern Roman"; then
  sudo apt-get update
  sudo apt-get install -y fonts-lmodern
fi
