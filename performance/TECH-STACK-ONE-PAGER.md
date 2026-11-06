# VIVIM Tech Stack — Living One-Pager

**As of:** 2026-11-06 · **Decision stage:** research / provisional · **Last measured baseline:** none established by this lab
**North star:** user-owned AI, portable local intelligence, deterministic owner-approved actions, fast and comprehensible experiences, no provider APIs.

| Area | Current research default | Why / primary tradeoff | Status | Source owner |
| --- | --- | --- | --- | --- |
| Shared kernel, graph, state machine, policy, sync logic | **Rust** with stable language bindings | Fast, portable, ownership/safety; integration and cross-language complexity | **Provisional** | repos-hq |
| Android OS access, lifecycle, permissions, foreground/background work | **Kotlin + Android SDK** | Native OS reach; OS restrictions apply equally to Rust | **Provisional** | vivim-web-harness |
| Android presentation | **Compete:** Compose vs TypeScript/WebView hybrid vs Tauri 2 | Native fidelity vs UI reuse; needs workload/device evidence | **Open** | vivim-web-harness + tierflow |
| Desktop host and system adapters | **Rust/Tauri 2 + platform adapters**, inspect implementation | Shared core; avoid accidental duplicate authorities | **Provisional** | vivim-web-harness |
| Dynamic visual runtime + deterministic UI intents | **TypeScript/contracts**, portable renderer boundary | Fast iterations, reusable schema; webview overhead to quantify | **Provisional** | tierflow |
| Knowledge storage | **SQLite/local-first**; benchmark specialized indices/graph engines | Reliable embedded persistence vs custom engine cost | **Provisional** | repos-hq |
| Network / pairing / cross-device transport | **Protocol contracts first**, implementations replaceable | Reliability and privacy before implementation language | **Open** | vivim-web-harness |
| Service/worker backend | **Do not standardize prematurely**; Rust/Go/TS by measured workload | Go may be valuable for services, not default Android core | **Open** | owning repo |
| Agent orchestration / Oracle | **Shared typed contracts**, host-enforced authority | Portability without bypassing permissions or owner consent | **Provisional** | repos-hq + Host |
| Bindings between Rust and Kotlin | **UniFFI candidate**; test packaging/ABI/errors | Code reuse vs FFI costs and lifecycle correctness | **Provisional** | shared core + Android |

**Decision rule:** ship the simplest safe implementation meeting measured UX/operational budgets. Choose custom Rust/other lower-level implementations when **measured end-user benefit + strategic reuse + maintenance savings** exceed **build time + verification cost + defects + portability/migration costs**. AI worker capacity reduces implementation cost only when measured end-to-end including review, integration and rework.

**Unresolved high-impact decisions:** Android UI (Compose/hybrid/Tauri); in-phone vs desktop execution split; background agent survivability; plugin/extension ABI; SQLite vs specialized graph paths; WebView UI/IPC overhead; battery/cold start. See [research roadmap](RESEARCH-ROADMAP.md).

**Evidence standard:** no performance winner declared without reproducible benchmarks on representative physical Android devices and Windows hardware. Label measured, modeled and conjectural claims separately; record benchmark outputs in [evidence log](EVIDENCE-LOG.md). **Promotion:** lab proposes; owner approves; owning production repo implements; tests + rollback required.

**Documentation checked (2026-11-06 via Context7):** UniFFI `/mozilla/uniffi-rs/v0.32.2`; Jetpack Compose `/websites/developer_android_develop_ui_compose` (version not surfaced); Tauri `/tauri-apps/tauri/tauri-v2.0.0` (older snapshot; validate latest before selecting). Actual repo SHAs, compatibility and performance remain unverified.
