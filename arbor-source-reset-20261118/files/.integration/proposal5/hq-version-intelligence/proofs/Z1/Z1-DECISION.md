# Z1X Independent Convergence Decision

> **LATER GOVERNING SCOPE OVERRIDE, main@539146ac741516e7d9711c63f4570585b3c3367e — NATIVE TOOLING ONLY.** The 2026-11-12 HVI `10-NATIVE-TOOLING-ONLY-SCOPE.md` explicitly overrides earlier real five-repository/361 semantic audit and X-lane requirements. Preserve historical S0/D proof as reference only. **G1 tests entirely synthetic 2/3/5 invented repos, provides a stable native callable tool API; H/U only after `GO_FOR_W2_TOOLING`; X deferred until expressly requested; Z2/G2/R certify tooling/readiness, not migration corpus.** See `SCOPE-CORRECTION-ADDENDUM.md`. Any older contradictory text below is **superseded**.


**GATE: REPAIR_REQUIRED**

**Work branch:** `work/p5-hvi-w1-z-referee-r001`; created from the immutable S0 worker source base `f9823a5131e73e7562db5a4ec14e1ec481557d69`. **No worker source modified. No integration or main branch merge. No beta/devices/runtime acceptance.**

## What passed independently
- All S0/A–F GitHub branch SHA refs match the precise frozen heads; comparisons to worker base show **behind=0** and changed files entirely confined to their respective exclusive write roots. B legitimately has 23 commits, F 16.
- S0 has `GO_TO_PARALLEL`; source provenance for original HQ `676` tracked paths is fixed and verifiable.
- D's 361 HQ triage paths match S0's original 361 donor path/blob/source SHA/target SHA values exactly. There are **zero duplicates**. Source-content inspection only `23/361` (**6.37%**); the remaining **338** are `UNKNOWN_UNREVIEWED`, not confirmed feature loss.
- B donor C4 mapping is **14 sources: seven Git-blob identical, two adapted and five historical evidence/handoff excluded**. Port is source-only and preserves original schema.
- C and E each recorded isolated Node22 executable receipts, with 9/9 and 13/13 respectively. This does not verify cross-lane A/C schema nor actual Node24 integration.

## Why a GO_FOR_G1_REVIEW gate would be unsound now
1. A `arbor.hvi.manifest/1` immutable digest and C `arbor.hvi.source-manifest/1` `digestManifest()` cover materially different canonical payloads. Direct consumption **fails**, and aliasing the SHA strings would silently destroy integrity.
2. F comparison schema `arbor.hvi.comparison/1` is not C/E `arbor.hvi.version-comparison/1`; strict required root/pin/coverage/summary/sourceDelta/capabilityDelta/evidence/limit/pairwise types disagree. F's schema MUST NOT be marked valid for C output.
3. E's export from actual C reports loses `originPath` and `originBlob` due missing aliases and does not preserve full C per-donor coverage and candidate detail. D parity metadata and test confidence do not project directly into C/E.
4. B owner-confirmed equivalence is source declaration **not cryptographic proof**; C presently rejects any nonempty identity decision adapter. Runtime/Host grant use is unimplemented.
5. There is no integrated Node24 test, full schema conformance, no-donor replay or product acceptance receipt. A/D used V8 crypto shims, B/F Node24 tests were UNRUN; CI `startup_failure` is **not a pass**.

## Gate policy
- Source lanes: `PREPARED_NOT_PROMOTED`.
- Referee: **REPAIR_REQUIRED** (G1 may review and execute the surgical repair plan, but no compatibility/promotion claim is authorized).
- After G1 stages repairs, it must execute exact **Node24** combined tests, dual-digest verification, source provenance preservation, standards-compliant schema validation, synthetic 3-SHA/synthetic 5-repo tests, real no-donor offline replay, security negatives and source closure. A verified and reviewed integration PR is the only route to accepted main.
- Later W2 H/U requires accepted `GO_FOR_W2_TOOLING` main SHA; X is deferred until an explicit user request. Any staging without observed exits remains `G1_SOURCE_STAGED_TESTS_UNRUN` / `REPAIR_REQUIRED`.
- Existing beta release gate stays **BLOCKED_NOT_BETA**; binary/native/Windows/PWA/Android/Host security acceptance is separate and UNRUN.

**Do not run G1 from Z1. STOP at proof commit and handoff.**
