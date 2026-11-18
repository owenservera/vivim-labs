# ARBOR HVI — immutable source snapshots (Lane A)

Source-only, read-only implementation at the HVI development-intelligence boundary.

## API

`resolvePin(source, { adapter, authorize, acquiredAt? })` resolves a development-source ref once to a 40-hex commit and its root Git tree. The immutable `pinId` keys the *repository identity + resolved commit SHA + subPath*, not a moving branch name. `verifyPin(pin, { adapter, authorize, manifest?, checkRef? })` detects branch drift, tree mismatch and digest mismatch. `snapshotAt(pin, { adapter, authorize, limits?, inspectContent?, signal? })` produces a stable, sorted manifest of committed Git-tree records, with explicit partial/truncated flags. `readPinnedBlob(pin, path, { adapter, authorize, manifest, maxBytes? })` returns a bounded Buffer only for permitted in-manifest regular files and checks their Git blob SHA-1.

`createGitObjectAdapter({ resolveRef, readCommit, listTree, readBlob })` accepts **injected, verified Git-object readers**. There is intentionally no ambient GitHub API, `gh`, Git CLI, network request, donor checkout, user-home lookup, source execution or Host effect in Lane A. New revisions require an explicitly authorized development-source adapter and a per-operation `authorize` callback. This adapter is **not** a Host grant implementation; Lane H must provide live actor/session/grant checks. The `listTree` implementation must prove it is returning the requested commit's verified tree and not a moving checkout.

## Scope and security

No persistent registry or new database is introduced. Neither parsed data nor source metadata claims runtime equivalence. The existing HQ catalogue, semantic-registry, graph, source registry, Host authority and Rust HQKG remain owners of their respective domains. Hidden credential-like filenames are excluded from byte reads, and no source bytes are interpolated into errors. Symlinks are metadata only, never traversed; submodules are gitlinks, never followed; Git LFS pointers are marked when actually inspected, not materialized. `uninspected` and `inspection_deferred` must not be read as proof that a file is non-LFS or textual.

Known deliberate constraints: only SHA-1 format 40-hex Git object IDs; Git SHA-256 repositories, GitHub-specific network authorization, verified flattening of tree pages, cross-repository permission grant composition and Host runtime reads require later owner adapters. Synthetic test provenance is not real repository acceptance.

## Run

```sh
node --test hq/version-intelligence/sources/snapshots.test.mjs
```

Node 24 is the intended G1 acceptance environment. A source-only commit or synthetic test is **PREPARED_NOT_PROMOTED**, never BETA_GO or RUNTIME_VERIFIED.
