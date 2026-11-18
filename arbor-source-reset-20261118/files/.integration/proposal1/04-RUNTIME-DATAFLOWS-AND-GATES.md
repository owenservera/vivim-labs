# Proposal 1 — Runtime Dataflows, Integration Mechanics and Acceptance Gates

Date: 2026-11-12. Design proposal only. No migration or build performed.

## A. Operational flows — exact owner transitions

### A1. User expression to source-grounded visual interpretation
\`\`\`text
DOM/IME local editor (no synchronous native dependency)
  └─ text/caret/draft revision
        └─ authenticated Tauri invocation OR paired PWA encrypted transport
              └─ Rust host.ingress — actor/session binding, privacy class, envelope validation
                    └─ node0 session actor — fast snapshot+CAS ACK
                         └─ graph scheduler — bounded pure reactor subscription
                              ├─ language: tokens, quotation, negation, commands
                              ├─ source: project/graph/entity refs + source revisions
                              ├─ intelligence: time, DCB, knowledge, context candidates
                              ├─ prediction: ranked confirmed habits, pure prewarm
                              ├─ observability: producer run/skip/deny status
                              └─ future voice/multimodal viewers (when eligible)
                                   └─ node0 evidence projection join (revision compatible)
                                         └─ audience-scoped semantic trace patch
                                              └─ desktop/PWA TierFlow renderer
\`\`\`

No stage prior to a separate explicit Submit can cause provider sends, permanent memory changes, scheduled effects or Host grant changes. Pure source reads can be selectively granted and cancelled; default is least exposure.

### A2. Two-Tab familiar project update (conditional)
1. User types \`full\`. The browser paints the original text immediately.
2. Rust command reactor finds a registered read-only full-update template **only if it truly exists in the canonical product action schema** (current Host action catalog v6 did not show one).
3. Rust source reactor resolves active project via authenticated current World/HQ development project read port, if permitted; learned ranker proposes focus \`next_steps\` only with valid workspace-scoped acceptance history.
4. UI overlays ghost sentence and source-linked selector chips. \`Tab 1\` commits exactly the displayed template/project with expected current revision. \`Tab 2\` commits the displayed focus. Ambiguity or stale evidence forces clarification instead.
5. Fully specified inert intent remains inspectable until explicit user Submit. Submit receives a stable idempotency ID; Host rechecks current grants and evidence before any effect.
6. Read-only project summary may be generated from versioned graph evidence; result status must distinguish successful source read from model conjecture. No magic "realtime verified" label from learned probability.

### A3. Voice to structured tasks
- Microphone local input → \`VoicePartial\` (never durable action) → independent transcript/language/intent hints → \`VoiceFinal\` with model/source provenance.
- Native task/proposal graph derives candidate task fields, owners, due date and source. The UI asks for clarification where needed.
- Task persistence is a separately accepted World write; device/owner authorization and exact receipt required.
- Same interaction protocol as typed text; no separate voice router, app or scheduler.

### A4. Personal knowledge acquisition and DCB
- Onboarding intake runs only after source-specific owner consent, signed-in web-app access or direct local file parsing.
- Immutable source references/observations, extraction hypotheses, validation and acceptance proceed through World-owned provenance lifecycle.
- Graph computes entity/claim edges, conflicts and relevance; it cannot silently commit permanent user memory.
- DCB synthesizes a source-linked, destination-budgeted candidate context packet and policy disclosure preview. Outbound provider-specific browser send requires exact Host admission. Assume provider memory off.
- Revocation invalidates eligible derived context, caches, predictions and pending source access; no hidden history upload.

### A5. Provider send / external action
- Rust Host effect owner admits exact thread/provider/body digest against current principal, device and grant.
- Host records exclusive claim before crossing worker IPC. Worker uses existing signed-in dedicated Chrome profile via CDP; no provider API; no Playwright/Puppeteer.
- Worker returns real send/capture evidence or an \`uncertain\` result if a response is lost after possible attempt.
- Host writes receipt; UI shows observed and verified statuses independently. No automatic replay of uncertain external sends.
- Browser worker crash does not freeze text editing or other graph computations.

### A6. Paired mobile and local Windows
- Desktop and phone have independent local edit buffers, but a **single PC session/effect authority**.
- PWA phone may show source-pinned local cached hints marked unverified; nothing effectful is queued offline for automatic execution.
- Reconnect performs actor/device/version negotiation and explicit rebase or conflict choice. No last-write-wins overwrite of selected action target.
- Device revocation prevents current and future private projections; erase caches per declared policy.

### A7. Media streaming, app builder and auxiliary product surfaces
- Media streams through authenticated Host capability port, preserving byte ranges, secure path checks, resume and cache identity; UI player stays TypeScript.
- Local app builder uses the same native session, source access, sandboxed file edits, explicit approvals and World receipts as other tasks. Build/test execution is scoped to a separate authorized tool capability, not an arbitrary Node 0 output.
- No auxiliary feature creates a new command catalog, effect writer, mobile application or visual publisher.

## B. Data consistency and storage

| Domain | Authoritative state | Derivable cache/projection | Forbidden shortcut |
| --- | --- | --- | --- |
| Active edit | DOM/editor session until explicit commit; optional user-approved local draft recovery | semantic suggestions and cursor overlays | posting raw keystrokes to permanent World history |
| Native interaction | bounded per-actor/session Rust state, revisioned ephemeral trace | forecast/candidate cache | using prediction score as action authorization |
| User World | Host owner-selected persistent source records, claims, preferences and decisions | graph materialization, DCB, search indexes | HQ dev DB or browser-side IndexedDB mock as authoritative personal store |
| Graph computation | one native algorithm/engine and versioned source snapshots | per-domain materialized relationships | second master graph store or hidden mirrored authority |
| Effect attempt | one Host-owned journal of admitted plans, claims, outcomes and receipts | UI status projection, replay inspection | two writers, blind resend on network timeout |
| Provider observation | browser worker source capture with provenance, current thread/provider ID | indexed conversation read views | treating untrusted provider text as Host command |
| HQ development | independent source-control-indexed dev intelligence | authenticated source-linked product-specific dev project reads only when permitted | shipping personal text to HQ automatically |
| PWA | device-local view caches, drafts and explicit recovery state | offline menu/shell | offline cache granting PC effect permissions |

Data format migration is a separate concern from source refactoring. A new schema is accepted only if records can be exported, checked, imported to shadow location, compared, and reversed without loss of identity/source lineage/receipt state. No direct Git import of user profiles, tokens, Chrome state or raw chats.

## C. Version/compatibility contract

Every assembled release pins:
- native host + Rust workspace build ID
- Tauri/WebView2 compatible version and Windows package ID
- \`contract\` major/minor schema
- UI bundle and PWA service-worker revision
- browser-worker protocol and provider-adapter revision
- World data schema version and effect journal compatibility
- graph engine generation and index schema revision
- mobile pairing protocol revision and grant policy
- migration plan ID and rollback eligibility

Compatibility negotiation must reject an unknown action/effect status, stale capability, downgraded policy, revoked source or incompatible risk enum. UI can degrade to safe read-only text when the PC source is unavailable.

## D. One-time full integration strategy (not a single giant commit)

**Stage D0 — Comparative review, no product changes.** Competing proposals submit source-backed scaffolds, invariants, risks, owner decisions and migration dispositions. Owner chooses one architecture or explicit hybrid. Ratify all permanent repo/process/graph/Host boundaries.

**Stage D1 — Clean workspace scaffold.** Create real Cargo+Bun workspace on a new ARBOR integration branch, pinned toolchains/dependencies, locked CI and minimal compile green. No old product copied wholesale.

**Stage D2 — Source provenance and bridges.** Import typed contract, read-only graph and deterministic command semantics with differential golden tests. Create explicit source-adapter traits, no private source bindings yet.

**Stage D3 — Native interactive vertical slice.** Trusted desktop input → bounded graph reactors → revisioned projection → UI visual render; no effect. Test high edit rates, IME, source revisions, accessibility and denial paths.

**Stage D4 — World, DCB and ingestion.** Owner-scoped source and data lifecycle import with read-only shadow projection, synthetic records and revoke tests. ChatGPT-first t0 workflow. Keep personal data out of Git.

**Stage D5 — Effects and provider worker.** Port exact Host journal/approval/receipt semantics via read-only shadow/differential comparison. One writer in production at all times. Browser worker handles signed-in Chrome/CDP with exact-thread send-once semantics.

**Stage D6 — One UI/PWA/surface.** Integrate TierFlow high-level UI, composer R007, desktop and paired phone, media and existing essential product capabilities. Existing runtime remains default until parity proven.

**Stage D7 — Full-source truth and dogfood.** Run package/security/real browser/Windows/PWA, performance and crash fault tests. Migrate user-owned data by explicit backed-up one-time operation; rollback path verified.

**Stage D8 — Single product cutover.** Freeze old action writers, verify compatible in-flight plan state, switch installed app/Host endpoints to ARBOR, observe and reconcile. Archive old runtime entrypoints only after accepted recovery; retain read-only donor history.

This permits **one product cutover** while generating small reviewed commits throughout. A single untested huge commit would violate the central mission of verified outcomes.

## E. Hard release gates

| Gate | Accept only with |
| --- | --- |
| G01 Auth/IPC | patched and tested Tauri version; restricted app origins; process-bound Host startup authentication; cross-device actor binding |
| G02 Native liveness | bounded scheduler and memory; rapid 500-edit/IME/slow reactor tests, output stability, no input stall |
| G03 Semantic truth | current source-catalog parity, negation/quote/locale/compound clause goldens; multiple meanings with explicit conflicts |
| G04 Source/data permissions | source owner evidence; private graph isolation; purpose/consent checks; revocation and cache invalidation |
| G05 Effect safety | exact idempotency, one durable writer, approvals, uncertain effect recovery and crash fault injection |
| G06 UI equivalence | desktop/PWA VIEW ALL and quiet masks share semantic IDs, no invented approvals/receipts |
| G07 User onboarding | ChatGPT-first bootstrap source permissions and progressive first value, real user journey on target Windows |
| G08 Packaging | reproducible signed Windows installer, current supported toolchain, PWA version negotiation and rollback |
| G09 Legacy retirement | no double active phone, command authority, scheduler, graph store, browser sender or doc generator |
| G10 Evidence | exact SHA, test command, OS/browser/device, expected/observed logs, status and owner approval |

## F. What not to deploy as part of the first backbone

Do not ship development Labs services, the full HQ code-indexer, experimental plugin marketplaces, legacy cloud provider API routers, old Cosmic browser singleton, old \`vivim-platform\` server/network stack, separate TierFlow server/optional .NET shell, duplicate native Rust graph sidecars, generated build debris or speculative ML model services. They may remain donors until selected and verified.

**Implementation architecture is not settled by this document.** Competing proposals should challenge process topology, crate boundaries, TierFlow packaging and Rust Host migration while keeping the safety, source-ownership and one-effect-writer invariants.
