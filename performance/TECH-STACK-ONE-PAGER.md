# VIVIM — Tech Stack One-Pager

**Updated:** 2026-11-06 · **State:** preliminary architecture review, not validated performance ranking

| Component | Working direction (verify against actual code) | High-leverage opportunity to investigate |
| --- | --- | --- |
| Shared core, knowledge graph, command/policy logic | Rust | Move genuinely shared hot paths into one portable kernel; avoid duplicate implementations |
| Android app | Kotlin Android adapter; Rust core candidate | Compare native Compose, WebView hybrid and Tauri mobile before committing UI |
| Desktop Host | Tauri/Rust direction | Share core with phone, keep OS-specific adapters |
| Dynamic UI / visual runtime | TypeScript and typed contracts | Reduce rendering/bridge overhead without losing fast UI composition |
| Data and search | SQLite + Rust compute candidates | Selectively replace slow graph/index/query paths only where advantageous |
| Sync, networking and pairing | Protocol-first, platform adapters | Efficient offline-first sync and compact transport |
| Agent/orchestration services | Mixed runtime; inventory needed | Identify hotspots where Rust, Go or another runtime has a clear advantage |
| Plugins and extension layer | Typed, portable contracts | Avoid per-platform duplication and expensive boundary crossings |

**Immediate candidates:** (1) shared Rust kernel on Android/desktop, (2) Android UI architecture, (3) knowledge-graph/query hot paths, (4) cross-language/renderer boundaries, (5) offline sync. These are **opportunities, not migration decisions**.

**Decision lens:** Will it be noticeably faster, more reliable, safer, more portable or simpler for users? Is the gain worth the migration risk and disruption to shipping? **Do not count AI implementation cost, AI token usage, model pricing or agent capacity as costs or scoring inputs.** AI capacity permits more ambitious custom work, but does not establish technical superiority.

**Next:** brief architecture inventory; mark 2–3 high-leverage candidates for later machine testing. No benchmarks or changes to production have been completed by this lab.
