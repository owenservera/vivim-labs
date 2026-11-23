# VIVIM / ARBOR execution pack R001 — 2026-11-23

> **Current-source reconciliation (2026-11-23):** [Live agent pickup and blocker delta](CURRENT-PICKUP-20261123.md) records ARBOR `0cd77a21`, **61/61 native GPUI Rust tests**, **215/215 source-pinned ZCode deterministic checks with four scopes serialized**, physically verified native theme/AutoWiki, and still-UNRUN live provider/Android parity. The creation-time pins and pending-run snapshot below are historical, not the latest receipt. No claims transferred.

**Purpose:** the shared, source-pinned agent pickup packet for the current owner-led development program. This **does not** create a second backlog, source owner, dispatcher, long-running agent, model router, or permission system.

**Observed source heads at packet creation:** ARBOR `84ca3c36980238c730875f647baedc18ec7e0747`; VIVIM Labs `adb885fb60dbc52aeda60c7715e50078c7f7a158`. These are observations, not immutable targets: **refresh main before every change and test**. Canonical master [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33) counted **6/71 accepted** as its last explicit snapshot; unchecked means *not acceptance-proven*, not necessarily unimplemented. Concrete provider issue [#32](https://github.com/owenservera/ARBOR/issues/32); cross-device release [#31](https://github.com/owenservera/ARBOR/issues/31).

## Agent bootstrap (read in order)
1. ARBOR [AGENTS.md](https://github.com/owenservera/ARBOR/blob/main/AGENTS.md), [single-surface rule](https://github.com/owenservera/ARBOR/blob/main/docs/product/ONE-CANONICAL-LIVE-SURFACE.md), and [owner handoff](https://github.com/owenservera/ARBOR/blob/main/docs/product/FULL-HANDOFF-GPUI-PWA-VLANG-ZCODE-20261122.md).
2. Labs `ZCODE-DESKTOP-COMMANDER-OPERATIONS/{TASK-CLAIMS.md,TEST-LEDGER.md,DEVOPS-PLANNING/BUNDLE-INDEX.md}`; existing Planning [master 37-candidate catalogue](../../MASTER-PLAN-R001.md). Claims are an overlay; do not invent transitions or count Planning proposals as 71 delivered items.
3. This pack's [AGENT-HANDOFF.md](AGENT-HANDOFF.md) and [ACCEPTANCE.md](ACCEPTANCE.md); machine-readable [manifest.json](manifest.json).
4. Check the actual GitHub HEAD/file SHA, local receipt SHA and task claim before acting. No stale-sha inheritance.

**Latest verification addendum:** [2026-11-23 native/ZCode exact-source receipt](VERIFICATION-RECEIPT.md) — 215/215 PASS at ARBOR `0cd77a21` with four lanes serialized on the constrained PC, plus separate 5/5 Chat UI tests at ARBOR `7991f675`; not a device/provider pass.

## Current verified state / cautions
- ARBOR recent main includes Host-theme GPUI projection and F00 Oracle/Wiki regression (commits `063f91cc`, `126f32e1`, `84ca3c36`); source updates are **not deployed/device acceptance**.
- Owner's Desktop Commander device `DESKTOP-9KB03LU` online at inspection. Local ARBOR HEAD `5b129485783c` (behind observed GitHub main); Labs local `adb885fb60db` clean tracked; `repos_hq` has two unrelated tracked changes. **Never reset, pull over, overwrite or use local checkout as main source authority**.
- ZCode GUI running. 93 stored agent definitions is **not** 93 live agents. Model-based parallel scouts earlier hit HTTP 429 and returned no verified findings; do not assume top-six working models or paid fallback. Reuse installed model registry and routing; do not build another.
- Existing source-pinned native + four-lane Windows observer reported PASS at ARBOR `7b9e7dbd5f17042febea33e6b8563d536987545f`: 22 VLANG + 160 safety + 12 AutoWiki + 21 visual = **215/215**, sourceWrites=0, hostEffects=0, providerSends=0. Native/test receipts match the recorded source/binary/runner hashes. This is not the `84ca3c36` head.
- The same existing scheduled observer `VIVIM-VLANG-Observer` was manually triggered at 2026-11-23 20:01 Europe/Madrid to verify refreshed source; it reported **RUNNING**, not PASS, at observation. Its next normal recurring run was listed for 20:46. Read `local-logs/native-verifier/latest.json` for actual eventual result.
- Windows probes: Studio `127.0.0.1:38730/` HTTP 200; Host `127.0.0.1:43187/` returned HTTP 308 on root; ZCode's old `:45278` endpoint timed out. These do not prove live AI chat, a healthy protected Host API, or phone sync. Several `vivim-product-pure.exe --serve` processes were observed; **inspect rather than kill**.

## Ready-to-pick-up work (these are handoffs, not automatic execution)
| Package | Canonical IDs | Primary source owner | Immediate safe action | Completion evidence |
|---|---|---|---|---|
| **P0 signed-in AI** | A01–A16 / #32 | ChatGPT Dev on ARBOR main | Read-only reconcile old uncertain ChatGPT turn and existing busy Studio session; inspect ready source-pinned guarded worker and actual browser controller | One fresh owner-submitted prompt: composer/thread pinned, one send, settled reply, Host receipt, physical phone ACK; no old-key retry |
| **P0 release and phone parity** | S02–S14 / #31 | ChatGPT Dev; DC verifies Windows/Android | Build immutable single-SHA native+Host+PWA candidate; isolated preflight; preserve live URL and rollback | Package hash, live served SHA, PC→phone and phone→PC revisions with phone evidence |
| **P1 native First Contact** | U01–U04, U09, G02–G05 | ChatGPT Dev; DC verifies real GUI | Test current Host-theme GPUI source and menu, real typing, IME, approval/feedback | Real Windows screenshot/interaction and matching source revision; no false pass from Win32 handle |
| **P1 VLANG / AutoWiki / Oracle** | U05–U08, O01–O06 | ChatGPT Dev on ARBOR; Planning stages Labs-only; Oracle audits | Existing deterministic Host NLCL → typed Host plan → approval/receipt and source-backed Wiki; inspect reuse and grant producers | No-LLM command cases, scoped Wiki freshness/grant tests; authorized real corpus, not synthetic personal records |
| **Verification & learning** | Z01–Z07, R01–R07 | ZCode read-only, DC local evidence, owner reconciles | Keep existing source-pinned observer; triage real failures; check CI startup_failure independently | SHA/binary hash/run ID/test counts; device evidence separate; no paid model fallback |

## Non-negotiable rules
- **ONE canonical surface**: native GPUI Windows authoring/renderer, responsive mobile PWA view, Host sole persisted-state/effect/grant/approval/receipt authority; same source revisions. No new browser owner, parser, DB, bus, renderer, Host, or Android app. TierFlow retirement only after verified parity. Native app shell menus live above product canvas.
- ChatGPT integrates application source via GitHub `main`, Rust first, TS 7+ where necessary; ZCode and Desktop Commander must not become duplicate source writers. A separate ChatGPT session's current file claim must be respected.
- **Do not resend** old ChatGPT `CREATION_UNPINNED/TYPING_UNCONFIRMED` uncertain request; do not disturb busy Studio session. No provider APIs, Playwright/Puppeteer or secret logging.
- Prefer deterministic offline/local model-free testing. Do not launch old effectful `arbor-life` or `arbor-product` workflows. Admit existing dynamic scout only for exact clean HEAD with already verified free quota; stop on 429.
- Reporting ladder: CODED / PUSHED / SOURCE TESTED / PACKAGED / STAGED / DEPLOYED / PROVIDER VERIFIED / GPUI RENDERED / PHONE RENDERED; UNRUN is never PASS. Exact matching SHA, timestamp and receipt for each.

## How others receive this
This file is committed to the *existing* Labs operations tree and linked from its `DEVOPS-PLANNING/BUNDLE-INDEX.md` plus master GitHub tracker #33. An agent reading either canonical entrypoint can pick up the correct scope and post a claim under `TASK-CLAIMS.md`. **Publication is not a live notification, automatic agent invocation or cross-chat acknowledgement.** Each agent must explicitly check out an unclaimed packet and return evidence; no silent task reassignment.

## R002 current wiring and ready-package overlay

The [ARBOR source-backed NOW→FUTURE wiring audit](https://github.com/owenservera/ARBOR/blob/main/docs/product/ARBOR-NOW-FUTURE-WIRING-20261123.md), [R002 executable pickup delta](WIRING-DELTA-R002.md), and [R002 machine manifest](wiring-delta-r002.json) supersede stale **observed-state snapshots** in this R001 README without changing the original five R001 work-package IDs, owners or safety contracts. They do **not** auto-dispatch work. Refresh current GitHub HEAD and task claims before accepting any package.

## R003 source-backed pickup — 2026-11-23

- [PY-001 explicit PWA Build/Change admission packet](PWA-BUILD-INTENT-R003.md): preserves the existing Preview/Activate and hidden design conversation but demands a **distinct explicit Work submit**, authenticated Host writer, CAS/idempotency, no automatic provider send, and 15 negative/phone acceptance tests. Existing Studio Work reducer **reused** but its owner-authorized Host binding is still **blocked at PY-002**.
- [P0-AI-CHAT safe Studio readiness](PROVIDER-READINESS-R003.md): current read-only probe observed `STUDIO_OWNER_IDLE` and ChatGPT target, **not a real authenticated provider reply**; warns that local reconciliation helper performs state-changing POST and must not run as a status check.
- [Machine-readable R003 pickup state](pickup-r003.json): precise owner scopes, no agent auto-dispatch, actual evidence versus UNRUN. Existing R002 [wiring map](WIRING-DELTA-R002.md) and [R001 assignment/claims](AGENT-HANDOFF.md) remain authoritative; these are narrow addenda, **not** new tasks/engines.
