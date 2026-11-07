# VIVIM Web Harness — Divergent Branch Code Autopsy and Integration Directive

**Audit date:** 2026-11-01 (Europe/Madrid)  
**Canonical repo:** https://github.com/owenservera/vivim-web-harness  
**Windows checkout:** `C:\0-BlackBoxProject-0\vivim-web-harness`  
**Read-only audit base:** `main` at `0a1ce42d2633e2a7cee80dfe19f6bec3fc604f2b` (GitHub timestamp 2026-11-01 00:19:07 UTC).  
**Scope:** 17 divergent branches recorded in `docs/library/GIT-RECONCILIATION-2026-11-01.md` on PR #40; selected complete source reads, commit patches, PR diff inventories, source SHA comparisons and commit/merge timestamps.  
**Verification boundary:** Remote source-only. No Windows worktree/process inspection, no Node/Cargo/Android tests, no live Chrome/phone exercise, no branch edits or merges. Do not convert source presence into verified runtime acceptance. Check current `main` SHA again before acting.

## ERA 1 — Forensic results

### 1. Historical and source-based verdicts

| Branch | Head UTC | Unique commits vs audited main | PR history | Code/content evidence | Disposition |
|---|---|---:|---|---|---|
| `feat/utc-nlcl-language-core` | 2026-10-31 23:52:04 | 2 ahead/4 behind | no PR recorded | 50 changed files; absent from main: `apps/mobile-relay/nlcl/index.mjs`, `apps/mobile-relay/control-language.mjs`, `apps/vivim-host/control/dispatcher.mjs`, `journal.mjs`; tests and KPI work | **P0 selective integration, pre-merge hardening** |
| `feature/personal-ai-bootstrap` | 2026-10-30 22:19:20 | 8 ahead/455 behind | no PR recorded | 27 changed; missing `studio/src/agents/personal-ai-baseline.mjs`, `harness/src/onboarding/grant.mjs`, `compare-runner.mjs`, `harness/src/providers/kimi.mjs`; offline tests | **P0/P1 selective integration, persistence fixes first** |
| `roundrobin` | 2026-10-30 17:41:34 | 1 ahead/481 behind | no PR | 92-line `harness/src/agents/roundrobin.mjs`; commit explicitly labeled **DRAFT**, two fleet tests pending | **P1 salvage design/engine, not direct merge** |
| `mobile-bootstrap-bridge` | 2026-10-30 23:00:44 | 2 ahead/470 behind | #22 open to main; #23 merged into branch | `harness/src/interface/target.mjs` **SHA-identical** to main; platform/relay files differ due subsequent main work; #22 PR originally 36 files | **Reconcile exact delta; avoid stale wholesale merge** |
| `mobile-relay` | 2026-10-30 22:54:08 | 1 ahead/470 behind | #23 merged to mobile-bootstrap-bridge, not main | Same changed content as parent `mobile-bootstrap-bridge` comparison; Windows target file already main | **Retire after parent integration/equivalence proof** |
| `feature/bootstrap-tab-v1` | 2026-10-30 20:39:37 | 2 ahead/470 behind | #35 open, base `mobile-relay` | Only `apps/mobile-relay/app.js`, `index.html`; localStorage `vivim.bootstrap.v1`, hardcoded four cards, local revisions; main now has `generative-ui/phone-surface.mjs` + Host-backed Build | **UX reference only; reject wholesale merge** |
| `design/m1` | 2026-10-30 18:41:00 | 1 ahead/473 behind | #31 open | `harness/docs/design/m1.md` (2,522 added lines): provider readiness, durable execution, reply completeness; says Gemini excluded from its M1 roster | **Extract/modernize spec, then close** |
| `dev-auto-approve-router` | 2026-10-30 22:14:38 | 1 ahead/455 behind | #39 closed unmerged 2026-10-30 23:16:48; #38 merged 23:16:32 | `harness/src/interface/dev-auto-approve.mjs` exactly the same blob `2baf90e85…` as main; old router/CLI differ | **Superseded; regression compare, then retire** |
| `feature/share-live-feed-20261030` | 2026-10-30 17:53:27 | 8 ahead/480 behind | #20 open draft | Seven absent files under `apps/share-web`, `harness/bin/share-feed.mjs`, publisher; Vercel/Blob/public telemetry | **Defer/archive; no deployment** |
| `handoff-scripts` | 2026-10-30 19:18:48 | 1 ahead/469 behind | no PR | 14 new `scripts/rcm` and `scripts/vd` files; Linux-absolute paths, fixed conversation URLs, direct CDP websocket, process killing | **DO NOT MERGE; quarantine historical only** |
| `gate1-north-star` | 2026-10-30 16:47:35 | 1 ahead/489 behind | related #18 merged from *different* `gate1-goals` branch at 16:55:45 | `GOALS.md`, `harness/docs/USER-JOURNEY.md` delta; historical end-to-end setup requirements | **Recover missing requirements only, no code** |
| `docs/handoff` | 2026-10-30 19:22:19 | 2 ahead/469 behind | #32 merged 19:23:34 | `docs/HANDOFF.md` exists on main (later edited); two older handoff notes, require semantic check | **Merged lineage; verify files then retire** |
| `docs/ux-shotgun-2026-10-30` | 2026-10-30 18:07:57 | 1 ahead/479 behind | #25 merged 18:08:13 | `catalog/research/ux-shotgun-2026-10-30/SYNTHESIS.md` exists on main | **Merged lineage; retire after hash/content check** |
| `handoff-redis` | 2026-10-30 19:35:33 | 1 ahead/468 behind | #34 merged 19:35:58 | Handoff note about Blob suspension/Redis migration later incorporated/edited | **Merged lineage; retire after semantic check** |
| `roadmap-chatv1` | 2026-10-30 18:35:22 | 1 ahead/474 behind | #30 merged 18:35:34 | One old `docs/roadmap.json` change; main has much newer 146-task ledger | **Merged lineage; do not restore old roadmap** |
| `roadmap-models` | 2026-10-30 18:21:20 | 1 ahead/477 behind | #27 merged 18:21:32 | Old `docs/ROADMAP.md` and `roadmap.json` | **Merged lineage; do not restore old roadmap** |
| `roadmap-prefs` | 2026-10-30 18:22:25 | 1 ahead/476 behind | #28 merged 18:22:37 | Old `docs/ROADMAP.md` and `roadmap.json` | **Merged lineage; do not restore old roadmap** |

**Note on evidence:** `main` and all 17 branches are not at equivalent epochs. The timestamp of an old branch does not by itself prove obsolescence; PR merge status is insufficient under squash merges. Presence of a file on `main` does not establish behavior equivalence: compare exact blob SHA, entrypoint wiring and tests separately.

### 2. Exact content comparisons verified

- `dev-auto-approve-router:harness/src/interface/dev-auto-approve.mjs` and `main` both have Git blob `2baf90e85…`: **identical implementation**.
- `mobile-bootstrap-bridge:harness/src/interface/target.mjs` and `main` both have Git blob `130041c59…`: **identical Windows attach-target implementation**.
- `mobile-bootstrap-bridge:studio/src/config.mjs` differs from main; main has 226 lines versus branch 195 and adds completed workspace-manifest resolution and related state. Never overwrite main with the older file.
- `mobile-bootstrap-bridge:studio/bin/commands/relay.mjs` differs; main has 99 lines versus branch 79 and includes worker heartbeat integration. Never overwrite main.
- `feature/bootstrap-tab-v1:apps/mobile-relay/app.js` differs (branch 406 lines, main 464). Main imports `transport` and mounts `generative-ui/phone-surface.mjs`; branch uses localStorage-only Bootstrap activation.
- `handoff-redis:docs/HANDOFF.md` differs (branch 213 lines, main 229), consistent with later edits, not proof the earlier work is missing.
- Main has `apps/vivim-host/control/intent-planner.mjs` with strict read/plan-only policy; UTC branch rewires it to the vendored parser and Host dispatcher. Main lacks branch dispatcher and NCLL engine.

## ERA 2 — Code-level autopsy and required repairs

### UTC/NLCL: `feat/utc-nlcl-language-core` — RECOVER, WITH SAFETY GATES

**Traceable commits:** [`7997ce096`](https://github.com/owenservera/vivim-web-harness/commit/7997ce096) (language + Host action executor, 2026-10-31 23:32:16 UTC); [`7a357a54d`](https://github.com/owenservera/vivim-web-harness/commit/7a357a54d) (KPI evidence and scoring, 23:52:04 UTC).

**Inspect/port in this dependency order:**

1. Vendored language parser: `apps/mobile-relay/nlcl/{index,types,lexer,grammar,frames,ground,interpret,recognize,project,symbols,text}.mjs`; `PROVENANCE.json`; provenance points to `owenservera/vivim-omega` source commit `79ee558d…`. Confirm the regeneration command/source and hash against actual files; do not edit vendored files by hand.
2. Pure adapter: `apps/mobile-relay/control-language.mjs` builds a WorldModel from catalog verbs, scopes and lexicon, interprets text to typed candidate; confirms `!` force modifier grants no privileges. Add determinism/ambiguity and injection tests.
3. Planner: `apps/vivim-host/control/intent-planner.mjs` replaces the current preview-only planner with action validation and `planTyped`, `planText`, `planCandidate`. Preserve all current catalog actions/semantics.
4. Host authority: `apps/vivim-host/control/{dispatcher,journal,lexicon,http-status}.mjs`; `apps/vivim-host/control/action-catalog.v1.json`. One immutable plan digest, unique plan ID, independent authorized execution and receipts.
5. Transport/desktop: `apps/vivim-host/server.mjs`, `direct-phone.mjs`; phone only via paired Direct transport and exact trusted identity, desktop via loopback owner token. Integrate with newer APK07 routes and never regress their authorization.
6. UI: `apps/mobile-relay/control-phone.mjs`, `apps/mobile-relay/index.html`, `app.js`, Direct static asset map. Main presently has a visible Control nav entry but `renderTabs()` does **not** set `#v-control.hidden` based on `S.tab`; hence the tab is not correctly wired. Main's `control-phone.mjs` remains explicitly preview-only. Port the new interactive planner into the CURRENT main UI, not old UI wholesale; remove stale preview disclaimers only after real execution acceptance.
7. Tests: `apps/vivim-host/test/control-{language,dispatcher,http,intent-planner}.test.mjs`, `kpi-utc.test.mjs`, tools/KPI sources if present; do not regenerate indexes until code/tests converge.

**Specific defects and risk controls observed:**

- `control/dispatcher.mjs::execute()` takes `approval.channel` as a string. The desktop and Direct HTTP routes *do* inject it after authentication; keep that boundary unforgeable and do not expose `execute()` as an unguarded RPC or map caller-provided approval fields. Bind actual authenticated principal/device identity and the exact proposed plan.
- `control/journal.mjs::approve()` detects only a digest conflict if an approval record already exists; it does not reconcile `channel` and `actor`. A replay through a different authenticated surface can produce an `approvedVia` in the receipt that does not match the stored approval. Require approval-record equality, consistent attribution and conflict tests.
- `control/dispatcher.mjs` catches an executor throw and labels it `failed` on the assumption that every Host write is atomic. Verify that every executor is transactional or idempotent under partial failure (notably `setHardNo`, `clearHardNo`, theme/undo and lexicon). Otherwise write `uncertain` after a claimed effect; never falsely report 'nothing changed.'
- `control/lexicon.mjs::save()` uses load/version check then a rename, which is **not** atomic compare-and-swap across multiple processes. Two writers can both read version N and clobber each other. Introduce an exclusive lock/transaction or enforce a demonstrably single-writer authority; test racing writers and recovery.
- `control/journal.mjs` uses exclusive hard-link publish for plans, `wx` for claim and append-only events. Exercise Windows NTFS semantics, simulated crash after claim, concurrent approvals, receipt replay and power loss; don't rely only on Unix tests.
- `direct-phone.mjs` trusts local Tailscale Serve proxy headers (`x-forwarded-proto`, `tailscale-user-login`) after loopback address/host checks. Verify local backend cannot be reached through an untrusted header-forging route; ensure Tailscale sanitizes/overwrites these headers; reject direct client forgery at perimeter.
- Branch's `control-phone.mjs` imports `control-language.mjs` and its nested NCLL files. The CURRENT main Direct asset allowlist serves `control-phone.mjs` but not the language modules. The branch's HTTP test checks only four module assets. Ensure **every imported module** is whitelisted and smoke-test real ES-module loading on device.
- Current main Control view (`index.html`) promises preview-only; branch implements mutations with receipts. UI wording, status semantics and action visibility must be synchronized when integration lands.
- Tests construct authenticated HTTP requests and confirm primary paths, but that is **not** live Android/Tailscale acceptance or proof of privacy guarantees. Explicitly test R0/R1/R2/R3 behavior, changed catalogs, expired plans, replay, wrong identity, two devices, stale lexicon, and network reconnection.
- AI-generated typed candidates must remain data and never be an execution authority; `!` must never skip approval. Maintain strict catalog/readiness and owner-approved Host dispatch.

**Completion gate:** Tests + lint/static import validation pass; `CONTROL_UNCERTAIN` cannot resend; every write has durable attribution; every Direct module loads; a real device executes a harmless theme change and undo; privacy control stays guarded; no regression to provider send-once.

### Personal AI/bootstrap: `feature/personal-ai-bootstrap` — RECOVER WITH JOURNAL CORRECTION

**History:** seven feature commits between 2026-10-30 22:18:53 and 22:19:00 UTC; branch head 22:19:20 UTC. The SHA sequence includes `99e496bae` (global status), `ae756c491` (baseline parser), `bce04d183` (unverified Kimi), `c09125e84` (grants/comparison runner), `2c27ff58e` (capability catalog).

**Preserve these separable assets:**

- `studio/src/agents/personal-ai-contracts.mjs` + status route/service: read-only `vivim.personal-ai/1` global status, explicit uninitialized state, no automatic provider login or fake verified readiness.
- `studio/src/agents/personal-ai-baseline.mjs`: deterministic prompt constructor, bounded nine-section JSON response, rejection codes, refusal of model-asserted approval/provenance fields; test file `studio/test/unit/personal-ai-baseline.test.mjs` (520 lines). Map its nine sections onto CURRENT F01/F02/D03–D07 and the primary Personal World contract; do not create a competing global identity database.
- `harness/src/onboarding/grant.mjs`: owner-confirmed, provider-scoped, fresh-thread-only grant, SHA-256 prompt digest, expiry and exclusive per-send slots. Combine with current Host consent/approval rather than instituting a second disconnected authority.
- `harness/src/onboarding/compare-runner.mjs`: provider comparison orchestration and metadata-only `results.json`; keep reply text separate from event metadata. Redesign its durable execution before enabling it.
- `harness/src/providers/kimi.mjs`: status explicitly `seeded-unverified`, threadPattern null, generic fallback selectors. **Do not enable live sends** until DOM selectors, login, streaming, model picker and thread URL have live evidence.
- `scripts/capability-catalog.mjs` and `catalog/harness-capabilities.json/.csv`: imported **246 unverified CAP claims**, not evidence. Reconcile with current atomic registry, source-symbol and capability-value system; do not establish a second canonical capability ledger or overwrite newer generated indexes.

**Specific crash/approval concerns in `compare-runner.mjs`:**

1. `results` begins as an empty in-memory array on each invocation, then `results.json` is rewritten. A process dying after `router.ask()` but before `finish()/flush()` can leave a submitted request missing from the comparison ledger. Starting over allocates a **new request ID** and may submit the same provider/prompt again if grant slots remain. Its 'never resends' comment only applies to a single attempt, **not durable restart semantics**. This must be fixed before integration.
2. The program consumes the grant slot before `router.ask()` and before a durable comparison job record. Persist `(runId, provider, promptDigest, requestId, state)` with exclusive reservation and `uncertain` reconciliation before any send, then read provider journal on resume; never re-ask an uncertain request.
3. Concurrent `issue()`/`revoke()` and `consume()` need transactional or explicit single-writer behavior. In `grant.mjs`, `gate()` checks the current grant and `consume()` later creates a slot; revocation after check can race slot creation. Validate owner auth at the issuance boundary, not just a caller-supplied `confirm` callback.
4. `assertSaveDirAllowed()` rejects lexical `.git`, `docs`, `catalog` path segments and walks `.git` ancestors. It is not a general symlink/reparse-point containment guarantee; use canonical real paths, ownership checks and platform-safe write targets.
5. A failed `ask()` still consumes a grant, which may be a valid conservative policy; expose it transparently in grant receipts rather than silently replenishing.

**Completion gate:** no replay after crash or restart; grant cannot be forged or used after revocation/expiry; provider-bound request journal survives restarts; `kimi` remains unverified; baseline output does not activate capabilities; tests integrate with current primary agent + Host status, not a parallel truth store.

### Multi-agent orchestration: `roundrobin` — SALVAGE ALGORITHM, NOT BRANCH

`harness/src/agents/roundrobin.mjs` (92 lines) adds seeded member order, 1–6 rounds, 2–6 members, 12-send cap and persisted turns. It is **not** equivalent to choosing different models by round-robin routing; it is a multi-turn council.

**Defects to address:**

- In `runRoundRobin()`, a saved turn with `requestId` calls `journal(requestId)`. When it returns `null`, the code sets `rec = null` and decides resending is safe. `null` is missing evidence, **not proof the prompt was never sent**. Fail closed with `uncertain`, unless the journal provides a signed authoritative `never_submitted` result.
- `state.threads` is keyed by model-reference string, but `planRoundRobin()` does not reject duplicate model references. Two logical members with the same reference would reuse the same thread. Assign unique agent identities, keep model identity separate from thread key.
- The latest turn gets every prior complete response in the prompt; no bounded transcript/context budget. Add deterministic summarization or explicit truncation with receipt/omission accounting, avoiding silent loss.
- Branch adds `DEFAULT_COORDINATOR` and a mandatory coordinator role to fleet/swarm. This is a behavioral redesign and can change costs and contracts. Do **not** make coordinator mandatory solely as a side effect of porting the council pattern.
- Commit explicitly states two fleet tests pending. Add plan invariants, restart, negative journal, duplicate model, timeout, no-resend, same-thread and budget tests before wiring CLI.

**Suggested integration:** pure `roundrobin` strategy behind existing coordinator/router; use existing Host-owned model authorization and send-once journal; allow current fleet/swarm designs to remain unchanged until integration tests pass.

### Mobile/bridge PR lineage: `mobile-bootstrap-bridge`, `mobile-relay`, Bootstrap tab

**Historical chain:** #23 was merged into `mobile-bootstrap-bridge` 2026-10-30 23:00:44 UTC; #22 remains open to main. #35 was branched against `mobile-relay` and remains open; merging #35 cannot put it directly on main.

**Current main already has:** `apps/mobile-relay/lib/relay-core.mjs`, `redis-store.mjs`, mobile Talk UI, `apps/vivim-host/direct-phone.mjs`, APK07-related paths, `apps/mobile-relay/generative-ui/{catalog,compiler,renderer,phone-surface}.mjs`, a Host revision/projection transport, Windows attach target and newer Studio workspace manifest/heartbeat logic.

**Non-regression rules:**

- Preserve `main:harness/src/interface/target.mjs` (hash-identical to the branch); don't duplicate Windows path logic.
- Preserve `main:studio/src/config.mjs` workspace-manifest resolution and `main:studio/bin/commands/relay.mjs` heartbeat. A wholesale #22 merge can regress both.
- The old Build/Bootstrap tab's `localStorage` revision and 'Active' cards are not actual Host capability activation. Compare its small UX gestures against the current Host-backed Build compiler and action/proposal contracts; port desirable copy/gesture behavior only.
- Confirm exact `#22` bridge command routes, durable claims, tunnel host allowlist, and `studio/android/build.sh` against current Direct/API/APK07 infrastructure; eliminate duplicated transport endpoints. The tunnel allowance must not weaken current private Direct-only control semantics.
- Treat #23 as already delivered into its parent branch, not independently unmerged main content. Close #35 only after feature coverage and phone acceptance, not because it targets an old base.

### M1 design: `design/m1`

`harness/docs/design/m1.md` has detailed readiness/settlement invariants (read-only sign-in probing, per-provider timeouts, exact thread binding, never resend uncertain requests, no credential logging) and no executable implementation. Preserve valuable acceptance cases, but the document explicitly excludes Gemini from its M1 roster; current product roadmap requires a wider provider matrix. Reconcile all provider counts, naming, model/effort behavior and current readiness infrastructure; label obsolete assumptions. Do not blindly declare 2,522 lines canonical.

### Share feed: `feature/share-live-feed-20261030`

A separate public Vercel/Blob feed: `apps/share-web/api/feed.mjs`, `harness/bin/share-feed.mjs`, `harness/src/share/feed-publisher.mjs`. Public events intentionally strip prompts, replies, URLs and personal identifiers; it is **not part of the current Direct phone acceptance path**. Keep PR #20 draft/deferred and do not deploy or turn on external publication.

**Source observations:** `share-feed.mjs` updates its local cursor only after remote POSTs, so an interrupted run after publish but before cursor rename may publish a duplicate on restart; make publish idempotent if revived. Feed listing uses a fixed 1,000-object listing without pagination; validate scale/retention. `api/feed.mjs` derives ISO time before checking its validity, so invalid input can generate a 500 instead of a clean 400. Public telemetry ingress needs independent data-governance approval.

### Old handoff scripts: `handoff-scripts` — QUARANTINE

`scripts/rcm/ask.mjs` hardcodes `/workspace/vivim-web-harness`, directly creates the router and approves sends, bypassing modern operational coordination. `scripts/rcm/readgpt.mjs` connects directly to `127.0.0.1:9234/json` and a hardcoded conversation. `scripts/vd/turn.sh` hardcodes a conversation/profile, sends via CLI, scrapes logs, then uses `pkill -f`. This violates the one-owner-browser/no-parallel-controller rule, is Linux-specific and risks acting on the wrong user thread. **Never merge or execute these scripts.** Extract only any missing diagnostic intent and reimplement with current CLI and scoped/serialized owner.

### Documentation and roadmap branches

The PR records confirm #25, #27, #28, #30, #32 and #34 merged on 2026-10-30. Their branch-tip SHA divergence is expected when GitHub squash-merges or later edits occur. Verify their unique files/content by PR patch vs later `main`, **not** raw branch-tip SHA or restoring stale `roadmap.json`. `gate1-north-star` is distinct from `gate1-goals` (#18) and should be checked for unique user-journey/installer assumptions. Close/retire only after an evidence-backed semantic checklist and restore SHA.

## ERA 3 — Execution directive to Git manager / local integration agent

### A. Operating contract

1. Use `main` as canonical. Fetch and snapshot all remote refs, PRs, SHAs and timestamps. Record an immutable restore map BEFORE any branch deletion; no force pushes. Do not replace a file with a version merely because it is newer/older by wall-clock.
2. On Windows, audit all active worktrees, detached processes, local untracked changes, and owner of Chrome/Studio before editing. Do not start a second browser controller, delete profiles, interact with personal sessions, or resend uncertain prompts. Keep all changes local/offline until a supported test explicitly authorizes a live effect.
3. One implementation owner per shared file. Keep the existing roadmap (`docs/roadmap.json`) the only task-status ledger, and keep generated indexes derivative. Mark source-only, offline-test and live acceptance separately.
4. No wholesale merge of PR #22, #35, UTC, Personal AI bootstrap, roundrobin or #39. Use file-level/commit-level porting with current-main regression tests, followed by full test suite.
5. No Vercel deployment, no public sharing, no new privileged cloud action; current Direct/Host/phone path has priority.

### B. Ordered lanes, dependencies and deliverables

**Lane 0 — forensic baseline (must precede mutations)**

- Save `main SHA`, `git status --porcelain=v1`, all worktrees, remote branch SHAs, all PR states/bases/merge dates; document `git merge-base` and both directional diffs for each divergent branch. Inspect every unique commit's tree, message, and author/committer dates.
- Produce `docs/library/BRANCH-FATE-MATRIX-2026-11-01.md`: 17 rows, current SHA, source symbol, current equivalent, port/don't-port, tests, owner, and restore SHA.
- Correct PR #40's discrepancy: its document claims 21 refs were deleted, but GitHub's branch API still showed those refs during this audit. Do not claim deletion without post-delete API proof.

**Lane 1 — UTC/NLCL, highest functionality priority**

- Port pure vendored parser and provenance, then catalog/planner, then transactional Host dispatcher/lexicon/journal, then desktop/Direct routes, then current mobile UI. Preserve latest main Direct APK routes and security.
- Fix authorization attribution, lexicon concurrency and partial-effect failure classification *before* exposing execution.
- Cross-platform unit + integration and device acceptance; no silent resends and no invented readiness.

**Lane 2 — Personal AI baseline and durable comparison**

- Extract pure baseline parser and read-only primary-status contract. Map to current F01/F02/D03–D07 and primary agent identity, maintaining provenance and approval separation.
- Implement a durable per-provider onboarding-run ledger before porting comparison runner; consume scoped grants via existing Host authority. Handle crash-before-send, crash-after-send, restart, grant revoke/expiry and duplicate providers.
- Keep Kimi seeded/unverified until live adapter measurement. Reconcile imported 246 CAP rows into existing atomic registry as unverified evidence only.

**Lane 3 — mobile bridge/Build UX**

- Inventory #22/#23 actual features against latest `main`; mark same-hash/semantically equivalent/newer/absent. Port only missing bridge or Android features and tests. Never overwrite workspace manifest, heartbeat, Direct enrollment or latest GenUI.
- Treat #35 as UX reference; ensure Build tab state is Host-owned, versioned, reversible and accepted on phone. Don't import `localStorage` as capability authority.

**Lane 4 — multi-agent patterns**

- Port `roundrobin` only as optional council strategy after its journal null behavior, duplicate-member thread keys, context budgets and tests are fixed. Do not enforce mandatory coordinator role in existing fleet/swarm without product decision.

**Lane 5 — design/doc reconciliation**

- Mine #31 for M1 acceptance requirements, reconcile Gemini/provider roster and current architecture. Extract unique Gate 1 objectives from `gate1-north-star`. Preserve current roadmap statuses.
- Retire merged documentation branch refs only after content-proof. Close #39 as superseded after full wiring equivalence. Keep #20 deferred/draft; quarantine handoff scripts.

### C. Test and acceptance matrix

| Gate | Required proof |
|---|---|
| Repository | `git status --short` clean after authorized changes; no untracked secrets; compare tree and PR restore map |
| Static integration | `npm run check:integration`; import graph resolves, including all browser NCLL assets |
| Offline suites | `npm run test:fast` (affected tests via `test/run.mjs --base main`), `npm run test:all` (all suites); host, studio, mobile and harness unit tests; run new targeted test files explicitly |
| Derived documentation | `npm run check:docs`, `npm run check:registry`, `npm run check:value-map`, `npm run check:work-queue`, `npm run check:skills` |
| UTC concurrency | parallel approval/claim, reboot after claim, lexicon lost-update test, expired/stale plan, failed effect, R0/R1/R2/R3, wrong identity/origin |
| Bootstrap durability | crash at every state transition, grant revoke race, exact provider+prompt+request bindings; no re-ask after uncertain |
| Browser/Host safety | one Chrome owner; no profile deletion; no second CDP controller; no send-once or no-resend regression |
| Phone integration | physical Android Direct connection; Control tab visible; all ES modules served; harmless approved theme apply/undo; receipt and status after reconnect |
| APK07 regression | enrollment, revocation, offline shell, Android back stack, existing build/QR/export routes remain functional |
| Documentation | no claim of `verified` without real corresponding receipt; no duplicated status authority; generated indexes converge |

### D. Safe first-pass Windows commands (non-mutating baseline)

```powershell
Set-Location 'C:\0-BlackBoxProject-0\vivim-web-harness'
git status --porcelain=v1
git branch -avv
git worktree list --porcelain
git remote -v
git log --all --date=iso-strict --pretty=format:'%h %cI %s' -n 80
# After the source snapshot is recorded and only if authorized to refresh remote refs:
git fetch --all --prune
# Example path-specific history / branch-vs-main evidence:
git log main..origin/feat/utc-nlcl-language-core --format='%H %aI %cI %s'
git diff --name-status main...origin/feat/utc-nlcl-language-core
git show origin/feat/utc-nlcl-language-core:apps/vivim-host/control/dispatcher.mjs
```

**Important:** `git diff main...branch` examines changes since common base, not semantic replacement equivalence. Check the `main` version of each file and run tests before porting.

### E. Definition of done

- Every divergent branch has a documented, testable fate: `ported`, `equivalent`, `deferred`, `quarantined`, or `retained-with-owner`, with exact evidence and restore SHA.
- No missing unique functionality silently discarded; no older file overwrites newer working systems.
- All feature ports integrated into latest main, with tests and receipts; no speculative live-accepted labels.
- Historical merged branches can be removed only after explicit post-merge content and SHA inventory checks. Unmerged functionality cannot be deleted until its recoverability is proven.
- PR #40 and `GIT.md` reflect the ACTUAL remote refs and worktree state, not intent.

### F. Minimal issue queue

| ID | Severity | Action |
|---|---|---|
| BR-UTC-01 | P0 | Host/phone authority contract and Direct module bundling; correct current Control tab view wiring |
| BR-UTC-02 | P0 | Fix journal attribution, partial-effect uncertainty and lexicon multiwriter loss |
| BR-PAI-01 | P0 | Durable per-provider run ledger before onboard compare; prevent restart resend |
| BR-PAI-02 | P1 | Baseline/primary-agent contract integration and Kimi verification gate |
| BR-MOB-01 | P1 | Compare #22/#23 against main, port only absent, retain manifest/heartbeat/APK07 |
| BR-MOB-02 | P1 | Extract #35 gestures into current Host-backed Build; no localStorage authority |
| BR-RR-01 | P1 | Treat journal-null as uncertain, unique members and context-budget tests |
| BR-DOC-01 | P1 | M1 and Gate 1 spec extraction; provider roster reconciliation |
| BR-GIT-01 | P1 | Correct #40 remote deletion mismatch and maintain recoverable ref ledger |
| BR-SEC-01 | P1 | Quarantine scripts/rcm and scripts/vd, prohibit multi-controller execution |
| BR-SHARE-01 | Deferred | Keep public share feed inactive; review idempotency and data policy before revival |

## Appendix — Source browser links and Windows paths

Each GitHub link selects the relevant **branch version**, not the current main file. The Windows path represents where the file belongs after a reviewed port; run `git show origin/<branch>:<path>` to inspect a branch file without switching the active checkout.

| Branch file on GitHub | Relative path under `C:\0-BlackBoxProject-0\vivim-web-harness` |
|---|---|
| [UTC dispatcher](https://github.com/owenservera/vivim-web-harness/blob/feat/utc-nlcl-language-core/apps/vivim-host/control/dispatcher.mjs) | `apps\vivim-host\control\dispatcher.mjs` |
| [UTC journal](https://github.com/owenservera/vivim-web-harness/blob/feat/utc-nlcl-language-core/apps/vivim-host/control/journal.mjs) | `apps\vivim-host\control\journal.mjs` |
| [UTC language adapter](https://github.com/owenservera/vivim-web-harness/blob/feat/utc-nlcl-language-core/apps/mobile-relay/control-language.mjs) | `apps\mobile-relay\control-language.mjs` |
| [UTC mobile UI](https://github.com/owenservera/vivim-web-harness/blob/feat/utc-nlcl-language-core/apps/mobile-relay/control-phone.mjs) | `apps\mobile-relay\control-phone.mjs` |
| [UTC Direct route](https://github.com/owenservera/vivim-web-harness/blob/feat/utc-nlcl-language-core/apps/vivim-host/direct-phone.mjs) | `apps\vivim-host\direct-phone.mjs` |
| [Bootstrap baseline parser](https://github.com/owenservera/vivim-web-harness/blob/feature/personal-ai-bootstrap/studio/src/agents/personal-ai-baseline.mjs) | `studio\src\agents\personal-ai-baseline.mjs` |
| [Onboarding grant](https://github.com/owenservera/vivim-web-harness/blob/feature/personal-ai-bootstrap/harness/src/onboarding/grant.mjs) | `harness\src\onboarding\grant.mjs` |
| [Comparison runner](https://github.com/owenservera/vivim-web-harness/blob/feature/personal-ai-bootstrap/harness/src/onboarding/compare-runner.mjs) | `harness\src\onboarding\compare-runner.mjs` |
| [Round-robin council](https://github.com/owenservera/vivim-web-harness/blob/roundrobin/harness/src/agents/roundrobin.mjs) | `harness\src\agents\roundrobin.mjs` |
| [M1 design](https://github.com/owenservera/vivim-web-harness/blob/design/m1/harness/docs/design/m1.md) | `harness\docs\design\m1.md` |
| [Old Bootstrap UI](https://github.com/owenservera/vivim-web-harness/blob/feature/bootstrap-tab-v1/apps/mobile-relay/app.js) | `apps\mobile-relay\app.js` |
| [Unsafe historical ask script](https://github.com/owenservera/vivim-web-harness/blob/handoff-scripts/scripts/rcm/ask.mjs) | `scripts\rcm\ask.mjs` (quarantine only) |
| [Share API](https://github.com/owenservera/vivim-web-harness/blob/feature/share-live-feed-20261030/apps/share-web/api/feed.mjs) | `apps\share-web\api\feed.mjs` (deferred) |

---

**Forensic conclusion:** Do **not** blanket-merge or blanket-delete. Reconcile by source symbols and behavior: recover UTC/NLCL and Personal AI carefully; preserve the current main Host/Build/APK work; treat round-robin as an unverified council strategy; quarantine hardcoded CDP scripts; retain a separate evidence-based tombstone ledger for merged docs; make all retirement actions reversible. No live runtime claims can be made from this remote inspection alone.
