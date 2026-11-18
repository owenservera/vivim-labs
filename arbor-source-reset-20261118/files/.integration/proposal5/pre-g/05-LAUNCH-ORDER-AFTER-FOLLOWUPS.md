# Proposal 5 — Updated Launch Order After First Follow-Up Completion

**Status:** As of latest GitHub inspection, W1 source landed in all six lanes; integration branch absent. Phase 2 has an isolated readiness audit, not merged product source.

## Current instructions

**A — no new prompt.** Current A `work/p5-w01-a-native` at `dda546a11294920d6521a0f64ab7ee62ec993c70` already committed `PREG-A.md` with FREEZE_READY. Preserve this exact SHA unless an explicitly coordinated bug fix changes it. Native security remains unimplemented/UNRUN.

**B–F — send exactly one final short freeze prompt in existing ChatGPT conversations, optionally in parallel:**

| Lane | Latest observed HEAD | Prompt | Purpose |
| --- | --- | --- | --- |
| B Host | c12f4a27a8c700c9df4cba17a9ff96efd5897748 | [B final freeze](prompts/R2-B-FINAL-FREEZE.md) | Host/PWA static asset, A/B launch contract, effect authority |
| C Studio | 4bca551d58a1972cc02a8b20b772e1e2417b874d | [C final freeze](prompts/R2-C-FINAL-FREEZE.md) | Missing complete Studio/harness source inventory |
| D HQ | aa6284812432cbc375bbe6b1f94bb867a0e72cfa | [D final freeze](prompts/R2-D-FINAL-FREEZE.md) | HQ authorization source patch, owner adapter, Node24 |
| E PWA/TierFlow | 4ed530f1702e9ce3f592e2ae1a7080fa36e5f93d | [E final freeze](prompts/R2-E-FINAL-FREEZE.md) | Browser WebCrypto signer already written; ensure Host compatible patch handoff |
| F Contract/QA | 7538ebc706f98c15530fe39ba3a40b479448b37f | [F final freeze](prompts/R2-F-FINAL-FREEZE.md) | Acceptance gate/Host catalog and no duplicate source truth |

## Launch three parallel Phase 2 preparation lanes NOW (not blockers for G)

| Phase2 chat | Direct ChatGPT prompt | Output |
| --- | --- | --- |
| **P2-T** tests/source truth | [P2-T preparation](prompts/P2-T-PREP.md) | Observatory/voice/R005/Chronos test fixtures and small regression contracts |
| **P2-K** personal knowledge T=0 | [P2-K preparation](prompts/P2-K-PREP.md) | Source-backed narrow migration DAG, consent/checkpoint/graph/Host interface and negative tests |
| **P2-C** native/Chronos/UI interop | [P2-C preparation](prompts/P2-C-PREP.md) | Rust/TS/Node0/Chronos reuse crosswalk, pure actor/source/revision fixtures |

Every Phase2 chat creates a separate branch from `work/p5-pre-g-convergence-r001`, writes only to its own `.integration/proposal5/phase2/pre-g/` staging subtree, and does not merge any product files. They can continue while G merges A–F, provided G has checked frozen worker commits and owners. No new runtime service, action catalog, graph store or speculative UI features.

## One serial freeze referee and G

Once B–F commit PREG-B through PREG-F and explicitly STOP WRITING, use [independent pre-G referee](prompts/H-PREG-REFEREE.md), then [G read-only preflight](prompts/G0-PRE-INTEGRATION-HANDOFF.md). These are source audits, not merges. When READY_FOR_G, execute the full W1-G merge prompt in the existing G conversation.

**Do not wait for P2-T/K/C** before launching the full G source merge. Their staged outputs are optional inputs to the later W2 owner waves after source collision review.

## Risks G must not lose

1. A/B native Tauri/Host bootstrap persistent token risk is unpatched; it requires actual authenticated per-process launch and negative tests.
2. E's browser PWA key path is a real staged implementation, but the B Host static allowlist patch is **not applied** and real browser/device acceptance is UNRUN. The active-session bearer replay risk still needs B/F policy.
3. C's exact Studio migration report is missing; incoming PREG-C should repair the evidence gap.
4. D HQ's owner-authenticated read adapter has source changes but B's Host grant integration isn't mounted.
5. F test-only Host v6 catalog is not an independent action authority.
6. G must resolve root Cargo/Bun/Node24, missing generated APK, Host support files, donor-offline binary/package proof and no-donor runtime.
7. Phase 2 source readiness 44 components includes pure tests and planned architectures; none is automatically shipping.

**Goal:** source-preserving one-time beta integration, not a feature sprint.
