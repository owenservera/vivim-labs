# VLang strategic SITREP — current architecture, destination and ambition

## North-star
Make ARBOR's source continuously **explainable, queryable, version-aware and independently falsifiable**. An agent changing a file declares short, typed purpose and capability membership; HQ combines those declarations with real source topology, registered identities and observed receipts; independent SV challenges claims; the existing Wiki and HVI expose only appropriately qualified evidence. The user can ask what a capability is, who owns it, what it actually does, what changed, what remains unverified, and why the answer is trustworthy.

## Why it matters
Current source intent is scattered across file comments, contracts, action catalogs, tests, Rust graph nodes, Host effects and TierFlow presentations. A feature may span dozens of files and a file may support several capabilities. Manual wiki documentation goes stale and simple file/name similarity is insufficient to assert semantic migration equivalence. VLang should turn that scattered structure into a compact, incremental, evidence-bound source knowledge fabric.

## Strategic principles
1. Source remains ordinary JavaScript, TypeScript, Rust and other established languages; VLang is **metadata + compiler + verifier + projections**, not an alternative coding runtime.
2. Every inline declaration is *untrusted hypothesis*. Compiler output is not proof. Agent-written tests alone do not establish independent correctness.
3. Preserve existing HQ v1/v2 semantics and owner registries; v3 additive syntax begins opt-in, only after grammar/registry/schema review.
4. Small touched-file annotations, not heavy documentation or forced vendor/generated-file rewrites.
5. Truth is multidimensional: declared, parsed, statically inferred, source-reviewed, tested, integrated/reachable, independently verified under explicit model, runtime/device observed. Never collapse to `verified:true`.
6. Same feature can have 2..N files and a file can support 2..N features. Stable unit/source/feature/action IDs are distinct.
7. Incremental, deterministic, bounded and local-first; preserve low-disk/cancellation/rollback paths and immutable Git pins.
8. Host owns personal World effects; HQ owns development intelligence; HQKG owns bounded graph compute; Studio owns browser sending; TierFlow owns presentation. Source comments grant nothing.
9. HVI installation and synthetic tooling can be built without *running* actual donor comparisons. No provider APIs or unrestricted browser automation.
10. Unknown, stale, partial and revoked are legitimate answers; explanation must be constrained by checked evidence.

## Three horizons
**Immediate:** S0 closure; source ownership, version pins, trust root/oracles, ABI and executable prerequisite audit. Freeze one collision-free implementation base.
**Next:** implement pure lexer/compiler/registry/passports/SV/test seams on isolated scopes; independently check a deliberately false pure-function claim and emit REFUTED counterexample; then Wiki projection and synthetic HVI adapter behind accepted gates.
**Long-term:** architecture time machine, auto-updating verified capability Wiki, pre-change impact forecasting, self-explaining VIVIM UI and contextual developer agents with proof-carrying explanations.

## Success criteria
A user can select a source-pinned capability and receive its implementers, dependencies, declared status, independent evidence, specific test exits, unknown boundaries, revision lineage and a truthful explanation. A false claim must be counterexample-refutable; missing trustworthy evidence stays UNKNOWN/BLOCKED. No extra runtime authority is created.

## Explicit non-goals
Not a second Git/version service, general purpose executable language, replacement for HQKG/Host/Studio/TierFlow, universal theorem prover, automated migration auditor in this wave, full code annotation mandate on first rollout, or release-readiness bypass.
