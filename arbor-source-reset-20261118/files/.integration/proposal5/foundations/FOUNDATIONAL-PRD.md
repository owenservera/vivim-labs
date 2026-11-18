# FA-PRD-001 — ARBOR Foundational VIVIM Product and Runtime PRD

**State:** proposed product requirements derived from source, NOT shipped features. **Mobile:** PWA-first; APK explicitly deferred. **Evidence:** [43 source pins](./SOURCE-ATLAS.v1.json), [43 prioritized source closure gaps](./GAP-REGISTER.v1.json), [100 planned atomic tasks](./WAVE-MANIFEST.v1.json).
**Priority:** initial user-language/semantic wiki/visual source foundations, then native machinery, personal intelligence, app building, orchestrated automation and secure cross-device expansion.

## ERA 1 — What VIVIM must feel like

The user controls their local AI workspace using **one understandable natural-language interface**, amplified with visual meaning, source transparency, reversible interactions and real native capabilities. The system becomes progressively more helpful through explicit learned preferences, not through hidden permission upgrades. A novice can ask "what did I work on?" and navigate, while a developer can inspect the exact code comment and source contract behind a command.

**Four non-negotiable UX outcomes:**
- **Fast first signal.** On ordinary text input, the UI immediately indicates a candidate action/subject/slot/ambiguity, without waiting on network or a model. If there is no accepted grammar, display uncertainty, not fabricated understanding.
- **Understanding is observable.** Quiet/Assisted/VIEW ALL are density masks over one typed interpretation trace; user can inspect source and graph edges and undo selections.
- **User agency is real.** The Host clearly separates read, write, publish, send and delete. Predictions, AutoWiki and code annotations cannot bypass owner approval, current world state or durable receipts.
- **Knowledge respects constraints.** AutoWiki is a contextual source mirror, Rock Core bounds rebuildable on-device materialization, personal data remains owner-scoped, and no empty-disk emergency silently destroys unique records.

### 20 canonical user journeys (required source-owner acceptance criteria)

| ID | User experience | Minimal source-truth behavior | Release state |
|---|---|---|---|
| U01 | Type `full` and Tab project, Tab focus | Real registered project/query and typed slots; no "full update" invented effect | Post-G candidate |
| U02 | Open VIEW ALL on arbitrary prompt | Actual producer statuses and source-linked spans, not a generic graph illustration | Post-G candidate |
| U03 | Ask "What does this command do?" | Existing HQ wiki semantic ID+contract+source evidence, no effect | P1 read proof |
| U04 | Type `do not send` while asking for summary | Negation preserved in compound DAG, Host send never admitted | P0 safety |
| U05 | Ask for current TierFlow progress | Selected current project sources and stale/missing states; source-based answer | P1 read proof |
| U06 | Flip current screen to AutoWiki and back | Trusted screen snapshot, owner-approved source read, same draft/caret on return | Post-G candidate |
| U07 | "Remember this preference" | Memory proposal and actual owner acceptance separate; no silent save | Post-G gated |
| U08 | "Connect this to my research DCBs" | Family hint distinct from real definition IDs; explicit Host consent | Post-G gated |
| U09 | Source code changes action comment | Source ID errors, compatibility and wiki read invalidation; no auto executor | Post-G gated |
| U10 | Install new pure semantic processor | Versioned manifest/source proof/old generation lease and rollback | Rust proof gate |
| U11 | Switch visual interpretation mode | Quiet, Assisted, VIEW ALL share exact source trace and accessible equivalent | PWA/desktop |
| U12 | Pair phone in Chrome as PWA | Browser WebCrypto key, Host owner grant and revocable session | Beta P0 |
| U13 | Work on phone offline | Shell and optional drafts visible; no queued effect replay or false source freshness | Beta PWA |
| U14 | Approve a sensitive action from phone | Same Host plan/risk/claim/replay protection, explicit real receipt | Beta P0 |
| U15 | Select signed-in AI conversation | Exact Studio browser-thread identity and source evidence | Beta P0 |
| U16 | Voice note becomes tasks | VoiceFinal typed proposal then actual Host owner approval; never from partial | Conditional |
| U17 | Build a small personal dashboard | Existing TierFlow UI proposal→review→approved draft/publish, not fake app runtime | Post-G pilot |
| U18 | "Every morning summarize mission status" | Owner-approved due-rule/trigger, Host admission and receipts; not HQ Git watcher | Post-beta pilot |
| U19 | Disk nearly full while using wiki | Protected Rock/read, bounded derived caches, transparent degraded state | Beta storage baseline |
| U20 | User revokes a private source | AutoWiki, VIEW ALL and context never display revoked details; source/graph cache invalidation | P0 privacy |

## ERA 2 — Nonfunctional foundation requirements

### Source and capability correctness

- **FR-01:** Every user-facing actionable capability resolves to exactly one current owning catalog/handler; provenance and test evidence are independently linked. Unknown remains unknown.
- **FR-02:** Every source identity includes repository/bundle digest, path, semantic symbol, source revision, owner scope and access status. Caller-supplied JSON hashes are untrusted until verified by HQ walker/publisher.
- **FR-03:** Language grammar versions are source reviewed and owner-admitted. v1/v2 comment annotations and existing Rust wiki-comment grammar stay backward compatible.
- **FR-04:** UI displays `interpreted`, `grounded`, `source available`, `approval`, `effect receipt` as separate states; no inferred completion.
- **FR-05:** All producer statuses (ran, skipped-budget, denied, stale, unavailable, failed, not-run) are explicit in VIEW ALL coverage. Private denied evidence does not expose even its existence/count.
- **FR-06:** Optional LLM response can only propose candidate typed work, not install rules, grant permissions or bypass current Host original-text validation.
- **FR-07:** A source comment and a generated wiki page can never create an executable action, as proven by negative tests.

### Reprogrammability and native correctness

- **FR-08:** Native pure capabilities are source-attested, versioned, resource bounded, typed and hot-swappable using A's existing `EngineRegistry`. Existing pinned generations remain distinguishable; rollback is tested.
- **FR-09:** Native parallel event fabric does not block user's local editor or serialize every semantic producer under one mutex. Late producer results cannot corrupt newer input.
- **FR-10:** No frontend/backend semantic fork. The portable browser-safe Omega first hop and Rust native pure compute must have testable typed parity or explicit status/capability differences.
- **FR-11:** Single Host dispatcher, World source and authoritative effect receipt. No plugin, native subscriber, PWA, TierFlow display or wiki program obtains independent permission/write authority.
- **FR-12:** Engine installation, rule learning and capability activation must be reversible with source/revision and owner approval; no automatic code load/eval of arbitrary plugins.
- **FR-13:** Program compatibility is testable across Rust/TS/JS with real toolchain commands, closed schemas and source-generated fixtures.

### Personal knowledge and storage

- **FR-14:** Personal exports, permanent preferences, actual DCB definitions and Host effect journals are protected owner data, not rebuildable cache. T=0 import requires consent, checkpoint/idempotency, revoke and deletion proof.
- **FR-15:** Dev wiki and index are bounded *in physical bytes* with source recovery proof, per-volume reserve and honest low-disk behavior; latest screen AutoWiki is ephemeral by default.
- **FR-16:** Source-based ranking must not auto-promote a recent or common term to an unreviewed entity/action; ambiguous identity is visually explainable.
- **FR-17:** All read projections retain audience/revocation/owner status, and no public Labs/CI fixture contains real user corpus.

### Cross-device and extension integrity

- **FR-18:** Mobile initial release is HTTPS PWA using existing E/B direct Host source. No APK packaging/build, Keystore dependency or second phone backend.
- **FR-19:** Service worker cannot cache/replay auth-bearing API calls or mutation requests. User edits offline are not automatically submitted later.
- **FR-20:** Browser private-key/grant/session security and Host native child trust are actual P0 penetration tests, not design checklists.
- **FR-21:** Studio provider operations use signed-in browser with existing one-sender exact conversation targeting; uncertain result never blindly retries.
- **FR-22:** Plugin/recipe/mission declarations are inert until owner review and bound handler. Initially support a single pure package with rigorous deny/rollback tests instead of claiming a complete marketplace.
- **FR-23:** App builder uses existing TierFlow draft publisher; the first accepted capability is preview/review/publish a UI document, not an instantly working arbitrary full-stack app.
- **FR-24:** Accessibility/keyboard/IME and real Windows/phone parity are required for the composer/semantic overlays.

### Quality targets are hypotheses until measured

- Local first semantic feedback target for a hot Rock pack: proposed p95 ≤ 50ms Windows / ≤ 100ms target Android PWA, *subject to actual device benchmark and fallback*. No accepted latency measured by this round.
- **90% no-LLM** ambition applies to correct, fully grounded interpretation of **frequent eligible registered VIVIM capability requests**, not all chats. Report eligible and all-turn denominators, negatives, entity/slot accuracy, approval and actual execution separately.
- Rust engine source activation must not increase effect authority; 0 unauthorized effects under the defined adversarial corpus is a required safety gate, to be measured rather than assumed.
- Storage default reserve / individual caps require representative real disk measurements, no fixed universal promise of fitting all unique original user data in a tiny disk budget.

## ERA 3 — Risk, deferral and product release decision

### What can reasonably land near the first combined beta

**Hard infrastructure:** A-F source union, root runtime tooling, secure Tauri/Host launch, PWA browser authentication, Host journal checks, HQ authorized read, source freshness, minimum low-disk protection, no-donor packaging and real Windows/phone/provider acceptance. These are beta critical.

**Low-risk functional slice (conditional):** one read-only semantic explanation strip, one registered Host read intent, one source-bound UI indicator and one source-linked wiki page. G can admit this only after hard security/build gates and positive Node/Host/PWA tests.

### What must not be bundled by default into beta

A new universal plugin marketplace, fully dynamic semantic v3 parser, all 43 interpretation facets, total per-screen AutoWiki, full ChatGPT historic corpus ingestion, all 100 predictive behaviors, broad user automation Queen, novel media/voice engines, third-party native module loaders, general full-stack app builder and signed native APK. Source/proposals should be preserved as **deferred** with exact owner/test acceptance, never mislabeled as shipping.

### Product acceptance thresholds

1. **Stable authority:** one Host World/effect writer and real permission/approval/receipt parity.
2. **Stable UX:** one desktop and PWA experience with correct source/coverage/uncertainty and no Android native bridge prerequisite.
3. **Stable native computation:** if reprogrammability is promoted, real Rust generation/cancel/rollback tests and source-attested manifest ABI, not a design proposal.
4. **Stable knowledge:** HQ private-development separation, personal source consent, low-disk read behavior and no unverified wiki memory writes.
5. **Stable recovery:** source and effect revision compatibility, restart/unknown-send/cancel/ENOSPC/drift tests, original project source recovery.
6. **Truthful release dossier:** exact source SHA, real command exit code, device evidence and explicitly deferred features.

**This PRD has no direct implementation authority.** Dedicated implementation prompts will be authored later from [PROMPT-FACTORY-SCAFFOLD](./PROMPT-FACTORY-SCAFFOLD.md), on the exact current accepted G source, in independent source-owner lanes.
