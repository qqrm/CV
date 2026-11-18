# Alexey Belyakov

**Head of Engineering**
Task-to-release time cut by ~50% • C++/Rust modernization • AI-native engineering

Contacts: qqrm@vivaldi.net • Telegram: @leqqrm • LinkedIn: linkedin.com/in/qqrm • GitHub: github.com/qqrm • Moscow · open to relocation

## Summary

Cut task-to-release lead time by approximately half and took an enterprise procurement product from one planning module to 20 modules in production at Inline Group.

Led an engineering organization of up to 45 specialists across 5+ cross-functional teams through team and tech leads. Owned technical strategy, architecture, development and infrastructure budgets. My focus is product delivery, modernization costs and engineering risk.

Currently responsible for a production software-defined storage (SDS) product following its C++ to Rust migration. Coding agents wrote the implementation; I owned architecture, delivery and acceptance. I took this hands-on role to test the delivery model on a production product. Seeking Head of Engineering or Director of Engineering roles where I can apply that experience across teams.

## Experience

### Staff Engineer — Rust & Agentic Engineering • NDA
*July 2025 – Present*

Infrastructure software vendor, approximately 300 employees; company name under NDA. Responsible for the technical domain of a production SDS product. Completed its C++ to Rust migration with a second engineer; the resulting project contains 400K+ lines of code. Agents wrote the code; architecture, acceptance and delivery were my responsibility.

- **Modernization delivered with estimated engineering labor and reported subscriptions of $61.2–73.2K:** approximately 12 engineer-months × $5–6K + $1.2K. Partial estimate; [cost period and excluded infrastructure/support costs].
- **Release decisions made traceable to evidence:** tied acceptance to the original C++ behavior, an accountable component owner, an executable check and a retained artifact. Covered recovery, data integrity, security and performance with explicit limits on what the checks establish. [Production impact; critical workload coverage].
- **A second engineer prepared to own the migrated product:** transferred architecture, acceptance and delivery practices. [Independent releases / operational ownership after handover].
- **A delivery model prepared for reuse:** documented task boundaries, agent budgets, independent review and acceptance rules for subsequent company projects. [Teams adopting the model; measured results].

**Subscription basis:** 3 × $200/month × 2 months = $1.2K. [Coverage of the full migration period].

**Stack:** Rust, C++, GitLab CI/CD, Docker, sccache; differential, property-based, mutation, fuzz and formal verification.

<!-- pdf-pagebreak -->

### Head of Development • Inline Group
*March 2023 – July 2025*

Led development of ASEZ 2.0, an enterprise e-procurement platform for a large oil-and-gas customer. Scope: up to 45 specialists in 5+ cross-functional teams through team and tech leads; technical strategy, architecture, development and infrastructure budgets.

- **Expanded the product from one planning module to 20 modules and brought it into production.** Built the engineering organization, hired key specialists and developed tech leads. [Active users / procurement volume / commercial impact].
- **Cut task-to-release lead time by approximately half** through GitLab CI/CD, GitOps, reproducible builds and release/rollback procedures. Established daily releases across development, test and release environments.
- **Cut API regression runtime by 23–25 minutes per run:** from 30 minutes to 5–7 minutes. [Method; runs/year; actual engineer time saved].
- **Recorded 1–2 production defects per quarter after hardening;** used observability, incident follow-up and root-cause analysis to manage production quality. [Measurement period; severity; earlier baseline].

**Stack:** Rust, Actix Web, PostgreSQL, Redis, RabbitMQ, Kubernetes, GitLab CI/CD, GitOps, Grafana, Sentry.

### Lead Rust Engineer • YADRO
*February 2022 – March 2023*

Tech lead of the core of a backup appliance with deduplication; responsible for storage and recovery components.

- Developed core storage and recovery components. [Release milestone; deployment scale; recovery-time result].
- Tuned RocksDB and storage behavior and fixed incident root causes. [Throughput / latency / recurring incidents before → after].
- Trained engineers moving from C++ to Rust and introduced review and automated verification. [Engineers reaching independent delivery; time to readiness].

**Stack:** Rust, RocksDB.

### Rust Developer • Kaspersky
*May 2020 – February 2022*

Developed and maintained a production voting service built on Exonum.

- Enabled weighted voting. [Customer scenarios; usage].
- Simplified ballot submission flows. [Completion rate / user effort / support impact].
- Expanded test coverage and streamlined CI during the move to Microsoft tooling. [Release time / escaped defects before → after].

**Stack:** Rust, Exonum, Protobuf, Serde.

<!-- pdf-pagebreak -->

### C++/Go Developer • B2Broker
*November 2018 – March 2020*

Automated infrastructure for a cryptocurrency broker using MT4/MT5 APIs.

- Enabled trade copying and multi-account management through C++/Go trade copiers and a Multi Account Manager. [Accounts served; commercial usage].
- Delivered normalized, real-time trading data streams. [Throughput; latency; consuming systems].
- Added data-quality controls for trading integrations. [Manual hours saved / data errors detected].

**Stack:** C++, Go, MT4/MT5, gRPC, NATS, PostgreSQL.

### Lead C++ Engineer • ASCON
*May 2016 – November 2018*

Developed CAD libraries and shipped 3D workflow features for KOMPAS; introduced C++/Python tests and led the move to C++17 and Git.
[Feature adoption / release-quality impact].

### Earlier experience (2011–2016)

**Concerteza:** optimized C++ protocol parsing for telecom analytics to approximately 1.5 Gbps per thread; added Python integration tests. [Earlier throughput / infrastructure impact].

**LiveTex:** built C++/JavaScript components, including Node.js bindings for PostgreSQL and ZeroMQ. [Production usage / product impact].

**T4B:** built C++ MetaTrader plugins and a management web application. [User adoption / manual work replaced].

## Education and languages

Bauman Moscow State Technical University, Department IU7 (Software Engineering), Specialist degree (five-year, master's-equivalent), 2015.
Russian — native. English — C1.

## Leadership and delivery

- Engineering organization design, hiring and development of team and tech leads; roadmap, budget and technical-debt decisions.
- Product launches, release management, production quality and incident follow-up.
- AI adoption with explicit acceptance criteria, independent verification and human accountability.
- Systems modernization in Rust/C++, storage and enterprise platforms; CI/CD, GitOps, DevSecOps.

## Technical foundation

**Systems and data:** Rust, C++17, Python, SQL, Linux; Actix Web, PostgreSQL, Redis, RabbitMQ, RocksDB, gRPC, Protobuf.

**Delivery and operations:** Docker, Kubernetes, GitLab CI/CD, GitHub Actions, GitOps, QEMU test environments, Grafana, Sentry.

**Verification and security:** differential and E2E testing, proptest, cargo-mutants, cargo-fuzz, Kani, performance testing; cargo-deny, OSV-Scanner, Semgrep, Trivy, Gitleaks.

**AI engineering:** Claude Code, Codex, MCP, agent orchestration, task isolation, executable acceptance and independent review.

## Outside of work

Film and poetry, ukulele and guitar, hiking; collecting phones with physical keyboards and vinyl.
