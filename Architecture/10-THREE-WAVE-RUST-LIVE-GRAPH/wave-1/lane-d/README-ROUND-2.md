# Lane D Round 2 — safe source implementation, not integrated product

Run the Round 1 + Round 2 local synthetic regressions from this directory:

```sh
node --test wiki-oracle.test.mjs test/*.test.mjs
```

Read `ROUND-2-HANDOFF.md` for evidence, blockers and the true native integration recipe. **Everything under `test/` uses synthetic source data**; the suite verifies bounded fail-closed D logic only.

### Read-only integration inspector (after E + HQ wiki owner)

```sh
node bin/inspect-readonly.mjs --exchange DEV/wave1.jsonl --wiki-dir DEV/generated-hq-wiki
```

This inspector never executes actions or writes pages. It always exits **2** and explicitly requires native Rust import/query, real generated wiki page manifest and independent F witness for integration acceptance. A good-looking source JSONL is not sufficient proof that Rust imported it or HQ generated the wiki.

### Paths

- `contracts/` — real A/B/C/E schema mismatch matrix, proposed HQ read-view and future product/visual contracts (not active).
- `lineage/` — typed E compiler chain, provenance and non-authorizing evidence statuses.
- `quality/` — per-root/domain denominators, links, known-pattern privacy, claimed page manifest and synthetic product projection.
- `bin/` — read-only dev inspection CLI.
- `fixtures/`, `test/` — synthetic negative fixtures and regressions.
- `evidence/` — raw test logs, honest evidence status, Rust-vs-SQLite deferred measurement guide.

The existing `wiki-oracle.mjs` remains unchanged; new code does not replace the HQ wiki renderer, semantic registry, KB, journal or UI owner.
