#!/usr/bin/env bash
set -euo pipefail

required_dist_assets=(
  dist/index.html
  dist/favicon.svg
  dist/avatar.jpg
  dist/sun.svg
  dist/moon.svg
  dist/robots.txt
)

for path in "${required_dist_assets[@]}"; do
  test -s "$path"
done

grep -q "Disallow: /" dist/robots.txt

routed_pages=(
  dist/ru/index.html
  dist/rust-developer/index.html
  dist/rust-developer/ru/index.html
  dist/cto/index.html
  dist/cto/ru/index.html
)

for page in "${routed_pages[@]}"; do
  mkdir -p "$(dirname "$page")"
  cp dist/index.html "$page"
  test -s "$page"
done


canonical_pdfs=(
  Belyakov_en_light.pdf
  Belyakov_en_dark.pdf
  Belyakov_ru_light.pdf
  Belyakov_ru_dark.pdf
  Belyakov_rustdev_en_light.pdf
  Belyakov_rustdev_en_dark.pdf
  Belyakov_rustdev_ru_light.pdf
  Belyakov_rustdev_ru_dark.pdf
  Belyakov_cto_en_light.pdf
  Belyakov_cto_en_dark.pdf
  Belyakov_cto_ru_light.pdf
  Belyakov_cto_ru_dark.pdf
)

for pdf in "${canonical_pdfs[@]}"; do
  if [[ "$pdf" == *_ru_* ]]; then
    src="typst/ru/$pdf"
  else
    src="typst/en/$pdf"
  fi

  test -s "$src"
  cp "$src" "dist/$pdf"
  test -s "dist/$pdf"
done

pdf_count=$(find dist -maxdepth 1 -type f -name '*.pdf' | wc -l)
if [ "$pdf_count" -ne 12 ]; then
  echo "Expected exactly 12 PDFs in dist root, found $pdf_count" >&2
  exit 1
fi

wasm_count=$(find dist -type f \( -name '*.wasm' -o -name '*.wasm.gz' -o -name '*.wasm.br' \) | wc -l)
js_count=$(find dist -type f \( -name '*.js' -o -name '*.js.gz' -o -name '*.js.br' \) | wc -l)
css_count=$(find dist -type f \( -name '*.css' -o -name '*.css.gz' -o -name '*.css.br' \) | wc -l)

if [ "$wasm_count" -lt 1 ] || [ "$js_count" -lt 1 ] || [ "$css_count" -lt 1 ]; then
  echo "Missing wasm/js/css assets in dist" >&2
  exit 1
fi

grep -q '/CV/' dist/index.html
grep -R -n 'releases/latest/download/' dist >/dev/null
