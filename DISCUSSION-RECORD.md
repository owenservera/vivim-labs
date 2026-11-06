# VIVIM Labs — discussion record

Recorded 2026-11-06. This captures topics, intent and boundaries from the conversation. **Not an approved implementation specification.** Local ZCode team owns the practical design and workflow proposals.

## Why create a Labs repository

The user wants a dedicated place for high-volume work that complements core VIVIM architecture and implementation rather than creating more drift in TierFlow, Repos HQ and the Web Harness. Labs should also **run tests and handle process-oriented work**. A major goal is to expand the **breadth and depth of what is known to work**, not simply generate more code.

The user has already executed three or more parallel ZCode threads with eight workers each. OpenRouter and NVIDIA free-tier models are available through the user's local environment; ZCode is configured for central fleet/swarms management. The discussion emphasized actual capacity and throughput measurements over assuming limitless provider or hardware concurrency.

## Topics explicitly raised by the user

1. Expanding and testing the natural-language command system at high volume.
2. Expanding reusable asset types needed across the platform.
3. Merging or federating the existing catalogue of catalogues.
4. Scaffolding a production environment while development continues.
5. Local execution of tests and integrations that ChatGPT's remote tooling cannot run.
6. Delegating process-oriented work, not merely code or research.
7. Broadening and deepening proven capability coverage using parallel workers.

## Additional candidate work areas discussed (not commitments)

- Language test corpora: paraphrases, synonyms, ambiguity, multilingual requests, chained commands, corrections, refusals and exact typed-command conformance.
- Catalogue discovery/indexing across commands, components, capabilities, providers, apps, models, algorithms, agent definitions and workflows; retain authoritative ownership rather than merging into a second authority.
- Reusable assets: UI components, templates, recipes, fixtures, adapters, agent/skill descriptors, test patterns and help content.
- Adversarial tests: cross-owner access, revocation, stale versions, forged receipts, prompt injection, unsafe action selection, idempotency and denial.
- Compatibility and differential testing across TierFlow/HQ/Harness, existing vs new renderers and language systems, version skew, migration readers and client adapters.
- Knowledge/context evaluation: graph/retrieval quality, provenance, ranking, freshness, redaction, deletion and temporal consistency.
- Signed-in webapp compatibility investigations under VIVIM's owner/consent and browser boundaries.
- Failure and state simulation: restart, lost acknowledgements, offline devices, corrupted state, cursor gaps, CAS concurrency and rollback.
- UX/accessibility and performance benchmarking on genuinely available environments.
- Documentation, recipes, process audits, toolchain reproducibility, release preparation and clean production scaffolding.

## Evidence emphasis from discussion

A generated asset, test case or integration candidate is not automatically a verified capability. The discussion distinguished discovery/specification, isolated testing, true cross-module integration, environment/device verification and production evidence. Proposed high-volume counts (e.g. thousands of command cases, catalogue entries and assets) were **illustrative targets**, not established inventory or guaranteed results.

Interesting evaluation methods raised: independent test oracles, property-based and mutation tests, differential testing, combinatorial policy/device/state coverage, real clean-checkout reproducibility and source/contract/version evidence. The local team should choose practical methods based on measured results.

## Relationship to Wave 3–6 and MiniMax closure

TierFlow's Visual Runtime Waves 3–6 were realigned around consolidation, one Founder Workspace slice, explicit gates and safe retirement of duplicates. Integration 2 closure was fast-forwarded to TierFlow main at `ca336613b28fa1ac48ad06e62be78575e524b261` following MiniMax's operator-reported green typecheck/verify and real A–H fixture suite. The closure did **not** establish live Host, Android, Windows or private KB acceptance. Labs can help generate tests, evidence and candidate migrations without independently granting itself production source or effect authority.

## Division of labor discussed

ChatGPT focuses on architectural coherence, contracts, integration review and planning; the local ZCode environment is particularly valuable for actual builds, tests, broad exploration, experiments, assets, process work and high-volume evidence collection. The user requested that **local agents design the actual workflows**. This seed does not prescribe worker topology, scheduling, branching, task schema, directory structure, model routing or promotion mechanics.

## Questions left to the local team

Which tooling already exists in ZCode/HQ and should be reused? Which proposed assets are genuinely reusable and who owns them? How should tests/evidence be run and retained? What is the safe concurrency level under real machine/provider limits? Which production scaffold components belong in Labs versus canonical repos? How should verified results be promoted without duplicate authorities? What is the lowest-friction operating process for many workers and a single accountable acceptance decision?

All remain open design and evaluation questions, not decisions made in this document.
