# Wave 1F interoperable READ/PROJECTION contract
These files are **pure vocabulary and guards**, not an alternate Host catalog/World, a new scheduler, provider API, or source identity mint. Version 1 mirrors source-backed Host journal states `planned/approved/uncertain/applied/failed` and keeps UI projections inert. A `claimed` event without a receipt projects `uncertain`, never auto-retry or applied.

- `protocol.schema.json`: declarative reference shape, not an authorization policy.
- `types.ts`, `types.rs`: minimal transport mirrors; the Rust mirror is **not wired** until Lane A/G runs `rustc --test contracts/interop/v1/types.rs` and Cargo integration.
- `validate.mjs`: fail-closed read/projection helpers, never approval/execute.
- Host action IDs/risk labels live only in `apps/vivim-host/control/action-catalog.v1.json` once Lane B lands. `tests/compatibility/host-actions.baseline.v1.json` is TEST ONLY.
- HQ semantic IDs remain HQ scoped; personal source refs require owner binding. Neither a UI frame nor a DCB is an effect grant.
- Node 0 event type descriptions are a beta minimum; do not treat these as a complete native InputSnapshot implementation.

See `tests/migration/W01-F-ledger.v1.json` for donor SHA, exact imported files and evidence conditions. No private user data is needed to run these fixtures.
