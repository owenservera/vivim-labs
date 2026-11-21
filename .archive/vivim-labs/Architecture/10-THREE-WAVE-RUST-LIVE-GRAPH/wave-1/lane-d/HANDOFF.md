# Lane D HANDOFF — Wiki Quality Oracle for A+B+C+E

## Verdict: BLOCKED for integrated runtime; executable D harness tested

- Mission: measure whether the existing deterministic HQ wiki faithfully presents verified code → semantic/KB → registered command links from the single Rust HQKG graph, with explicit provenance and unknowns.
- Owner scope: new files in `vivim-labs/Architecture/10-THREE-WAVE-RUST-LIVE-GRAPH/wave-1/lane-d/`. No canonical HQ KB/SQLite/semantic/command/wiki writer modified.
- Source inspection (Repos HQ `main` baseline `2c9bfb4c515cfabce6f8775473e461cd16418e46`): `hq/lib/wiki.mjs` blob `3efcde513bf5fdf538b413dce209ec9e6bcf0253`; `hq/lib/wiki-program.mjs` `97cde5d6dbb14fc421bfa5eddb11236e9b71c745`; `hq/lib/comment-wiki-bridge.mjs` `996b0f75362a66b823d95b321754f2ff17275325`; `research/rust-knowledge-graph/src/import.rs` source read; `hq/bin/hq.mjs` source read.
- Executed test: `node --test wiki-oracle.test.mjs` on Node `v22.16.0`, **exit 0**, `19/19 passing`; tests are entirely synthetic and clearly labeled. `node --check wiki-oracle.mjs`, **exit 0**.
- Code/executable: `wiki-oracle.mjs` reads pinned HQKGX1 + existing generated Markdown, refuses duplicate/bad/secret metadata, checks actual A/B/C evidence joins, emits machine JSON diagnostics; exit 2 when blocked.
- No generated real wiki, no real E journal, no machine-local A/B/C index run; **do not promote synthetic passes to native graph proof**.
- `README.md` contains reproducible native HQ wiki generation command, E→D exchange fields, CLI use, refusal semantics and known integration conditions.

## E interface dependencies

1. E must publish its **actual running** imported exchange with source-owned A+B+C nodes, explicit `hqg1:` edge identities, verified semantics, evidence classes and source SHA/path, and a digest-matched Rust journal/query receipt. Do not use D as a fourth source feed.
2. B must provide one genuine privacy-safe KB/decision with accepted/proposed authority and a semantic ID; C must provide a *registered* handler with exact syntax and wired/declared truth; A must provide one indexed source with SHA/symbol/provenance after the scoped reset.
3. Existing HQ wiki owner must regenerate with the same integrated meanings in its own deterministic builder. This D tool will correctly mark an index-only wiki deficient; it does not paper over source/semantic/command omissions.
4. F/integration owner must independently verify source → KB/semantic → command → Rust query → HQ-generated wiki, `no-op`, restart, invalid/collision, deterministic regeneration, and native graph/wiki diagnostics. Owner must pin exact source SHA and run transcripts.

## Output contract and errors

- `vivim.wave1.wiki-quality/1` JSON: `verdict`, `source`, `exchangeDigest`, `rustEvidenceSupplied`, `rustEvidenceIndependentlyVerified=false`, `totals`, `quality` (five scored, denominator-bearing dimensions), `chains`, `defects`.
- Quality defect codes include `missing_source_provenance`, `missing_wiki_source_reference`, `missing_semantic_description`, `unbound_semantic_identity`, `unknown_knowledge_status`, `missing_command_syntax`, `unwired_or_unverified_command`, `orphan_knowledge_record`, `orphan_command_record`, `unverified_relationship`, `missing_cross_domain_wiki_page`, `no_code_semantic_command_chain`, `broken_wiki_link` and `unknown_graph_domain`.
- Failure behavior: malformed / tampered HQKGX1, duplicate identity, bad endpoint, suspicious metadata, symlinked wiki file, excessive input, existing report file all hard-refuse. No DB or wiki writes.
- Omitted by design: any source scan/index reset, Rust graph journal, E query implementation, wiki page renderer, configuration activation, user NL/Host action execution, product release.
- This lane's tests validate an *oracle* against synthetic fixtures, not coverage of repos-hq native builder. This is a hard blocker under the charter, not a waiver.

**DO NOT design Wave 2 until the integrated Wave 1 run is independently evaluated.**