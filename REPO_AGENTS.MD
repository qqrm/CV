# Agent guide

All source code, documentation, and comments must be in English. Agents must respond to the user in Russian.

Critically evaluate each user request. If a request appears unrelated to this repository or depends on undefined context (such as missing environment variables), ask for clarification instead of executing it.

Do not edit README.md or README\_RU.md unless the task explicitly requires it.
Run the repository setup script before starting any task:

```bash
./repo-setup.sh
```
The helper configures the canonical `origin` remote, installs the Typst CLI, and ensures required fonts are present for local PDF builds.
The GitHub CLI (`gh`) is already authenticated in this environment. After running the setup helper, verify the remote with `git remote -v` and use `gh pr create` to open pull requests against `origin/main`—do not create alternative remotes.
Before committing, run tests:

```bash
cargo test --manifest-path sitegen/Cargo.toml
```

Verify local PDF builds with Typst:

```bash
typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en_light.pdf
typst compile --root . typst/en/Belyakov_en_dark.typ typst/en/Belyakov_en_dark.pdf
typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru_light.pdf
typst compile --root . typst/ru/Belyakov_ru_dark.typ typst/ru/Belyakov_ru_dark.pdf
```

Before starting any task, load the persona catalog from https\://qqrm.github.io/codex-tools/ and pick a persona whose guidance fits the assignment. Read the persona's Markdown body, and in the final response and PR description state which persona was used and why. If the site is unreachable, note this in the PR.

Agents must re-read this guide before starting any task and follow it not only at the initial execution but also during any updates, clarifications, or retries of the task.
