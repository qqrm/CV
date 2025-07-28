All documentation in this repository is maintained in English. Source code comments must also be written in English.

README files are usually edited manually. Do not change `README.md` or `README_ru.md` unless the task explicitly requires it.

Before committing, run the tests:

```
cargo test --manifest-path sitegen/Cargo.toml
```

Also verify local PDF builds with LaTeX and Typst:

```
latexmk -pdf -quiet -cd latex/en/Belyakov_en.tex
latexmk -pdf -quiet -cd latex/ru/Belyakov_ru.tex
typst compile typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf
typst compile typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf
```

If any of the PDF build tools (`latexmk` or `typst`) are missing, try installing
them with `apt-get` (for LaTeX) or `cargo install typst-cli`. When installation
fails because of network or permission issues, note this in the PR summary.

For LaTeX builds you may need packages `texlive-latex-recommended`,
`texlive-latex-extra`, `texlive-fonts-recommended`, `texlive-lang-cyrillic` and
`latexmk`. Install them with:

```
sudo apt-get update
sudo apt-get install -y texlive-latex-recommended texlive-latex-extra \
  texlive-fonts-recommended texlive-lang-cyrillic latexmk
```

To replicate CI pipelines locally you can use the `act` tool.

Ensure binary files (for example PDFs) do not appear in the diff and are not added to the repository.
