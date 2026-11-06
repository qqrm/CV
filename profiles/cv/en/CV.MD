# Alexey Belyakov

**Head of Engineering**
Engineering leadership • AI-native software delivery • C++/Rust systems modernization

Contacts: qqrm@vivaldi.net • Telegram: @leqqrm • LinkedIn: linkedin.com/in/qqrm • GitHub: github.com/qqrm • Moscow · open to relocation

## Summary
Head of Engineering / Engineering Manager with 15+ years in software development; 7+ years in engineering leadership — from tech lead of an SDS core to Head of Development: an organization of up to 45 specialists, 5+ cross-functional teams through team and tech leads. Background in C++ and Rust systems engineering and DevOps.

Focus: AI-native software delivery — in my current team all production code is written by LLM coding agents; I build the engineering system around them: decomposition, contracts, orchestration, independent verification, final acceptance. I haven't reviewed code line-by-line in over a year — acceptance runs on executable evidence, accountability stays with the human. Currently owning a production software-defined storage product after completing its C++ to Rust migration, with a second engineer alongside. I stepped back into a hands-on role deliberately — to prove an AI-first delivery model on a production product. My next step is into management: what interests me is not implementing tasks but the processes and systems that make implementation maximally effective. Target roles: Head of Engineering, Director of Engineering, CTO at a product company.

## Core competencies
**Engineering leadership:** engineering org design, Agile, Scrum and Kanban processes, running 1:1s, performance reviews, headcount planning, goal setting, hiring and onboarding, growing tech leads, ownership models, architecture governance, roadmap, delivery and release management, stakeholder management, technical-debt management.

**AI adoption and agentic delivery:** AI adoption in engineering organizations, agentic SDLC, coding-agent workflows (Claude Code, Codex, MCP), agent harness and skill design, context and token optimization, task decomposition and isolation, executable acceptance, CI verification gates, effective AI review of AI-generated code, behavioral equivalence testing.

**DevOps:** CI/CD pipelines, GitOps, Kubernetes, containerized and virtualized test environments, build and cache infrastructure, observability.

**DevSecOps and AppSec:** supply-chain security, secret scanning, SAST, infrastructure and configuration linting, protected merge flow.

## How I work with coding agents
- **Honest division of labor:** agents write all of the code in my current team; my work is the engineering system around them — from decomposition and contracts to verification and final acceptance. Accountability stays with the human.
- **Portable harness:** one operating model across harnesses; parallel agents isolated in disposable environments.
- **Model-task fit:** model choice is the single largest factor in agent effectiveness.
- **The boundary I push:** maximum delegation without losing quality or accountability — humans keep intent, constraints and accountability, not routine implementation.

<!-- pdf-pagebreak -->

## Experience

### Staff Engineer — Rust & Agentic Engineering • NDA
*July 2025 – Present*

A Russian infrastructure software vendor (~300 people, name under NDA). Own the technical side of my domain — a production SDS product — after completing its end-to-end migration from C++ to Rust. LLM agents wrote all of the code; architecture, execution model, acceptance and delivery were mine throughout.

Key results:
- **400k+ lines of code in the product after the C++→Rust migration** — all of it written by agents (the Rust core: ≈1.2M lines written in the first four months, rewrites included); executable behavioral-equivalence gates and performance budgets on production-critical workloads.
- **Pinned semantic oracle** — the original C++ system frozen as an isolated golden reference; Rust behavior is checked against it by scheduled differential runs.
- **Proof-carrying acceptance** — a migration claim lands only when reference behavior, component owner, executable gate and retained artifact agree; a green CI alone proves nothing.
- **Governed agent operating model** — repository policies, isolated tasks, explicit budgets and stop conditions, independent review: architecture drift and gratuitous abstractions do not survive.
- **Tiered verification** — MR and merge-train gates; scheduled differential, property-based, mutation, fuzz and formal checks, full-stack E2E and AppSec controls; durability scenarios (reopen/replay, crash windows, power loss, raw-device, integrity soaks) with explicit confidence boundaries.
- **CI and lab platform** — GitLab fast lanes and full gates, disposable environments, shared compiler cache, exact merge-train validation; performance qualification on an isolated target: CPU/NUMA pinning, controlled I/O preflight, retained provenance, alternating Original/Rust runs, throughput and p99 latency budgets.
- **Team** — the second engineer is now ready to own the migrated product; architecture, acceptance and delivery practices packaged for the company's next AI-first projects.
- **Direct agent-tooling spend** — $1.2K for the entire migration (three $200/mo subscriptions across two engineers, two months) — two orders of magnitude below the token bills GitHub ($120K) and Bun ($165K) published for migrations of comparable scale (table below).
- **Full cost including payroll** — ~$70K (~12 engineer-months × $5–6K + $1.2K of subscriptions) — less than GitHub and Bun spent on tokens alone; pre-agent migrations of this scale took years and were measured in millions (table below).

Comparison with published migrations — direct agent spend and full cost:

| Migration | Scale and timeline | Direct agent spend | Full cost |
| --- | --- | --- | --- |
| [Government system (published study)](https://research.vu.nl/files/229574141/Cost_driven_software_migration.pdf) | 4 years; 156 + 78 person-months; 95 of 126 use cases | \* | €5M (budget) |
| [GitHub: Copilot runtime, TypeScript → Rust](https://github.blog/ai-and-ml/generative-ai/migrating-the-github-copilot-runtime-to-rust-using-copilot/) | 430K lines of source → 800K+ lines of Rust; 14.5 weeks | ~$120K — actual token bill | >$130K — lower bound |
| [Bun: Zig → Rust](https://bun.com/blog/bun-in-rust) | ~1M lines; up to 64 agents in parallel; full test suite green in 11 days | ~$165K — estimated at API rates | >$175K — lower bound |
| **This migration: production SDS, C++ → Rust** | **400K+ lines in the project; core — ≈1.2M lines in the first 4 months, rewrites included** | **$1.2K — actual subscriptions** | **~$70K — incl. payroll: ~12 eng-months × $5–6K + subscriptions** |

\* Migration carried out without coding agents. Only direct agent spend is directly comparable; full budgets are almost never disclosed. Lower bounds for GitHub and Bun — published token spend plus publicly known engineer time (~3 weeks and ~11 days of one engineer) at ~$20K/mo.

**Stack:** Rust, C++, GitLab CI/CD and merge trains, Docker, Dev Containers, sccache, Cargo/xtask, cargo-nextest, proptest, cargo-mutants, cargo-fuzz, Kani, LikeC4/MCP, Claude Code, Codex, cargo-deny, OSV-Scanner, Semgrep, Trivy, Gitleaks.

### Head of Development • Inline Group
*March 2023 – July 2025*

Created ASEZ 2.0 — an enterprise e-procurement system for a large state-owned enterprise customer (oil & gas) — and took it from a single planning module to twenty modules, release and production, where it runs to this day. Organization of up to 45 specialists and 5+ cross-functional teams through team and tech leads; owned technical strategy, architecture, development and infrastructure budget, TCO and cost optimization.

Key results:
- **Engineering from scratch** — service ownership, architecture review, release-readiness rules, technical decision loop; implemented Agile, Scrum and Kanban; made key hires and grew tech leads.
- **Microservices on shared infrastructure** — the monolith became about twenty domain and platform services: PostgreSQL per service, Redis, asynchronous RabbitMQ integration, forward-only schema migrations.
- **Delivery** — daily releases across dev, test and release environments, task-to-release lead time halved: GitLab CI/CD and GitOps with reproducible builds, release tags and rollbacks.
- **Quality** — API regression from 30 minutes to 5–7; observability, incident lifecycle, reliability and recovery metrics, root-cause analysis: 1–2 production defects per quarter after hardening.
- **Rust adoption** — competence transfer to the team, including developers coming from SAP; supporting people through the stack change was part of the leadership work.
- **Controlled AI adoption** — task and review templates, risk boundaries, team training, acceptance rules and quality control.

**Stack:** Rust, Actix Web, PostgreSQL, Redis, RabbitMQ, Docker, Kubernetes, GitLab CI/CD, GitOps, Grafana, Sentry, OpenAPI, SonarQube, cargo audit.

<!-- pdf-pagebreak -->

### Lead Rust Engineer • YADRO
*February 2022 – March 2023*

Product: a backup appliance (hardware and software) with deduplication. Role: tech lead of the system core.

Key results:
- **Core storage and recovery components** — development and architecture ownership.
- **Engineering practices** — planning, review, quality, CI/CD, test automation.
- **Team migration to Rust** — training and reviewing developers coming from C++, introducing practices and approaches.
- **Performance and reliability** — RocksDB and storage tuning, load testing, incident root-cause fixes.

**Stack:** Rust, RocksDB.

### Rust Developer • Kaspersky
*May 2020 – February 2022*

Developed and maintained a production voting service built on Exonum: added weighted voting, simplified the ballot flows, expanded test coverage and streamlined CI during the move to Microsoft tooling.

**Stack:** Rust, Exonum, Protobuf, Serde, Git.

### C++/Go Developer • B2Broker
*November 2018 – March 2020*

Automated broker infrastructure at a cryptocurrency broker on the MT4/MT5 APIs: built C++/Go trade copiers and a Multi Account Manager, normalized and streamed real-time trading data, and developed data-quality controls.

**Stack:** C++, Go, MSVC, CMake, Protobuf, gRPC, NATS, YAML, PostgreSQL, vcpkg, Git.

### Lead C++ Engineer • ASCON
*May 2016 – November 2018*

Developed CAD libraries for KOMPAS and shipped 3D workflow features. Introduced automated testing in C++ and Python, led the move to C++17, migrated the team from SVN to Git, and established review, planning and documentation practices.

**Stack:** C++17, MSVC, Python 3, Boost, PostgreSQL, Git, Jenkins.

### Earlier experience (2011–2016)
- Concerteza: optimized real-time C++ protocol parsing for telecom signaling analytics to about 1.5 Gbps per thread; added Python integration tests for continuous event processing.
- LiveTex: developed C++ and JavaScript components, including Node.js bindings for PostgreSQL and ZeroMQ.
- T4B: started as a C++ developer building MetaTrader plugins and a management web application.

<!-- pdf-pagebreak -->

## Education
Bauman Moscow State Technical University, Department IU7 (Software Engineering), Specialist degree (five-year, master's-equivalent), 2015.

## Languages
- Russian — native
- English — C1

## Technical skills
**Programming languages:** Rust, C++ (C++17), Python, SQL, Bash, JavaScript.

**Backend and data:** Actix Web, async Rust, PostgreSQL, Redis, RabbitMQ, RocksDB, gRPC, Protobuf, REST/OpenAPI.

**Infrastructure and delivery:** Linux, Docker, devcontainers, Kubernetes, GitLab CI/CD, GitHub Actions, GitOps, sccache, QEMU-based test labs.

**DevSecOps and AppSec:** cargo-deny, cargo-audit, OSV-Scanner, gitleaks, Semgrep, Trivy, hadolint, yamllint, shellcheck, CODEOWNERS, protected branches and merge trains, pinned toolchains and image digests.

**Quality and verification:** cargo-nextest, clippy, rustfmt, taplo, cargo-llvm-cov, proptest, cargo-mutants, cargo-fuzz, Kani, differential and end-to-end testing, load and performance testing.

**Observability and operations:** Grafana, Sentry, incident lifecycle, SRE practices.

**AI tooling:** Claude Code, Codex, MCP servers, agent skills and hooks, subagent orchestration, LLM-based code review, token and context optimization.

**Domains:** AI-assisted software development, backend, microservices, platform engineering, software-defined storage (SDS), storage and distributed systems, virtualization, private cloud, remote and distributed teams, systems modernization, enterprise platforms, CAD, fintech and trading integrations, blockchain systems, high-throughput systems.

## Outside of work
I enjoy film and poetry, play ukulele and guitar, and hike. I collect phones with physical keyboards and vinyl.
