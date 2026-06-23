# Agent Integration Brief — CV Positioning Upgrade

## Goal
Turn this CV into a high-signal leadership CV for the following roles:

- AI Engineering Transformation Lead
- Head of Engineering / Head of Development
- CTO / Fractional CTO
- Director of Engineering

The CV must score strongly with LLM/ATS-style evaluation for AI adoption, AI-assisted engineering transformation, agentic SDLC, engineering leadership, systems engineering, high-performance backend, Rust/C++, legacy modernization, CI/CD, reliability, and delivery governance.

## Positioning
Primary positioning:

**CTO / Head of Engineering / Fractional CTO - AI-enabled engineering teams**

Secondary positioning:

**Engineering organizations for complex backend/platform/systems products • AI-assisted SDLC • Rust/C++ modernization • Production quality**

Do not reduce the candidate to “Rust developer” or “AI tools user”. The intended positioning is:

> Engineering leader who builds AI-enabled engineering organizations, turns AI-agent adoption into a controlled production engineering system, and retains CTO/Head-level ownership over architecture, delivery governance, quality, reliability, and performance.

## Files changed
- `.github/workflows/post-deploy-verify.yml`
- `AGENT_INTEGRATION_BRIEF.md`
- `index.html`
- `profiles/cv/en/CV.MD`
- `profiles/cv/ru/CV_RU.MD`
- `scripts/assemble_pages_dist.sh`
- `src/lib.rs`
- `templates/contact_config.typ`
- `templates/resume.typ`

## Rendering changes
The role title and subtitle are kept in the top-level hero/header area and now match the CTO/Head/Fractional CTO positioning:

- Typst/PDF: `templates/resume.typ` renders a cleaner PDF header with the title/subtitle, service-labeled contact links, compact margins, and an Inter-first font stack with Noto Sans fallback for Cyrillic rendering.
- Web: `src/lib.rs` renders the same title/subtitle under the name in the hero header.
- Web markdown rendering strips the raw Markdown heading/title area up to the first `##` section, so the Markdown can retain title/subtitle text for LLM parsing without duplicating it in the rendered site.
- Pages output includes static machine-readable artifacts: `/cv.md`, `/cv_ru.md`, `/cv.txt`, `/cv_ru.txt`, `/cv.html`, `/cv_ru.html`, and `/llms.txt`.
- CI: `.github/actions/build-pdfs/action.yml` installs Noto Sans and Liberation Sans, and tries to install Inter. The Typst font stack is Inter-first with Noto Sans as the main Cyrillic fallback.

## Content changes
The English and Russian CVs are written around five signals:

1. CTO / Head / Fractional CTO positioning for AI-enabled engineering teams
2. Engineering leadership, organization design, hiring, technical-lead development, and Head-level ownership
3. Rust/C++ legacy modernization and C++ to Rust migration
4. Systems and high-performance backend engineering
5. Delivery governance, quality gates, CI/CD, reliability, and production discipline

Keep role positioning in the header/title area only; do not add a second role-list section in the body. The top evidence strip is allowed, but it must stay factual and compact, not a generic highlight reel.

The CV intentionally includes explicit keyword coverage for LLM/ATS matching:

- AI engineering transformation
- AI adoption in engineering organizations
- Agentic SDLC
- Coding-agent workflows
- Engineering enablement
- AI readiness audit
- C++ to Rust migration
- Systems modernization
- High-performance engineering
- Architecture governance
- Behavioral equivalence testing
- Performance retention
- CI verification gates
- Engineering productivity
- CTO / Fractional CTO
- Head of Engineering leadership
- DevSecOps / SRE / release governance

Do not remove these terms unless replacing them with stronger equivalent wording.

## Claims that must be verified before publishing
Check that the candidate is comfortable publishing these claims:

- “15+ years in software engineering”
- “5 years in engineering leadership”
- “engineering scope of 25-30 people”
- “20 services”
- “task-to-release lead time by 50%”
- “API regression pipeline from 30 minutes to 5-7 minutes”
- “1-2 production defects per quarter after service stabilization”
- “~200k LOC C++ codebase”
- “initial three-month delivery milestone”
- “lead a small Rust engineering team”
- “train engineers to operate coding agents”

If any claim is too sensitive, replace it with a less specific but still strong formulation. Do not invent metrics.

## Build/validation checklist
1. Confirm Markdown renders correctly on the web version.
2. Confirm Typst PDF builds for EN/RU and light/dark themes.
3. Confirm the PDF header shows:
   - name
   - title
   - subtitle
   - plain contact links
4. Confirm the title/subtitle are not duplicated in the body.
5. Confirm English CV is the canonical version for international applications.
6. Confirm Russian CV keeps important English terms where useful for search/LLM matching.

## Style guidance
Keep the tone senior, concrete, and direct. Avoid generic phrases like “passionate engineer”, “team player”, “AI enthusiast”, or “responsible for development”. Prefer ownership language:

- led
- owned
- designed
- built
- introduced
- reduced
- stabilized
- trained
- established
- governed
- validated

The CV should read as a leadership and transformation profile, not as a task list.
