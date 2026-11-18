# G1 source and import closure — static candidate evidence

Source base: f9823a5131e73e7562db5a4ec14e1ec481557d69
Integration starting main: aece3618395f215d825bf4eb41dcc07d6d656ab7
Current G1 code head: 0dfb08406c50df2e385c0ff924694e0aeaa33565
Candidate tree: 6a78da85d5a661e090c77ce20d8cedf3e2d544eb

All six worker heads were verified against the source base, behind=0, then imported serially A/B/C/D/E/F as exact worker object blobs with secondary parent preserving frozen worker history. No worker path overlap or direct main mutation was observed. A=12, B=17, C=4, D=9, E=21 and F=32 changed files; S0=6 and Z1X=14 evidence files imported separately.

GitHub compare from starting main to code head: status=ahead, ahead=81, behind=0, changed paths=130. Counts cover inherited worker commits and supporting proofs, not 81 new G1 features. The imported G1 branch includes history merged from all worker parents. No GitHub ref was force-updated.

13 selected source/test modules under HVI were inspected against a full candidate tree index (1904 entries at the earlier G1 code head) for relative import closure: missing relative imports=0. Built-in Node imports are the main runtime dependency; isolated strict schema test additionally requires dev-only ajv/dist/2020.js at version 8.17.1. Static import existence is not proof of load/compile success.

G1 seam files are only under hq/version-intelligence/integration/, contracts/hq-version-intelligence/schemas/v2/, tests/hq-version-intelligence/, integrator CI workflow and G1 proofs/report. Worker A-F implementation sources are not overwritten. The older F v1 comparison schema remains intact and cannot be silently accepted as C's canonical arbor.hvi.version-comparison/1.

No donor Git checkout, real five-repo migration analysis, current 361-path semantic reclassification, personal corpus, provider session or browser action was performed. Native/local Cargo/Bun locks remain uncreated; existing integration/beta-gates.json is untouched and BLOCKED_NOT_BETA.

STATIC INSPECTION ONLY: full Node24 import/module closure, runtime behavior, tamper negatives and source-security acceptance remain UNRUN.
