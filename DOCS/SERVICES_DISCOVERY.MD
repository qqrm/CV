# Services: Search Visibility and Qualified Enquiries

Research and implementation review: 2026-11-04.

The goal is a relevant business conversation that can lead to an agreed pilot.
Search impressions, AI citations, visits, and contact clicks are earlier steps;
none of them establishes that a customer contacted the author or bought work.
There is no traffic baseline or measured conversion lift for these pages yet.

## Evidence and decisions

| Finding | Evidence | Decision for this site |
| --- | --- | --- |
| An accessible page with `noindex` is excluded from Google Search. | [Google: block indexing](https://developers.google.com/search/docs/crawling-indexing/block-indexing) | Remove the restrictive tags from the two service pages. Preserve the CV's existing tags. |
| Google's AI search features use the same search foundations; eligibility requires indexing and snippet eligibility. Inclusion is not guaranteed. | [Google: AI features](https://developers.google.com/search/docs/appearance/ai-features) | Keep all offers, boundaries, acceptance criteria, and contact links in static HTML. |
| Google recommends original, useful expertise and does not require AI-specific text files, a special writing style, or special schema. | [Google: generative AI optimization](https://developers.google.com/search/docs/fundamentals/ai-optimization-guide) | Use existing technical work as evidence. Do not generate keyword variants, fake mentions, or a new `llms.txt` marketing bundle. |
| ChatGPT search uses OAI-SearchBot; GPTBot concerns model training and is independently controlled. | [OpenAI: crawlers](https://developers.openai.com/api/docs/bots) | Verify the effective host policy before claiming search access. No change to training permissions is needed for this work. |
| Google requires fully qualified, reciprocal language alternatives. | [Google: localized pages](https://developers.google.com/search/docs/specialty/international/localized-versions) | Give RU and EN their own canonical URLs and absolute alternate links; choose EN as `x-default`. |
| A sitemap helps discover URLs but does not guarantee indexing. | [Google: sitemaps](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap) | Publish one sitemap containing only the two indexable service URLs. Do not list CV exports. |
| Expert readers need clear, specific information and ways to verify it. | [NNGroup: writing for domain experts](https://www.nngroup.com/articles/writing-domain-experts/) | Name the work directly, link the CV and source examples early, and retain the engineering detail in each offer. |
| B2B visitors need enough information to evaluate fit and budget; visible contact options reduce avoidable barriers. | [NNGroup: B2B websites](https://www.nngroup.com/articles/b2b-vs-b2c/), [contact guidelines](https://www.nngroup.com/articles/contact-us-pages/) | State scope and how a price is agreed. Offer email and Telegram without a required form or complete brief. |

The NNGroup sources report qualitative usability research from 2016–2019.
They inform design choices; they are not an experiment on this service or a
forecast of conversion gains. The changes should be assessed against actual
enquiries after publication.

## Changes prepared in PR #509

- Replace generic marketing language with three concrete tasks: introducing
  coding agents into a team's workflow, automating an engineering process,
  and building one working B2B product scenario.
- Name the author's Rust/C++, CI/CD, and coding-agent specialization, already
  described in both CV versions. Link to experience without presenting former
  employers as customers of this service.
- Retain all pilot limits, inputs, twelve acceptance checks, and indicative
  durations. Keep q-cold-rs marked as archived and distinguish technical examples
  from customer outcomes.
- Provide an email link for each offer with the service and locale in its
  editable subject. Keep the general email address and Telegram visible. The
  short brief is optional guidance, not a gate before contact.
- Add localized search and sharing descriptions, self-canonicals, absolute
  RU/EN alternatives, and the services-only sitemap.
- Describe the person, page, and three services in JSON-LD using only facts
  visible on the page. [Schema.org Service](https://schema.org/Service) supports
  a person as provider. No prices, ratings, company, location, or clients are
  invented. This is descriptive markup, not a promise of a rich result or AI
  recommendation.
- Extend the browser checks to cover metadata, sitemap, structured-data/content
  consistency, offer-specific contact links, and the absence of a WASM dependency
  on the service pages. Retain narrow-screen, theme, navigation, and no-JS checks.

The existing `llms.txt` remains a CV export. It is not treated as a search ranking
signal. The service pages use shared CV styles and fonts, require no WASM to read,
and add no external analytics requests. No field Core Web Vitals result or
performance score has been measured or claimed.

## Hosting boundary: robots.txt

This is a GitHub Pages project at `https://qqrm.github.io/CV/`. The existing
build copies `content/robots.txt`, containing `Disallow: /`, to
`https://qqrm.github.io/CV/robots.txt`. A crawler's effective file is at the
origin root, `https://qqrm.github.io/robots.txt`, not the project subdirectory.
See [Google's placement rules](https://developers.google.com/crawling/docs/robots-txt/create-robots-txt)
and [Yandex's robots guidance](https://www.yandex.com/support/webmaster/en/controlling-robot/robots-txt).

The live origin-root response could not be retrieved in this review. The GitHub
connector also returned 404 for the repository `qqrm/qqrm.github.io`; that is not
evidence of the live robots URL's HTTP status. The project robots file is left
unchanged. It must not be described as an effective host-wide block or as proof
that a search crawler can access the services.

After approved publication, check the actual root robots response, page response
headers, and fetched HTML. A host-level deny or `X-Robots-Tag: noindex` can still
prevent discovery despite the HTML changes. If the root policy blocks the
services or required assets, the owner must change that policy at its actual
hosting location. Do not attempt to solve it with another nested robots file.
Robots directives are not access control for private data.

## Search intent to validate

These phrases describe plausible buying tasks. They are hypotheses, not measured
keyword volumes, ranking opportunities, or evidence of current demand.

| Buyer task | Russian query hypothesis | English query hypothesis | Page section |
| --- | --- | --- | --- |
| Introduce coding agents into an existing team | внедрение ИИ-агентов в разработку | AI coding agent implementation consulting | `#agentic` |
| Check a team's agent workflow | аудит процесса разработки с ИИ-агентами | AI coding workflow audit | `#agentic` |
| Remove repeated release work | автоматизация CI/CD и подготовки релизов | CI/CD and release automation consulting | `#automation` |
| Test a product's main integration | разработка прототипа B2B-продукта с API-интеграцией | B2B software prototype with API integration | `#product` |

Start with one substantive page per language. Create a separate service or case
page only when it has distinct, useful material: a real problem, constraints,
the author's decisions, a reproducible example, and measured results with the
owner's permission to publish. Repeating the same offer under many query variants
does not improve the buyer's ability to evaluate the work.

## Publication and measurement sequence

1. Review the RU/EN previews and CI results. Keep PR #509 in Draft until the owner
   approves publication; merging to `main` starts the existing release workflow.
2. After approval and deployment, verify HTTP 200 responses, effective robots and
   headers, canonical URLs, the reciprocal language links, and
   `https://qqrm.github.io/CV/sitemap.xml` on the public host.
3. With the owner's authorized accounts, verify the site in Google Search Console,
   Bing Webmaster Tools, and Yandex Webmaster. Submit the sitemap and inspect both
   service URLs. No account is connected or verified by this PR.
4. Record search queries, impressions, and clicks by landing-page language. Google
   documents a [Generative AI performance report](https://developers.google.com/search/docs/fundamentals/ai-optimization-guide).
   Bing's [AI Performance](https://blogs.bing.com/webmaster/2026/2/Introducing-AI-Performance-in-Bing-Webmaster-Tools-Public-Preview/)
   reports citations, cited pages, and grounding queries. Treat these as discovery
   data, not leads or a stable ranking across all AI chats.
5. Track received enquiries manually at first: date, language, service, source
   when known, task, fit, next step, and outcome. The email subject identifies the
   selected offer; it does not prove the traffic source. Ask about the source
   optionally in the conversation. Unknown sources remain unknown.
6. Use the first month after indexing as an initial observation window, not a
   deadline for results. Low impressions suggest a discovery or demand problem;
   impressions without relevant clicks suggest title or intent mismatch; relevant
   visits without enquiries suggest a fit, trust, price, or contact problem.
   Improve the corresponding part using observed queries and buyer questions.

A qualified enquiry concerns work within the stated services and has enough
context to discuss a pilot. Track proposals and paid pilots separately. Do not
report a mailto or Telegram click as a submitted enquiry. Add a real analytics
collector only after choosing a provider and a useful measurement purpose;
unused event code would not produce evidence.

## Facts still needed from the owner

Actual minimum budget or price ranges, current availability, contracting
geography and arrangements, and publishable customer cases would help buyers
judge fit. None is inferred from the author's CV or invented for SEO. A case
needs a baseline, outcome, measurement method, attribution of the author's role,
and permission to publish. These facts can improve a later revision without
holding up review of the current changes.
