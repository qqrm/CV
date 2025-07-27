README files in this repository are usually edited manually. Do not change README.md or README_ru.md unless the task explicitly requests it.

Run tests before commits:

```
cargo test --manifest-path sitegen/Cargo.toml
```

Also check the local PDF build with LaTeX and Typst:

```
latexmk -pdf -quiet -cd latex/en/Belyakov_en.tex
latexmk -pdf -quiet -cd latex/ru/Belyakov_ru.tex
typst compile typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf
typst compile typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf
```

Before committing, ensure that binary files (e.g., PDF) are not included in the diff or added to the repository.

All project documentation must be written in English. Use English for code comments as well.

Make a best effort to fix compilation errors, linter warnings, and failing tests before submitting a pull request.
