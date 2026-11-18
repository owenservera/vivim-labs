# HVI risks, owner decisions and non-goals
## Accepted strategic decisions
- ARBOR `main` is the only canonical product development branch.
- HQ comparison is **read-only** by default; tools may create derived local source indexes under Host-granted development storage but never issue an effect without Host approval/receipt.
- `repos-hq/hq/catalogue-identity` is the preferred pure crosswalk donor; port its best source rather than reimplement parallel identity registry.
- `hq/lib/catalogue-delta.mjs`, `hq/lib/semantic-registry.mjs`, `hq/lib/graph.mjs`, `hq/lib/arbor-service.mjs` remain canonical existing HQ boundary; new modules compose them, not replace them.
- 2..N pins in one repo and N repositories are first-class; each pin is immutable at resolved SHA.
- Physical parity and source closure run before claims of semantic equivalence, before runtime parity.
- No automatic mass import of omitted HQ code, no replacement of Host security system, no second graph/store/scheduler/publisher/sender.
- PWA-first; no APK beta dependency. Do not infer real UI acceptance from synthetic snapshots.

## Open integration questions (owners resolve with evidence, not owner interrupt)
1. Which of the 361 HQ missing paths represent unique live behavior? D classification; B/C lineage; X actual corpus audit.
2. Original `hq/supervisor` — reusable read/admission utilities or competing Host? D/source owner and Z1.
3. Live-graph Wave 1 and Rust-graph Lane B — superseded by native/hqkg or valuable tests/reads? D+F, not wholesale port.
4. Can existing `catalogue-delta` source ID authority be adapted to ARBOR safely while preserving schema compatibility? B/C, Z1.
5. Can Host pass exact current actor/session/grants to HQ for each new read? H with security proof; fail closed.
6. How does TierFlow display source confidence and unknowns without claiming an effect/action? U.
7. Which Labs main and research source is implemented, pure test, or design-only? D/X.
8. Can actual Node24/Cargo/Bun/Windows tests execute? G. Missing runner => UNRUN, never GO.
9. How are git remotes fetched under explicit authorization and stored locally for offline provenance without hidden donor-runtime coupling? A/H.

## Priorities if scope tight
P0: immutable pins, original crosswalk, N-way diffs, explicit omitted classification, tests.
P1: real five-repo audit, Host-granted read port, deterministic export.
P2: interactive TierFlow inspector; preserve summary report if UI cannot be accepted.
Deferred: full AI semantic embeddings, personal corpus migration, autonomous repository modification, general automated refactors, new graph scheduler, speculative AutoWiki production, native interpreter rework.
