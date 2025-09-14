#!/usr/bin/env python3
"""Convert a Markdown CV to HeadHunter JSON."""
import json
import sys
from pathlib import Path

import markdown


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Usage: convert_cv.py <input.md>")

    src = Path(sys.argv[1])
    html = markdown.markdown(src.read_text(encoding="utf-8"))
    data = {"body": html}
    json.dump(data, sys.stdout, ensure_ascii=False)


if __name__ == "__main__":
    main()
