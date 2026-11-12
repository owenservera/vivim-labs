# R009 — Rock Core: Intelligence That Shrinks Instead of Failing

**Date:** 2026-11-12 · **Status:** owner product vision, backed by HQ research design and pure test prototypes. **Not wired into the installed VIVIM app.**

**Primary design and prototype home:** [HQ Rock Core storage research](https://github.com/owenservera/repos-hq/tree/research/rock-core-storage-bounds-20261112/research/rock-core-storage) · [PRD](https://github.com/owenservera/repos-hq/blob/research/rock-core-storage-bounds-20261112/research/rock-core-storage/PRD-RC-001.md) · [Architecture](https://github.com/owenservera/repos-hq/blob/research/rock-core-storage-bounds-20261112/research/rock-core-storage/ARCHITECTURE-RC-001.md) · [48 work orders](https://github.com/owenservera/repos-hq/blob/research/rock-core-storage-bounds-20261112/research/rock-core-storage/IMPLEMENTATION-ORDERS-RC-001.md).

## ERA 1 — The everyday experience

Imagine the user's computer has very little disk space. Today most AI systems would continue writing opaque caches, stop working unpredictably, or leave the user hunting giant folders.

**VIVIM should instead immediately know which intelligence is essential, which can be reloaded, and what is genuinely irreplaceable.**

A small source-bound **Rock Core** always remains eligible for local access:
- owner/world/source identities and recovery/status references;
- accepted current project context, user-approved preferences and minimal navigation capsules;
- important recent verified decisions and live task pointers;
- safe source-provenance map so cold knowledge can be recovered when actually available;
- no fake permanent-memory duplication or unbounded copied chat histories.

The Rock differs from person to person: one developer's core might emphasize active projects and source code; a student's core might emphasize lessons and learning goals. Both preserve one shared, stable kernel contract and owner-scoped data boundaries.

When disk pressure rises, VIVIM automatically makes **secondary intelligence smaller** without silently deleting the original sources.

### Proposed status representation

~~~text
Storage mode:    AUTO  [Survival] [Lean] [Balanced] [Full] [Custom]

ROCK CORE   ████        42 MB protected              ✓ available
INDEX       ███████     118 MB, some cold sections
WIKI        ███         36 MB materialized
GRAPH       ██          19 MB derived snapshots
CACHE       █            8 MB safe to evict
ORIGINALS   ║           user-owned source vault      NOT auto-delete
FREE DISK   ░░          measured OS capacity          actual device required

AutoWiki: current topic remains fast; older generated pages may rehydrate.
VIEW ALL: indexes or producer runs omitted for storage are marked NOT_RUN / COLD.
~~~

Illustrative measurements only; not actual device disk usage.

### What users can control

- Simple mode (Auto default) and advanced physical byte caps separately for index, wiki, graph and caches.
- Which projects, context bundles, memories, documents or conversations to **keep accessible offline**, subject to real source/permission.
- Whether the generated wiki keeps only current pages, recent/frequent pages or all within quota.
- Source vault choices (originals stay in place, explicit approved external drive, or pause import when space runs out).
- Transparent reclaimed bytes and protected versus recoverable size.
- A clear degraded-state explanation when the system cannot safely continue allocating disk.

## ERA 2 — Connect to AutoWiki and the semantic composer

**AutoWiki** is normally an ephemeral page assembled from the *existing* corpus. It should not create a new markdown file every time the user flips views or changes the screen. Its logical page identity survives cache eviction; content can rehydrate from authorized source-backed HQ wiki/KB/context reads.

**VIEW ALL** should show the storage truth behind intelligence:
- grounded facts and DCB refs retain source/revision;
- a cold source is `rehydratable`, not silently considered nonexistent;
- an offline or revoked source is `unavailable` or `denied`, never fabricated;
- skip/partial states from storage pressure are distinct from "no semantic interpretation";
- old source-backed knowledge may be accessed on demand, while immediate command grammar and Rock remain fast.

**Predictive composer** should consult the remaining budget. Under low-disk mode it can reuse the user's compact command grammar, but cannot speculatively materialize gigabytes of history or prewarm an unauthorized personal archive.

**Dynamic Context Bundles** remain life-cycle-governed, owner-authorized content. Derived cached DCB materializations may be compacted if they can be rebuilt, but persistent definitions and unique source evidence cannot be deleted in the name of cache optimization.

## ERA 3 — Architectural promises and anti-promises

### Promises
- A **measured byte budget**, not just a page count, across all reconstructible index, graph, wiki and cache artifacts.
- A device-aware physical free-space reserve that Full mode cannot bypass.
- Owner-personalized, size-conscious protected Rock manifest, using existing Rust computation as shared policy foundation.
- Versioned source reference, recovery proof and owner-authorized writer maintenance.
- Emergency reading experience that remains useful when optional indexing and wiki generation stop.
- Full visibility of source coverage, staleness, cold-cache state, and actual bytes recovered.

### Anti-promises
- It is impossible to guarantee unlimited historical originals plus a tiny fixed local total cap without offload or deletion decisions.
- "Always available" cannot guarantee new writes if the filesystem runs out of space or the underlying disk fails.
- No automated rewriting/summarization of irreplaceable personal originals to satisfy byte caps.
- No independent governance, new graph DB, new permanent wiki writer or third-party API.
- No public Labs storage of private user content.

### The five highest-value acceptance journeys

1. **Survival:** owner has less free disk than the OS reserve. VIVIM pauses heavy writes, reclaims verified temporary caches and retains active Rock reads.
2. **Personalization:** Alice and Bob share the same runtime but have different active projects and learned focus; their protected Rock membership differs without leaking source refs across owners.
3. **Wiki bound:** with `maxWiki=48 MiB`, source-backed cold pages vanish from physical materialization while their logical page/source identity remains queryable; rehydration requires a fresh byte/reserve gate.
4. **Original protection:** an imported personal conversation export remains in source owner custody even when managed derived storage is beyond its limit.
5. **Recovery:** an interrupted compaction or stale Rust epoch leaves the last verified source intact; VIVIM reports degraded status instead of an unsupported success claim.

**Repo work:** HQ contains PRD/architecture, a typed settings schema, an audited source-gap census, 48 implementation work orders, and pure research policy + membership modules with deterministic tests. No production cleanup is active.
