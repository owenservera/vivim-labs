# SCF product vision — the codebase that explains itself
## Product proposition
Every owned source file carries a compact, versioned **intent signature**. The HQ compiler translates these signatures, real code, contracts and independently verified evidence into a **Capability Passport**. Open any supported app feature and you can ask: What am I looking at? What capabilities does this module support? Who owns it? How is it called? What depends on it? What is actually wired and proven? What changed across revisions? What is still only designed? No manual wiki authoring for routine source changes.

## The 10 next-generation experiences
1. **Self-describing file** — open file; its stable semantic unit, owner, 2–N feature memberships, dependencies, public contract, tests and known restrictions appear from compiler IR.
2. **Capability Passport** — feature page aggregates all actual implementers, consumers, tests, runtime evidence, risks, variants and gaps; a file can contribute to X/Y/Z without claiming exclusive ownership.
3. **Instant system map** — source→contract→Host owner→Rust graph/read→TierFlow projection→test/receipt paths with provenance and unresolved crossings.
4. **Architecture time machine** — select 2,3,N immutable Git SHAs; compare capability passports, lineage and contract changes, not just file names.
5. **Migration diff without guessing** — when an agent carries stable identity/lineage, HVI can deterministically prove what migrated vs changed; old unannotated code still needs bounded evidence-based matching.
6. **Pre-change impact preview** — on changing a public contract, compiler lists affected features, tests, language packs, UI projections and approvals; ranked by evidence, not speculative similarity.
7. **Agent context on demand** — each agent task receives a small signed digest/pin bundle of only relevant capability passports and invariants, with budget and missing evidence.
8. **Wiki from actual code** — one canonical index feeds human prose, machine query, docs, architecture diagrams and cross-repo dashboards. No independent hand-maintained truth.
9. **Semantic UI mirror** — open a VIVIM screen; with trusted view binding, show its concept, source owner, current state, capability lineage and exact receipt boundaries. Missing/unbound source yields Unknown.
10. **Traceable change history** — PR shows `feature membership added/removed`, `contract changed`, `tests needing run`, `unproven runtime` rather than only source diff.

## Mental model
Source code is not rewritten into a special language. The *normal* code remains JS/TS/Rust/etc. Small `@hq:` comments act as typed interface declarations; the compiler infers cheap, verifiable structure automatically. The wiki is a derived, reversible **read model**, not an executable script, personal-memory store or new graph DB.

## Feature knowledge states
Every fact is marked **declared by agent**, **derived from syntax**, **source-linked and reviewed**, **tested by observed runner**, **integrated and reachable under actual owner**, or **runtime/device verified**. These are distinct dimensions, not one scalar score. The UI never turns a comment, test path or screenshot into a permission or runtime receipt.

## Non-goals
No universal omniscience for arbitrary external applications. No requirement to tag third-party or generated code. No huge comments containing duplicate implementation documentation. No uncontrolled automatic feature promotion, merging, code deletion or deployment. No policy that source annotation alone makes a behavior available.
