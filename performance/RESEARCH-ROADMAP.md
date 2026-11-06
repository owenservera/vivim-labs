# Performance Lab — Research Roadmap and Experiments

**Research state:** planned, not executed. Never convert a proposal into a measured result.

## Era 1 — Inventory and baseline (first)
P01 Inventory actual deployed languages/frameworks/build targets in `repos-hq`, `vivim-web-harness`, `tierflow` at exact SHAs. Identify duplicated cores and boundary crossings. Output dependency/ownership map, *not* rewrite.
P02 Measure baseline Android cold/warm start, time to first meaningful UI, text→intent→approved action→receipt, native UI updates, background resume, permissions failure recovery, local query, graph update, phone↔desktop sync; include CPU/memory/battery/size and p50/p95/p99.
P03 Reproduce on representative physical Android 15+ device(s), low-end Android target and Windows host; test release builds and realistic data (e.g. 1k/100k/1m graph objects where feasible). Label emulators separately. Check repeated runs, thermal throttling and power conditions.
P04 Confirm the existing contracts: typed action, authority boundaries, approval, receipt, offline/recovery, deterministic test fixture. Freeze identical experiment inputs across contenders.

## Era 2 — Competing architectures, one vertical slice
P05 A: Kotlin + Jetpack Compose UI / Kotlin Android adapter / Rust kernel + UniFFI.
P06 B: TypeScript/WebView UI + Kotlin Android adapter / Rust kernel + UniFFI (native Compose for device-specific surfaces).
P07 C: Tauri 2 Android WebView UI / Rust commands / Kotlin plugins for device capabilities.
P08 Compare Rust vs Go vs Kotlin for *specific* tasks (graph traversal, indexed retrieval, serialization, encryption, sync merge, request routing); never compare language microbenchmarks alone.
P09 SQLite and engine experiments: SQLite WAL/FTS indices vs Rust in-memory graph and cold-store hybrid. Measure corruption recovery, migrations, write amplification, incremental projection, and query quality.
P10 Real-world Android constraints: Doze, foreground-service types, battery limits, accessibility consent, app-to-app intents, background work, storage limits, encryption, 32/64-bit ABI coverage, installed-size and crash recovery.
P11 Mobile↔desktop: local-first reconciliation, intermittent connectivity, encryption/key rotation, zero-duplicate effects, approval on phone, signed receipts and offline replay.
P12 UI visual runtime: complex screens, gesture and touch latency, accessibility tree, keyboard parity, streaming render throughput, cross-boundary IPC overhead and memory growth.

## Era 3 — Decision and adoption economics
P13 Build-vs-buy cost model: human + AI engineer hours, review/integration overhead, debugging, security audit, maintenance, upgrade cadence, recruiting and rollback; compare 3/12/24-month total cost.
P14 Sensitivity analysis: low-end device, user graph size, plugin count, burst throughput, offline mode, number of connected services; isolate crossover points where custom Rust becomes worthwhile.
P15 Score finalists; record uncertainties and recommended direction per platform component; publish an adoption proposal for owning repo, never automatically migrate production.
P16 Re-evaluate on upstream API/security changes, new devices and measured regressions; update one-pager at each material decision, even if decision is **keep current**.

## Benchmark protocol
**Comparison controls:** same UX behavior and approval/receipt contract; same device, OS version, power profile and input fixture; release build with comparable optimization; randomized run order; warmup; >=30 samples/condition when practical. Report sample count and confidence/variance, p50/p95/p99, CPU, peak+steady RAM, energy/battery (with methodology), APK bytes, compilation duration, implementation and human verification effort. Never infer device battery from emulator CPU alone.

**Suggested scoring rubric (provisional, adapt after baseline):** UX latency/responsiveness 20%; safety/reliability 20%; time to market 20%; battery/memory/size 15%; portability/reuse 15%; lifetime maintenance 10%. Hard veto: security/consent violation, loss of receipts, inadequate recovery, or unsupported mandatory Android functionality. Score each dimension 1–5 with source evidence, confidence and sensitivity. Do not conceal hard vetoes behind aggregate scores.

**Decision thresholds:** prefer current stack absent a material measurable improvement; for a custom rewrite require clear functional necessity or compelling benefit that remains under uncertainty *and* has an affordable adoption + rollback plan. Set numeric budgets only after measurement and product requirement review.

## Source-review queue
Context7 documentation first for SDK/library APIs: UniFFI `/mozilla/uniffi-rs/v0.32.2`, Compose `/websites/developer_android_develop_ui_compose`, Tauri `/tauri-apps/tauri/tauri-v2.0.0`; re-resolve latest versions before benchmarks. Also verify Android NDK, Gradle/AGP, Jetpack background work, Android Keystore, profiling instrumentation, gomobile, Flutter, and mobile Rust cross compilation. Record exact library versions and unsupported assumptions in evidence files.

## Deliverables for each experiment
Hypothesis; owner; affected repository/source SHAs; reference docs; fixture hash; hardware/OS/build; commands; raw outputs/metrics; failure/adversarial results; outcome; confidence; cost; rollback; next question. Use [EVIDENCE-LOG.md](EVIDENCE-LOG.md).
