All documentation in this repository is maintained in English. Source code comments must also be written in English.

README files are usually edited manually. Do not change `README.md` or `README_ru.md` unless the task explicitly requires it.

Before committing, run the tests:

```
cargo test --manifest-path sitegen/Cargo.toml
```

Also verify local PDF builds with Typst:

```
typst compile typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf
typst compile typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf
```

If the Typst CLI is missing, install it with `cargo install typst-cli`. When installation
fails because of network or permission issues, note this in the PR summary.

To replicate CI pipelines locally you can use the `act` tool.

Tooling notes:
- The GitHub CLI (`gh`) is available for interacting with GitHub.
- Always rebase your work onto the latest `main` branch before pushing.

Ensure binary files (for example PDFs) do not appear in the diff and are not added to the repository.

When analyzing incoming tasks, apply the following roles:
- **R Business Analytica** reviews initial requirements and creates backlog items.
- **Architect** provides technical input after the analyst stage.
- **R DevOps** handles CI or infrastructure related tasks.
- **Seniora** (senior developer) responds to general development tasks.
