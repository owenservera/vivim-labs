# Baseline — what is ACTUALLY in code

| Current owner | Actual source | Already implemented | VEC target | First migration mode |
|---|---|---|---|---|
| HQ inline comments | `hq/lib/protocol.mjs`, `annotation-v2.mjs` | v1/v2 inline source-tag parsers | VEC owns language/compiler frontends, HQ keeps source indexing | shadow parse; match exact source positions/errors |
| HQ semantic registry | `hq/lib/semantic-registry.mjs` | owner-confirmed versioned development semantic IDs | VEC consumes reviewed SDK descriptors, not a second identity writer | read-only manifest crosswalk |
| HQ Wiki generator | `hq/lib/wiki.mjs` | deterministic incremental Markdown/browser Wiki from HQ index | VEC gradually takes Wiki compilation and potentially projection assembly | preserve generator while comparing compiled Wiki program output |
| HQ Wiki program | `hq/lib/wiki-program.mjs` | data-only config validation/preview/diff/activate/rollback | VEC eventually owns compilation; HQ storage/approval remain until separately migrated | dual compile, old writer only |
| Rust-comment Wiki | `hq/lib/comment-wiki-bridge.mjs`, `hq/lib/wiki-runtime/rust-comment-adapter.mjs` | source-pin checks and comment → Wiki config/gesture candidate | VEC source/capsule metadata compiler | shadow with exact source digest/refusal parity; no Rust execution |
| HQ optimizer | `hq/lib/semantic-optimizers.mjs` | alias matching, ranking and bounded action-plan ordering | reuse pure algorithms, measure before port | delegate, no duplicate implementation |
| Phone NLCL | `apps/mobile-relay/control-language.mjs`, `nlcl/*.mjs` | shared deterministic text meaning/slot discovery | VEC single NL frontend, commands distinct from UI authoring | reuse existing interpreter, differential parity |
| Host command intent | `apps/vivim-host/control/command-intent.mjs`, `intent-planner.mjs` | catalog/risk/current personal semantics and typed candidate | VEC owns pure candidate compilation in stages | shadow; no new effect path |
| Host command DAG | `apps/vivim-host/control/command-program.mjs` | declarative ordered action steps, no execution | VEC command-plan output type | parity before promotion |
| Host effects | `apps/vivim-host/control/dispatcher.mjs`, `journal.mjs` | actual approvals/journal/claims | **NEVER** VEC-owned | no migration |
| PWA UI catalog/compiler | `apps/mobile-relay/generative-ui/{catalog,compiler}.mjs` | `vivim.ui-surface/1`, `vivim.ui-proposal/1`, safe read-only previews | VEC UI adapter composes them; no copy of validator | **SOURCE IMPLEMENTED** |
| PWA Build | `apps/mobile-relay/{build-composer,build-canvas}.mjs` | AI design first; preview/Activate/undo existing | deterministic first for known edits, AI fallback | **SOURCE IMPLEMENTED** |
| Phone signed Host head | `apps/mobile-relay/generative-ui/{host-revision-guard,phone-surface}.mjs` | read-only applied revision guard + existing `apply-proposal` transport | remain product writer/authority | unchanged |
| TierFlow | `packages/tierflow/src/visual-runtime/**`, `src/ui/**` | World/1, Visual/1, static safe subset bridge, inert templates | future UI composition + two-way reprogramming | integration after phone Surface/1 MVP |
| Product Rust | `core/programmability/src/lib.rs` | bounded inert typed DAG validation | eventual portable VEC pure-IR backend | staged differential tests, no Rust rewrite now |

**Not implemented:** new Wiki/compiler takeover, command parser replacement, dynamic data bindings, scheduled user programs, Rust executable capsules, full TierFlow World/Visual/Surface bridge, Windows + physical paired PWA acceptance. **Source implementation now begun:** pure PWA deterministic UI preview for four narrow grammar families with no provider call.

Historical HVI native-tooling constraints still apply: no real donor-repository migration comparison under native-tooling install work, only invented synthetic fixtures. This PWA study is source analysis within ARBOR and does not execute HVI donor comparisons.