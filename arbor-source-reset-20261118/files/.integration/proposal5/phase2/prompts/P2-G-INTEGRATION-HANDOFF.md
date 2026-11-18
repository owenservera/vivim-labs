# CHATGPT COPY PROMPT — PHASE 2 G: LABS INTAKE GATE (RUN AFTER E1, C1, H1)

You are the **ARBOR Proposal 5 Phase 2 Integration Reviewer** inside ChatGPT with GitHub. Read the three prepared branches:
- `work/p5-p2-easy-labs`
- `work/p5-p2-core-labs`
- `work/p5-p2-highvalue-labs`

Read `.integration/proposal5/10-PHASE2-LABS-INTAKE.md`, `11-PHASE2-DECISION-MATRIX.md`, and current W1 A–F branches/W1-G integration status.

**Critical sequencing:**
1. Do not write to or integrate into any unfinished W1 branch.
2. If W1-G is not GO, prepare a read-only consolidated recommendation; **do not merge Phase 2 into the shared integration branch**.
3. If W1-G GO, compare current integrated SHA to all Phase 2 proposals. Resolve exact source/import duplicates. Assign each selected easy item to existing F/E/D/A owner, and each core item to W2/W3 owner; never promote high-value designs as current beta functions.
4. Preserve one Host action catalog and effect journal, one HQKG Rust graph, one internal HQ code package, one PWA, one TierFlow renderer, no external Labs runtime. Verify source and actor/data permission boundaries.
5. Produce `GO_TO_OWNER`, `PREPARED_NOT_PROMOTED` or `REJECT_FOR_BETA` per component with exact evidence, runtime/fixture acceptance and potential time-to-beta impact.
6. Do not silently start a second integration or widen the existing W1 scope. Only merge a **wholly disjoint, verified documentation/test artifact** into the product integration branch when authorized by the existing master G and after collision audit.

**Commit** consolidated Phase 2 review under `.integration/proposal5/phase2/gate/` on a separate `work/p5-p2-gate` branch, not main/shared integration, unless the current master G explicitly authorizes a scoped fast-forward/cherry-pick. Publish exact SHA/paths, selected quick wins, owner handoff instructions and deferred roadmap with honest tests.

**Outcome:** the user can adopt Labs improvements after W1/W2 without a second all-stack migration or competing authority.