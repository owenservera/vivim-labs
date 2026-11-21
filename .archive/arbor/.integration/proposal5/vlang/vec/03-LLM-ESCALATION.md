# LLM-minimal strategy: resolve first, escalate only when the SDK cannot

## Target, not an established benchmark
Aim to support a very high share of frequent VIVIM customization and automation requests without calling an LLM. Do not assert arbitrary natural-language comprehension, fixed percentages, or measured savings before an actual user corpus/evaluation and privacy review.

## Local deterministic ladder
1. Exact registered phrase or user-authored alias maps to one typed intent.
2. Localized finite grammar and slot resolver recognizes a parameterized request.
3. Capability composition searches a bounded, typed graph with permitted transitions, costs and prerequisites.
4. Multi-turn deterministic clarification for missing required fields and ambiguous referents.
5. Only then offer optional LLM-assisted proposal for novel expression, unfamiliar capability, complex multi-domain reasoning or missing SDK primitive.
6. Once independently reviewed and explicitly accepted, store a reusable **recipe/grammar/SDK contract** so repeated requests can follow steps 1–3. No automatic elevation of model-generated code, credentials, registry IDs, permissions or security policy.

## Separate model-dependent execution
Some **tasks themselves** involve creative text, perception or reasoning; these may still use an approved, user-selected model during execution. This is different from requiring an LLM to interpret the same trigger and automation rules repeatedly. E.g. 'summarize articles each morning' can have a deterministic trigger/composition but model-dependent summary step.

## Decision/result vocabulary
- `DETERMINISTIC_EXACT`, `DETERMINISTIC_GRAMMAR`, `DETERMINISTIC_COMPOSED`
- `NEEDS_CLARIFICATION`, `CAPABILITY_UNAVAILABLE`, `PERMISSION_REQUIRED`, `POLICY_DENIED`
- `MODEL_PROPOSAL_ALLOWED`, `MODEL_PROPOSAL_NOT_ALLOWED`
- `INSTALLED`, `ENABLED`, `PAUSED`, `REVOKED`, `FAILED`
These are **proposed program semantics**, not production enum declarations.

## Cost engineering and privacy
Cache canonical program plans by program revision, capability ABI and permission scope, not by raw chat text or global user identity. Test local grammar and compiler offline. Bound capability-graph search and clarify before speculative broad execution. Log only scoped receipts and minimal explanation, redacting private inputs. No provider APIs: any optional LLM path must follow already approved VIVIM user-owned, signed-in browser/local model channels; do not add direct provider API as a shortcut.

## Evaluation
Build privacy-safe invented phrase corpus plus voluntarily supplied approved real user phrasing; measure parser coverage/accuracy, false positives (worse than unknown), clarification quality, time-to-program, cold/warm cost, LLM invocation ratio (by eligible task and by all tasks separately), normalized behavior replay and security negative tests. Report real commands, exits, toolchain and exact source SHA; until then all targets are hypotheses.
