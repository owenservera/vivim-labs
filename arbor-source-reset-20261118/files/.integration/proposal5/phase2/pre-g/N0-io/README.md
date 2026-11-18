# Proposal 5 / N0-IO pre-G staging

**PREPARED_NOT_PROMOTED.** Sources and hashes: SOURCE-MAP.md. Exact R001 technical differential: R001-DIFFERENTIAL.md. Post-G owner handoff: W2-NATIVE-HANDOFF.md. Bounded schema: IO-CONTRACT.json; no client Actor or grant. Inert type mirrors: reference.types.ts / reference.types.rs (not runtime-linked). Self-contained dependency-free negative proof: synthetic-proof.mjs. Tests and non-tests: RUN-LEDGER.json.

Run the synthetic proof from repo root:

    node .integration/proposal5/phase2/pre-g/N0-io/synthetic-proof.mjs

This is not a Host connector, semantic router, graph engine, scheduler, daemon, data store or second publisher. No source files outside this staging directory are written. Design source is Labs parallel event fabric and Node-0-Prime/R001; actual prototype exists only on the Harness R001 branch. Only G can integrate post-freeze code and run release gates.
