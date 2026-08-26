# Alexey Belyakov

**Head of Engineering**
Engineering leadership • AI-native software delivery • C++/Rust systems modernization

Contacts: qqrm@vivaldi.net • Telegram: @leqqrm • LinkedIn: linkedin.com/in/qqrm • GitHub: github.com/qqrm

## Summary
Head of Engineering / Engineering Manager with 15+ years in software development and 5+ years of engineering leadership, running teams of up to 30 people. Background in C++ and Rust systems engineering and DevOps. Focus: AI adoption in software engineering — coding agents write most of the code, run the different layers of testing and use the results to keep systems within the required performance boundaries. Humans choose the architecture, methodologies, engineering practices and tools — and, of course, retain accountability. Currently leading an AI-first migration of a production software-defined storage product from C++ to Rust. Target roles: Head of Engineering, Director of Engineering, CTO at a product company, fractional CTO.

## Core competencies
**Engineering leadership:** engineering org design, hiring and onboarding, growing tech leads, ownership models, architecture governance, roadmap, delivery and release management, stakeholder management, technical-debt management.

**AI adoption and agentic delivery:** AI adoption in engineering organizations, agentic SDLC, coding-agent workflows (Claude Code, Codex, MCP), agent harness and skill design, context and token optimization, task decomposition and isolation, executable acceptance, CI verification gates, effective AI review of AI-generated code, behavioral equivalence testing.

**DevOps:** CI/CD pipelines, GitOps, Kubernetes, containerized and virtualized test environments, build and cache infrastructure, observability.

**DevSecOps and AppSec:** supply-chain security, secret scanning, SAST, infrastructure and configuration linting, protected merge flow.

## How I work with coding agents
- LLM agents write most changes, but personal accountability remains with the humans. Code acceptance decisions are based on CI gates and metrics; automation produces reproducible evidence to an engineering-grade standard.
- Evidence-based acceptance: agent-written code is accepted only against executable checks relevant to the scope of the change, not progress reports. Fast and full CI gates, combined with independent review, provide the foundation for safe delivery automation.
- Cost and scope discipline: explicit budgets and stop conditions prevent token burn and scope drift; context is engineered, sessions are reviewed, and repeatable practices are synthesized into skills.
- Portable harness: one operating model works across agent harnesses, while parallel agents are isolated from one another in disposable environments.
- Model-task fit: in my experience, selecting the right model for each task accounts for roughly half of overall agent effectiveness.

<!-- pdf-pagebreak -->

## Experience

### AI-first Rust Chapter Lead • NDA
*July 2025 – Present*

Lead an AI-first Rust migration program: a production software-defined storage (SDS) product in C++ is being rebuilt in Rust. LLM agents do most of the implementation; I own the program's architecture direction, acceptance model and delivery pipeline, and I am accountable for everything that enters the product.

- Built, in four months, a Rust implementation of over 400k lines covering the storage engine, OSD, client, control plane, monitor and the NBD/ublk/QEMU/fio integrations — with executable gates proving behavioral and performance equivalence to the original C++ for the key modules.
- Made the original C++ system a pinned, isolated semantic Oracle for the Rust implementation; enforce fidelity to the reference architecture.
- Established proof-carrying delivery and acceptance: a migration claim is accepted only when its reference behavior, Rust owner, executable gate and retained artifact agree; a green CI result alone never widens the claim.
- Built a governed agent operating model: repository-scoped skills and policies, isolated tasks, curated context, tool hooks, explicit budgets and stop conditions, plus independent agent review.
- Run a tiered verification program: deterministic MR and merge-train gates; scheduled differential search, property-based and mutation testing, fuzzing, formal model checking, full-stack E2E and AppSec controls.
- Treat durability as a separate release discipline through reopen/replay, controlled crash windows, raw-device and power-loss scenarios, and integrity soaks — each with an explicit confidence boundary.
- Built the CI and lab platform: GitLab fast lanes and full gates, disposable container lanes, QEMU-based labs, shared compiler cache and exact merge-train validation. Performance qualification uses an isolated target, CPU/NUMA pinning, direct-I/O/io_uring preflight, retained provenance and alternating Original/Rust samples with throughput and p99 latency budgets.
- Own controlled compatibility across public client contracts, CLI and QEMU, fio, NBD and ublk integrations; compatibility is established by dedicated scenarios, not by a successful build alone.
- Develop the AI-first Rust direction: architecture and engineering standards, mentoring, and expertise the company can reuse on its next projects.

**Stack:** Rust, C++, GitLab CI/CD and merge trains, Docker, Dev Containers, QEMU, fio, NBD/ublk, sccache, Cargo/xtask, cargo-nextest, proptest, cargo-mutants, cargo-fuzz, Kani, LikeC4/MCP, Claude Code, Codex, cargo-deny, OSV-Scanner, Semgrep, Trivy, Gitleaks.

### Head of Development • Inline Group
*March 2023 – July 2025*

Led engineering for an enterprise procurement and internal-process platform: an organization of 25–30 people — backend directly, DevOps and QA through their team leads, frontend through API contracts — plus architecture, releases and production operations.

- Established service and platform ownership, architecture review, release-readiness rules and a clear technical decision-making loop; hired key specialists and grew tech leads.
- Drove the move from a monolith to about 20 domain and platform services, with shared components, logging, messaging flows and integration utilities; a PostgreSQL database per service, Redis caching, asynchronous integration over RabbitMQ and forward-only schema migrations.
- Built GitLab CI/CD and GitOps delivery with reproducible builds, release tagging and rollbacks; sustained daily releases across dev, test and release environments and cut task-to-release lead time by about half.
- Cut the API regression suite from 30 minutes to 5–7; introduced observability, an incident lifecycle, reliability and recovery metrics and root-cause analysis, bringing production defects down to one or two per quarter once the services were hardened.
- Started controlled AI adoption: task and review templates, risk boundaries, team training, acceptance rules and quality control.

**Stack:** Rust, Actix Web, PostgreSQL, Redis, RabbitMQ, Docker, Kubernetes, GitLab CI/CD, GitOps, Grafana, Sentry, OpenAPI, SonarQube, cargo audit.

<!-- pdf-pagebreak -->

### Lead Rust Engineer • YADRO
*February 2022 – March 2023*

Product: a backup appliance (hardware and software) with deduplication.

- Led development of the key storage and recovery components and owned their architecture decisions.
- Established planning, review, quality, CI/CD and test-automation practices.
- Mentored engineers and supported the team's transition from C++ to Rust.
- Owned performance and reliability work, including RocksDB and storage tuning, load testing, incident analysis and root-cause fixes.

**Stack:** Rust, RocksDB.

### Rust Developer • Kaspersky
*May 2020 – February 2022*

Developed and maintained a production voting service built on Exonum: added weighted voting, simplified the ballot flows, expanded test coverage and streamlined CI during the move to Microsoft tooling.

**Stack:** Rust, Exonum, Protobuf, Serde, Git.

### C++/Go Developer • B2Broker
*November 2018 – March 2020*

Automated MT4/MT5 broker infrastructure: built C++/Go trade copiers and a Multi Account Manager, normalized and streamed real-time trading data, and developed data-quality controls.

**Stack:** C++, Go, MSVC, CMake, Protobuf, gRPC, NATS, YAML, PostgreSQL, vcpkg, Git.

### Lead C++ Engineer • ASCON
*May 2016 – November 2018*

Developed CAD libraries for KOMPAS and shipped 3D workflow features. Introduced automated testing in C++ and Python, led the move to C++17, migrated the team from SVN to Git, and established review, planning and documentation practices.

**Stack:** C++17, MSVC, Python 3, Boost, PostgreSQL, Git, Jenkins.

### Earlier experience
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
**Programming languages:** Rust, C++ (C++17), Go, Python, SQL, Bash, JavaScript.

**Backend and data:** Actix Web, async Rust, PostgreSQL, Redis, RabbitMQ, RocksDB, ZeroMQ, NATS, gRPC, Protobuf, REST/OpenAPI.

**Infrastructure and delivery:** Linux, Docker, devcontainers, Kubernetes, GitLab CI/CD, GitHub Actions, Jenkins, GitOps, sccache, QEMU-based test labs.

**DevSecOps and AppSec:** cargo-deny, cargo-audit, OSV-Scanner, gitleaks, Semgrep, Trivy, hadolint, yamllint, shellcheck, SonarQube, CODEOWNERS, protected branches and merge trains, pinned toolchains and image digests.

**Quality and verification:** cargo-nextest, clippy, rustfmt, taplo, cargo-llvm-cov, proptest, cargo-mutants, cargo-fuzz, Kani, differential and end-to-end testing, load and performance testing.

**Observability and operations:** Grafana, Sentry, incident lifecycle, SRE practices.

**AI tooling:** Claude Code, Codex, MCP servers, agent skills and hooks, subagent orchestration, LLM-based code review, token and context optimization.

**Domains:** AI-assisted software development, backend, platform engineering, software-defined storage (SDS), storage and distributed systems, virtualization, systems modernization, enterprise platforms, CAD, fintech and trading integrations, blockchain systems, high-throughput systems.

## Outside of work
I enjoy film and poetry, play ukulele and guitar, and hike. I collect keyboard phones and vinyl.
