# Candidate contracts, implementation sequencing and acceptance

**Status:** V2 annotation proposal and wiki V1 architecture, not implemented.

## Contract surfaces

1. `hq.annotation/2` (proposed): typed comment records `{kind, value, projectId, path, contentHash, symbolId?, line, schemaVersion, provenance}`. Preserve v1 `@hq:` behavior and backward compatibility. Reject malformed or unsupported annotation values deterministically; no automatic permission.
2. `hq.semantic-registry/1` (proposed): versioned IDs for types, capabilities, actions, source evidence, policy data and alias/lexicon records. Validate duplicate stable IDs and referential integrity.
3. `hq.wiki-program/1` (proposed): declarative view configuration `{id, version, project, source, filters, layout, selection, expectedRevision}`; preview/diff, authorization and receipt separate. Read-only wiki commands remain read-only.
4. `hq.semantic-intent/1` (proposed): input-independent `{actionId, args, worldRevision, provenance, risk, requiredScopes}`; parser, clicks, and agent tool calls produce identical IR shape.
5. `hq.source-link/1` (proposed): source hash, symbol/line, test, schema/capability, documentation binding with stale/drift status. Never infer implemented behavior solely from annotation.
6. `hq.coordination/1` (version proposal subject to current sync-bus owner): stable sender/recipient/task, claim, ack, receipts, idempotency and cursor; **no delegated authority**.

## Planned coding agent annotation rules

Keep `@hq:context`, `why`, `invariant`, `see`, `todo`, `owner`, `task`, `risk`, `deprecated`. Add semantic tags only behind a v2 grammar gate: `@hq:type type-id@version`, `@hq:capability action-id@version`, `@hq:effect effect-class`, `@hq:evidence test:path-or-ref`, `@hq:contract schema-id@version`, `@hq:emits event-id`, `@hq:consumes event-id`. Require registered IDs for hard validation; permit explicit proposal status for experimental IDs with no automatic canon promotion. A declaration must be bounded, nonsecret, linked to actual code, and backed by conformance tests. Symbol/capability ownership must not be established by a comment alone.

## Proposed acceptance fixtures

- JS/TS/Rust/Python/Go/PowerShell comments each parse deterministically; strings containing `@hq:` ignored.
- Existing v1 files still lint unchanged.
- Unknown or deprecated type refs report diagnostics, never auto-accept.
- Invalid source path, line movement, content hash drift and stale test refs reported.
- Secret-bearing comments are refused.
- Wiki click/save/undo produces canonical typed intent and stable revision; read-only queries cannot mutate.
- NLCL recognized string returns same typed intent as equivalent selector; ambiguous string requires disambiguation.
- Host effect denial/expired digest/uncertain operation never turns into an implicit retry.
- TierFlow composes on arbitrary non-HQ project with optional semantic provider, and VIVIM is functional without TierFlow.
- HQ+KB kernel migration proves data/backups/rebuildable indexes separation before switching writers.
- Agent messaging cannot unlock any operation; crash/replay/dedup tests prove receipts and recovery.

## Ordered implementation work (do NOT run during HQ baseline)

1. Inventory existing annotation parser and linter; identify minimal additive v2 seams and regression cases.
2. Audit VIVIM Host capability catalog, Ω grammar/WorldModel and TierFlow protocol for intersecting stable IDs; publish a machine-readable crosswalk.
3. Add schema-backed annotation v2 parser/linter with fixtures; avoid generalized code-executing DSL.
4. Register semantic type/action IDs, evidence anchors, drift checks, versioning and rollback.
5. Deliver wiki V1 selector-driven program edits through separate bounded config writer; preserve read-only console.
6. Add NLCL adapter to produce same typed IR; do not fork authorizer.
7. Integrate TierFlow/VIVIM as independently versioned consumers; keep separate baselines.
8. Test three-repo coordination and receipt pathways over HQ-owned bus without effect-privilege transfer.

## Anti-goals

No fourth tokenizer, no Python/JS eval in wiki, no shell execution from inline comments, no LLM self-approval, no unscoped Host writes, no TierFlow dependency in VIVIM, no duplicate canonical KB, no direct edits to TierFlow `.hq-tool/`, no unverified production migration.
