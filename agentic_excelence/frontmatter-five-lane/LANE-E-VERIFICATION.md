# Lane E — Independent frontmatter verification and virtual-agent evaluation

**Owns:** independent tests and acceptance over all 217 files, *no B/C/D source plans*. Begin by deriving test oracles from the existing corpus and provider-native documentation; never certify own generated instructions.

## Gates
**E0 completeness:** 217 unique file IDs; B 60 + C 79 + D 78; 19 file patterns, 8 observed families, 13 shared section types, 168 unclassified additions (or individually evidenced resolutions), and all 12 private source bodies kept private.

**E1 syntax:** parse native YAML for verified `SKILL.md` and OpenCode agent formats; ordinary Markdown must remain provider-readable; all 11 JSON records have valid JSON and reject unknown metadata injection. Sidecar validates against A's typed schema. Test unicode, duplicate keys, empty mandatory fields, unescaped delimiters and malicious YAML types.

**E2 semantics:** verify each field is source-pinned, correct file/family/role, correct section variant, non-invented tool/permissions/status and correct active source precedence. Inert source quote never becomes a command. Run source SHA mismatch and stale context rejection.

**E3 full authority/privacy:** no secret, cookie, private conversation, personal memory, file body from one of 12 private sources or live owner grant in public artifact. Security restrictions are hard filters, not retrieval weights. A sidecar never authorizes action.

**E4 simulated retrieval:** use existing LAB#2 experimental approach. Compare static DCB, dynamic DCB without frontmatter, dynamic DCB with frontmatter metadata, and dynamic DCB with frontmatter+LRU on the same test event traces. Test whether a coding agent gets the right instructions; a verifier gets current receipts; a personal assistant never gets denied private context; and a handoff never triggers duplicate effects.

**E5 performance/invalidation:** same pinned inputs produce identical projection digest, budget ≤ hard limit, LRU hit does not bypass freshness/ACL. Track per-file retrieval precision and recall, section-level diversity, token efficiency, queue churn, cache utilization, event→prepared-context p95 and correctness after user changes task.

## Required outputs
- `tests/corpus-frontmatter-receipts.jsonl`: per-file attempted/pass/fail/blocked reason with exact source digest and schema version.
- `tests/native-loader-fixtures/`: native parser fixtures, failed load cases, no accidental activation.
- `tests/virtual-agent-goldset/`: role/scenario/seed/event/gold source spans, hidden holdout set and model route.
- `E-INDEPENDENT-RESULT.md`: denominator-based verdict, broken assumptions, missing owner approvals and residual defects, not optimistic pass language.

## Acceptance semantics
A file can be `DESIGNED` with unresolved native compatibility; `VALIDATED` only with actual parser/semantic tests; `LIVE` only with a separate owner-authorized real runtime observation. Do not mistake a GitHub commit, test fixture or documentation URL for runtime execution.
