# Source audit log and cross-repository provenance

Review date: 2026-11-12. Direct GitHub read only; no local Node, Rust, integration or device tests executed.

## Current source and core evidence
- ARBOR merged main: [exact commit](https://github.com/owenservera/ARBOR/commit/3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298); current HQ implementation read: `hq/lib/protocol.mjs`, `annotation-v2.mjs`, `semantic-registry.mjs`, `indexer.mjs`, `schema.mjs`, `wiki.mjs`, `wiki-program.mjs`, `comment-wiki-bridge.mjs`, `arbor-service.mjs`, and `hq/package.json` (Node >=24).
- SCF draft [PR #13](https://github.com/owenservera/ARBOR/pull/13): branch `work/p5-semantic-capability-fabric-r001`, observed HEAD `0e35c54105f7b8ee8a772b3159d243c6788fb7d4`. Read 00-source SITREP, vision, agent protocol, comment language, implementation-ready master/source-lock/test gates/manifest and verifier thesis/kernel.
- SEVP draft [PR #19](https://github.com/owenservera/ARBOR/pull/19): branch `work/p5-semantic-engineering-program-scaffold-r001`, observed HEAD `e41b943e2eb0fd0c60b73918596b219c087f0052`. Read R002 master convergence, ownership/backlog and predev machine state.
- HVI main policy: [native-tooling-only scope](https://github.com/owenservera/ARBOR/blob/3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298/.integration/proposal5/hq-version-intelligence/10-NATIVE-TOOLING-ONLY-SCOPE.md). Observed HVI integration `0539d23404ef5b1de12ad646d807a2fda57c6ab6`, H `d24daa9966abb222cd25210f41dcbb4758aa1209`, U `5836e9c9f5ba3bee36dbe6896ebfd22364b4c450`, Z2 `725132c640a1f21d3af5c4b9123b314e4df3133d`; head observations **do not establish gate acceptance**.

## Secondary anchors from SCF's existing source audit, not revalidated this turn
- repos-hq SWC research branch `7cb2fadd88c5a08c0ec7e084410e150a5b6e0f3d` (research compiler + v3 language design).
- vivim-labs R010 lab branch `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8` (research ideas, no product authorization).
- ARBOR pre-G Semantic Wiki `d7e59a389a6ec5278595e317dfa904a801783c5b` and PG02 semantic fabric `021864846783d28679b29e563490d4bb100ac079`: previously prepared unpromoted branches, historical test claims not independently rerun.

## Evidence interpretation
`SOURCE_PRESENT` is a statement about GitHub file contents only. `PREDEV_PREPPED` is a statement about engineering plans only. `tests_executed_this_sitrep=false` and unaccepted gates are explicit. Live execution, proof receipt integrity, release and device acceptance are outside this documentation-only review.

## Machine contracts
[Source pin JSON](machine/source-pins.json), [component register](machine/component-register.json), [gates](machine/gate-register.json), [program state](machine/program-status.json). Valid status vocabulary documented in [machine README](machine/README.md).
