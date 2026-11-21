# Pre-development readiness — actual blockers vs prepared work

Overall state: `PREDEV_CONVERGED_WITH_OPEN_GATES`, NOT `PREDEV_COMPLETE`. Strategically aligned does not equal source/ABI/test acceptance. Do not launch SCF A–F based only on this document.

| Gate | Current | Why it matters |
|---|---|---|
| Exact baseline/source pins | OBSERVED_NOT_FROZEN | Main and drafts were inspected; S0 still must reconcile newer commits and freeze workerBaseSha. |
| Exclusive source owners | OPEN | PR13 A/B/C/D/E/F/H/U and PR19 ambitions collide unless one writer owns each path. |
| Trusted SV oracle/spec roots | OPEN | Author cannot silently rewrite the specification or checker and then certify own source. |
| Schema/semantic registry v3 | OPEN | Existing semantic kinds/version schema cannot be presumed to support all new IDs. |
| HVI accepted live ABI | OPEN | HVI native tooling has independent G/Z gates. Observed branch head is not acceptance. |
| Executable Node24 + negative tests | UNRUN | `hq/package.json` requires Node >=24; this remote review cannot claim actual exits. |
| Host revocation + TierFlow guard | NOT_ACCEPTED | H/U remain downstream, and separate PWA/device security may stay blocked. |
| Falsification vertical slice | NOT_IMPLEMENTED | Demonstrate deliberate wrong code + counterexample before trusted explanations. |
| Rollback and reproducibility | UNRUN | Interrupted transactions, ENOSPC, Git pin mismatch and stale receipts require actual tests. |

## S0 acceptance packet (before any code lane)
1. `source-sha-lock.json`: main/SCF/SEVP/HVI exact immutable heads, drift diff, accepted source owner, exact `workerBaseSha`.
2. `ownership-map.json`: one source writer per path, including interfaces with active HVI workers.
3. `trust-root-decision.md`: ClaimSpec and oracle approvers, independent verifier build/checker pins, downgrade/revocation and mutation test policy.
4. `abi-compatibility.md`: real `annotation-v2`, `semantic-registry`, `schema`, `arbor-service`, Wiki, accepted HVI native ports and action discovery.
5. `test-prerequisites.json`: environment/Node version, path existence, runnable commands, exact exit receipts or UNRUN reason.
6. `gate-decision.json`: independent S0 acceptance or named blockers; no bootstrap merge.

## Proof ladder
The first accepted slice needs an actual negative test that causes `REFUTED` and a minimal replayable counterexample under a reviewed oracle. If all it can show is a passing finite test, classify as `SUPPORTED_BY_TESTS`; if source/artifact/spec trust is missing, `BLOCKED` or `UNKNOWN`. That separation protects every downstream Wiki claim.

All HVI comparison development and testing in this wave must use **invented synthetic Git fixtures**. Do not run real ARBOR, VIVIM or Labs comparisons until a later explicit user-requested, authorized operation.
