# Workstream map — not yet parallel launch lanes
| ID | Workstream | First implementation outputs | Candidate owner |
|---|---|---|---|
| W0 | Source/baseline/overlap | exact pinned inventory, architecture map, branch gate | S0/G |
| W1 | Agent/source protocol | minimal file/function tags, many-feature IDs, waivers | D HQ |
| W2 | Language/compiler | v1/v2 compatibility, opt-in v3, typed registry + SC-ABI, incrementality | D HQ |
| W3 | Specs/oracle roots | reviewed ClaimSpec and predicates, versioned schema, approval | F independent QA |
| W4 | Independent checker | AST source witness, claim checking, negative test truth | F separate from D |
| W5 | Runners/build identity | safe Node/Rust tests, mutation/falsification, artifact provenance | A + F |
| W6 | Runtime witness ports | read-only Host/Studio/TierFlow auth and event completeness | B/C/E |
| W7 | Explanations/AutoWiki | clause-bound narrative, verified Wiki and uncertainty | D + E |
| W8 | HVI semantic history | 2..N proof delta, migration equivalence and regression | HVI owner + D/F |
| W9 | Developer interface/SDK | native agent discovery, CLI/CI/IDE/optional UI, versioned SDK | D/B/E/F/G |
| W10 | Safety and resilience | source permissions, secret redaction, ENOSPC, bounded storage | A/B/D/F |
| W11 | Refereeing/integration | independent adversarial checks, serial G, final R | Z/G/R |

## Candidate thin vertical slices (sequence provisional)
S1 source comment → compiled fact → independent AST witness → wiki claim;
S2 known-false pure function → reviewed property → REFUTED counterexample;
S3 real in-repo pure function → source/build-pinned SUPPORTED_BY_TESTS;
S4 synthetic Host action with authority-origin read-only receipt and denial;
S5 synthetic three-commit HVI semantic/proof delta; no real donor audit;
S6 agent edit → compiler/verifier/CI → passport + optional visual drill-down;
S7 external SDK example uses stable pure/verification ABI without external VIVIM checkout.
Actual wave count and concurrency determined next pass after source-owner collision audit.
