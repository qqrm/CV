В этом репозитории README файлы редактируются вручную. Не изменяйте README.md и README_ru.md.

Перед коммитами запускайте тесты:

```
cargo test --manifest-path sitegen/Cargo.toml
```

Также проверяйте локальную сборку PDF через LaTeX и Typst:

```
latexmk -pdf -quiet -cd latex/en/Belyakov_en.tex
latexmk -pdf -quiet -cd latex/ru/Belyakov_ru.tex
typst compile typst/en/Belyakov_en.typ typst/en/Belyakov_en.pdf
typst compile typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru.pdf
```
