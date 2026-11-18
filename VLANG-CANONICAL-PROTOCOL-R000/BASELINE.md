# Pinned source baseline

[ARBOR bd516b9](https://github.com/owenservera/ARBOR/commit/bd516b9d1ecc0794b5af39bfda919371a488b019). Recheck before each lane. Findings from preceding source audit; tests UNRUN.

| Target | Observed role / gap |
|---|---|
| hq/lib/protocol.mjs | PROTOCOL_VERSION=1; human guidance and lint |
| hq/lib/annotation-v2.mjs | JS-style comment scanner, typed tags and registry resolution |
| hq/lib/indexer.mjs | Incremental indexing and semantic changes; v2 scanner restricted to JS/TS excluding JSX/TSX/Rust |
| hq/lib/semantic-registry.mjs | Versioned identities, owner promotion, anchors, aliases and drift |
| hq/lib/context.mjs | Source-hash checks, bounded mandatory semantic packet and verification |
| hq/bin/hq.mjs | protocol, lint, annotate-v2, semantic list/show/drift; do not assume proposed semantic check exists |
| hq/bin/arbor-hq.mjs | Authorized source context with --allow and --semantic |
| hq/test/semantic-packet.test.mjs; hq/test/vlang-compiled-context.test.mjs | Relevant tests; execution unverified |

Prior research: ARBOR .integration/proposal5/semantic-capability-fabric/02-AGENT-CONTRIBUTOR-PROTOCOL.md, implementation-ready/ and verifier/ within that directory; .integration/proposal5/vlang/; Labs ARBOR-VLANG-UPGRADE. Research does not establish runtime authority.

Verify gaps: symbol binding, rename lineage, evidence validation, language coverage, capability passports, agent lifecycle, incremental adoption.