#!/usr/bin/env python3
"""Compute deterministic resume coverage for an explicit keyword list."""

from __future__ import annotations

import argparse
import json
import re
import unicodedata
from pathlib import Path


def normalize(value: str) -> str:
    value = unicodedata.normalize("NFKC", value).casefold()
    value = re.sub(r"[\s\-_–—/]+", " ", value)
    return re.sub(r"[^\w+#. ]+", "", value)


def load_keywords(path: Path) -> list[tuple[str, float]]:
    result: list[tuple[str, float]] = []
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        weight = 1.0
        keyword = line
        if "\t" in line:
            maybe_weight, keyword = line.split("\t", 1)
            weight = float(maybe_weight)
        result.append((keyword.strip(), weight))
    if not result:
        raise ValueError("keyword file contains no keywords")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Measure weighted exact-phrase coverage in a resume."
    )
    parser.add_argument("--resume", required=True, type=Path)
    parser.add_argument(
        "--keywords",
        required=True,
        type=Path,
        help="UTF-8 file: one keyword per line, optionally WEIGHT<TAB>KEYWORD",
    )
    parser.add_argument("--json", action="store_true", dest="as_json")
    args = parser.parse_args()

    resume_text = normalize(args.resume.read_text(encoding="utf-8"))
    keywords = load_keywords(args.keywords)
    matched = [keyword for keyword, _ in keywords if normalize(keyword) in resume_text]
    missing = [keyword for keyword, _ in keywords if normalize(keyword) not in resume_text]
    total_weight = sum(weight for _, weight in keywords)
    matched_weight = sum(
        weight for keyword, weight in keywords if normalize(keyword) in resume_text
    )
    score = round(100.0 * matched_weight / total_weight, 2)
    report = {
        "score": score,
        "matched": matched,
        "missing": missing,
        "matched_count": len(matched),
        "keyword_count": len(keywords),
    }

    if args.as_json:
        print(json.dumps(report, ensure_ascii=False, indent=2))
    else:
        print(f"Keyword coverage: {score:.2f}% ({len(matched)}/{len(keywords)})")
        if missing:
            print("Missing:")
            for keyword in missing:
                print(f"- {keyword}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
