#!/usr/bin/env bash
# scripts/repo-setup.sh

set -Eeuo pipefail
trap 'rc=$?; echo -e "\n!! repo-setup failed at line $LINENO while running: $BASH_COMMAND (exit $rc)" >&2; exit $rc' ERR

REPO_SLUG="${REPO_SLUG:-qqrm/CV}"

command -v gh >/dev/null || { echo "gh not found" >&2; exit 1; }
gh auth status >/dev/null 2>&1 || { echo "gh not authenticated" >&2; exit 1; }

git config --global credential.helper '!gh auth git-credential'
git config --global url."https://github.com/".insteadOf git@github.com:
git config --global push.autoSetupRemote true
git config --global fetch.prune true
git config --global init.defaultBranch main

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || git init

if git remote get-url origin &>/dev/null; then
  : # origin already exists
else
  git remote add origin "https://github.com/${REPO_SLUG}.git"
fi

git fetch origin --prune --tags || true
git rev-parse --verify origin/main >/dev/null 2>&1 && git branch --track main origin/main 2>/dev/null || true
git checkout -q main 2>/dev/null || true
git merge --ff-only origin/main 2>/dev/null || true

echo "Repo setup ready"
