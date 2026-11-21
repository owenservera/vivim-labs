# VLANG archive — source catalog

**Archive as of 21 November 2026.** This catalog is a navigation document, not a new canonical VLANG specification.

Before this index refresh: 728 files, 124 nested directories and 23 source collections. All 21 complete source trees have exact Git SHA matches.

## Primary reading route

1. Begin with the original VLANG conversation in `vivim-labs/ARBOR-VLANG-UPGRADE/research/chat-VLANG Semantic Protocol Design (2).txt` and its JSON export (sensitive metadata).
2. Compare the original 2 November semantic-language-kernel under `repos-hq/commit-2651bc8/docs/research/semantic-language-kernel/` with the later `vivim-labs/documentation-consolidation/...` copy, which contains an additional optimization note.
3. Read `arbor/.integration/proposal5/vlang/` followed by `arbor/.integration/proposal5/semantic-capability-fabric/03-SEMANTIC-COMMENT-LANGUAGE.md` and `verifier/`.
4. Read `repos-hq/current/CHATGPT-SPRINT/SPRINT-02/prompts/LANE-2-COMMENT-RUST.md` and `arbor/.integration/proposal5/phase2/pre-g/S-semantic/` for Rust/JS parser and source mapping handoff.
5. Compare dated prior copies in `vivim-labs/arbor-source-reset-20261118/` before treating any later document as authoritative.
6. Read `arbor/source-evidence-c573411/` and `repos-hq/source-evidence-1e7a0db/` for pinned historical executable source and test snapshots; these are not live runtime files.
7. Only after historical reconciliation, use the living canonical protocol package above `.archive/` for R000 decisions and R001 implementation scope.

## Collection inventory

| Collection | Files | Provenance | Exact full-tree match |
|---|---:|---|---|
| Original VLANG product upgrade | 17 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| PWA convergence/U7 | 19 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| Rust live graph architecture | 106 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| Original research chat exports | 12 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| Consolidated old semantic-language-kernel | 5 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| Knowledge-base consolidation | 6 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| 2026-11-18 VLANG reset | 23 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| 2026-11-18 SCF reset | 58 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| 2026-11-18 semantic engineering reset | 17 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| 2026-11-18 VEC PWA reset | 13 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| 2026-11-18 VEC UI reset | 113 | `owenservera/vivim-labs@d880e7dba5dd` | Yes |
| ARBOR VLANG historical protocol | 33 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR semantic capability fabric | 79 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR semantic engineering convergence | 19 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR VEC/PWA | 17 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR VEC UI engines | 118 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR semantic pre-G | 15 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR AutoWiki pre-G | 9 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR foundational prompts | 11 | `owenservera/ARBOR@c5734110a584` | Yes |
| ARBOR HQ/Rust source evidence | 14 | `owenservera/ARBOR@c5734110a584` | Curated source subset |
| Original repos-hq kernel 2026-11-02 | 4 | `owenservera/repos-hq@2651bc8` | Yes |
| repos-hq Rust comments sprint | 7 | `owenservera/repos-hq@1e7a0db0dedc` | Yes |
| repos-hq parser/KB source evidence | 10 | `owenservera/repos-hq@1e7a0db0dedc` | Curated source subset |

## Interpretation rules

- A historical annotation claim is not automatically a verified symbol binding. Keep declared, observed, tested and inferred truths separate.
- Rust/JS/TS/JSX/TSX coverage may differ across snapshots. Inspect the exact parser and test version before claiming support.
- Host is the only effect/write/approval authority; AutoWiki and VEC are projections/validated computation, not alternate authority.
- The 2026-11-18 R000 package was scaffolded with no lane results at the inspected baseline; archival copies do not change that.
- Conflicting nomenclature around VEC, VLANG, VSM and native Rust should be preserved and reconciled, not silently normalized.

## Machine provenance

Use `COPY-MANIFEST.json` for source repositories, refs, relative paths, archived tree SHA, file counts and sizes. Git SHA equality covers all nested blob content and file modes of copied whole trees.

## Conversation coverage

Two source conversation exports were found and preserved in more than one original collection; the supplementary conversation chronology is a clearly labeled recollection, not a fabricated transcript.
