# HVI Lane A — source map and pinned reading

**Worker base:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` (S0 GO_TO_PARALLEL). **S0 proof:** `2ef7a9f66419d681e040b459ba4762a63aa9564f`. **Status:** implementation candidate, not integrated.

## Actual ARBOR source inspected at frozen worker base

| File | Git blob SHA | Reuse decision |
|---|---|---|
| `hq/lib/git.mjs` | `32e775537f74aa886239783081a82d2b8a670e04` | Read-only Git conveniences; *not* used for immutable snapshots because helpers can read mutable HEAD/worktree |
| `hq/lib/git-context.mjs` | `5296114e0fa5d3b0e30da4f8f4c7d8aa2ef3d8be` | Repository context and duplicate checkout identity are not pinned version manifests |
| `hq/lib/walk.mjs` | `38f8bfe82220909cbd456d3115ecad350656fe1e` | Working-tree file discovery; not a source of committed-object evidence |
| `hq/lib/indexer.mjs` | `4f5cf53c861d8052e0c56be38307f93cc65e2a5f` | Existing HQ SQLite indexer; no shadow DB built |
| `hq/lib/registry.mjs` | `cc46131e051c30fbef1b022310f68b619814da86` | Existing project registry; not permission authority |
| `hq/lib/arbor-service.mjs` | `a47c232c188d69758daa8d6a2f18070ce8c9a27b` | Existing Host-bound development adapter, read-only by Lane A; Lane H owner for Host binding |

Original `repos-hq` at `9b922ea049dbadc94e504b5b9432743995c020ca`: `hq/git/sync-state.json`, blob `cdff770999da20f692555362d037ca4e365f352a`, records branch/dirty observations (not immutable committed-object manifests). This is state evidence only and is **not** imported as a new scheduler or authority.

## S0 five pinned source trees (read-only provenance)

| Repo | Frozen revision | Root tree |
|---|---|---|
| ARBOR worker source | `f9823a5131e73e7562db5a4ec14e1ec481557d69` | `13eda0d9c7f717a46d00eee46a22b1a19edb65be` |
| repos-hq | `9b922ea049dbadc94e504b5b9432743995c020ca` | `a31e926bff898b02273b2044a871881f58ca61a9` |
| vivim-web-harness | `cae97afe848d79957c9d6c698a90996a758bfd56` | `835ea6c1a7e05a73fea5b5fc4c02409a034d4525` |
| tierflow | `50822f11ee63c3d80f631f8fea7dcb9b19bffc38` | `2117f5dde23d4769db38e8fdde0b4871051129f7` |
| vivim-labs | `022608f5797e7b4a14ccc4c393bd3c5a4704a4be` | `22c96f972b31153497c845c12e28efcf912278f9` |

S0 recorded `1500` tracked blob files in the post-plan worker base and its full tree as non-truncated; raw recursive tree count includes directories. Original HQ `676 / 315 / 314 / 1 / 361` direct-path source census was reproduced; this Lane A does not reinterpret absence as missing functionality.

## Boundaries

All new A source is under `hq/version-intelligence/sources/`. New proofs only under `.integration/proposal5/hq-version-intelligence/proofs/A/`. No edit to `hq/lib`, `integration` locks, Rust, Host, TierFlow, provider browser state, donor checkout or `main`.

Sensitive file content, local working trees and credentials were not read. This proof references Git object metadata and inspected code, not runtime acceptance.
