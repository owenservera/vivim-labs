# Define and measure the “90% of normal commands” target

**The 90% target is currently an aspiration, not a measured statistic.** A semantic catalog of 279 intent classes cannot establish any percentage of user requests supported. There are at least four separate coverage questions.

## 1. Four distinct metrics

1. **Intent recognition:** Did the interpreter select the correct user-intended semantic action and slots, with safe ambiguity detection?
2. **Grounding:** Did it resolve the correct concrete source, device, file/contact/window/version and scope without guessing?
3. **Safe plan acceptance:** Is the proposed plan legitimate, correctly risk-rated, allowed or explicitly refused/clarified when necessary?
4. **Verified task completion:** Did actual authorized effects produce the requested user-observable result and exact Host receipt, or correctly report that the action is unavailable? A polite refusal is *safe handling* but not task completion.

**Primary release metric:** real user tasks completed as requested among tasks in the product's declared capability scope **without unsafe effects**. Report the denominator and separately classify unsupported, unauthorized, ambiguous and unsafe requests. A request that requires user approval is complete only after the approved action is executed/verified, not when a preview is displayed.

## 2. What 90% means operationally

For a consented, anonymized evaluation set `U` of *actual sampled user requests*, with independently labeled intent, destination, expected effect and device, define:

`weighted_verified_completion = Σ(weight_i * verified_completed_i) / Σ(weight_i)`.

Report at least:
- `in_scope_weight` and `out_of_scope_weight` (never bury the latter).
- `intent_top1_accuracy` and `grounding_accuracy` on unambiguous tasks.
- `safe_clarification_rate` for ambiguous tasks.
- `false_action_rate` (wrong target, unexpected mutation, permission escalation, fabricated receipt).
- `end_to_end_authorized_completion_rate` **including** user-approved effects and observed outcomes.
- `time_to_first_useful_result`, result quality, cancellation and error recovery by domain/device.
- Counts and confidence intervals (Wilson intervals for binary outcomes); zero serious unauthorized effects is a hard requirement rather than an aggregate tradeoff.

**Target statement:** `>=90% weighted verified task completion on agreed real-world in-scope benchmark` plus `0 critical unauthorized effects`. Also publish all-user denominator and hard-case failures. Do **not** inflate success by silently removing difficult requests from the benchmark.

## 3. Build the corpus honestly

**Seed, not proof:** `eval/seed-utterances.v0.jsonl` has one illustrative positive utterance for each of 279 candidate IDs plus ambiguity/attack probes. These are authored design strings, *not* representative user traffic, and are not labeled “tests passed.”

**Real corpus plan:**
1. Opt-in user collection or locally labeled on-device prompt inventory; do not upload raw files, path names, contacts, browser profiles or personal secrets.
2. At least 1,000 diverse, consented utterances for a first exploratory baseline, with 2,000–5,000 as a stronger diversity goal; sampling should include novice, power-user, desktop, mobile and multiple locales. These are suggested collection sizes, not statistical guarantees.
3. Stratify by source domain, user frequency, device, language, risk, complexity (single-step vs DAG), target ambiguity, and online/offline constraints.
4. Independent human annotation of intended result, entity targets, whether authorized, and acceptable refusals; disagreements adjudicated and tracked.
5. Hold out truly new users and new wording. Prevent leaked paraphrases, canned examples and repository docs from dominating the evaluation set.
6. Replay non-effect examples using deterministic recorded OS snapshots in *tests* only. **Final verified-completion tests require actual user-owned OS work or a safely controlled instrumented environment**, not invented result data.
7. Run adversarial prompt injection, dangerous path, race/revision, stale process, offline device and provider-side unknown-effect cases independently.

## 4. Suggested threshold gates (proposed, not met)

| Stage | Objective | Evidence |
|---|---|---|
| A: Semantic seeds | Every catalog example maps to expected candidate or explicit safe nonexecution | Reproducible test runner + exact counts |
| B: User language beta | >90% top-1 recognition **for labeled unambiguous in-scope intents**, with documented uncertainty | Held-out labeled set |
| C: Grounding | >95% exact target agreement where sufficiently specified, otherwise clarification | Real file/device/window selection cases |
| D: Safe plan | 0 critical unauthorized effects; no silent R3 execution; 100% approval binding for writes | Threat and regression runs |
| E: Product 90% | >=90% frequency-weighted verified successful tasks *within predeclared supported domain scope* | Human acceptance + real Host receipts + denominator |

Treat metrics as independent. For example, 95% parse accuracy with only 25% adapter availability is not 90% product usefulness.

## 5. Dangerous denominator games to avoid

- Counting 279 defined intents as 279 implemented actions.
- Counting a generated answer/plan as successful filesystem mutation.
- Counting repeated canonical examples as independent user utterances.
- Marking an unsupported tool “completed” because the model explained how to do it.
- Treating source-wired as verified runtime.
- Removing all failure-prone multi-step commands from the denominator post hoc.
- Counting R0 reads without identity/scope authorization as safe.
- Claiming “90% of users” from one power user's phrasing.

## 6. Continuous feedback loop

After each actual authorized command, store *redacted semantic outcome metadata* in Host journal with user control: registered action ID, parsed vs clarified, adapter readiness, target-confirmation success, risk, final receipt state, device, duration bucket. Keep raw utterances and private paths opt-in and local by default. Aggregate the most frequent misses to expand the grammar **only after** corresponding real capability exists.

Source audit and catalog crosswalk are in sibling documents.
