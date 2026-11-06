# VIVIM Performance & Technology Strategy Lab

**Status:** research initiated · **Opened:** 2026-11-06 · **Authority:** advisory only · **Scope:** cross-platform VIVIM / TierFlow / HQ
**One-page living decision map:** [TECH-STACK-ONE-PAGER.md](TECH-STACK-ONE-PAGER.md)

## Mission
Continuously test whether VIVIM's technology stack is optimal for real user value, accounting for latency, memory, battery, startup, reliability, privacy, security, portability, engineering time, AI-assisted implementation throughput and lifetime cost. Distinguish today's shippable choice from the plausible ultimate architecture.

## Boundaries
This repository is an **evidence and recommendation lab**, not a competing production authority. Never directly modify live Host, Android, TierFlow visual runtime, KB, permissions, or action contracts from lab work. Repositories own implementation: `repos-hq` shared intelligence/kernel; `vivim-web-harness` Host, Android and device adapters; `tierflow` visual/command runtime. Cross-repo changes require a separately approved promotion with owners, tests, rollback and measured evidence.

## Files
- [TECH-STACK-ONE-PAGER.md](TECH-STACK-ONE-PAGER.md): human-readable decision dashboard, *kept at approximately one page*.
- [RESEARCH-ROADMAP.md](RESEARCH-ROADMAP.md): questions, hypothesis-driven experiments and scoring protocol.
- [EVIDENCE-LOG.md](EVIDENCE-LOG.md): immutable-result pointers and decision history; no fabricated benchmarks.

## Operating loop
1. Inventory deployed code, contracts and dependencies at exact SHAs, without mistaking discussion for implementation.
2. Establish reproducible baseline using representative devices/workloads, then vary **one architecture variable at a time**.
3. Measure p50/p95/p99 latency; cold/warm launch; memory and allocation; CPU; battery/energy; APK size; binary footprint; build/release duration; crash/recovery; permission/security failure behavior; development effort and AI rework.
4. Keep raw commands, device model, Android version, thermals, power profile, sample size, build types, data set hash, errors, and outputs. Run adversarial negative tests.
5. Classify findings as **Verified / Provisional / Hypothesis / Rejected**. Update one-pager only with evidence or explicitly labeled architectural judgment.
6. Recommend adopt/keep/experiment/retire and explain switch cost, time-to-market, risk, portability, and rollback. Owner approves production changes.
7. Revisit when major SDK/runtime changes, benchmark regressions, security advisories or new production requirements arrive. No automatic production migration.

## Research guardrails
- No provider APIs, no Playwright/Puppeteer, no competing Chrome controller, no new action authority.
- Preserve owner approval and effect receipts; local-first and no unauthorized data export.
- Separate synthetic device measurements from real physical-phone measurements.
- Avoid rewriting working components merely because Rust could implement them.
- A research conclusion is not a release gate until explicitly adopted by the owning repo.

## Near-term output
First benchmark candidates: (A) Rust + Kotlin + Compose, (B) Rust + Kotlin + TypeScript/WebView hybrid, (C) Tauri 2 mobile + Rust + TypeScript; plus targeted Go/native approaches only when they solve a measured gap. Compare a common functional vertical slice with the same Android permissions and fixtures. No winner has yet been measured.
