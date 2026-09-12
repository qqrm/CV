# Agent guide

All source code, documentation, and comments must be in English. Agents must respond to the user in Russian.

Critically evaluate each user request. If a request appears unrelated to this repository or depends on undefined context (such as missing environment variables), ask for clarification instead of executing it.

Do not edit README.md or README\_RU.md unless the task explicitly requires it.

For user-requested changes to the published CV or website, local edits are not a complete result. Finish the task by committing the change, pushing it to `origin/main`, waiting for the GitHub Pages deployment workflow to complete successfully, and verifying that the public site reflects the update. When the change affects CV content or assets, also run and verify the history-collapse workflow so the public repository history ends as a single final commit.

Use the tools already available in the environment for normal local edits, and install missing build tools in user space when needed.
The GitHub CLI (`gh`) is already authenticated in this environment. Verify the remote with `git remote -v` and use `origin/main` as the canonical target—do not create alternative remotes.
When changing CV content, keep English and Russian versions in sync and update any related fixtures accordingly.
Before committing, run tests:

```bash
cargo test
cargo test --manifest-path scripts/convert_cv/Cargo.toml
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
