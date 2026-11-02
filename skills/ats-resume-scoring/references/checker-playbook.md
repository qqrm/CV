# ATS Checker Playbook

Use this reference only when external scoring services are part of the task.

## Select Checkers by Purpose

- Client-side deterministic checker: best for privacy, parser diagnostics, and reproducible keyword coverage.
- Semantic or AI-backed matcher: useful for role alignment and rewrite ideas, but scores may vary between runs.
- Platform-native preview or parser: best evidence for the exact document structure accepted by that platform.

Prefer redacted pasted text over uploading a personal document. Treat every external checker as a third party even when it advertises local processing.

## Record Each Run

For every score, retain:

- checker name and URL;
- timestamp;
- resume variant identifier or hash;
- whether the input was plain text, PDF, or DOCX;
- aggregate score and component scores;
- matched and missing keywords;
- warnings, rate limits, or login gates.

Do not store contact details or confidential resume text in a committed report.

## Calibrate Before Chasing a Number

When a requested target such as 90-95% appears unreachable:

1. Run the real resume against the vacancy.
2. Create a positive control that includes the vacancy's own important wording.
3. Score the control once or twice.
4. If the control remains below the target, treat that value as the observed ceiling for the checker and report it.

Do not bypass rate limits or create extra accounts to obtain more runs. Switch to a deterministic local calculation or wait for the service to recover.

## Interpret Components Separately

- A low parser score with high keyword coverage usually means headings, dates, contacts, or file structure are not recognized.
- A high keyword score with weak semantic alignment means the words exist without enough evidence of scope or outcomes.
- A semantic checker can miss exact terms or change its keyword list between runs. Verify important claims with deterministic matching.
- A language mismatch can depress heading and date recognition. Do not rewrite a Russian resume with English headings solely to satisfy an English-only demo checker.

## Dense Keyword Coverage

If dense coverage is requested, use a concise `Core competencies` block and keep it readable. Add exact variants that differ materially, such as `AI SDLC`, `AI-assisted development`, and `AI agents`, while avoiding repeated filler.

Classify sensitive additions:

- Confirmed experience: write as an achievement with evidence.
- Defensible capability: write as scope the candidate can own.
- Domain interest: label it as a target domain, not past employment.
- Genuine gap: keep it out of historical claims and report it separately.
