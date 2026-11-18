# S0 source ownership and trust-root collision resolution
## One-writer map (provisional pending review)
| Owner | Exclusive future target | Conflict/constraint |
|---|---|---|
| SCF A | `hq/lib/annotation-v2.mjs`; new `hq/lib/semantic-capability/{comment-v3,source-observation,compiler}.mjs` | Existing HQ parser/indexer: additive scanner only; do not edit `extract.mjs` or `indexer.mjs` without owner review |
| SCF B | `semantic-registry.mjs`, `schema.mjs`; new SCF store/registry adapter | Existing HQ semantic ID and SQLite owners must approve additive kinds, migrations and rollback; no second authority |
| SCF C | passports/HVI adapter; optional `hq/lib/wiki.mjs` projection | Existing single Wiki generator; HVI comparison remains HVI-owned |
| SCF D | verifier/proof/explanation adapters | Independent of code author A/B; original SV schema is read-only until separate independent approval |
| SCF E | `hq/lib/protocol.mjs`, `hq/bin/arbor-hq.mjs`, `hq/lib/arbor-service.mjs` | **Collision:** HVI H reserves `arbor-service.mjs`. No E edits until HVI H/G owner reviews one-writer handoff |
| SCF F | `hq/test/scf-*.test.mjs` | F authors independent negative oracles; A/B/D cannot self-approve tests |
| SCF H | `apps/vivim-host/**` | **Collision:** HVI H/Host. Defer until accepted HVI Host grant and separate SCF G1 product gate |
| SCF U | `packages/tierflow/**` | **Collision:** HVI U/single UiCore publisher. Defer; reuse one renderer and revision guard |
| SCF G | root scripts/integration lockfiles and serial merged fixes | Sole reviewed PR integrator; S0 cannot edit |

## PR13 vs SEVP PR19 role decision
PR #13 is the single detailed SCF+SV code implementation order (A/B/F→C/D/E, independent Z, serial G). PR #19 is **umbrella strategy only**; its proposed D=compiler and F=verifier must NOT launch as competing runtime workers against SCF A/B/D/F. Its SDK/language/runner research remains post-SCF separately scoped work. Program G0–G8 names are not HVI/SCF G gates. Both PRs remain unmerged design.

## Independent verifier trust-root requirements (specified, not attested)
1. **Verified Git source root:** approved repository + immutable commit/tree/blob bytes; no caller-provided content digest accepted without source adapter verification.
2. **Registry/authority root:** reviewed canonical ID and owner catalog; comments are untrusted declarations, never action grants.
3. **Spec/oracle root:** independently approved immutable ClaimSpec/Oracle/Model, `approvedBy.ownerRef` plus `decisionRef`, bound to reviewed spec digest; prohibit code-author simultaneous unilateral spec weakening. Missing approval → BLOCKED.
4. **Checker root:** independent AST/symbol re-derivation, separate checker executable hash, verifier version, cross-version downgrade/revocation and mutation negatives. Schema parse alone and compiler results cannot certify themselves.
5. **Runner root:** Node24, strict JSON Schema Draft 2020-12, isolated synthetic/pure execution, exact test artifact hash, runner attestation, observed command/exit/coverage and a deliberately wrong implementation producing `REFUTED` with replayable counterexample. Finite tests → `SUPPORTED_BY_TESTS`, not universal proof.
6. **Host effect root:** only authenticated owner-origin actor/session/live-grant/revision receipts justify effect claims; absence of telemetry never proves absence of effects. Recheck grants at every read, page and cached projection.
7. **Explanation root:** each rendered clause bound to same pinned source/spec/build and accepted claim receipt; modality cannot become stronger in summaries/translations.

**Unresolved approval:** No signed/attested independent oracle/checker/runner policy or accepted spec-review root was observed in canonical main. SV owner/F independent reviewer must ratify, and G must reconcile the proof receipt schema without cloning competing schemas.
## Required HVI handoffs
HVI G: run Node24 combined synthetic/AJV tests with real exits and independently accept/merge; check latest moving integration candidate `0539d23404ef5b1de12ad646d807a2fda57c6ab6`. HVI H security: real live development-source grant provider and read-time revocation. HVI U: only sanctioned safe typed report, true UiCore current revision. HQ owners: approve v3 grammar/schema/wiki seams. No worker starts while S0 blocked.
