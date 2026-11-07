# VIVIM documentation relocation — execution report

Date: **2026-11-07**. Destination: `documentation-consolidation/` in `owenservera/vivim-labs`. Changes published directly to `main`, without branches, PRs, force pushes or Actions.

## Result

- **owenservera/tierflow** — relocated **10**, retained **302** documentation-format files. Latest source `main`: `4a4f9c9cf1d61456d4ee4f9f74b47b1eee025aaf`. Initial inspected `main`: `baa859c70a4de8fca326f2083d76ab2bd41cc9dc`.
- **owenservera/vivim-web-harness** — relocated **96**, retained **273** documentation-format files. Latest source `main`: `2fb5d1a17331c7a7359b1c9b9c08f7814bdb447e`. Initial inspected `main`: `09382c61f74a304f120b05445f6dfa900b58494b`.
- **owenservera/repos-hq** — relocated **31**, retained **522** documentation-format files. Latest source `main`: `110256d00240218265f5138c159dee7625fdb5bf`. Initial inspected `main`: `e46ce10c631cafa913907ccea1dc007de9a8d3fe`.

**137 relocated / 1234 enumerated**. All relocated entries were copied with original text and relative path in Labs, and destination Git blob SHAs were checked against source before source deletion. Source removals were committed using expected-head, non-force compare-and-swap operations.

### Protected operational materials

- Root and nested `AGENTS.md` and `SKILL.md`; OpenCode and other agent definitions; HQ tool bundles.
- Source code, JSON schemas, structured capability registries, UI/runtime contracts, generated/wiki data and text fixtures.
- TierFlow active visual-runtime waves and launch orders, HQ active sprints/oracle/intelligence work, Harness active PRDs and its document discovery/index entrypoints.
- Original `README.md` and other minimal repository entrypoints.

### What this has **not** done

- No local Windows, Bun, Rust, Android or agent session verification; no app/runtime tests were executed.
- No blanket rewrite of intra-document links or the Harness document catalog. Existing references to relocated historical files may need remapping; use the manifest as the path/sha source.
- No claim that every one of the 1097 retained files is truly operational. `remaining-doc-review` entries are candidates for another reference-aware move pass; protect everything with live imports or agent dependencies until verified.
- No deduplication, canonical authority merging or KB schema transformation. This is physical documentation housekeeping, not semantic consolidation.

## Remaining classification counts

- `working-data-or-generated`: 314
- `remaining-doc-review`: 309
- `active-visual-wave`: 188
- `working-agent-instruction`: 117
- `active-development-wave`: 106
- `component-entrypoint`: 32
- `product-authority-or-agent-reference`: 31

## Follow-through

Use `INVENTORY.json` to reconcile the Harness document catalog, source-relative Markdown links, and any scripts/agents depending on historical paths. After validating those consumers, move `remaining-doc-review` files with the same copy → SHA verify → CAS delete protocol. Never make Labs a second runtime document publisher.
