# FA-PROMPT-FACTORY-001 — Dedicated Lane Prompt Scaffolding and First PG Cohort

**Status:** SOURCE-GROUNDED DESIGN/PREPARATION ONLY. **The four actual runnable pre-G prompts plus PGZ referee have now been authored** in [prompts/README.md](./prompts/README.md). This factory document is **not itself a runnable lane prompt**, does not authorize a merge, and is intentionally not replacing the original `.integration/proposal5/prompts/W0..W6` or master G.

## 1. Why separate orchestration from worker prompts

ARBOR Proposal 5 already has one G-only integration path, six frozen source-owner lanes (A–F), and Phase 2/pre-G source prototypes. Launching a new set of "do everything" agents based on the current **pre-G docs branch** would duplicate imports, collide with frozen source, invalidate SHA ownership and risk a second action catalog, graph or wiki writer.

Instead, this folder is **the source of truth for future prompt generation**:

- [Source Atlas](./SOURCE-ATLAS.v1.json) — verified Git blob path and owning branch for 43 observed items.
- [Gap Register](./GAP-REGISTER.v1.json) — 43 source-specific, owner-allocated deficiencies/negative proofs.
- [Wave Manifest](./WAVE-MANIFEST.v1.json) — **14 stages, 100 atomic tasks** with ordered dependencies and gates.
- [Implementation Waves](./IMPLEMENTATION-WAVES.md) — readable work packages.
- [Native machinery](./NATIVE-MACHINERY-AND-CONTRACTS.md) — exactly which existing Rust engine/typed graph interfaces to reuse.
- [PWA-first](./PWA-FIRST-DELIVERY.md) — executable mobile scope and blockers, with APK deferred.

## 2. Required source refresh before generating any lane

At launch time, a prompt author must:

1. **Fetch current** `refs/heads/work/p5-w01-a-native` through `f-verify`, each final `integration/worker-reports/PREG-X.md`, and verify no later changes. Record exact HEAD and SHA lineage. A frozen report is not a recent branch revision unless ref confirms.
2. **Check original G status**: is `integrate/proposal5-beta-r001` present, which exact commit? Does `integration/reports/W01-GATE.md` or equivalent show **real G GO**? If not, use only a `PG-*` read/isolated-proof lane; do not create post-G implementation branches.
3. **Read merged source tree** at accepted G HEAD for intended paths. Verify `hq/lib`, `native/hqkg`, `core/programmability`, `apps/vivim-host`, `studio`, `apps/mobile-relay`, `packages/tierflow`, `contracts/interop`; compare to SOURCE-ATLAS with source blob identity, not old branch path assumptions.
4. **Compare actual owner code and tests** against existing S-semantic/V-visual/W-autowiki/N0-io/P2-H1/P2-T/P2-K/P2-C worker outputs. Adopt only differential missing, owner-approved code, never re-import a second full registry or a duplicate source tree.
5. **Resolve blocker risks** from G's actual new issue ledger, including native Host attestation, browser PWA static signer/active replay, Node24 HQ source read, PWA-first APK packaging, Rust/TS version parity, protected Rock storage.
6. **Select at most one sub-capability domain per worker** and write exactly its source owned subtree, tests and handoff. If multiple owners change the same file, split their contributions into separate branches with review handoff, not parallel conflicting writes.
7. **Generate one short runnable work order per owner** with exact expected source states, code paths, negative tests, gate, and stop condition. Avoid vague "wire everything" or "complete all issues" prompts.
8. **Record machine-readable evidence** of every worker and keep status `PREPARED_NOT_PROMOTED` until the actual acceptance process.

## 3. Owner and allowed-path map for later prompts

| Owner | Original frozen code paths | Candidate writable paths *after G owner approval* | Never own |
|---|---|---|---|
| A native | `native/hqkg/`, `core/programmability/`, `src-tauri/` | Same imported crates/native modules, selected new typed submodules after accepted root | Root Cargo locks, Host World, effects, second DAG store |
| B Host | `apps/vivim-host/` | Existing control, bootstrap, grant, personal source and journal ports | New parallel dispatcher; HQ dev writer; Studio CDP |
| C Studio | `studio/`, `harness/` | Browser/agent/recipe owners and tests | Host action grants; second browser sender |
| D HQ | `hq/` | Existing source reader, wiki/KB, semantic registry/compiler read projections | Personal source owner, Host effects, standalone HQ service |
| E TierFlow/PWA | `apps/mobile-relay/`, `packages/tierflow/` | UI World/revision/renderer, Direct PWA, tested phone projections | Duplicate UI publisher, phone effect writer, APK work in initial scope |
| F QA/contracts | `contracts/`, `tests/`, `capability-spaces/` | Test-only adapters, ABI fixtures, acceptance and reports | Runtime implementation, independent catalog/executor |
| G serial integrator | Root, source locks, single product manifests, combined branch | Sole merge/build/lockfile decision | Research owner feature implementation absent source review |

**Pre-G writing authority remains a separate limited staging subtree per lane** under `.integration/proposal5/phase2/pre-g/` or `.integration/proposal5/foundations/proofs/` as coordinated. No general permission to change the above product files until the actual G baseline is accepted.

## 4. Prompt instance template (fill later, do not execute now)

```markdown
# ARBOR Proposal 5 — <WAVE-ID> / <OWNER> — <FOCUSED CAPABILITY>

Disposition: <PRE-G PREP ONLY | POST-G OWNER TASK>
Repository: owenservera/ARBOR
Exact base ref: <RESOLVED CURRENT SHA, NOT OLD RESEARCH HEAD>
Owned write prefix: <ONE PRECISE PATH> (never root/A–F/G unless authorized)
Read-only source pins: <REPO@FULL_SHA:path + Git blob + current import comparison>
Prerequisites: <G GO and exact required predecessor acceptance or PRE-G scope>
Non-goals: no second graph/Host effect writer/wiki registry/UI publisher/scheduler;
          no provider API, personal source ingestion, APK-first packaging or unsafe code loader.

FIRST READ ACTUAL SOURCE:
<3–10 source code files and test files, including current target + donor research>
<exact source/source-delta classification: imported, changed, missing, design-only>

IMPLEMENT:
<one bounded source-anchored capability, exact API and allowed modules>
<typed source/revision/provenance contract and negative refusal behavior>

ACCEPT:
<actual executable node/bun/cargo/tsc tests and exact exit status>
<authz and stale/replay/negation/rollback/ENOSPC tests as applicable>
<no private user data, no cross-owner unauthorized effects>

COMMIT:
<exact changed file list, source SHA pins and provenance>
<test output vs UNRUN>
<ownership blockers/needed Host integration>
<complete handoff artifact>

STOP:
<PREPARED_NOT_PROMOTED for all pre-G; OWNER_READY_FOR_G_REVIEW for post-G>;
no automatic merge, release publication, owner World migration or main write.
```

## 5. First pre-G prompt cohort generated; post-G cohorts remain conditional

**Cohort P (pre-G, no product writes):** executable instructions are committed under [prompts/](./prompts/), but **worker missions are NOT started**.
- **P0-SOURCE-TRUTH:** A–F frozen audit, source duplication and runtime module census; outputs owner source-read and collisions.
- **P1-NATIVE-ABI:** pure Rust source compare, generational engine swap/typed manifest proposal, negative compatibility tests.
- **P2-SEMANTIC-FABRIC:** reconcile HQ annotation/wiki/S-semantic + V/N0/W protos and exact trace contracts, candidate source map only.
- **P3-PWA-PRIVACY-STORAGE:** source-based phone auth/IME/packaging gap, personal T=0 boundaries, Rock physical bytes/ENOSPC negative fixtures.

**Cohort W2 (after accepted G):**
- A: native trusted ingress + Rust manifest/engine generational proof, one source owner per submodule.
- B: Host launch trust, catalog/lexicon binding, personal grants and single durable effect path.
- D: in-repo semantic wiki machine read projection and byte-bounded HQ source service.
- E: PWA-only composer/visual trace and TierFlow source revision propagation.
- C: Studio exact-thread/existing recipe/agent/knowledge read adapters (no second sender).
- F: real ABI/negative/security/device fixture independent verification.

**Cohort W3/W4 (after accepted source/actor tests):** source-aware language packs, native parallel event fabric, auto wiki, DCB personal read, reversible visual app draft, voice final proposal, PWA and protected Rock storage. Keep complex advanced app builder/automation/plugin marketplace in later candidate wave.

**Cohort W5/W6:** only independent security/size/device/release closure, not source-wide feature redesign.

**Cohort FB:** post-beta open ecosystem/automation/advanced learning and optional APK as a separate project with its own owner-approved release contract.

## 6. Prompt receipt / STOP contract

One worker artifact per lane:
```json
{
  "schema": "arbor.proposal5.foundations.worker-receipt/1",
  "wave": "<id>",
  "owner": "<A|B|C|D|E|F>",
  "baseSha": "<40-hex>",
  "headSha": "<40-hex>",
  "filesChanged": [],
  "sourcePins": [],
  "tests": [{"command":"...", "status":"PASS|FAIL|UNRUN", "exitCode":null}],
  "sourceScope": "development|personal|control|ui",
  "effectsPerformed": false,
  "riskFindings": [],
  "ownerDependencies": [],
  "promotion": "PREPARED_NOT_PROMOTED"
}
```

If a worker cannot establish full source/G baseline, it must still produce an evidence-only `BLOCKED_SOURCE` report under its staging prefix rather than inventing a target, starting from `main`, or installing a new service.

## 7. Staged integration decisions, not new parallel systems

- Is the agreed **SemanticWikiIR** simply a *read projection* of existing HQ semantic registry, or does a new typed frame schema need versioned admission? Decide with current compiler tests and Host catalog crosswalk.
- Does A's current `EngineRegistry` suffice for pure hot swap and independent subscriber leases? Validate with real Cargo before designing a generic plugin host.
- Can E's existing TierFlow visual compilation and rendering consume V-visual trace without additional publisher? Refuse a parallel renderer.
- Which current Host/B command sources can support a real two-Tab read capability? Only use a registered, actually accepted read action.
- Which data categories can actually be evicted? The writer and source recovery proof must decide, not a high-level Rock classifier.
- Do existing Studio/Host personal agent and DCB definitions provide a complete owner-context read? If not, keep the UI "candidate" and add a narrow authorized read adapter.
- Are offline PWA semantics sufficient from the existing browser-safe language module? Avoid pushing Rust/WASM to phone as a pre-beta requirement.
- Does any hard beta issue require an APK? Per owner policy: no; remove the packaging dependency while preserving the existing PWA.

**Future prompt authors must not turn an architecture unknown into a claimed implementation.**


## 8. Cohort 1 authoring result (2026-11-12)

- [PG00](./prompts/FA-PG00-SOURCE-TRUTH.md), [PG01](./prompts/FA-PG01-NATIVE-ABI.md), [PG02](./prompts/FA-PG02-SEMANTIC-FABRIC.md), [PG03](./prompts/FA-PG03-PWA-ROCK-SECURITY.md) and [PGZ independent referee](./prompts/FA-PGZ-CONVERGENCE-REFEREE.md) are authored with exclusive proof prefixes and concrete source/read/negative-test requirements.
- [Machine lane registry](./prompts/LANE-REGISTRY.v1.json) tracks four parallel + one later referee and covers **all 32** pre-G tasks in waves PG-00–03.
- [Shared worker execution contract](./prompts/00-SHARED-PRE-G-CONTRACT.md) preserves frozen A–F, sole original G, no product writes, exact Git SHA evidence and test truth.
- Post-G owner implementation prompts remain **NOT CREATED** because original W1-G has no accepted integrated HEAD. Do not infer that generating worker prompts started/ran workers.
