# Proposal 5 / 05 — Full Beta Release Contract

**Beta is an integrated, internally self-consistent working product, not a collection of proofs.** The release has one Windows identity and one paired PWA, one Host effect writer, one native graph engine implementation, internal HQ intelligence and internal TierFlow views, with explicit unavailable/research status for functions not accepted.

## A. Required user journeys

| ID | Actual user journey | Minimum observed beta acceptance | Not an acceptable substitute |
| --- | --- | --- | --- |
| B01 | Fresh Windows install and Host launch | Installer starts the **in-repo** native Tauri+JS Host, native graph and UI; closes/restarts safely; no donor processes | CLI tests passing in donor repo |
| B02 | First provider connection | User signs into supported provider website using dedicated Chrome profile; Host detects valid session, preserves user credentials locally | Provider API test or fake UI |
| B03 | Conversation send/receive | At least one supported signed-in web provider receives an exact user-confirmed message; response is displayed with source/target thread evidence, no blind resend | Synthetic DOM selector test |
| B04 | Paired PWA | Phone pairs/revokes with current Host, can read permitted source state and submit a confirmed request with device binding | Offline mock screen |
| B05 | Native Node 0 composer | Editor/IME remains responsive; typed events produce source-backed read-only candidate and versioned trace; Tab never grants effect; stale hints cannot overwrite | R001 fixture `full` alone |
| B06 | Host command and safety | Safe current catalog read works, risky action stays approval-gated; Host journal claim/receipt survives forced process stop; uncertain action not resent | Red/green status in UI without receipt |
| B07 | Internal HQ developer intelligence | User selects a local project; imported HQ code indexes/searches, emits source refs and context/wiki read projection **inside ARBOR**, native HQKG processes authorized Development graph evidence | Calling `../repos-hq/hq/bin/hq.mjs` |
| B08 | Personal World + DCB source | The current verified personal read source can be inspected and context previewed with correct consent/limitations; private source revocation invalidates derived view | Using developer HQ index as personal-memory store |
| B09 | Visual/status consistency | Desktop and phone show same Host command IDs, grounding, mission/effect status and source revision; in-repo TierFlow view uses inert projection | Duplicate UI-specific action catalog |
| B10 | Recovery and data integrity | Export/backup/restore of supported state, live Windows restart, resumed owner view, no second effect writer, no erased user data | Clean install only |
| B11 | One-system diagnostic view | User can inspect installed vs wired vs unavailable capabilities, current native/HQ/Studio versions, source freshness and error state | Hard-coded marketing list |
| B12 | Low-storage baseline | On constrained Windows machine, source indexes and wiki materializations obey config budgets; cache is evictable, World/effect receipts protected | Building a several-GB full-index target by default |

### Conditional but non-discardable source capabilities

Media streaming, voice-to-structured-tasks, mission experience, owner personalization, AutoWiki and visual app-builder are part of the migration inventory. For each:
- **Already operational in donor:** it must work with equivalent behavior in ARBOR or receive an explicit owner-approved beta exclusion backed by a blocker.
- **Only research/scaffold:** import the accepted contract/test/view into its capability space, but display `preview`, `unavailable` or `later`. Do not label it working.
- **Partially wired:** beta may ship a verified narrower path with a precise public limitation.
- **Sensitive write/effect:** always same Host admission/grant/receipt owner.

"Full beta" means a complete, internally coherent release of the declared operational scope. It cannot truthfully mean 100% of speculative Labs features.

## B. No outside wiring — the distinguishing gate

A release candidate must survive all of these:
1. Donor repo checkouts renamed or unavailable and all donor processes terminated.
2. No `git submodule`, symlink, path dependency, import, git-URL dependency, dynamic require, shell command or local HTTP endpoint targeting donor repo infrastructure.
3. The imported HQ intelligence, HQKG Rust crate, TierFlow package, Host, Studio, PWA and accepted Labs adapters compile/test from **only ARBOR**.
4. Product installer contains required runtime modules and assets; no hardcoded `C:\0-BlackBoxProject-0` developer path or developer-only configuration.
5. Normal internal semantic, index, graph and UI functions remain available if internet is unavailable; external AI websites transparently become unavailable when disconnected.
6. A user-selected local source repository is treated as **input data**, not as a product source dependency.

This is a **hard stop**: any runtime request to a donor development service means ARBOR is not internally integrated.

## C. Authority/truth gates

- One action catalog owner (Host), one durable effect writer, one World owner and one native graph implementation.
- Source claims include owner/version/evidence and may be `unknown`, `stale` or `not-verified`.
- Heterogeneous Development and Personal graph namespaces do not leak through shared algorithms.
- No auto-effect from predictive typing, visual view, AI hypothesis, graph relation or extension.
- Cross-device input preserves principal and revision, never silently executes offline speculation.
- The same outcome/receipt statuses appear on desktop, PWA and diagnostics.
- No unsigned or unexpected remote origin can invoke private native commands.

## D. Release evidence package

Each B01–B12 case must include: ARBOR commit SHA, Windows version/architecture, app/package version, PWA/browser version, test script and exact exit/result, relevant source IDs, pass/fail/blocked outcome, security/privacy classification and rollback instructions. **A test file existing is not test evidence.**

Require an owner-visible beta limitations document that lists unavailable source capabilities and deferred optimizations without pretending they are operational.

## E. Beta performance and economics

No fictional 15ms/30ms service-level promise. Measure on representative modest Windows hardware:
- local text input responsiveness during native/HQ/Studio failure;
- native source candidate latency p50/p95/p99;
- task queue/RAM bounds under rapid 500-edit and paste;
- clean install/first-run size, disk use by Cargo caches, local source indexes and AutoWiki caches;
- provider conversation send/receive time, auth pairing/revocation and recovery time;
- result correctness and user corrections, not simply prediction acceptance rate.

Protect the **Rock Core**: authoritative owner World, grants, effect receipts, personally authored documents and essential consented memory are never silently removed by compaction. Index/wiki/prediction/materialized graph caches can be bounded, rebuilt or discarded with truthful missing/stale states.

## F. Final verdict

**Beta GO = B01–B12 gates met or explicitly scope-approved conditional gates, no P0 trust/authority failures, no external donor wiring, single canonical product release artifact.** Release optimization debt is acceptable; source-truth, privacy, security and data-loss debt is not.
