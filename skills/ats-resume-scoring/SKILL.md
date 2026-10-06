---
name: ats-resume-scoring
description: Score a resume against a job description, diagnose parser and keyword gaps, calibrate unreliable ATS checkers, and produce a tailored but defensible resume variant. Use for ATS screening, vacancy matching, resume keyword optimization, or comparing several resume variants.
---

# ATS Resume Scoring

Treat ATS scoring as three separate measurements:

1. Parseability: whether the document exposes headings, dates, contacts, and plain text correctly.
2. Keyword coverage: whether important terms from the vacancy occur in the resume.
3. Semantic alignment: whether the described scope, seniority, and evidence fit the role.

Never present one tool's aggregate number as a universal hiring probability.

## Workflow

1. Preserve the original resume and create a separate variant for each materially different target role.
2. Extract the vacancy's required and preferred terms. Classify each missing term as confirmed experience, defensible capability, target-domain interest, or genuine gap.
3. Prefer a client-side checker for the first pass. Redact contact details, confidential employer names, and NDA-specific product identifiers before sending text to any external service.
4. Run at least one independent checker when practical. Record parser score, keyword score, semantic score, missing terms, and the exact resume version used.
5. If checkers disagree or a target score appears unreachable, calibrate the tool with a positive control. Re-score a synthetic input containing the vacancy wording. If the control also scores below the requested threshold, report the checker's observed ceiling instead of endlessly rewriting the resume.
6. Add confirmed terms inside the role where they were used. Put cross-cutting technologies and compact keyword coverage in a normal `Core competencies` section.
7. When the user explicitly requests dense keyword coverage, optimize for it, but do not silently turn an unverified keyword into a historical achievement. Preserve the distinction between experience, capability, interest, and gap.
8. Re-score the final variant and report `before -> after`, remaining hard gaps, checker limitations, and any terms that still require user confirmation.

Read [references/checker-playbook.md](references/checker-playbook.md) before using third-party ATS services. Use [scripts/keyword_coverage.py](scripts/keyword_coverage.py) when external services are unavailable, rate-limited, or too noisy for deterministic coverage checks.

## Operational Constraints

- Stop on CAPTCHA, account restriction, or rate limiting. Do not bypass or evade platform controls.
- Scoring does not authorize publishing a resume, changing its visibility, applying to a vacancy, or messaging an employer.
- Do not put the target employer's name in the resume title unless the user explicitly requests it. Keep target-to-variant mappings in a private local file when they contain personal data or application strategy.
- Avoid public project links in anonymous resumes unless the user explicitly wants them.
- Keep confidential product details abstract while retaining permitted scale, domain, duration, and outcome evidence.

## Output

Provide a compact comparison table with the resume variant, checker, parseability, keyword coverage, semantic alignment, and remaining gaps. Distinguish measured values from inference. If a checker produces a single opaque score, label it as that checker's score rather than an ATS ground truth.
