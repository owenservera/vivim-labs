# Handoff, serial integration, feature flags and recovery
## Current state
PR #13 draft contains SCF-R011 design and SV-R012 design contracts; this implementation-ready kit extends that PR. It is documentation only. ARBOR main currently advancing through HVI. Source implementation must be forked from **a fresh verified accepted main** and read this draft as instruction, not blindly merge its stale parent source revision.

## Precise order and responsibilities
S0 reconcile current main, HVI accepted API + actual source and freeze workerBaseSha. If HVI still BLOCKED/UNRUN, S0 produces source-only preparatory evidence and stops promotion.
A parser/core and B registry/storage can run parallel against S0 base; F prepares independent fixture/tests parallel. C passports/wiki, D SV and E agent/CLI/service require A/B contracts but may work on their own independent branches after frozen interface contracts. Z1 referee verifies all exact branch heads, source pins, exports, negative security and anti-duplicate ownership. G1 integrates A→B→C→D→E→F serially on a dedicated integration branch with lease checks. H/U product read surfaces follow only after G1 accepted and current Host security gates. Z2 and G2 independently validate/integrate; R issues source/truth readiness receipt, not beta GO.

## Branch names (reserved proposal; create when actually implementing)
- `work/p5-scf-s0-lock-r001`
- `work/p5-scf-a-compiler-r001`
- `work/p5-scf-b-registry-r001`
- `work/p5-scf-c-passport-r001`
- `work/p5-scf-d-verifier-r001`
- `work/p5-scf-e-agent-surface-r001`
- `work/p5-scf-f-qa-r001`
- `work/p5-scf-z1-review-r001`
- `integrate/p5-scf-r001`
- `work/p5-scf-h-host-r001`
- `work/p5-scf-u-tierflow-r001`
- `work/p5-scf-z2-review-r001`
No feature worker may update main or another lane. G is only merge authority, and must use reviewed PR/actual test exits.

## Feature flags / compatibility
- `SCF_ENABLED=false` at product service initially. Opt-in local diagnostic possible on owner-approved development roots after source tests.
- `SCF_V3_ENABLED=false` until parser/registry+independent negative acceptance. Existing v1/v2 unaffected by default.
- `SCF_HOST_READ_ENABLED=false` until H security and native HVI tool registry accepted. No remote privileged PWA bypass.
- `SCF_UI_ENABLED=false` until TierFlow publisher/revision guard and Host read accepted.
- When disabled: no new DB schema needed on unrelated app boot if HQ service unused; existing CLI/wiki behavior, output and tests unchanged.
- Once enabled: partial scan visible as incomplete, no autopromotion of capability/permission. No new branch that owns source truth.

## Merge gates
- S0: `GO_FOR_IMPLEMENTATION` only when workerBaseSha and source interfaces verified; `BLOCKED_SOURCE_DRIFT` otherwise.
- A/B/F: `PREPARED_NOT_INTEGRATED` after scoped commits and real tests/UNRUN.
- Z1: `GO_FOR_G1_REVIEW` only source/ABI/authority compatible and no unchecked blocker.
- G1: `GO_FOR_PRODUCT_WIRING` only after end-to-end Node24 tests, no-donor synthetic HVI, SV independent failure and source closure with executed exits.
- H/U: `PREPARED_NOT_PROMOTED` until independent Host refusal/revoke and UI integrity proof.
- Z2: `GO_FOR_G2_REVIEW` only with evidence for current grant and no new effect writer.
- G2: `SCF_TOOLING_READY` only if native service, CLI, wiki and authorized UI paths are actually tested. Does not authorize real repository comparisons or beta release.
- Any `UNRUN` required gate => `BLOCKED` or `PARTIAL_SOURCE_PREP`, not GO.

## Conflict and rollback
1. Inspect main new commits/PR heads and compare source paths before every integration; on conflict stop, reconcile by exact owner, avoid force push.
2. Retain source branch/source manifest and original accepted main revision. G PR merger records merge SHA and tested tree SHA.
3. Roll back by reverting G merge via reviewed PR and disabling feature flags; leave immutable old source/receipts untouched.
4. Derived SCF tables/caches may be discarded in bounded cleanup after backup; never delete user World, original HQ index tables, private owner data or committed source history.
5. If experimental parser changes v1/v2 outputs, revert parser extension first; do not hide mismatch with changed golden tests.
6. If SV catches a false proof or explanation, surface as REFUTED and disable semantic claim promotion; do not remove counterexample.
7. If HVI native service incompatible, keep SCF passport-only read path disabled and issue exact HVI owner handoff, not implement duplicate comparison logic.
