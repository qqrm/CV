# Agent guide

All documentation and code comments must be in English.

Do not edit README.md or README\_RU.md unless the task explicitly requires it.

Before committing, run tests:

```bash
cargo test --manifest-path sitegen/Cargo.toml
```

Verify local PDF builds with Typst:

```bash
typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf
typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf
```

Before starting any task, connect to the avatars.mcp server at https\://qqrm.github.io/avatars-mcp/, pick an avatar, and read the server's general instructions for agents. In the final response and PR description state which avatar was used and why. If the server is unreachable, note this in the PR.

Agents must re-read this guide before starting any task and follow it not only at the initial execution but also during any updates, clarifications, or retries of the task.

## Git and PR workflow

Always start from a fresh branch created off the latest origin/main. Never try to rebase an existing branch. On each run or retry, recreate the branch from origin/main and reapply the solution.

### Procedure

> All commands must be executed from the repository root.

1. Fetch the latest changes from origin:

```bash
git fetch origin
```

2. Reset local changes and remove untracked files:

```bash
git reset --hard
git clean -fdx
```

3. Create a new branch from origin/main:

```bash
git switch -C "<task-branch>" origin/main
```

4. Implement and commit your changes:

```bash
git add -A
git commit -m "task: <short summary>"
```

5. Push and create/update PR:

```bash
git push -u origin "<task-branch>" --force-with-lease
# If PR already exists, update instead of creating
if gh pr view --head "<task-branch>" >/dev/null 2>&1; then
  gh pr edit --title "task: <short summary>"
else
  gh pr create --base main --head "<task-branch>" --fill
fi
```

Branch naming:

```
codex/<date>-<short-slug>
# example: codex/2025-08-09-typst-lint
```

