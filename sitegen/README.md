# Генератор сайта

Этот каталог содержит исходный код утилиты, создающей HTML‑версии резюме.

## Требования

- Rust (установить можно через `rustup`)
- Утилиты `latexmk` и `typst` для проверки PDF
- Полноценный дистрибутив TeX (например, `texlive-full`) для сборки PDF

## Запуск

1. Установите Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```
2. Выполните генерацию сайта:
   ```bash
   cargo run --manifest-path sitegen/Cargo.toml
   ```

Сгенерированные HTML‑файлы появятся в каталоге `docs/` (английская версия) и в `docs/ru/` (русская версия).
