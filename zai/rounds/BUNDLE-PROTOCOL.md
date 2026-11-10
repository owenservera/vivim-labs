# Four-repository Git bundle and manifest protocol

A round ends with **four repository checkpoints**, one immutable consistency manifest and independently replayable evidence. Each repo maintains its own Git history. The manifest, not a synthetic cross-repo commit, defines the coherent release set.

## Required deliverables

- `RNN-repos-hq.bundle`
- `RNN-vivim-web-harness.bundle`
- `RNN-tierflow.bundle`
- `RNN-vivim-labs.bundle`
- `RNN-manifest.json` (public-safe sanitized metadata only)
- `RNN-REPORT.md` (using ROUND-TEMPLATE)

Each bundle is either FULL/self-contained with refs and full prerequisite objects, or INCREMENTAL with exact base SHA(s), documented prerequisite bundles and a replay plan. Do not label an incremental bundle standalone. An unchanged repo still receives a pinned checkpoint and valid full bundle when the platform permits it; do not create filler commits.

## Mandatory manifest fields

`schema=vivim.zai-round-manifest/1`; round ID; UTC timestamp; software/toolchain environment; campaign policy revision; each repo's GitHub URL, base/full end SHA, included refs, dirty-tree status, bundle mode, dependency/base SHAs if incremental, artifact name, byte length, cryptographic SHA256, `git bundle verify` actual result, clone/restore actual result, commands and test exits; cross-repo consistency checks; decision/open-item delta links; publication/security restrictions; reviewer; admission outcome.

## Gate checks

1. Clone from approved clean source at pinned full SHA and verify clean checkout.
2. Record any concurrently advanced public HEAD; never force-push or discard other changes.
3. Create bundle from actual committed refs, verify with `git bundle verify` in the intended receiver/prerequisite context and separately test full restore when possible.
4. Verify HEADs of restored repos equal four manifest SHAs and that no unexpected code/index/history delta exists.
5. Execute applicable tests; pin input and output provenance for indexes, generated docs and roadmap views. Report UNRUN honestly.
6. Scan reports, bundle contents and included commit history for public-private exposure before distributing artifacts.
7. Use source-owned branches/PRs or authorized release method to integrate; never silently publish to main or change visibility.

**Important:** A Git bundle contains reachable Git history and can expose removed secrets. Do not publish bundled history into Labs by default. Keep manifests and sanitized receipts here; distribute sensitive bundles only through approved private transfer destinations. Z.ai cloud is not an automatic private preservation authority.
