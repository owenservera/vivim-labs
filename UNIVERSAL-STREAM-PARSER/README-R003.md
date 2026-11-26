# R003 — Goal-directed extraction by demonstrations

R003 adds an independent native Rust intelligence module to the existing R001 crate. It does not depend on the unintegrated R002 overlay. The module is library-only; the original CLI and live browser adapter are not yet wired to the learner.

## Contract

The capture caller supplies ordered observation windows with stable, sanitized source identity, per-request stream identity, record byte span, and decoded JSON value. A positive label is the independently observed exact desired text. A negative label means there should be no target output in that window. The learner searches all string leaves across all sources, generalizes array positions, and compares two sequence strategies: append deltas vs latest cumulative snapshot. It ranks candidates by exact matches, contradictions, absent observations, negative-control false positives and ambiguity. A plan requires three positive trials, at least two distinct output texts, one negative control, and a unique consistent candidate. Reused trial IDs are rejected. The caller is responsible for ensuring distinct IDs correspond to truly independent trials.

The selected plan is serializable and executes only the learned typed path. It retains source spans and refuses mixed request instances, incompatible container/terminal types and bounded output violations. Validate on a separate holdout before trusting a plan. A missing path is treated as no observation (possibly a control frame); absence alone is not proof of drift. Same-type semantic drift requires independent observed output or another invariant.

## Limits and follow-up

Exact labelled output is required; no arbitrary semantic inference or LLM calls. Only string leaves, append and latest are implemented. No automatic capture, user-interface labelling, persistent store or automatic repair is claimed. Source identities must be stable across requests but sanitized to omit credentials, query secrets and session identifiers. Span offsets must be normalized by the caller. Training and execution are bounded; never commit raw captures or private outputs to public Labs.

Next: connect existing authorized Studio browser events, collect independently labelled windows, validate against actual front-end output, add candidate transforms for nested encoded structures, and wire drift-triggered rediscovery. Test R002 and merge it only after independent verification.


Active-learning probe: ranks unlabeled windows by the number of candidate pairs they distinguish, without returning or persisting candidate output. Native Windows cargo test --offline: 14 passed, 0 failed (7 R001 + 7 R003), exit 0. Session ctl-525450ee9ca30786. No live browser acceptance claimed.
