# Alexey Belyakov

**Head of Engineering**
AI-native delivery • agent-run systems migration • Rust/C++

Contacts: qqrm@vivaldi.net • Telegram: @leqqrm • LinkedIn: linkedin.com/in/qqrm • GitHub: github.com/qqrm

## Profile
Engineering leader with 15+ years in software development, including 5+ years leading engineering teams. I work at the intersection of organizational leadership, AI-native transformation and systems engineering, taking complex backend, platform and systems products from ambiguous problems to accountable production outcomes.

I combine organizational design and technical leadership with a Rust/C++ systems background. My value is not tied to a particular stack: I make high-risk engineering programs controllable and measurable while retaining clear ownership of the product outcome.

## Core competencies
- Engineering organization design, hiring, onboarding and technical-lead development.
- Architecture governance for complex backend, platform and systems products.
- AI-native engineering transformation and operating models for LLM-agent execution.
- High-risk legacy modernization and C++-to-Rust migration programs.
- Delivery and release governance, CI/CD, observability, SRE and incident prevention.
- Performance and reliability engineering for storage, distributed and high-load systems.

## Target roles
- Head of Engineering / Head of Development.
- Director of Engineering or CTO in a focused product organization.
- AI Engineering Transformation Lead with organization-wide ownership, not a coordinator-only mandate.
- Complex backend, platform, infrastructure and systems products; Rust is a differentiating specialization, not a boundary on the role.

<!-- pdf-pagebreak -->

## Experience

### Rust Chapter Lead • Orion Soft
*July 2025 - Present*

Lead an NDA-covered, AI-native migration program for Orion Soft: a production software-defined storage (SDS) product with about 200k LOC of C++ is being rebuilt as a 24-package Rust system with zVirt integration. LLM agents provide the primary implementation capacity; I own program architecture, engineering invariants, acceptance design and delivery, and remain accountable for what enters the product. In parallel, I lead Orion Soft's Rust chapter.

- Built and operate the delivery system behind 1,200+ merged changes and 6,400+ CI pipelines in four months; personally drove 87% of the changes through acceptance and merge.
- Designed executable oracle-equivalence: recorded operations replay through both the incumbent C++ product and the Rust implementation; behavior and state are compared exactly, with load-independent counters assessed before wall-clock performance.
- Established mandatory independent fresh-context review, automatic path-selected validation, isolated task workspaces, merge-train serialization and hard failure when required proof is missing.
- Set and enforce the fleet's standing engineering invariants: reproduce the reference's exact structures and parameters; treat a performance gap as architectural divergence until disproved; never remove a check that caught a real defect; repair a measurement instrument when its noise exceeds the effect.
- Direct the verification program across differential and property-based testing, mutation analysis, formal model checking, durability testing and full-stack E2E validation, with retained evidence as the basis for acceptance.
- Lead the Rust chapter: shape architecture and engineering practices, mentor engineers and build reusable expertise for future Rust initiatives.

**Technology and methods:** AI-native delivery, LLM-agent orchestration, executable acceptance, oracle-equivalence, Rust, C++, software-defined storage (SDS), zVirt, virtualization integration, CI/CD, differential testing, performance engineering, verification.

### Head of Development • Inline Group
*March 2023 - July 2025*

Led engineering for an enterprise procurement and internal-process platform, with direct ownership of backend development and functional leadership across DevOps, QA, architecture, releases and production operations.

- Led an engineering scope of 25-30 people; managed backend directly, worked with DevOps and QA through team leads, and coordinated frontend through API contracts.
- Established service and platform ownership, architecture-review and release-readiness rules, and a clear technical decision loop.
- Hired key specialists, developed technical leads, and established standards for review, planning, documentation, CI/CD and production ownership.
- Drove a controlled monolith-to-services transformation covering about 20 domain and platform services, shared components, logging, communication flows and integration utilities.
- Designed PostgreSQL-per-service data ownership, Redis caching, RabbitMQ-based asynchronous integration and forward-only schema migrations.
- Built GitLab CI/CD and GitOps delivery with reproducible builds, release tagging and rollback practices; supported daily releases across development, test and release environments.
- Turned API regression testing into a fast feedback loop, reducing runtime from 30 minutes to 5-7 minutes.
- Introduced observability, incident lifecycle, reliability and recovery metrics, root-cause analysis and SRE-style preventive improvements.
- Cut task-to-release lead time by about 50%; after service hardening, reduced production defects to 1-2 per quarter.
- Started controlled AI adoption through task and review templates, risk boundaries, team training, acceptance rules and quality control.

**Technology and methods:** Rust, Actix Web, PostgreSQL, Redis, RabbitMQ, Docker, GitLab CI/CD, GitOps, Grafana, Sentry, OpenAPI, SonarQube, cargo audit, Kubernetes.

<!-- pdf-pagebreak -->

### Lead Rust Engineer • YADRO
*February 2022 - March 2023*

Product: hardware/software backup appliance with deduplication.

- Led development of key storage and recovery components and owned architecture decisions for them.
- Established planning, review, quality, CI/CD and test-automation practices.
- Mentored engineers and supported the team's transition from C++ to Rust.
- Owned performance and reliability work, including RocksDB and storage tuning, load testing, incident analysis and root-cause fixes.

**Technology and methods:** Rust, RocksDB, CI/CD, performance testing, observability.

### Rust Developer • Kaspersky
*May 2020 - February 2022*

Developed and maintained a production Exonum-based voting service. Added weighted voting, simplified ballot flows, expanded test coverage and streamlined CI during the transition to Microsoft tooling.

**Technology:** Rust, Exonum, Protobuf, Serde, Git.

### C++/Go Developer • B2Broker
*November 2018 - March 2020*

Automated MT4/MT5 broker infrastructure: built C++/Go trade copiers and a Multi Account Manager, normalized and streamed real-time trading data, and developed data-quality controls.

**Technology:** C++, Go, MSVC, CMake, Protobuf, gRPC, NATS, YAML, PostgreSQL, vcpkg, Git.

### Lead C++ Engineer • ASCON
*May 2016 - November 2018*

Developed CAD libraries for KOMPAS and delivered 3D workflow capabilities. Introduced C++/Python automated testing, led the move to C++17, migrated the team from SVN to Git, and established review, planning and documentation practices.

**Technology:** C++17, MSVC, Python 3, Boost, PostgreSQL, Git, Jenkins.

### Earlier experience
- Concerteza: optimized C++ real-time protocol parsing for telecom signaling analytics up to about 1.5 Gbps per thread; added Python integration tests for continuous event processing.
- LiveTex: developed C++ and JavaScript components, including Node.js bindings for PostgreSQL and ZeroMQ.
- T4B: started as a C++ developer building MetaTrader plugins and a management web application.

<!-- pdf-pagebreak -->

## Education
Bauman Moscow State Technical University, IU7 Software Engineering, Specialist degree, 2015.

## Languages
- Russian - native
- English - C1

## Technologies and Domains
**Technologies:** Rust, C++, Go, Python, PostgreSQL, Redis, RabbitMQ, ZeroMQ, NATS, RocksDB, Docker, GitLab CI/CD, GitOps, Kubernetes, DevSecOps, SRE, Grafana, Sentry, SonarQube, cargo audit.

**Domains:** AI-native software delivery, agent-run systems migration, backend, platform engineering, software-defined storage (SDS), storage and distributed systems, virtualization, systems modernization, enterprise platforms, CAD, fintech and trading integrations, blockchain systems, high-load systems.

## Outside of Work
I enjoy films and poetry, play ukulele and guitar, and go hiking. I collect keyboard smartphones and vinyl records.
