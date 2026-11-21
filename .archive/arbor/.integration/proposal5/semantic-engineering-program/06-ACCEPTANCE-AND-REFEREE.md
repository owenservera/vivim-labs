# Acceptance and falsification framework — draft
Proposed gates:
G0 BASELINE_PINN (pins, existing code/test status, conflicting branches, owner authority);
G1 CONTRACTS_REVIEWED (source language, typed ABI, oracle acceptance/trust roots);
G2 COMPILER_FACTS_CHECKED (independent AST verifies tags/relations);
G3 VERIFIER_FALSIFIES (known-false code and false prose correctly rejected);
G4 PURE_EXECUTION_ACCEPTED (actual safe runner and build identity, recorded tests);
G5 AUTHORITY_OBSERVED (read-only origin receipts, grants/denials/replay/uncertainty);
G6 KNOWLEDGE_PROJECTIONS_ACCEPTED (Wiki/HVI/agent use checked facts);
G7 SDK_ADOPTION_ACCEPTED (CLI/CI/SDK and staged source policy);
G8 READINESS_REVIEW (security/low disk/rollback/no-donor proof). These are placeholders, not HVI G1/G2 authorizations.

Each worker receipt: BASE_SHA, SOURCE_PINS, BRANCH, EXACT_SHA, CHANGED_PATHS, INPUT_ABI, TEST_COMMAND/ENVIRONMENT/EXIT, UNRUN, FALSE-CASE RESULTS, CLAIM_VERDICTS, COVERAGE/ASSUMPTIONS, AUTHORITY/PRIVACY, BLOCKERS, HANDOFF, GATE. Independent Z reviews actual source/tests before G stages. R cannot declare runtime GO from synthetic-only fixture evidence.

Acceptance matrix must include positive, known-wrong, missing-evidence, stale-source/build and permission-denied cases. Inject bogus tags, forged tests, weak oracles, changed binaries, empty incomplete effect logs, false "always" prose, mutation survivors, secret leaks, revoked grants, stale receipts and ENOSPC. A verifier's own passing tests are insufficient unless it detects seeded defects.
