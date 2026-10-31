# Russian B2B Services Page

## Scope and delivery

The page at `/CV/services/ru/` describes three proposed engagements: an agent-assisted
development audit and pilot, automation of one engineering process, and an
end-to-end B2B product pilot. Each offer identifies the deliverables, scope,
customer inputs, acceptance checks, and an indicative pilot duration.

These are proposed service formats, not historical client outcomes. Duration,
price, tool and infrastructure budgets, access rules, and acceptance criteria
must be agreed before work starts. The page makes no commitment to a legal
entity, a billing arrangement, a response SLA, or a guaranteed business benefit.

The original English and Russian CV content, PDF templates, and CV routes remain
unchanged. Both interactive CV views and the no-JavaScript fallback link to the
new page. The service page itself requires neither JavaScript nor WebAssembly.

## Source layout and review

- `services/ru/index.html`: Russian copy and semantic HTML.
- `services/services.css`: responsive light/dark presentation, separate from CV styles.
- `scripts/assemble_pages_dist.sh`: copies both files into the final Pages artifact.

Run the existing checks and production build:

```bash
cargo test --locked
cargo test --locked --manifest-path scripts/convert_cv/Cargo.toml
python3 -m unittest discover -s .github/scripts -p 'test_*.py'
trunk build --release --config Trunk.toml
# Build the four PDFs as described in NEW_SITE_BUILD.MD before assembly.
scripts/assemble_pages_dist.sh
```

PR CI builds the complete site and four PDFs and uploads `site-preview` without
deploying it. To inspect a downloaded artifact locally, extract its contents
into a `CV/` directory beneath a temporary web root, serve that root, and open
`/CV/services/ru/`. This preserves the same base path used by GitHub Pages.
Check the service and CV links, narrow screens, keyboard navigation, and print.

The initial delivery is a draft PR. Publication requires the owner's approval:
merging to `main` triggers the existing `release-site` workflow. Do not enable
auto-merge or manually dispatch publication during review. This page does not
change the site's existing `noindex, nofollow, noarchive` or `robots.txt` policy.

## Public evidence review — 2026-10-31

The page uses narrow technical statements supported by public source files at
fixed commits. Reading tests establishes what is implemented and exercised by
their source; it does not establish that those tests ran successfully during
this content review or prove commercial deployment.

| Page statement | Public source | Limit |
| --- | --- | --- |
| Q-COLD contains a task queue with dependencies. | [Dependency graph code](https://github.com/qqrm/q-cold-rs/blob/126374595d150a325bba67ef4113e48d74e93407/src/webapp/queue_graph.rs) | Invalid dependency references can be normalized with warnings; this is not a claim of strict validation of every input. |
| Q-COLD records attempts and bounds recovery retries. | [Recovery implementation](https://github.com/qqrm/q-cold-rs/blob/126374595d150a325bba67ef4113e48d74e93407/src/webapp/queue_worker_recovery.rs), [recovery test scenarios](https://github.com/qqrm/q-cold-rs/blob/126374595d150a325bba67ef4113e48d74e93407/src/webapp/tests_queue_taskflow_attempts.rs) | Retry limits do not establish a monetary budget, successful repair, or idempotent external effects. |
| Q-COLD can launch project-defined checks through an adapter. | [Manifest adapter](https://github.com/qqrm/q-cold-rs/blob/126374595d150a325bba67ef4113e48d74e93407/src/adapter/manifest.rs) | Command execution and exit status do not prove independent authorship, isolation, or complete verification. |
| CV has a pipeline for Rust tests, a WASM site, and four PDFs. | [CI](https://github.com/qqrm/CV/blob/bc84a46255c258bc390b668d8350e905e35987f3/.github/workflows/build-test.yml), [PDF build action](https://github.com/qqrm/CV/blob/bc84a46255c258bc390b668d8350e905e35987f3/.github/actions/build-pdfs/action.yml), [artifact assembly](https://github.com/qqrm/CV/blob/bc84a46255c258bc390b668d8350e905e35987f3/scripts/assemble_pages_dist.sh) | A personal project's automation is not a customer case study or a reliability/ROI guarantee. |
| Contact channels are published by the author. | [Public profile](https://github.com/qqrm/qqrm/blob/99a9552e407723752a4b1692a6a93e497f599b4c/README.md) | Email and Telegram are contact routes; no response-time commitment is inferred. |

Q-COLD was public and archived when checked; the page explicitly labels it as
an archived technical example. It is not offered as a maintained product or
subscription. Repositories that are currently private are excluded entirely
from the public examples. Recheck visibility and archive status before reusing
these examples in other marketing materials.

## CV claim screening

The [Russian CV](https://github.com/qqrm/CV/blob/bc84a46255c258bc390b668d8350e905e35987f3/profiles/cv/ru/CV_RU.MD),
[English CV](https://github.com/qqrm/CV/blob/bc84a46255c258bc390b668d8350e905e35987f3/profiles/cv/en/CV.MD),
[integration brief](https://github.com/qqrm/CV/blob/bc84a46255c258bc390b668d8350e905e35987f3/AGENT_INTEGRATION_BRIEF.md),
and public profile were screened separately from the code examples. They are
self-authored accounts, not independent confirmation of commercial outcomes.

The new page excludes employment and team-size metrics, delivery-speed and
defect-rate claims, financial comparisons, customer logos, and all NDA-project
details. Leadership-tenure descriptions also differ between the CV and profile;
no numerical tenure claim is carried into the service offer. Existing employers
are not presented as clients of this service. The original CV is not rewritten
by this change.

The page describes independent acceptance checks as a proposed pilot practice,
not as a previously certified capability. Any future claim about clients,
profits, savings, production deployments, or regulated compliance needs its own
publishable evidence and authorization.
