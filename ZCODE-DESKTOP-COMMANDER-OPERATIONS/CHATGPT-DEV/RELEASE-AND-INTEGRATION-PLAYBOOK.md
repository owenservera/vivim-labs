# ChatGPT Dev — source-to-live closed-loop playbook
**Revision:** 2026-11-21. **Authority:** [cloud-first policy](../CLOUD-FIRST-CODING-POLICY.md) and owner instructions override examples here.

This is an operational coding and validation aid, **not a claim that a session or device is connected**. Use it for OW-007 / OW-009 / OW-014 / OW-017 and analogous Rust/Host/PWA deliveries. The goal is one working slice and one truthful release receipt, not more orchestration.

## 1. Three independently verifiable outcomes

1. **Human text -> AutoWiki:** type in the **first-contact PWA** composer; after the existing debounce, the same authorized Wiki read surface updates **without Send**. Do not conflate this with the older Wiki tab's explicit Search/Enter.
2. **Agent draft -> same AutoWiki:** a Host-authorized agent proposes/stages a draft through the existing composer bridge; phone input updates with a source revision / grant fence and ACK; the same AutoWiki reacts. No synthetic touch, implied provider Send, or UI-only ACK.
3. **Build -> stable phone link:** deploy one verified product commit through a bounded local executor *only after* ChatGPT's cloud GitHub source push. Confirm live served commit, HTTP assets, Host auth behavior, HTTPS/TLS, and separately the **actual phone screen**. Each result has its own status.

## 2. Start coding, not rebuilding

- Read [AGENTS](AGENTS.md), [SITREP](SITREP.md), [delivery contract](DELIVERY-CONTRACT.md), and only the task's current [Planning bundle index](../DEVOPS-PLANNING/BUNDLE-INDEX.md) pointer. A Labs local-only path in a historical SITREP may not exist in GitHub remote; label **REMOTE_SOURCE_UNAVAILABLE** instead of inferring content.
- Refresh **both** `owenservera/ARBOR/main` and `owenservera/vivim-labs/main` before a write. Locate current implementation and tests: `core/programmability/src/{agent_composer,live_system,activity_fabric,wiki_read,product_kernel}.rs`, `apps/vivim-host/control/agent-composer-bridge.mjs`, `apps/mobile-relay/{app.js,first-contact-composer.ts,agent-composer-adapter.ts,sw.js}`. These are *navigation hints*, never proof the files still exist or are wired.
- Classify every requested feature **REUSE / REPAIR / NEW / UNKNOWN**, and identify active owner and touched paths. If Planning staged Rust modules while Dev was integrating related code, diff actual GitHub main first. **Do not copy blindly, parallel-build a second graph, journal, authority router or composer.**
- Work in ChatGPT's cloud/container using direct GitHub tools. Rust default; TypeScript 7+ only where browser/runtime glue demands it. Narrow non-force writes directly to `main` after fresh file SHA checks. Re-fetch final paths/commit. A pushed Labs pre-code bundle is not an ARBOR implementation.
- If cloud compilation is unavailable, mark `CLOUD_TEST_UNRUN` precisely. Local building after push is a **release validation step**, not an excuse to locally rewrite product code or hand coding ownership to ZCode.

## 3. Integration trust fences — concrete regression targets

For human and agent typing:
- Latest input/session wins; old epoch, revoked grant, stale source revision, wrong scope or out-of-order response must not mutate the textarea or private Wiki.
- Preserve caret, focus, IME composition, text selection, accessibility, existing keyboard semantics and user edits during streaming or polling.
- An agent-visible activity indicator may be shown **only after** Host verified source/receipt; it is neither a synthetic touch event nor an execution receipt.
- Browser activity capture or telemetry must have explicit scope, approved ingress, bounded retention and Host-controlled admission. Don't wire staged observer code to arbitrary global events, persist raw private drafts, or assert consent from an inert Rust model.
- Pure Rust Activity Fabric operations are computations, **not** authorization, retention, ingestion, consent, Host effect execution, or a deployed Wiki display. Do not equate a passing `activity_fabric` unit test with live activity tracking.

## 4. Cloud implementation -> bounded local build

**Before calling ZCode/Remote Desktop Commander:** GitHub source SHA and required source changes must already be committed, and local use must be authorized for workstation-specific build, launch, packaging or device inspection. Explicitly scope the executor: no source edits, no `git reset/stash/checkout` on dirty owner checkout, no commits, no agent fanout, no model/provider reconfiguration.

A previous session exposed the following failure modes; treat these as conditional checks, not timeless machine facts:

| Observed failure | Reliable practice |
| --- | --- |
| Local ARBOR checkout was 134 commits behind and dirty | Fetch remote ref **without modifying** dirty checkout. Build a SHA-pinned immutable release archive in a separate location. Do not overwrite developer work. |
| ZCode free OpenRouter provider returned HTTP 429 before executing | Stop ZCode attempt; report model/provider error. If owner authorized local build, a **ChatGPT-authored script** may be executed by an appropriate local tool; that tool still does not write app source. Never change credentials/budgets to force progress. |
| Broken Bun command shim | Verify the actual executable path/version, not an alias; use a frozen lockfile. |
| Host HTTP composer negative test used `assert.rejects` for a synchronous throw | Verify sync vs async contract; use `assert.throws` for sync refusal and preserve the negative assertion. The test, not production authority, was wrong in that case. |
| Native Rust binary compiled under custom `CARGO_TARGET_DIR`; Host HTTP test returned `LIVE_SYSTEM_NATIVE_UNAVAILABLE` (503) | Build the binary and **package it into the exact path resolved by the Host** before testing/starting. Inspect Host resolver source; do not treat a successful Cargo build as evidence the Host can invoke it. |
| Host health probe omitted required same-origin `Origin` header | Match the real request contract. A protected endpoint returning **401** to an otherwise valid unauthenticated probe can prove route/auth fencing; a 403 or other failure must be interpreted from actual source, not assumed. |
| Tailscale hostname did not resolve on Windows despite Serve/TLS working | Check Tailscale state, Serve mapping, DNS, HTTPS and TLS separately. A diagnostic `curl --resolve` can establish TLS/routing **on the tested PC**, but not Android DNS/reachability. |
| New Host health check failed and rollback was invoked | A safe switch must validate old PID/path, build gates and new PID/asset/auth, preserve rollback command/data, and report both attempts. Never leave two Host instances contending for the same listening port. |

**Suggested build gates** (verify current manifests/scripts before running):
- `cargo test -p vivim-programmability-core agent_composer`
- `cargo test -p vivim-programmability-core activity_fabric`
- `cargo build -p vivim-programmability-core --bin vivim-product-pure`
- `node tools/packaging/build-product-wasm.mjs`
- `node --check apps/mobile-relay/app.js` and `node --check apps/mobile-relay/sw.js`
- `node --experimental-strip-types --test apps/mobile-relay/test/agent-composer-browser.test.mjs`
- `node --experimental-strip-types --test apps/vivim-host/test/agent-composer-bridge.test.mjs`
- `node --experimental-strip-types --test apps/vivim-host/test/agent-composer-http-roundtrip.test.mjs`

These were reported passing on a historical ARBOR release `95089a3fae7534728bce3006efadeea78c209acb`; **rerun against the exact new SHA**. Build commands must not silently start unrestricted GitHub Actions.

## 5. Closed-loop acceptance (no false green)

Record evidence for every step:

| Gate | What counts | What does not count |
| --- | --- | --- |
| **SOURCE** | Fresh remote GitHub SHA and inspected implementation path | Local-only untracked path or stale plan |
| **TEST** | Actual command, environment, exit code, counts / failure text | Authored tests, type hints, static count, or previous SHA |
| **PACKAGE** | Exact native binary and WASM assets in release-resolved paths | Cargo compiled somewhere else |
| **SERVE** | Listener PID -> release commit and requested assets HTTP 200 | Start command issued without listener/health readback |
| **AUTH/ACK** | Valid request fencing, grant, revision and Host-confirmed source receipt | Visual dot or optimistic client success |
| **HTTPS** | Same stable host routing and certificate verification | Loopback 200 alone |
| **PHONE** | Actual Android screenshot/observable typing -> Wiki repaint, on-device source/receipt, non-Send | Desktop `curl`, mocked phone HTTP, Android absent |
| **ROLLBACK** | Old Host restart/readback after failed new release health | Log saying rollback requested |

Report **CODED**, **PUSHED**, **DEPLOYED**, **PHONE_VERIFIED** separately, even if the test suite is entirely green. No retitling a historical green test as current live proof.

## 6. Collaboration without overlapping work

- Planning prepares targeted source disposition, Rust candidates/tests and exact handoff, but Dev owns product integration. Oracle audits ownership and evidence, not product code.
- Role docs and GitHub commits are **asynchronous artifacts**, not evidence that another ChatGPT session has read them. If no acknowledgment exists, mark `HANDOFF_ACK_UNOBSERVED`.
- Keep one OW registry **only when accessible**; if remote GitHub lacks local `OWNER-WORK`, do not invent a second tracker or claim the local receipt is pushed. The Dev role's SITREP is a pointer, never the sole source of truth.
- On failures, identify the **smallest responsible layer** (source/test/ABI/package/origin/Host/Serve/phone), fix cloud application code yourself if required, push new SHA, then retest the changed layer and dependent release gates. No repeated full architecture redesign for one broken health probe.

## Historical session learning receipt (not live status)
On 2026-11-21, an ARBOR source release at `95089a3` reported 4/4 agent composer, 20/20 Rust Activity Fabric, 7/7 browser composer, 5/5 Host bridge and 1/1 Host HTTP round-trip tests passing; the local deployment probe eventually passed and served PWA assets. This was **not** a real Android agent-typing proof. Some deployment receipts were written to a **local operations folder not visible in Labs GitHub main**; treat such evidence as session-reported until explicitly retrieved, not as a canonical remotely readable file.
