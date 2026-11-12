# N0P-03 — Prime Engineering Directive: Safe Convergence, Not Feature Expansion

Date: 2026-11-12
Status: executable work-order **design**, not a claimed completed integration. Scoped to the existing four canonical repositories. Existing R001 PR #55 remains draft.

## Mission
Turn Node 0 Prime into one reliable first user-input-to-last verified-output native system. The goal is to prevent architectural debt that an unlimited AI coding budget could otherwise multiply: duplicate interpreters, false authority, impossible latency promises, privacy leakage, unbounded concurrency and unverifiable status.

**Do not ship more high-level UI features before the native semantic envelope, trace and trust boundaries stabilize.**

## Work packages ordered by dependencies, not arbitrary lane count

### Gate PRIME-0 — Security quarantine and first-hop truth (P0)
**Owners:** Host native/Tauri + independent security verification.
**Implementation work:**
1. Audit exact Cargo.lock dependency resolution and current Tauri command/capability behavior on Windows. Determine whether current production binary actually ships pinned Tauri 2.0.0; the source manifest does pin it.
2. Upgrade the coordinated Tauri ecosystem to a security-patched supported v2 set **at least 2.11.6**, preferably a reviewed newer compatible 2.x; regenerate lockfile, revalidate installer and all custom commands.
3. Restrict remote IPC capabilities to required production origins/webviews; prefer bundled trusted desktop UI assets and direct native IPC, or an exact localhost origin plus strict verified policy. Remove wildcard \`http://127.0.0.1:*\` from production capability. Audit \`withGlobalTauri:true\`, origin bypass, window navigation and raw NativeUserIo future exposure.
4. Prove a foreign localhost origin, malicious iframe, and remote page cannot call private Rust commands or retrieve native result channels. Prove authorized desktop commands still function.
4a. **Loopback identity gate:** source review shows desktop health uses HTTP success on 127.0.0.1:43187 and later posts the owner bootstrap bearer token to that port. Prove the process owning the listener is the intended Host before any secret transfer; replace plain health-code trust with an authenticated process-bound handshake or native session mint. Include preoccupied-port and spoofed-healthy-response tests using synthetic tokens.
5. Draw actual path for first draft byte: keyboard/IME -> desktop DOM (local echo) -> allowed Tauri webview + actor-bound IPC -> native Rust; phone input -> PWA DOM -> authenticated PC transport -> same Rust semantic ingress. Existing JS semantic paths must be classified explicitly.
**Proof:** packaged Windows binary, exact manifest/lockfile, command scopes, malicious-origin test, no privilege downgrade. **Merge blocker** on failures.

### Gate PRIME-1 — One typed interaction/evidence contract (P0)
**Owners:** Rust core with Host authority and UI/TierFlow reviewers.
**Implementation work:**
1. Specify one versioned \`UserInput\`, \`DerivedEvidence\`, \`InteractionFrame\`, \`CommittedUserIntent\`, \`EffectStatus\` and \`ProjectionPatch\`. Use generated Rust/TS bindings.
2. Include authenticated actor, session, surface, source-owner, purpose/consent, data class, input and interaction revisions, dependency source versions, producer ID/run status, source spans and trace correlation.
3. Use explicit state taxonomy: \`candidate\`, \`grounded\`, \`selected\`, \`committed\`, \`authorized\`, \`claimed\`, \`attempted\`, \`observed\`, \`verified\`, \`uncertain\`, \`failed\`, \`revoked\` — no accidental coercions.
4. Define canonical unicode/editor span mapping (UTF-16 ↔ UTF-8 ↔ grapheme), quotation/negation/condition scope and reversible selection binding.
5. Support multiple simultaneous meanings and producer coverage: \`ran\`, \`skipped_budget\`, \`skipped_privacy\`, \`timed_out\`, \`unavailable\`, \`not_registered\`. UI R007 expects true interpretation X-ray, not synthetic "100%" coverage.
6. Produce a machine-verifiable schema corpus; reject unknown effect states and test backwards compatibility with existing PWA/JS consumers.
**Proof:** strict roundtrips, invalid field tests, ambiguous and multilingual text fixtures, safe input/output redaction tests.

### Gate PRIME-2 — Rebuild concurrency, not features (P0)
**Owners:** Rust native engine/graph runtime, Host integration.
**Implementation work:**
1. Delete the \`Mutex\`-held full reactor-computation critical path from the R001 spike; use short session snapshot/CAS and compute outside lock.
2. Replace per-event scoped thread creation with existing shared bounded engine Scheduler/Registry or one approved bounded native executor. Keep global/per-session memory/CPU/IO budgets, cancellation, fairness and priority lanes.
3. Distinguish replace-latest ephemeral drafts from non-droppable exact \`Submit\`, \`Approve\`, \`Cancel\` and receipt events.
4. Have reactors declare typed input/dependency interests and privacy requirements; no raw full-text fan-out to every producer merely because they are registered.
5. Implement asynchronous evidence projection with compatible source-watermark vectors, out-of-order rejection and deterministic conflict representation. Give early useful partial results; do not wait for slowest producer.
6. Reuse graph-change-set invalidation for selective re-evaluation rather than clearing all sessions on any source refresh.
7. Add fault isolation so hanging/crashing provider/read reactor doesn't freeze the editor or lose the committed input.
**Proof:** type 500 rapid edits, inject one hanging producer, concurrent desktop/phone events, forced source refresh and cancellation; observe no UI stall, no stale commit and bounded resources. Confirm on real Windows hardware, not synthetic benchmark only.

### Gate PRIME-3 — Consolidate semantics and graph ownership (P0)
**Owners:** Host command canonical owner, Rust compiler, HQ native graph consumer.
**Implementation work:**
1. Inventory all authoritative/provisional command readers: Host \`intent-planner.mjs\`, \`control-language.mjs\`/Omega NLCL, TierFlow command compiler, Labs semantic lens, HQ action router/annotation, R001 Rust prefix matcher. Classify canonical semantics, source adapters, render projections, historical and duplicate candidates; leave no unreviewed parallel parser.
2. Build a differential corpus preserving source clauses, quoted text, negation, conditions, compound operations, personal rules, slot typing and risk. Existing Host output is baseline for *current behavior*; don't preserve unsafe old behavior without explicit reviewed changes.
3. Define a single canonical IR with original-text source anchors, multiple proposed meanings and provenance. Build one product Rust semantic compiler in parity-verified slices; keep JavaScript local quick hints as projections only until removed.
4. Package exactly one canonical versioned Rust graph library from existing HQ research. Normalize namespace and ownership; Host personal graph is not HQ development graph. Avoid duplicate stores; verify graph traversal, typed DAG, invalidation and Rust scheduler workloads.
5. Establish source-admission read ports: Host World, verified command catalog, project graph and privacy-scoped DCB/personal source. Test no history or unmounted index returns unknown/unavailable rather than fabricated project.
6. Register/review a **real read-only full project update** capability if one doesn't exist, rather than using the synthetic \`fixture.full.update\` in R001 as a pretend Host command. Keep effects separate.
**Proof:** byte/scheme-compatible typed IR on golden corpus, verified source pins, real project grounding and no false action availability.

### Gate PRIME-4 — Restore the complete user journey (P0)
**Owners:** Rust I/O + TierFlow UI + Host desktop/mobile.
**Implementation work:**
1. Connect real desktop editor to native \`DraftChanged\`, \`AcceptSelection\`, \`Submit\`, \`Correct\`, \`Cancel\`, reconciling local editor revisions with Rust semantic projections. UI never paints from stale events.
2. Wire semantic output to existing UI R007 visual layers: lexical, identity, graph, memory proposal, DCB, operations, prediction and epistemic status; keep Quiet/Assisted/VIEW ALL presentation-only.
3. For typing \`full\`, obtain real current project from authenticated source, display predicted focus and let two explicit Tab actions complete an inert intent. Missing/ambiguous project must ask; first Tab must never silently grant.
4. Connect phone PWA to same native session authority via secure transport. Run stale/revoked/offline/latency reconciliation; no second mobile semantic planner as independent truth.
5. Keep Tauri JS Node Host browser adapter available for provider sends, but remove it from required pure interpretation hot path. Keep legacy behavior behind controlled toggle and compare outputs until cutover.
6. Test accessibility, focus/IME/undo, paste, localization and overlaid annotations on real Windows/Android browsers. UI team owns geometry, not interpretation.
**Proof:** physical packaged Windows and paired PWA can submit the same selected intent and receive compatible grounded semantic traces without extra JS semantic routing.

### Gate PRIME-5 — Commit, effect, recovery and verified delivery (P0)
**Owners:** Host effect/journal, Rust I/O, Studio provider browser, independent verifier.
**Implementation work:**
1. Define durable \`CommittedUserIntent\` and resume semantics without creating a second master journal. Preserve exact user wording and chosen slot revisions.
2. Compile to existing Host-owned exact plan candidate; Host authenticates, checks current source/grant/revocation and journals exclusive effect claim before action. The Rust predictor only ranks inert candidates.
3. Browser adapter executes signed-in web app action and returns observed evidence. On uncertain provider sends, do not automatically retry.
4. Emit truthful UI status: received / planning / approval needed / running / blocked / partial / uncertain / failed / observed / independently verified; never render "Done" because a model generated a plausible answer.
5. Recover after every crash boundary: before commit, after durable intent, after admission, after claim, after attempted browser send, after receipt but before UI delivery.
6. Correlate original source expression, derived evidence graph, approved plan, actual effect receipt and final delivered artifact.
7. **Gated long-term native cutover:** migrate the Host admission checks, exclusive claim fencing and durable control journal into the resident Rust Host only after verified JS/Rust behavioral parity, read-only shadow comparison, exact Windows crash/fault tests, format compatibility, rollback and source-owner acceptance. One writer only. Node/Studio stays the signed-in-browser adapter.
**Proof:** fault-injection matrices, repeated/duplicate clicks, reconnect, revoked access, stale graph, crash recovery and no double effect. A user can always understand what happened.

### Gate PRIME-6 — User experience and operational proof (P1)
**Owners:** product, UI, Rust performance, observability.
**Implementation work:**
1. Measure not just parse latency but **time from first user word to useful understanding**, time to correctly specified intent, manual correction burden, source credibility, action-needed time and verified completion.
2. Introduce content-minimized, local opt-in trace sampling. Preserve producer run/skipped coverage without exposing personal raw bodies to HQ.
3. Benchmark p50/p95/p99 on the real older-class Windows laptop and paired phone; record CPU/memory/battery/thermal use and privacy budgets. No fictional "<16 ms" guarantee.
4. Run tasks users actually perform, including normal conversation without registered commands and complex negative/personal-memory utterances from UI R007.
5. Roll back a dangerous native feature independently, without reintroducing an unauthorized old effect path.
**Proof:** representative real-user task completion and first-run journey, safety/privacy review and release owner GO.

## Per-workstream ownership without extra governing layer

| Existing workstream | Prime handoff |
| --- | --- |
| Product Rust core (Harness) | Owns typed ingress, active per-session interaction state, bounded compute fabric, evidence convergence, generated Rust/TS contract |
| HQ shared Rust graph/research | Supplies reusable graph/DAG/invalidation algorithms, package tests and source-observed development interpretation producer manifest |
| Host/Studio | Supplies authenticated source ports, plans/grants, effect journal, provider web execution and receipts |
| TierFlow/UI Lab | Uses semantic output as immutable data to implement the high-abstraction composer, VIEW ALL masks, Tab/IME/accessibility and correction UX |
| Independent verifier | Validates cross-boundary behavior, security, origin, crash and source-truth/latency, without marking unrun gates green |

## Copyable implementation directive for local integration owner

\`\`\`
VIVIM NODE 0 PRIME — SAFETY-FIRST REBUILD, NO NEW FEATURE LAYER

Canonical repos: repos-hq, vivim-web-harness, tierflow, vivim-labs.
Source PR: vivim-web-harness draft #55. Labs package: user-io-rust-core/node-0-prime/.
Read existing UI R007 and HQ interpretation surface audit before touching implementation.

Do NOT merge R001 as a production runtime. Verify source SHAs and current branch changes;
isolate concurrent work. Priorities:
P0 upgrade and audit pinned Tauri IPC security and exact origins;
P0 define a typed actor-bound bidirectional interaction/trace protocol;
P0 replace R001 mutex-held join and per-keystroke OS threads with bounded,
priority-aware pure compute outside the session lock;
P0 source-ground current project/catalog, no fake fixture command;
P0 differential semantics vs existing Host/Omega before retiring JS;
P0 preserve Host's effect admission/journal, no provider APIs or implicit send;
P0 test the real desktop and paired PWA path, crash recovery and accessibility.

No independent graph store, event DB, scheduler, command registry, service, app,
permission owner or orchestration layer. One shared Rust compute foundation.
UI owns presentation geometry and modes, not the semantic truth. Commit isolated
changes with exact file:line evidence, real commands/output, BLOCKED states,
rollback and source-owner approvals. Never declare GO from source presence alone.
\`\`\`

## Round 0 regression corpus (source-observable, not executed)

| Case | Expected user-visible behavior |
| --- | --- |
| Type \`f\`, \`fu\`, \`full\` rapidly | Text/caret immediate; stale source candidates cannot overwrite current draft |
| \`full\` with no actual source project | Possible template may be suggested but project remains unresolved, no "grounded" claim |
| \`full\` with explicitly grounded XYZ and real approved read-only action | At most two deliberate Tab selections for an eligible case; no effect until explicit Submit |
| \`full but don't send it\` | Negation applies to send, may retain read-only information request; no send candidate silently executed |
| \`"full update" is the title of a document\` | Quoted string not automatically treated as an executable command |
| \`Remember that I prefer rigorous source-backed updates ... but do not send yet\` | Overlapping preference/memory/DCB/effect candidates, negated provider disclosure, zero mutation |
| Project source revokes while Tab menu open | Old project chip invalidated; Tab must not accept stale identity |
| Device pairing revoked between view and Submit | No stale grant; denied/needs-reconnect status |
| One slow graph producer / one crashing producer | Local typing never freezes; partial trace shows pending/timeout; no false absence claim |
| 500 edits/paste burst / long IME composition | Bounded memory and threads; no callback storm; edit remains lossless |
| Malicious localhost page/iframe invokes native IPC | Refused, no personal source/command access |
| External browser send times out after attempt | Unknown/uncertain, no auto resend |
| A stale future PWA has unknown effect status | Fail closed and require update, never treat as success |
| Offline phone selects a cached suggestion | Explicitly speculative, cannot become Host authoritative effect |
| Two simultaneous same-account surfaces | Independent drafts, precise commit conflict; no last-write-wins action target |
| Original text contains emoji/Spanish accents/Catalan apostrophe | Exact source, cursor, quote and slot spans preserved |
| User asks ordinary question no registered action | Free-form conversation path remains available; no "unknown command" global failure |

## Acceptance artifacts per gate
\`gateId\`, repo, exact base/head SHA, file:line, source-data class, producer identities, test command, target OS/device, exit code, measured output, expected/observed, auth scope, effect journal reference where relevant, status, rollback, owner sign-off.

**Current acceptance verdict:** architecture and sources reviewed; R001 source committed/draft PR; no native build, compiled tests, live catalog grounding, full UI integration, real mobile acceptance or production-security mitigation is claimed.
