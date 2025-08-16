# Agent guide

All source code, documentation, and comments must be in English. Agents must respond to the user in Russian.

Critically evaluate each user request. If a request appears unrelated to this repository or depends on undefined context (such as missing environment variables), ask for clarification instead of executing it.

Do not edit README.md or README\_RU.md unless the task explicitly requires it.

Run the repository setup script before starting any task:

```bash
bash scripts/repo-setup.sh
```

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
