#!/usr/bin/env bash
set -euo pipefail

required_dist_assets=(
  dist/index.html
  dist/favicon.svg
  dist/avatar.jpg
  dist/sun.svg
  dist/moon.svg
  dist/robots.txt
  dist/NotoSans-Regular.ttf
  dist/NotoSans-Bold.ttf
  dist/NotoSans-Italic.ttf
  dist/NotoSans-BoldItalic.ttf
  dist/OFL.txt
)

for path in "${required_dist_assets[@]}"; do
  test -s "$path"
done

grep -q "Disallow: /" dist/robots.txt

routed_pages=(
  dist/ru/index.html
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

write_static_html() {
  local lang="$1"
  local title="$2"
  local src="$3"
  local out="$4"

  {
    cat <<HTML
<!DOCTYPE html>
<html lang="$lang">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <meta name="robots" content="noindex, nofollow, noarchive" />
  <title>$title</title>
  <style>
    @font-face { font-family: "Noto Sans"; src: url("/CV/NotoSans-Regular.ttf") format("truetype"); font-style: normal; font-weight: 400; font-display: swap; }
    @font-face { font-family: "Noto Sans"; src: url("/CV/NotoSans-Bold.ttf") format("truetype"); font-style: normal; font-weight: 700; font-display: swap; }
    body { max-width: 920px; margin: 40px auto; padding: 0 20px; font: 16px/1.6 "Noto Sans", system-ui, -apple-system, sans-serif; }
    pre { white-space: pre-wrap; overflow-wrap: anywhere; }
  </style>
</head>
<body>
<pre>
HTML
    sed -e 's/&/\&amp;/g' -e 's/</\&lt;/g' -e 's/>/\&gt;/g' "$src"
    cat <<HTML
</pre>
</body>
</html>
HTML
  } > "$out"
}

cp profiles/cv/en/CV.MD dist/cv.md
cp profiles/cv/ru/CV_RU.MD dist/cv_ru.md
cp profiles/cv/en/CV.MD dist/cv.txt
cp profiles/cv/ru/CV_RU.MD dist/cv_ru.txt
write_static_html "en" "Alexey Belyakov - CV" profiles/cv/en/CV.MD dist/cv.html
write_static_html "ru" "Алексей Беляков - CV" profiles/cv/ru/CV_RU.MD dist/cv_ru.html

{
  printf '# Alexey Belyakov CV\n\n'
  printf 'Canonical web: https://qqrm.github.io/CV/\n'
  printf 'English Markdown: https://qqrm.github.io/CV/cv.md\n'
  printf 'Russian Markdown: https://qqrm.github.io/CV/cv_ru.md\n'
  printf 'English static HTML: https://qqrm.github.io/CV/cv.html\n'
  printf 'Russian static HTML: https://qqrm.github.io/CV/cv_ru.html\n\n'
  printf '## English CV\n\n'
  cat profiles/cv/en/CV.MD
  printf '\n\n## Russian CV\n\n'
  cat profiles/cv/ru/CV_RU.MD
} > dist/llms.txt

for text_asset in cv.md cv_ru.md cv.txt cv_ru.txt cv.html cv_ru.html llms.txt; do
  test -s "dist/$text_asset"
done

pdf_count=$(find dist -maxdepth 1 -type f -name '*.pdf' | wc -l)
if [ "$pdf_count" -ne 4 ]; then
  echo "Expected exactly 4 PDFs in dist root, found $pdf_count" >&2
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
if grep -R -n 'releases/latest/download/' dist >/dev/null; then
  echo "GitHub Release PDF links are not allowed in Pages output" >&2
  exit 1
fi
