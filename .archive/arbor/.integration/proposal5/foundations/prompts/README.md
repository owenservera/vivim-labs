# ARBOR Proposal 5 — Foundational PG Cohort / Launch Desk

**Prepared:** 2026-11-12 · **Status:** **READY_TO_LAUNCH_PROMPTS_ONLY** · **G:** original W1 integration has not run as of latest verified branch check.
**Source of truth:** [Foundation overview](../README.md) · [Machine lane registry](LANE-REGISTRY.v1.json) · [Shared worker contract](00-SHARED-PRE-G-CONTRACT.md) · [Implementation waves](../IMPLEMENTATION-WAVES.md) · [Native machinery](../NATIVE-MACHINERY-AND-CONTRACTS.md). · [Validation receipt](COHORT-VALIDATION.v1.json).

**Decision:** Run four independent **pre-G** evidence and pure-prototype lanes from the *current pinned foundation branch HEAD*. They must not merge anything into main, frozen A–F, original G, root locks, other pre-G lanes or product code. When the available workers have handed off or been blocked, run a fifth independent convergence referee. **None should wait for or block G.**

## Launch the four concurrent worker conversations

| Order | Lane | Source-owned mission | Full executable work order | Branch created BY worker | Sole allowed write prefix |
|---|---|---|---|---|---|
| Parallel | PG00 | Actual code/owner/duplicate/source truth | [FA-PG00-SOURCE-TRUTH.md](FA-PG00-SOURCE-TRUTH.md) | `work/p5-fa-pg00-source-truth-r001` | `foundations/proofs/PG00-source-truth/` |
| Parallel | PG01 | Rust HQKG/programmability engine generations, native ABI and typed event proof | [FA-PG01-NATIVE-ABI.md](FA-PG01-NATIVE-ABI.md) | `work/p5-fa-pg01-native-abi-r001` | `foundations/proofs/PG01-native-abi/` |
| Parallel | PG02 | SemanticWikiIR + Host NLCL + V-visual/W-autowiki/N0 + TierFlow convergence | [FA-PG02-SEMANTIC-FABRIC.md](FA-PG02-SEMANTIC-FABRIC.md) | `work/p5-fa-pg02-semantic-fabric-r001` | `foundations/proofs/PG02-semantic-fabric/` |
| Parallel | PG03 | PWA browser/Host/native trust, APK deferral and real Rock source vs storage boundaries | [FA-PG03-PWA-ROCK-SECURITY.md](FA-PG03-PWA-ROCK-SECURITY.md) | `work/p5-fa-pg03-pwa-storage-r001` | `foundations/proofs/PG03-pwa-storage/` |
| After available four | PGZ | Independent evidence/collision referee, ≤5 exact post-G source owner handoffs | [FA-PGZ-CONVERGENCE-REFEREE.md](FA-PGZ-CONVERGENCE-REFEREE.md) | `work/p5-fa-pgz-convergence-r001` | `foundations/proofs/PGZ-convergence/` |

All full write prefixes above start with `.integration/proposal5/`. The table abbreviates this shared prefix for readability. **Do not create branches in the architecture authoring conversation unless actually beginning that worker.**

## Five one-message starters (copy each into a SEPARATE ChatGPT/GitHub worker conversation)

### PG00 starter

~~~text
Execute the ARBOR Proposal 5 PG00 Source Truth pre-G worker now using connected GitHub. Read the complete source-pinned work order at https://github.com/owenservera/ARBOR/blob/work/p5-foundational-architecture-prep-r001/.integration/proposal5/foundations/prompts/FA-PG00-SOURCE-TRUTH.md and the shared contract in that folder. Create work/p5-fa-pg00-source-truth-r001 from the current pinned foundation-branch HEAD; write ONLY .integration/proposal5/foundations/proofs/PG00-source-truth/. Audit actual frozen A-F, Labs and pre-G source, commit complete evidence and test status, report PREPARED_NOT_PROMOTED and stop. Never edit G, A-F, product code or main.
~~~

### PG01 starter

~~~text
Execute the ARBOR Proposal 5 PG01 Native ABI pre-G worker now through connected GitHub. Read https://github.com/owenservera/ARBOR/blob/work/p5-foundational-architecture-prep-r001/.integration/proposal5/foundations/prompts/FA-PG01-NATIVE-ABI.md plus shared contract. Create work/p5-fa-pg01-native-abi-r001 from pinned current foundation HEAD; write ONLY .integration/proposal5/foundations/proofs/PG01-native-abi/. Inspect real Rust EngineRegistry/typed DAG/Program and N0/F source; prepare real source-compatible hot-swap, old-lease, rollback and actor rejection proofs. Run Cargo only if runtime exists, otherwise explicit UNRUN. Commit and stop PREPARED_NOT_PROMOTED. No G/main/product edits.
~~~

### PG02 starter

~~~text
Execute ARBOR Proposal 5 PG02 Semantic Fabric pre-G work order now through connected GitHub. Read https://github.com/owenservera/ARBOR/blob/work/p5-foundational-architecture-prep-r001/.integration/proposal5/foundations/prompts/FA-PG02-SEMANTIC-FABRIC.md and shared contract. Create work/p5-fa-pg02-semantic-fabric-r001 from pinned current foundation HEAD; write ONLY .integration/proposal5/foundations/proofs/PG02-semantic-fabric/. Diff actual HQ/Host/TierFlow source against S-semantic, V-visual, W-autowiki and N0, prepare one inert source-linked IR/trace crosswalk and synthetic negative tests. No new compiler/dispatcher/wiki writer or G modification. Commit PREPARED_NOT_PROMOTED and stop.
~~~

### PG03 starter

~~~text
Execute ARBOR Proposal 5 PG03 PWA + Rock pre-G work order now using connected GitHub. Read https://github.com/owenservera/ARBOR/blob/work/p5-foundational-architecture-prep-r001/.integration/proposal5/foundations/prompts/FA-PG03-PWA-ROCK-SECURITY.md and shared contract. Create work/p5-fa-pg03-pwa-storage-r001 from pinned current foundation HEAD; write ONLY .integration/proposal5/foundations/proofs/PG03-pwa-storage/. Inspect actual B Host, E PWA, A Tauri, D wiki/SQLite, Rock policy and Labs private import source. Document exact auth/static/APK packaging, OS free-space reserve, protected data and ENOSPC negative proofs. Mobile PWA FIRST; full APK deferred. Commit PREPARED_NOT_PROMOTED, no product writes, stop.
~~~

### PGZ starter

~~~text
Referee the ARBOR Proposal 5 PG00-03 foundational pre-G handoffs using connected GitHub. Read https://github.com/owenservera/ARBOR/blob/work/p5-foundational-architecture-prep-r001/.integration/proposal5/foundations/prompts/FA-PGZ-CONVERGENCE-REFEREE.md and shared contract. Begin only after the available PG00/PG01/PG02/PG03 branches have committed or been explicitly blocked. Create work/p5-fa-pgz-convergence-r001 from current pinned foundation HEAD, write ONLY .integration/proposal5/foundations/proofs/PGZ-convergence/, independently verify real sources/tests, produce minimal source-owner post-G handoff, report PREPARED_NOT_PROMOTED and STOP. This is NOT Lane G and must not merge anything.
~~~

## Concurrency and acceptance

- **PG00–PG03** can operate concurrently. Each makes its own branch and writes to a uniquely assigned proof subtree. They do not edit each other's reports, code or original source.
- **PGZ** compares whatever final, committed branches exist; it does not wait indefinitely for missing workers or merge their work.
- **Original G** runs under its own `.integration/proposal5/prompts/W1/G.md` and is never subordinate to the PG cohort. **Do not send any of these prompts to G to execute as its lane.**
- After G eventually finishes, owner/wave handoffs must be rebased mentally and technically onto the **actual G accepted HEAD**, not inherited as source-verified from these prototype branches.
- A worker cannot claim PASS for Cargo/Node/Windows/Android without an actual executed command+exit. The only required PG disposition is `PREPARED_NOT_PROMOTED`.

## Minimum workstream goals

**PG00:** one canonical owner per action/graph/wiki/publisher/browser/phone source; source/duplicate coverage with exact blobs.
**PG01:** native engine generation, old leases, CAS, cancellation, manifest source/auth boundaries, tests that can run against the frozen Rust source.
**PG02:** existing compiler/comment/wiki/lang/visual/AutoWiki types reconciled into inert source-linked read projection with no permission.
**PG03:** concrete PWA/Host signer routes, native bootstrap/IPC and missing APK resource, protected Rock/ENOSPC, personal consent, device gate.
**PGZ:** ≤5 minimal post-G source-owner actions and unambiguous remaining blockers.

**No product implementation wave is launched by committing this launch desk.** These starter messages are ready for separate workers; the current review PR remains draft documentation/preparation only.
