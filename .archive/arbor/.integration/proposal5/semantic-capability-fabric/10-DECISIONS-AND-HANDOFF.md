# Handoff and decisions
## Accepted for *design*
- Agent annotations are **typed, reviewable declarations**. Compiler verifies, it does not trust.
- Stable capability identity is owner-controlled; file unit identity and Git commit/blob identity are separate; many-to-many feature membership is normal.
- One parser/registry/index/wiki/HQKG stack; no new operational graph DB, source writer, Host dispatcher, scheduler, phone backend, or browser sender.
- Existing v1/v2 tags remain compatible. Additive v3 `unit,role` plus SWC proposed language/migration tags require review, tests and version negotiation.
- Immutable HVI pins are the version keys. Comments produce richer compare input; legacy raw source analysis remains mandatory.
- Protected user/World/private DCB data stay under Host source authority. Plain source metadata cannot grant permission or attest runtime.
- First enforcement on touched first-party code; do not auto-tag 2,000 legacy files with guessed IDs.

## Priority handoffs to later implementers
1. **HQ D+F:** v1/v2 touched-file templates, strict file/symbol annotation scoping, source/registry checker; test false comment, missing/unknown IDs.
2. **HQ D:** adapt existing research SWC `vivim.semantic-wiki-ir/0.1` to an evidence-joined Semantic Capability ABI, with deterministic incremental index/wiki projection and version-cached digests.
3. **HVI B/C/D:** consume capability passports, preserve exact 2..N pins and ambiguous legacy paths; test original 361 missing HQ path rows without sweeping semantic equivalence.
4. **Host B + E UI:** read-only, scope-safe capability passport inspector and time machine under existing Host actor/grants and TierFlow publisher; prove revoke and stale revision failure.
5. **F/G:** independent three-SHA same-repo and five-donor migration self-audit, negative security, no-donor offline replay, and executable command receipt before merging code into main.

## Explicitly unresolved
- New v3 tag `@hq:unit` and `@hq:role` names/grammar must be reviewed against existing `META-CODING-LANGUAGE-SWC-001`; no silent parser extension.
- Stable symbol identity when a function moves/splits and when Rust/TS cross-language boundaries align; require line/blob/ABI tests and owner decision.
- Observed test-to-symbol coverage instrumentation: path or test file name is not proof.
- How user-facing generalized "open any application" maps to only trustworthy typed app state, not arbitrary external web pages.
- HVI Stage 1 actual API and worker branch changes may differ from authored plans; reconcile exact accepted G SHA before implementation.
- Full Windows Node24/Bun/Cargo/phone and owner security acceptance remain unrun.

## Coordination state
Work products here are a **design addition** to Proposal 5, deliberately in an isolated ARBOR branch. Do not auto-merge while HVI is running; a designated serial integrator should intake this proposal after checking worker SHAs and existing SWC/Labs ownership. A design PR may be opened as draft, not merged as production.
