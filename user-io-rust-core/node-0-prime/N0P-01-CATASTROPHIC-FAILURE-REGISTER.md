# N0P-01 — Catastrophic Failure Register

Date: 2026-11-12. **Threat model, not a list of proven production incidents.** Each row distinguishes a source-observed weakness from a plausible failure mechanism or design gap.

## Ranking rule
- **P0**: can compromise confidentiality, execute unintended effects, make the product routinely unusable, or undermine user trust at the first interaction boundary.
- **P1**: creates persistent inconsistency, privacy or UX deterioration that will be expensive to retrofit.
- **P2**: quality/operability risk to address before broad distribution, not necessarily a present blocker.

## I. Trust boundary and effects — the irreversible failures

### N0-01 — Tauri remote-origin/IPC abuse (P0, source-observed configuration + published upstream vulnerability)
**Evidence:** \`vivim-web-harness/src-tauri/Cargo.toml\` pins \`tauri = "=2.0.0"\`. \`src-tauri/capabilities/default.json\` sets \`remote.urls = ["http://127.0.0.1:*"]\`; \`src-tauri/tauri.conf.json\` sets \`withGlobalTauri:true\`. Official GitHub advisories May 6, 2026 (GHSA-7gmj-67g7-phm9, patched >=2.11.1) and Sep 26, 2026 (GHSA-w28w-mhc8-qvjv, patched >=2.11.6) cover the pinned version range.
**Failure:** untrusted content gains a native IPC path or confidential channel response; even an innocent-looking native command becomes dangerous once user graph data is attached.
**Intervention:** coordinated patch upgrade to >=2.11.6 or newer security-audited current 2.x; pin and build all Tauri runtime/build dependencies coherently; replace wildcard remote origin with separately scoped exact production origin or bundled app assets; least-privilege window/webview capabilities; real origin-abuse tests. **No current exploit against VIVIM has been demonstrated.**

### N0-02 — Forged actor, session or trusted source (P0, live risk on future source binding)
**Evidence:** R001 \`UserIoCore::apply_json\` receives caller-supplied \`session_id\`, \`sequence\`, and \`expected_revision\`; no authenticated principal is bound to the session in the pure module. Default source set is empty.
**Failure:** user A gets source-derived suggestions or accepts candidate belonging to user B; a webview or remote phone asserts project identity and receives personal data.
**Intervention:** session identity minted by authenticated native ingress, principal and device bound server-side, non-exportable; ACL at every source read and projection, not just input parse. Integrate only when principal/renewal/revocation tests pass.

### N0-03 — Predictions become implicit permission (P0, prevented by current inert model but future danger)
**Evidence:** R001 deliberately \`execution_performed:false\`; current Host effects use separate planner/approval/journal. 
**Failure:** ghost-text acceptance or high prediction score is mistaken for user approval or permitted browser send.
**Intervention:** distinct types for \`Predicted\`, \`Grounded\`, \`Accepted\`, \`Committed\`, \`Admitted\`, \`Claimed\`, \`Observed\`, \`Verified\`. Native type system must forbid direct conversion; host effect owner independently re-evaluates grants and exact target.

### N0-04 — Unknown external effect gets retried (P0, existing protection should survive migration)
**Evidence:** Host control dispatcher and phone client explicitly treat claimed/uncertain execution as not safe to repeat.
**Failure:** network timeout and automatic "helpful" recovery resends an email, publishes twice, revokes twice or duplicates state changes.
**Intervention:** retain durable Host claim/receipt fence, separate command idempotency vs external-effect observation, no auto-retry after uncertain send, user explanation plus reconciliation.

### N0-05 — Draft typing leaks private content to unneeded subscribers (P0, architecture design hazard)
**Evidence:** current R001 scoped threads see the same \`Session\` text; planned fan-out is broad. UI R007 asks to expose many intelligence layers, including personal memory and DCBs.
**Failure:** partial names, pasted secrets or unsent messages are handed to HQ indexing, debug traces, browser provider or ranker without consent.
**Intervention:** per-event privacy/classification and per-reactor read entitlement; cheapest lexical tier receives bounded text only as needed; high-privacy producers require intent/purpose and explicit disclosure; raw bodies never travel to development HQ or remote model by default. \`VIEW ALL\` shows allowed producer status, not unauthorized private existence.

### N0-06 — Provenance or data digest mistaken for source authenticity (P0)
**Evidence:** HQ Rust scoped graph and Host candidate contracts explicitly state caller-provided SHA/provenance aren't authentication.
**Failure:** malicious content promotes its claimed source ID or checksum to "verified project/current permission."
**Intervention:** source owner attestation, authenticated read adapters, revision pinning and anti-injection labeling; provenance **link** distinct from authority **proof**.

### N0-07 — Model/HTML output re-enters the action stream as a user command (P0)
**Failure:** retrieved conversation, web content or AI answer containing "ignore previous / send this" becomes a native command or feedback event.
**Intervention:** typed origin + trust label, separate user-authored vs quoted/source text, one-way proposal boundary, no execute-capable instructions from untrusted content. Add corpus of injection and quoted-command adversarial tests.

## II. Liveness and concurrency — the unusable experience failures

### N0-08 — One slow reactor freezes typing or Tab (P0, source-observed)
**Evidence:** R001 Tauri \`Mutex<UserIoCore>\` held during \`apply_json\`; \`UserIoCore::apply\` invokes \`thread::scope\` and joins all reactors before returning.
**Failure:** graph/IO slowdown blocks next input, cancel, update, phone event or native shutdown.
**Intervention:** short session lock solely for source snapshot/CAS commit; bounded compute outside lock; publish incremental results; never await all reactors for first useful projection. UI text remains editable locally.

### N0-09 — Unbounded per-keystroke work (P0, source-observed scale hazard)
**Evidence:** scoped threads per accepted event. Existing R005 lab uses ~260ms debounce for HQ source query; R001 has no global scheduler or admission token.
**Failure:** keyboard paste/dictation, multiple users/surfaces and background graph engines create CPU/memory thrash; session reacts later and later.
**Intervention:** bounded common executor, per-session replace-latest draft slot, deadline/priority, only ready graph dependencies, resource/energy limits, low-power fallback, no unbounded channels. User-committed events cannot be silently coalesced.

### N0-10 — Completion order changes displayed interpretation (P0)
**Evidence:** R001 sorts output by static reactor order (good for the synchronous slice), but no real asynchronous source evidence pipeline exists.
**Failure:** stale project from revision 41 appears alongside command from revision 46; UI shows false grounding.
**Intervention:** typed snapshot vector over input, active workspace, World, catalog, personal source, source index and engine generation; per-derivation compatible version checks; snapshot watermark and partial/stale states.

### N0-11 — Full-input global invalidation destroys live performance (P1, source-observed)
**Evidence:** R001 \`install_trusted_sources\` clears all sessions and ephemeral feedback, without dependency-selective invalidation. 
**Failure:** source index refresh or one project change causes every composer to reset; users lose selected slots.
**Intervention:** dependency-linked invalidation using existing graph change set; source revisions invalidate only affected projections, user-authored text/committed slots preserved where still valid, stale selections made explicit.

### N0-12 — Slow information blocks fast meaning (P0)
**Failure:** semantic prefix, graph lookup, personal read, model reply and network status share one critical path; fast operations wait for slowest.
**Intervention:** isolation by latency class: immediate local edit, hot deterministic native candidate, bounded source-backed read, optional deeper AI. Deliver partial projections as soon as valid; hard deadlines; stable last-valid view marked stale.

### N0-13 — Shutdown, crash or reconnect loses human intent (P0)
**Failure:** user sees "sent" but the engine crashed before durable submission, or a browser send was attempted before its receipt became visible.
**Intervention:** ephemeral drafts with opt-in local recovery, separate durable \`CommittedUserIntent\` record before effect admission, correlation to existing Host effect journal, reconnect reconciliation with no double send. Do not build a second competing journal.

## III. Semantics, language and epistemic integrity

### N0-14 — Same sentence means different things on PC, phone and CLI (P0, source-observed fragmentation)
**Evidence:** Host NLCL/intent-planner, mobile local control language, TierFlow inert visual compiler and R001 Rust prefix matching are separate implementations; no full behavior-parity proof.
**Failure:** user approves an action they did not expect because a surface interpreted "not", "if", "and" or alias differently.
**Intervention:** one canonical typed semantic IR with explicit clause/negation/condition/quotation scopes; all surfaces can offer provisional hints, but submitted interpretation is derived from one authoritative native semantics path. Differential corpus before retiring JS.

### N0-15 — One winner discards important simultaneous meanings (P1, source-identified R007 gap)
**Evidence:** UI R007 requires overlapping semantic hypergraph, including memory proposal, DCB flavor, potential effect and negated send in one sentence; HQ audit says no end-to-end shared utterance envelope exists.
**Failure:** the machine treats a user instruction as only a memory request and loses a no-send constraint; visual certainty hides competing interpretations.
**Intervention:** concurrent typed facets bound to source spans; no silent semantic winner; cross-facet integrity validation and explicit unresolved conflicts. \`VIEW ALL\` is a privacy-scoped trace of what *actually ran*.

### N0-16 — Position/spans corrupt when words become chips (P1)
**Evidence:** Labs R005 tracks UTF-16 source spans; Rust naturally handles UTF-8 byte offsets. Browser IME, emoji, Spanish accents, Catalan apostrophes, bidirectional text and undo complicate mapping.
**Failure:** wrong slot text is replaced or negation applies to the wrong phrase; a user sends to the wrong recipient.
**Intervention:** canonical original text plus stable editor operation IDs; explicit UTF-8 byte ↔ UTF-16 code unit ↔ grapheme segment mapping, immutable span anchors, no destructive normalization of literal text. UI owns actual edit/IME; Rust projection proposes only revision-bound patches.

### N0-17 — False freshness and "verified" claims (P0)
**Evidence:** UI R005 explicitly warns Host \`readiness:"verified"\` is not live acceptance. R001 example uses synthetic \`fixture.full.update\` absent from the Host action catalog (version 6; 19 actions observed).
**Failure:** user trusts the availability of commands or up-to-date project summaries that haven't actually been checked.
**Intervention:** distinguish registered, wired, available now, authorized, attempted, observed, independently verified; each with source-owner receipts and expiry. No success icon from a guessed model claim.

### N0-18 — Prediction models silently rewrite the user's intent (P0)
**Failure:** Tab accepts wrong project or focus, learned pattern bypasses negation or auto-composes a compound effect without explicit selection.
**Intervention:** preview vs selected vs committed distinct; exact displayed ID, event revision and explicit accessible selection; no one-press automatic external effect; retain original text and reversible edit history.

### N0-19 — Habit learning poisons future behavior (P1)
**Failure:** model trains on untrusted documents, its own suggestions, an AI echo, or misattributed outcomes; a workspace-specific shortcut bleeds to another person or context.
**Intervention:** owner-scoped accepted/corrected evidence only; use bounded local counters initially, stable cold start, rejection/undo signal, drift detection and reset/inspect controls. Never make rank score a policy bit.

### N0-20 — One invalid command grammar invalidates the whole dialogue (P1)
**Failure:** strict command parser refuses a legitimate conversation, or a complex statement without a registered action is treated as unsupported user intent.
**Intervention:** **conversation is always accepted as human expression** even when no executable command exists. The system must be able to ask, research, explain or simply chat; command recognition is a *facet*, not a gate on everything.

### N0-21 — The "truth hypergraph" becomes UI overload (P1)
**Failure:** rendering every possible interpretation destroys readability, accessibility and confidence; 100 features become 100 interruptions.
**Intervention:** one underlying complete-for-consulted-producers trace with explicit coverage; Quiet/Assisted/VIEW ALL differ solely in presentation mask. User edit and status remain primary; no animation that implies effect status.

## IV. Cross-device, privacy, operational and economic traps

### N0-22 — Phone and desktop diverge on session authority (P0)
**Evidence:** mobile currently talks via direct phone JS route and Host control client; native Rust not on real mobile ingress yet.
**Failure:** phone approves a stale plan, or PC acts on a phone's offline speculative selection without current grants.
**Intervention:** PC owns session and effect authority, mobile drafts are local/offline proposals, rebase under current snapshots and exact device binding, explicit conflict resolution, single action idempotency identity.

### N0-23 — Source revocation leaves searchable ghosts (P0)
**Failure:** a revoked provider or forgotten personal record remains in personalized suggestion counts, cache keys, projection logs or restored session.
**Intervention:** source-lineage invalidation + erasure policy across ephemeral cache, persisted preferences, DCB derived packets and observability. Revocation controls source visibility independently of previous interpretation pins.

### N0-24 — Observability becomes surveillance (P1)
**Failure:** "VIEW ALL" or debug traces retain all drafts, quoted text, personal graph edges and provider outputs forever; developers inadvertently gain data.
**Intervention:** by-default redacted traces and opt-in debug retention, reason-coded producer coverage, scoped access, data-minimized telemetry, opaque source refs, user control over local trace lifecycle. Unknown producer is not "ran and found nothing."

### N0-25 — The system passes unit tests but not the user's first minute (P0, source-observed testing gap)
**Evidence:** UI R005 28/28 Node and 100 synthetic probes, no actual browser/device visual acceptance. R001 Rust source has six unit tests but no observed cargo test or Windows Tauri integration.
**Failure:** IME, focus, Tab, mobile reconnection, installer, CPU load, source-freshness or IPC origin breaks the first useful interaction.
**Intervention:** real packaged Windows desktop and physical paired PWA acceptance as release gate, plus differential corpus, adversarial tests, model-free offline, real owner command catalog and real source grounding.

### N0-26 — The architecture outruns the hardware (P1)
**Failure:** dozens of concurrent reactors, small ML models and persistent graph clones consume RAM/CPU/battery on everyday Windows machines; fan-out becomes more expensive than writing a direct prompt.
**Intervention:** bounded lazy subscriptions and incremental cache, avoid graph cloning on hot path, memory caps and representative lower-end hardware benchmarks, distinguish CPU from network/wall-clock p95 latency.

### N0-27 — Third-party browser automation becomes a product-wide failure domain (P0)
**Evidence:** desktop Tauri starts Node Host child; Studio browser adapter is separate from graph code, but Host owns related control behavior. 
**Failure:** provider page change or hung browser lease breaks the entire native event loop or causes the UI to display a false completed state.
**Intervention:** one native session authority and isolated failure boundary for browser adapter, bounded child process IPC, per-provider health, circuit breaker and unambiguous attempted/unknown reconciliation. A dead browser must not stop local editing, graph queries or user cancellation.

### N0-28 — Schema evolution breaks silently (P1)
**Failure:** native Rust changes candidate shape; an older PWA treats a newer risk/refusal state as approved, or ignores unknown provenance fields.
**Intervention:** generated Rust↔TS bindings from a versioned contract, explicit major-version negotiation, fail-closed unknown effect statuses and backward-compatible read-only projection changes. Do not rely on scattered hand-maintained JSON shapes.

### N0-29 — No actual product-level outcome definition (P0)
**Failure:** we optimize model latency, cache hit rate, icons and Tab counts while users fail to get correct outcomes; no one can identify where requests silently died.
**Intervention:** end-to-end outcome contract from captured intent through receipt/verified delivery, instrumentation of abandonment, wrong-target corrections, source freshness, uncertain sends, latency p50/p95/p99 and successful human completion. A "full update" is done only when the expected artifact/facts are actually delivered.

### N0-30 — Infinite coding capacity creates an infinite competing architecture (P0 organizational)
**Failure:** each agent creates its own parser, scheduler, event bus, graph store, memory subsystem, command catalog or PWA identity; integration never converges.
**Intervention:** freeze the Node 0 Prime boundaries and executable golden contracts. Require each workstream to identify canonical producer/consumer and give a retirement map for replaced code. One product-native kernel, one Host effect authority, one source-owned graph family, one primary mobile app. Large code budget should remove duplicate paths rather than create more layers.

## The single most important engineering principle

**Fan-out is a computation strategy, not a privacy permission and not an excuse to eliminate sequencing.**

The source-edits and pure read candidates can be highly parallel. The confirmed intent boundary needs deterministic versioned snapshots. The Host effect boundary must be admitted and fenced. All three can coexist without a central semantic router.

## Release interpretation
No entry marked P0 should be considered discharged by design prose, green synthetic tests, or a merged draft PR. Each demands a reproducible, real-runtime safety or usability proof with owner acceptance and rollback.
