#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${SCRIPT_DIR}"
ORIGIN_URL="https://github.com/qqrm/CV.git"

run_with_privileges() {
  if command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    "$@"
  fi
}

configure_git_remote() {
  if ! git -C "${REPO_ROOT}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "${REPO_ROOT} is not a git repository." >&2
    exit 1
  fi

  if ! git -C "${REPO_ROOT}" remote get-url origin >/dev/null 2>&1; then
    git -C "${REPO_ROOT}" remote add origin "${ORIGIN_URL}"
  elif [[ "$(git -C "${REPO_ROOT}" remote get-url origin)" != "${ORIGIN_URL}" ]]; then
    git -C "${REPO_ROOT}" remote set-url origin "${ORIGIN_URL}"
  fi
}

install_typst() {
  if command -v typst >/dev/null 2>&1; then
    return
  fi

  if ! command -v curl >/dev/null 2>&1; then
    run_with_privileges apt-get update
    run_with_privileges apt-get install -y curl
  fi

  tmpdir="$(mktemp -d)"
  curl -Ls "https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz" -o "${tmpdir}/typst.tar.xz"
  tar -xf "${tmpdir}/typst.tar.xz" -C "${tmpdir}"
  run_with_privileges mv "${tmpdir}/typst-x86_64-unknown-linux-musl/typst" /usr/local/bin/typst
  rm -rf "${tmpdir}"
}

install_fonts() {
  if command -v fc-list >/dev/null 2>&1 && fc-list | grep -qi "Latin Modern Roman"; then
    return
  fi

  run_with_privileges apt-get update
  run_with_privileges apt-get install -y fontconfig fonts-lmodern
}

configure_git_remote
install_typst
install_fonts
