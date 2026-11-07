# SITREP — Browser CLI Work Versus Rust/Core Integration
**Audit date:** 2026-11-04. **Scope:** remotely accessible GitHub `main` of `repos-hq`, `vivim-web-harness`, `hq`, `SOFTWARE_HQ`, and `tierflow`; source, commit history, focused docs and tests. **Not a live Windows inspection.**

## ERA 1 — What landed, and in which runtime

| Track | Remote source/evidence | Verified from GitHub source | Not established |
| --- | --- | --- | --- |
| Rust HQKG research | `repos-hq/research/rust-knowledge-graph`, `Cargo.toml`, `src/lib.rs`, `src/engine.rs`, `src/database.rs` | Graph, journal, compute-engine slots/hot swaps, optional read-only SQLite/Postgres adapters, importer/CLI source; source moved from `hq` Nov 3 | An installed operational kernel replacing HQ's Bun/SQLite runtime |
| Browser-to-Rust web memory | `vivim-web-harness/apps/browser-companion`; `studio/src/studio/browser-companion/{ingress,web-memory-core}.mjs`; `repos-hq/research/rust-knowledge-graph/src/bin/hqkg-web-feed.rs` | Opt-in Chrome extension navigation observation, strict loopback ingress, serialized Rust child process, acknowledged graph visit/site nodes, local build/test script | Successful real Windows Chrome + Rust + restart execution; full page contents; Android browsing; continuous knowledge search |
| Browser automation | `vivim-web-harness/harness/src/core/{cdp,pinchtab,send,reply,journal,tabs}.mjs` and `studio/src/capture/{recorder,live-tap,network,dom}.mjs` | Browser/Page abstraction and provider send-once path, PinchTab HTTP adapter, CDP recorder, durable send records, safety refusals | Universal working provider coverage and stable UI selectors in current signed-in accounts |
| CLI tools | `studio/bin/studio.mjs`, `studio/bin/commands/chat.mjs`, `studio/scripts/windows/CLI-INSTALL.md`, `apps/vivim-host/control/cli.mjs`, `harness/src/agents/cli.mjs` | One-shot website-turn CLI, Windows `vivim` launcher, Host natural-language control, fleet/swarm/round-robin CLI code | One unified consumer/runtime CLI across all mechanisms and fresh-install release acceptance |
| Seat orchestration (dev harness) | `repos-hq/hq/lib/{channel-seats,channel-transport,seat-watch}.mjs`, `docs/AGENT-BUS.md` | ZCode CLI `--resume -p` seat waking, ACP fallback, local state/dispatch records, sync bus, explicit accepted vs actual reply evidence | Consumer runtime free of ZCode; provider-web-app control performed directly by HQ instead of Host/Studio |
| Action routing and UI core | `repos-hq/hq/lib/{action-registry,action-router}.mjs`; `repos-hq/vivim-core/src/{contract.ts,revision/apply.ts}` | Pure action registry/router, scoped approval ideas, typed UI capability and reversible revision contracts | Production integration of those prototypes with the VIVIM Host and Android view |
| Personal/operational KB | `vivim-web-harness/apps/vivim-host/bootstrap/ledger.mjs`, `repos-hq/hq/lib/kb*.mjs`, `docs/VIVIM-KB-INTAKE.md` | Separate Host SQLite bootstrap ledger and HQ ingest/index contracts | One reconciled source-of-truth across Rust graph, Host ledger, HQ index and phone |

### Precise chronology and divergence

- Nov 3: `hq` retired the old Rust source after relocation to `repos-hq`.
- Nov 4 around 03:24–03:40Z: browser-to-Rust importer and opt-in all-site metadata feed created in the two canonical repos. The Rust bridge was only source-implemented; no real Windows acceptance receipt.
- Nov 4 05:44–07:36Z: `repos-hq` independently added wake/seat recovery, channel transport, action registry/router, fleet feed, typed capability contract and agent operations documentation.
- Nov 4 07:51Z: `vivim-web-harness` head advanced to `ab393a3df`, with Vercel Git deployment changes; browser memory remains committed on main from earlier in the day.
- A **local channel-manager progress report** (`repos-hq/docs/integration/PROGRESS-REPORT-channel-manager-20261104.md`) claims `vivim-web-harness/studio/src/studio/chat-channel.mjs`, `studio/src/studio/channels/gemini.mjs`, `schemas/chat-receipt.v1.json` and unit tests were landed in local checkouts, but those paths **are absent from the fetched remote `vivim-web-harness/main` tree**. They must be compared with the Windows working tree and reconciled before relying on them. Avoid blindly recreating them or falsely marking them committed.

## ERA 2 — Status and risks

**Source present, not yet accepted operationally:** Rust graph + browse feed + browser harness + several CLIs.

**Reported local proof (historical, not re-executed here):**
- Channel-manager report: 20/20 core channel tests, 35/36 Gemini tests (1 failure), 30 passes + 1 skipped Claude test. Local paths presently not visible in the remote harness tree.
- Same report: Gemini long wake originally pending at 200 seconds, later accepted on a separate wake; the report explicitly says no provider send from the channel-manager session. It lists stale composer handle, thread-URL verification, and wake back-fill as unfinished.
- HQ channel matrix current remote document: 100 items = **1 Y, 22 P, 77 N**. Separate channel-goals queue = **5 checked, 95 unchecked**. These are not measures of the six-lane 600 consumer capabilities.
- Rust `README.md` labels HQKG an isolated prototype, not the operational Bun HQ indexer. `docs/RUST-WORKSTREAM.md` retains gates G1–G6 for replacing HQ's active runtime.

**Top five delivery blockers:**
1. **Unmerged local channel changes:** local claimed chat-channel work absent on remote harness main; preserve and diff exact local changes first.
2. **No executed full browser-to-Rust acceptance:** compiler/test script exists, but no real extension→Studio→Rust→KB→mobile proof bundle.
3. **Split durable truth:** Rust journal has browser nodes; Host SQLite ledger and HQ Bun/SQLite knowledge projections are separate. No demonstrated one-state reconciliation or consumer query.
4. **Rust persistence scale:** `Journal::save` appends whole graph snapshots; spawning a fresh Rust process for each batch loads/scans state. This is correct as a prototype but the wrong indefinite-history throughput profile.
5. **CLI proliferation and actor ambiguity:** Windows `vivim`, Studio `studio chat`, harness `wh`, HQ `hq seat`, and SOFTWARE_HQ `ochq` serve different audiences. HQ ZCode seat wake is development orchestration, **not** provider webapp browser control, and must not become a consumer dependency.

**Secondary risks:** stale provider page handles and incomplete reply capture, token/plan quotas, duplicate send uncertainty, fixed-size extension offline spool, Chrome-only browsing source, strict privacy exclusions, missing common status/health read model, and report claims not backed by current checked-out revisions.

### Ownership decision

- **Host/Studio:** one browser-effect owner, user authorization, send journal, Chrome profile, UI/runtime and source event intake.
- **Browser adapters:** MV3 extension, CDP, PinchTab as a selectable alternative, site-specific providers; never competing controllers of the same profile/turn.
- **Rust core:** deterministic event ID/provenance, graph mutation, durable logical state and compute nodes. Do not move Chrome/CDP, selectors, GUI or provider API behavior into Rust.
- **HQ Bun/SQLite:** currently operational context/index and development orchestration, with explicit read-only projections from trusted product evidence; don't silently replace it.
- **ZCode/OpenCode CLI:** development tools, not shipped product requirements. App-building and passive memory must operate without them.
- **Core/engine/adapter:** core holds irreconstructible truth and single-writer authority; engine computes derived context/ranking; adapter translates source envelopes and is replaceable. The existing browser extension's bounded offline queue is a transport-specific durability buffer, not a second knowledge authority.

## ERA 3 — Exact next integration sequence

**0. Protect local work:** on Windows capture `git status --short`, `git log -10 --oneline`, untracked file inventory and diff for `repos_hq`, `vivim-web-harness`, and actual local channel source paths. Do not reset profiles, wipe local state, overwrite pending receipts, or force-push.

**1. Reconcile the channel split:** locate the missing `chat-channel` and provider-specific modules in local worktrees; compare to remote `main`, validate 20/20/35/36/30+skip assertions against actual installed versions; commit only non-conflicting verified source with ownership preserved.

**2. Single browser actor route:** prove a browser session has one owner. Choose existing Studio CDP as the effect owner; retain PinchTab only as an adapter behind the same contract/owner. Expose `observe`, `sendOnce`, `readReply`, `reconcile` and explicit statuses to Host tools/CLI. Never route ZCode seat-wake as a provider send.

**3. Close first Rust feed vertically:** build `hqkg-web-feed`, run Rust/Node tests, load/pair opt-in Chrome extension, browse normal external pages, inspect persisted `web:visit` nodes; simulate Host stop/replay, confirm no duplicate, report queue loss. The setup script is `studio/scripts/install-web-memory.ps1`.

**4. Make evidence useful:** project Rust visit observations into the existing Host KB read-model with a one-way versioned intake, maintaining source authority; expose one user-facing `what did I browse?` query from PC and phone. This, not another CLI, is the first true V1 user acceptance.

**5. Measure before Rust migration:** benchmark batch latency, journal growth, memory and startup at realistic sustained browsing throughput; only then introduce incremental append + checkpoint/compaction and a persistent sidecar if warranted. Preserve parity with Bun/SQLite and a tested rollback. Don't move unproven browser automation into Rust.

**6. Bridge intelligent workflows:** join provider send/reply receipts and personal browsing observations with source provenance. Reuse the graph for context invalidation/retrieval, model orchestration and durable agent tasks, with action effects still Host-owned.

**7. Consolidate command surfaces:** expose a stable consumer Host text action vocabulary; CLI aliases become development adapters to the same contracts, not parallel engines. Retire duplicate logic only after parity and end-to-end receipts.

**Acceptance benchmark:** A real Windows/Android owner installation opens an arbitrary ordinary website on the paired Chrome profile, passively captures a sanitized navigation event with consent, persists it into the Rust graph, makes it queryable via VIVIM knowledge on both devices, survives Host interruption without duplicate visits, and surfaces accurate provenance and opt-out. A different signed-in AI webapp can then use that knowledge via an authorized provider conversation, still with no ZCode/OpenCode requirement.

### Evidence grading

- **SOURCE:** current source file or commit is visible on remote main.
- **REPORTED:** historical local test or command transcript in repo documentation, not re-executed in this audit.
- **UNVERIFIED:** requires live Windows run or local-only changes that the GitHub connector cannot see.
- **BLOCKED:** known missing file, failed test or absent end-to-end contract.
- **DO NOT CALL COMPLETE:** a planning spec, schema, SDK wrapper or CLI command whose dependent real-world user journey hasn't passed.

No new executable code was changed by this SITREP; it is a read/triage pass. Remaining status may differ in local Windows files that have not been pushed.
