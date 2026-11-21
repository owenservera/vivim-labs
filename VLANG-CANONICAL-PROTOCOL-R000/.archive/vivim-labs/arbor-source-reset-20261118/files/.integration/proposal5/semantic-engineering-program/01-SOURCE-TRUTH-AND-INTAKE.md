# Source truth and provenance — first-pass observations
Observed ARBOR main `539146ac741516e7d9711c63f4570585b3c3367e` (2026-11-12). The following files exist on that ref: `hq/lib/protocol.mjs`, `annotation-v2.mjs`, `semantic-registry.mjs`, `indexer.mjs`, `wiki.mjs`, `arbor-service.mjs`, `comment-wiki-bridge.mjs`. **Path presence is not runtime, tests or wire-level acceptance.**

Design inputs:
- SCF + Semantic Verifier draft: `work/p5-semantic-capability-fabric-r001` at `90bd81028fab71c6fadef608404f216e84b1629a`; draft PR #13, unmerged.
- HQ SWC research `repos-hq:research/semantic-wiki-compiler-20261112` at `7cb2fadd88c5a08c0ec7e084410e150a5b6e0f3d`; proposed v3 and compiler v0.1; research not product.
- Labs `vivim-labs:lab/nlp-visual-command-ui-20261111` at `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8`; R010/R008 concepts, not production proof.
- ARBOR pre-G semantic `work/p5-preg-semantic-wiki-r001` at `d7e59a389a6ec5278595e317dfa904a801783c5b`, and PG02 `work/p5-fa-pg02-semantic-fabric-r001` at `021864846783d28679b29e563490d4bb100ac079`; prepared not promoted.
- HVI main `.integration/proposal5/hq-version-intelligence/`: current governing "tooling-only, synthetic tests, real comparison deferred" correction. HVI manifests have historic pins/statuses; inspect actual gate branches afresh rather than treating manifest NOT_STARTED as real current status.

## Next pass source inventory
For each capability: canonical owner; source symbol/file/blob SHA; present vs wired vs executed vs design; actual tests and exits; donor candidates; overlap/duplicate; user permission surface; production build closure; deterministic proof and runtime witness feasibility; REUSE/ADAPT/PORT/REFERENCE_ONLY/REJECT decision.

No new donor analysis via HVI tooling. Reading authorized design inputs and inspecting implementation source during planning is distinct from executing a real cross-repo migration comparison.
