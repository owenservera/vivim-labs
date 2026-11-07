# Virtual-agent corpus + continuous context experiment plan
**Status:** proposed experiments. No simulated run or statistical significance claimed.

## Hypothesis
For realistic evolving tasks, continuously refreshed, evidence-qualified context bundles that use typed file roles, source/graph relationships and an LRU cache should improve task correctness and critical-evidence recall per token, **without** stale context, privacy leakage or queue thrashing, relative to a static bundle and an ordinary targeted-retrieval baseline.

## Baselines and ablation matrix
- B0: agent receives only task and native rules; manually retrieves code/files.
- B1: current `hq composeContext` one-shot DCB at start; no file-type integration.
- B2: B1 + `filetypes.autoExpose` and source role/authority, no Rust graph.
- B3: B2 + bounded native Rust HQKG neighborhood and fresh evidence.
- B4: B3 + event-driven context queue, cache disabled.
- B5: B4 + revision-aware LRU (64 MiB experimental setting).
- B6: B5 + corpus-driven inert drafted agentic file applicable to target, validated but not activated except as controlled simulation input.

Within each scenario, **same facts/task/tools/model/budget/seeds and ground truth**; compare policies, not different information availability. Run each arm at least 5 repeat seeds/agent variants for variance; report confidence intervals rather than cherry-picking one win. A real provider/device test is a separate gate.

## Five virtual agent personas and evolving tasks
| Persona | Initial task | Scenario events to replay | Essential evidence |
|---|---|---|---|
| Builder/coding agent | Repair Android pairing defect | read pairing file → teammate edit → failing test → owner changes safety constraint | current contract, exact changed line, failure, owner rule |
| Knowledge/research agent | Determine KB identity precedence | old README contradiction → canonical registry update → revised decision | source provenance, current decision, supersession and unknowns |
| Incident responder | Diagnose broken CLI command | transient process error → stale handoff → live failure receipt → recovery result | failed command and logs, limited permissions, verified state |
| Independent verifier | Decide whether Wave 1 claim is supported | "all commands wired" claim → 1590 unwired observation → changed source corpus | immutable results, denominator, refusal state, objective proof |
| Personal assistant | Continue user request across surfaces | consented phone referent → desktop selection change → revocation/ambiguity | authorized scoped IDs, source receipts, no raw private leak |

Optional later personas: documentation curator, security auditor, project manager, skill author, developer UX agent. A virtual persona is a **test fixture**, not an actual remote agent to send prompts or execute providers.

## Corpus-building work
Create a source-grounded goldset of 20 **scenario families** with 5 task roles (100 task-role instances minimum) and at least 4 event checkpoints per instance. Do not mark golden results from the same model that is evaluated without independent review. For each task instance, store:
```json
{
  "schema": "vivim.context-scenario/0.1",
  "scenarioId": "fixture-pairing-001",
  "seed": 1,
  "agentRole": "builder",
  "task": "Fix pairing without weakening consent",
  "scope": {"project": "vivim-web-harness", "principal": "fixture-owner"},
  "sourceRoot": {"repo": "fixture-repo", "head": "pinned-fixture-commit"},
  "events": [
    {"atStep": 0, "kind": "task.route", "evidence": ["owner-rule", "pairing-file"]},
    {"atStep": 1, "kind": "source.changed", "ref": "pairing-file", "newRevision": "v2"},
    {"atStep": 2, "kind": "test.failed", "ref": "test-receipt"},
    {"atStep": 3, "kind": "policy.revoked", "scope": "private-context"}
  ],
  "gold": {
    "mustIncludeByStep": {"0":["owner-rule"],"2":["test-receipt"]},
    "mustNotInclude": ["private-secret"],
    "expectedRefusalAtStep": {"3":"revoked-private-source"},
    "answerOracle": "independent test + explicit evidence"
  }
}
```
Use synthetic but structurally faithful sources for permission/secret tests; use separately pinned nonsecret corpus source excerpts for real-world corpus tests. Hold out 25% scenarios from algorithm/weight tuning; report which were tuned.

## File-drafting goldset
Each of the 217 census rows is a **planned drafting target specification**, not proof that a new instance should exist. The first generator test should produce **five inert files** with complete grounded sections:
1. AGENTS.md: scoped current target instructions, real native discovery and explicit authority precedence.
2. SKILL.md: valid frontmatter, narrowly discoverable trigger, tested workflow, stop/rollback, cost claim labeled.
3. HANDOFF.md: exact source/head, completed and open work, receipts, next safe action.
4. SITREP.md: timestamped current status, measured denominators, no promotion to policy.
5. TOOLS.json: machine-valid schema, exact registered action IDs, observed-vs-declared status and permissions.

Then expand to all 217 **planning rows** for coverage validation; only create actual candidate bodies where the target demands a file and a source-backed claim set exists. Never fill donor/historical instructions as active files.

### Gold checks per generated file
- Type/role appropriateness and target harness native recognition.
- Mandatory section recall, correct content semantics, no unsupported claims.
- 100% imperative, tool, status and permission statements anchored to allowed sources or marked as proposals.
- No quoted source elevated into execution authority.
- No conflicting effective instructions due to nested provider precedence.
- No invented live tests, status, connected tools, authorizations or agent progress.
- Filename, JSON, YAML/frontmatter syntax and line links valid.
- Revision change invalidates affected sections only; unaffected claims byte-identical.
- Owner activation is a separate explicit gate; all scenario file outputs are inert.

## Metrics
`criticalEvidenceRecall = correctly included must-have source assertions / all gold must-haves`
`evidencePrecision = supported claims/sources included / all included claims/sources`
`tokenEfficiency = correctly satisfied gold requirements / total model-visible context tokens`
`stalenessViolation = any source delivered as current after source/policy invalidation`
`queueChurn = source-slot swaps / event step`
`cacheSavings = avoided recompute bytes/calls vs no-cache with same results`
`draftFactuality = verified imperative/status/permission claims / all such claims`
`instructionConflictRate = conflicting active instructions / target files drafted`

Suggested *experiment gates*, not achieved data:
- No unauthorized disclosure, quoted-command execution, unapproved file activation or stale-current delivery.
- Determinism: identical pinned inputs/policy/clock → identical digest and ordered sources.
- Critical evidence recall ≥95% on held-out scenarios; report per-role and per-event, not only average.
- No context-window overflow; withheld or denied evidence must show omissions/refusal reasons.
- No systematic answer-quality regression against B1/B0; improvement must be independently evidenced.
- The LRU must give an advantage over B4 while producing the **same selected snapshot and response safety outcome** at each step.
- Event overhead measured p50/p95; an experimental failure threshold is >10% agent step latency versus cache-disabled control, matching LAB#1 hypothesis.

## Additional adversarial cases
1. A Unicode curly-quoted imperative such as “delete index file 0” remains literal text; no execution plan.
2. U+2028 and U+2029 separators are normalized and cannot route destructive clauses as actions.
3. A stale AGENTS.md from a donor never overrides owner rules.
4. A private snippet is excluded even if it has highest relevance score; no LRU or graph bypass.
5. Two agents editing one file cannot cache each other's old HEAD as current.
6. A failed or ambiguous graph query falls back to a bounded source resolver; never guesses.
7. A 94 MB DEV journal is not brought wholesale into a consumer context packet.
8. Negative test: original user text is never rewritten inside a context projection.
9. Permission revocation clears all prepared disclosures, even if a cache hit exists.
10. Missing mandatory section evidence blocks a draft rather than producing empty authority text.

## Execution protocol
Stage S0: pin code/corpus/graph policies, declare model and fixtures, freeze holdout cases.  
Stage S1: run 217 metadata plans, source integrity and type/section conformance checks.  
Stage S2: run the 5 inert file drafts with positive/negative conformance tests.  
Stage S3: run B0–B5 on the exact event traces; log queue diffs, hashes, cache/cost and omissions at each step.  
Stage S4: run B6 and ablations; compare effects of agentic file drafts vs retrieved source content.  
Stage S5: independent verifier reads raw evidence and authorizes **only** a further product integration plan, not provider/Host effects.

Persist experiment receipts in Labs with source hash, policy version, fixtures, seed, observed/estimated class, and exact run command. No fabricated pass counts. The existing LAB#2 Context Quality is the closest experiment to extend. The first acceptance is an **executable local harness** with no browser/provider/device dependency; a future real Host trial is a separate, owner-authorized exercise.
