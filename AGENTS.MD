All documentation in this repository is maintained in English. Source code comments must also be written in English.

README files are usually edited manually. Do not change `README.MD` or `README_RU.MD` unless the task explicitly requires it.

Before committing, run the tests:

```
cargo test --manifest-path sitegen/Cargo.toml
```

Also verify local PDF builds with Typst:

```
typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf
typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf
```

If the Typst CLI is missing, install it with `cargo install typst-cli`. When installation
fails because of network or permission issues, note this in the PR summary.

Always connect to the `avatars.mcp` server at <https://qqrm.github.io/avatars-mcp/> to choose an avatar persona for each task (see <https://github.com/qqrm/avatars-mcp> for details). In your final response for each task and in the PR description, state which avatar was selected and briefly why; if no avatar is used, explicitly note this. If the server is unreachable, mention this in the PR.

To replicate CI pipelines locally you can use the `act` tool.

Tooling notes:
- The GitHub CLI (`gh`) is available for interacting with GitHub.
- Always base your work on the latest `main` branch before starting work or pushing. To recreate your branch from an updated `main`, use:
  1. `git fetch origin`
  2. `git checkout main && git pull origin main`
  3. `git branch -D <your-branch>` # remove the old branch if it exists locally
  4. `git checkout -b <your-branch> main`
  5. reapply your changes

- When the user explicitly requests a rebase, update `main` to the latest state and reapply the task solution from scratch using the above procedure.

Ensure binary files (for example PDFs) do not appear in the diff and are not added to the repository.

When analyzing incoming tasks, apply the following roles:
- **R Business Analytica** reviews initial requirements and creates backlog items.
- **Architect** provides technical input after the analyst stage.
- **R DevOps** handles CI or infrastructure related tasks.
- **Seniora** (senior developer) responds to general development tasks.
