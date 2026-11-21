# Session handoff — GPUI / AutoWiki dual milestones (2026-11-21 10:32 UTC)

**State: PAUSED BY OWNER.** This session is DevOps/Planning **pre-integration**, NOT the ChatGPT Dev ARBOR implementation executor. No task is actively checked out. No package for this GPUI milestone has been created; no new ARBOR source changes, tests, deployment or GPUI runtime verification were performed in this session. Resume only when the owner requests. Read [TASK-CLAIMS](../TASK-CLAIMS.md) before claiming anything.

## Owner's precise contract
- Prepare **all necessary proposed code, tests, exact file placement and integration instructions** in **VIVIM Labs first**. **Do not integrate it into ARBOR until the owner's next round**.
- **Milestone 1:** human types -> AutoWiki appears/updates automatically in the native GPUI surface, without pressing Send/Search.
- **Milestone 2:** authorized agent types -> the **same** AutoWiki projection updates, with observable, verified typing; agent edit requires normal Host plan, explicit approval, source acknowledgment and revision correlation. No fabricated dot/keystrokes or auto-send.
- **NO ANDROID DEPENDENCY.** Rust Core is authoritative; GPUI is preferred native validation; PWA remains a separate UI of the same core. Android/ADB is not a gate. See [VALIDATION-SURFACES](../VALIDATION-SURFACES.md).
- Rust-first, browser TypeScript 7+ only if needed. ChatGPT codes in cloud through GitHub main; ZCode/Desktop Commander cannot serve as an application coding fallback.

## What this session actually inspected (GitHub main, NOT immutable future pin)
- `owenservera/ARBOR/native/gpui-desktop/src/main.rs`: existing GPUI 0.2.2 + gpui-component input `Entity<InputState>`, `Desktop`, `find_wiki()`, `select_panel()`, `refresh()`, `Render`. Current `find_wiki` reads text only **on Search current Wiki button**; `wiki_query:Option<String>` drives `wiki_read::search_view` on currently observed Wiki. Idle refresh every 15s is NOT live typing. Native GPUI is real source, Windows GPU acceptance not established.
- `native/gpui-desktop/src/lib.rs`: `read_overview()` authenticates Host owner, double-brackets existing `/api/host/control/wiki-view` and `/api/host/control/pwa-view`; `Overview` has `wiki_revision`, `wiki_digest`, `wiki_search_index` and read source. Reuse exact auth + provenance fences.
- `core/programmability/src/wiki_read.rs`: `WikiRead`, `SearchRead`, `search_view(&WikiRead,query,limit)`, `RevisionGuard`, `source_search`; bounded and read-only, no new Wiki engine required.
- `core/programmability/src/live_system.rs`: typed `LiveSystemState` with source/view revisions, `RegionKind::Input` and `ChangeEvent`.
- `core/programmability/src/agent_composer.rs`: `prepare_draft`, `verify_source_observation`, no effect authority. Existing `phone.composer` actor proposal restricted to Host-authorized edit.
- `apps/vivim-host/control/agent-composer-bridge.mjs`: `observeWithFence`, `stage`, `execute`, `acknowledge`, `status`. Existing claimed Host Control effect + source-observation verified receipt, not arbitrary agent keystrokes.
- `apps/vivim-host/control/live-system-read-port.mjs`: current allowlist includes `phone.composer`, `wiki.summary`, etc.; **not** a GPUI composer region. Host live-system change route `/api/host/live-system/changes` is owner-protected and bounded.
- `apps/mobile-relay/first-contact-composer.ts`: current browser source and `onVerified` callback for a verified Host acknowledgment. Existing PWA UI and search-as-you-type path belong to Dev; do not race them.
- `native/gpui-desktop/README.md`: GPUI source exists; Windows GPU smoke and real native event/visual acceptance still unproven. Reuse native GPUI control rather than assuming PWA HTML is rendered inside GPUI.
- Existing **different** Labs workstream [AutoWiki Foundation R003](../DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md) contains staged Rust semantic/context modules and 37 authored but UNRUN tests. **Reuse, do not duplicate** its graph/semantic projection or take over its claimed scope.

## Next agent's exact sequence, after the owner says continue
1. Refresh `TASK-CLAIMS.md`, Planning `AGENTS.md`/`PROFILE.md`/`SITREP.md` and actual canonical OW task registry (local `OWNER-WORK/NOW.md` exists, but `OWNER-WORK/` is **not on Labs GitHub main**). Keep source provenance distinct; do not fabricate a remote OW update. Check any other agent's scope before checkout.
2. **Claim the Labs-only GPUI dual-milestone PREPARATION** with a fresh SHA/collision-safe GitHub update. Do not claim Dev's ARBOR/PWA live implementation tasks. Re-fetch the accepted claim.
3. Inspect current `ARBOR/main` (it may have changed) around the above exact symbols. Classify REUSE/REPAIR/NEW and re-pin commit SHA. Do not rely on the frozen source snapshots above.
4. Create a new narrow **Labs staging package** under `DESIGN/GPUI-AUTOWIKI-LIVE-R001/`: `README.md`, `SOURCE-MAP.md`, `INTEGRATION.md`, `ACCEPTANCE.md`, and actual Rust source/test files (`src/`, `tests/`). Put GPUI glue proposals in isolated `integration/` as patch-ready segments; mark proposed insertion points. Do not touch ARBOR.
5. **Human milestone proposed implementation:** observe GPUI `InputState` change events (consult exact gpui-component 0.5.0 APIs before authoring); debounce ~180ms using latest-request generation token; validate chars/limits and Unicode/IME/composition; run the canonical Rust Wiki search/projection over latest authorized `WikiRead`; set `wiki_query` and automatically select/retain visible Wiki panel without submitting/planning anything. On source revision change, rerun latest query; stale/out-of-order results never overwrite newest. Give empty/unauthorized/unavailable truthfully.
6. **Agent milestone proposed implementation:** do NOT reuse `phone.composer` as if it were the GPUI input. Design explicit native composer region registration **using existing LiveSystemState and existing Host approval/control path**, gated by owner authority. Reuse Rust `agent_composer` and Host ACK semantics, but a native GPUI source observer/ACK adapter requires new verified integration. Agent proposal is inert until claimed approval. On authoritative native applied+readback proof, broadcast the same source-stamped draft revision into the same search pathway used by human edits; no simulated success or provider send. Document whether an existing Host action catalog can accept the new source; otherwise mark that specific addition NEW and do not claim seamless execution.
7. Use existing Host/wiki owner read and `wiki_read::search_view`, not a second Wiki DB or graph. Keep all private draft data out of diagnostic logs. Never treat source prediction as a verified fact; context enrichment from the existing AutoWiki R003 package is a **separate downstream enhancement**, not necessary to pass basic live-search milestones.
8. Author runnable Rust unit tests and precise GPUI/Host boundary tests: human updates without button/Send; empty/IME; rapid edits; stale Wiki revision; owner auth refusal; revoked grants; agent authorization and one-shot ACK; human/agent race; same-revision conflict; no duplicate effect. Run tests only when available, log PASS/FAIL/UNRUN exactly.
9. Commit **only Labs staging files** to `vivim-labs/main`, update `DEVOPS-PLANNING/BUNDLE-INDEX.md` and `TASK-CLAIMS.md` with source SHA, test receipts, and check-in `READY_FOR_HANDOFF` only if it is genuinely complete. Handoff to Dev remains `OFFERED_TO:CHATGPT-DEV` until Dev explicitly accepts.
10. **STOP** before ARBOR implementation. Owner explicitly reserved the next round for integration.

## Integration gates for next round (all UNRUN this session)
- G0: latest ARBOR exact source/ownership verified.
- G1: Labs Rust GPUI preview engine and tests compile.
- G2: owner-authenticated Host native composer source/ACK integration passes tests.
- G3: Windows GPUI human input -> on-screen Wiki change, no Search/Send.
- G4: Host-authorized agent edit -> GPUI observable update -> same Wiki source revision, no Send.
- G5: PWA read/Native GPUI source parity, optionally tested with browser/HTTP; no Android required.

**Known traps:** GPUI native Input is currently also used for Host commands/graph queries; don't accidentally trigger `submit()`. The existing Host agent delivery targets **phone.composer**, not GPUI. A code-only GPUI mock does not prove a real source-backed Host/GPUI loop. Preserve already claimed source files. Rust Wiki `search_view` searches committed Wiki content, not generative pages from typed words; do not claim a full semantic AutoWiki without additional source-authorized context.

## Status / handoff receipt
- **Observed:** 2026-11-21 10:32 UTC (Europe/Madrid 11:32), source audit only; exact main SHA to refresh.
- **Scope:** DevOps/Planning staging in Labs; downstream Dev-owned ARBOR.
- **Actual status:** PAUSED / NOT CHECKED OUT; design path and requirements documented, implementation candidate source UNWRITTEN; tests UNRUN.
- **Resume trigger:** explicit owner “continue”.
- **Verification:** this document can be fetched from Labs main; no cross-chat notification or implicit Dev acceptance.
