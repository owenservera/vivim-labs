# Proposal 4 — One System Truth: Identity, State, Causality and Views

**Status:** architecture proposal. This is not a new data-store specification or a claim that the runtime already works.

## 1. "One system truth" is a strong invariant, not a single table

A truly unified system cannot have a UI truth, a backend truth, an agent truth, a wiki truth and a command truth. It must have **one logical model of identity, state, capabilities, evidence and allowed transitions**.

But there is a distinction between *one logical truth* and *one unbounded physical source of truth*. Product data has real owners:
- The owner/Host controls World state, personal memories, grants, committed plans and effects.
- HQ controls development-source observations and documentation intelligence.
- Studio/browser worker observes signed-in external webapp states, but provider text cannot grant commands.
- TierFlow controls renderer code and local visual documents, not the semantics or source truth of World.
- The editor controls the unpublished words and caret until an explicit submission.
- A personal graph or a generated wiki is a **read/materialization** from these domains; it cannot silently authoritatively overwrite them.

**One system truth = one typed, addressable, revisioned, source-bound *interpretation* of these owner states, exposed consistently through all surfaces.**

There may be contradictions, uncertainty and different historical views. The system must preserve them and explain their source rather than manufacture a winner.

## 2. Universal typed object vocabulary

These are candidate *types* in the existing shared Rust graph/contract model, **not** a command to create a new database, ontology registry, governance tier, or runtime entity service.

| Kind | Meaning | Canonical owner / persistence |
| --- | --- | --- |
| `SourceRef` | Authoritative domain, principal/scope, source locator, exact revision and visibility | Source owner; opaque to unauthorized callers |
| `Resource` | User document, project, file, conversation, memory, media, task, device, context definition | Owner domain; never duplicated just because multiple views use it |
| `Observation` | What was actually seen/read from a source at a revision/time | Trusted observing adapter; provenance and limitations retained |
| `Claim` | Assertion about resource or concept with truth label and conflicting evidence | Derived or accepted by its proper owner; not always verified |
| `Signal` | Immutable input/change/derived observation offered to typed subscribers | Ephemeral during interaction unless explicitly promoted |
| `Capability` | Stable operation identity, typed ports, declared risk/source/placement/availability | Existing owning subsystem catalog / verified adapter |
| `Transform` | Pure source-scoped computation mapping typed input to typed output | Shared Rust engine or trusted adapter; no effects |
| `Program` | Versioned composition of capabilities, transforms, conditions and views | Versioned configuration/owner program state, separately activated |
| `View` | Parameterized, privacy-filtered projection of typed objects/relations/status | Derived; UI/TierFlow may render, not authoritatively store its own copies |
| `Intent` | Human-authored or human-selected typed request tied to original expression | Host upon explicit commitment; prediction is not commitment |
| `Effect` | Consequential attempted action, Host-owned admission/claim/receipt lifecycle | One Host journal writer |
| `PolicyRef` | A source-owned grant/scope/policy revision reference | Existing Host policy authority, never invented by module |
| `Configuration` | Owner-controlled parameters, choice of view/program/version/profile, retention budget | Versioned World config; not effect permission |
| `Receipt` | Observed outcome and independently verified postcondition evidence | Writer/observer as appropriate, correlated with original intent |

**A visual chip, a CLI argument, a wiki link, a program input port and a graph node may all refer to the exact same `ResourceId` or `CapabilityId`.** They are not five independently named implementations.

### Proposed identity envelope

```typescript
type IdentityRef = {
  id: string;                  // stable namespaced canonical ID
  kind: string;                // resource | capability | view | program | ...
  ownerDomain: string;         // host.world | hq.dev | studio.observation | ...
  sourceRevision: string;      // exact owner-observed revision, not caller-claimed
  sourceProof?: string;        // trusted verification reference, not a grant
  audience: string;            // authorized data-class/scope at projection
};
```

Illustrative only. Internal Rust types should disallow free-form string privilege checks and use stable enums/newtypes where safe. A source checksum alone proves neither source authenticity nor authorization.

## 3. A single reference state machine

A user request travels through states that are **not interchangeable**:

```text
Original words / local edit
     │
     ▼
Recognized or proposed interpretations  [candidate]
     │
     ▼
Source identity and typed slots checked [grounded / ambiguous / unavailable]
     │
     ▼
Human selects exact displayed meaning   [selected]
     │
     ▼
Human explicitly submits                [committed]
     │
     ▼
Host validates current scope/grants      [admitted / refused / approval_required]
     │
     ▼
Single-writer effect claim              [claimed]
     │
     ▼
Browser/OS/World operation              [attempted]
     │
     ├──> unknown / uncertain / failed / partial
     └──> observed receipt
                │
                └──> independently verified when postconditions support it
```

A **read-only answer** may stop at a source-grounded projection and never perform an effect. A conversation may be useful despite no registered command. A user may correct a prediction without confirming anything. A model may produce a claim, not a source-proof or effect receipt.

Unknown transitions or older PWA versions fail closed for effects.

## 4. Multi-authority state joins without split-brain

A graph relationship can connect a development repo and a project in personal World, but the relation carries **two source references with their own revision and access restrictions**. A composer query cannot simply stamp the result as authoritative by sharing one current `worldRevision`.

Derivation metadata for each compute output:
- `inputRevision`, `sessionRevision` and authenticated actor/device scope.
- `sourceVector` for the particular sources it read (World, project, catalog, HQ index, provider observation, policy).
- `engineId`, implementation generation and operation version.
- `dataClass`, allowed output audience and purpose.
- `derivation` + `provenance`: observed, pure derived, model-proposed, accepted, verified or stale.
- `producerRunStatus`: ran, skipped_privacy, skipped_budget, not_installed, timed_out, failed.
- `dependencyIds` for precise invalidation and lawful erasure.
- `validity`: event time, observed time, effective time, expiration and as-of request where relevant.

**Compatibility rule:** a view may combine outputs with distinct source vectors if the dependencies are compatible for the requested use; a revoked policy/superseded target makes the affected join stale or refused. Missing source data is `unknown`, not `false`.

A user may inspect historical state **as of** an earlier source revision, but a historical view cannot authorize a present-time effect.

## 5. One system, multiple projections

The same program/state can produce:

| Projection | User experience | Truth invariant |
| --- | --- | --- |
| Chat composer | Inline predictions, underlines, selector chips | Same input/source IDs as full interpretation graph |
| VIEW ALL / X-ray | Multi-layer semantic graph, overlaps, suppression reasons | Reveals eligible producers and actual run status, not imaginary understanding |
| AutoWiki | Current project snapshot, source links, decisions, issues, next steps | Materialized from actual current source graph, not separately hand-edited truth |
| Mission feed | Working, requires action, blocked, uncertain, completed | Reads the same Host mission/receipt lifecycle |
| Visual app builder | Drag/drop capability ports and views into a program | Builds the same typed IR as natural-language composition |
| CLI | List, explain, preview, submit a program | Same source IDs, risk, grants and revisions as any surface |
| Phone PWA | Responsive subset of same workspace view | Paired to same Host authority; offline hints not current grants |
| Accessibility | Verbal/tactile equivalent of semantic state | Identical meaning and approval distinction; geometry differs |

A view recipe is a **pure query + presentation profile**, never a second source-owned state write. When the user edits layout or config, that is a typed configuration proposal that goes through its proper owner/version check. Source data is not altered unless an explicit authorized command says so.

### View continuity under change

When source or engine generation changes, the view receives **revisioned deltas** rather than an imperative "reload app". The UI keeps original local text and focus; it applies only compatible semantic patches. Stale source evidence is marked and rederived selectively. The same node identity survives Quiet/Assisted/VIEW ALL modes.

## 6. No frontend/backend distinction: what we **do** mean

We reject these *semantic* splits:
- UI-owned action registry versus backend-owned action registry.
- Frontend-derived "complete" versus backend-derived "complete".
- Browser view of "project" versus native graph concept of "project".
- Separate memory item identities in chat, wiki and World.
- Renderer-based grants or local model prediction treated as policy.
- A new app builder language unrelated to the command/graph language.
- AutoWiki content maintained as a second living knowledge base.

We **do not** reject these necessary boundaries:
- OS/browser editing is local and does not block on native compute.
- Rust Host has privileges that the renderer must not receive.
- Paired phone has network latency and can be offline; no false immediacy.
- External provider browser text and automation is untrusted.
- Owner/private data requires permission filters even when queries originate from user-owned devices.
- Memory/corpus/graph relationships can be derived and cached but have distinct authoritative source writers.
- JS code can remain while stable contracts and native capability ownership converge.

**The unity is in the operation/state model; trust stays explicit in the physical architecture.**

## 7. One configuration/programmability truth

A capability's behavior is determined by three distinct, separately inspectable inputs:
1. **Definition:** immutable versioned contract and implementation binding (what it can do).
2. **Configuration:** owner-selected nonprivileged parameters and view/program choices (how it should behave).
3. **Admission:** current user/device/source permissions and effect owner policy (whether it is allowed now).

Configurations can modify ranking, view density, quotas, context relevance or available module versions **within declared constraints**. Configuration cannot grant new authority or change risk.

A public program recipe can be installed disabled, inspected and configured; only an explicit Host owner admission grants any requested effect scopes.

## 8. Storage and size-bound foundation

A resource-constrained laptop is a first-class product target. The *logical* system may be rich while its *local materializations* remain bounded.

Define a durable **Rock Core** of user-selected essential data, exact grants/receipts, active project pointers, required schema/index metadata, minimal context/persona and configured recovery information. Then define quota-controlled rebuildable materializations:
- **Hot**: active interaction and current source cache (short TTL, strict bytes).
- **Warm**: recent project graph neighborhoods, DCB candidates, AutoWiki sections.
- **Cold**: optional personal corpus/cache with owner-selected budget and retention.
- **Rebuildable**: indexes, derivations, rendered wiki snapshots, model prediction caches, temporary media segments.
- **Protected**: committed World facts, effect receipts, grants and user-authored documents; never silently evicted.

Compaction must preserve source references and provide a truthful "source currently unavailable" marker when local evidence was evicted. Erasure/revocation propagates through derived caches, not a "keep a secret copy" fallback. Do not duplicate raw corpora to generate AutoWiki/VIEW ALL.

**This is a storage policy on existing World/data owners, not a new product database.** Exact quota numbers require actual profiling.

## 9. Critical acceptance examples

- Opening the same project as chat, wiki, mission and visual canvas returns the same source identity and compatible source revision.
- Editing a widget or rearranging a visual surface cannot change an authoritative task status or grant.
- `full` + two Tabs uses current grounded project/focus only when a real source-owned capability exists. No admitted effect.
- A sentence contains three legitimate interpretations and one negated provider send: all relevant facets appear with provenance; nothing is sent.
- An unauthorized source is never disclosed merely by showing VIEW ALL; `skipped_privacy` conveys producer status without revealing hidden existence.
- A source is revoked while the phone is offline: its cache is marked stale/purged on reconnect, never used as present authorization.
- Legacy JS and native Rust modules give identical source/status semantics under the same generated contract; disagreements are visible and block replacement.
- AutoWiki storage is compacted: source links remain and actual missing data is labeled missing, not reconstructed as fact.
- New renderer/view module can consume existing typed data without adding a backend API or bespoke state store.
- No part of a speculative program can execute until an explicit committed request and Host admission.
