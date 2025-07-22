import markdown
from pathlib import Path

readme_text = Path('README.md').read_text(encoding='utf-8')
html_body = markdown.markdown(readme_text)

html_template = f"""
<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <title>Alexey Belyakov - CV</title>
    <link rel='stylesheet' href='style.css'>
</head>
<body>
<header>
    <h1>Alexey Belyakov</h1>
</header>
<div class='content'>
{html_body}
</div>
<footer>
    <p><a href='latex/en/Belyakov_en.pdf'>Download PDF (EN)</a></p>
    <p><a href='latex/ru/Belyakov_ru.pdf'>Скачать PDF (RU)</a></p>
</footer>
</body>
</html>
"""

Path('index.html').write_text(html_template, encoding='utf-8')
