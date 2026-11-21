# ARBOR Proposal 5 — Pre-G W-AutoWiki
## ERA 1 — What exists
Source-pinned read-only **synthetic** AutoWiki mirror: `mirror.mjs`, `negative-tests.mjs` and `run-tests.mjs`. It borrows the schema vocabulary from repos-hq, preserves source-native item IDs, and never persists generated pages. This branch contains no changes outside `.integration/proposal5/phase2/pre-g/W-autowiki/`.

## ERA 2 — How to verify
From this directory run `node run-tests.mjs` on Node 22+. Tests inject synthetic owner scope and read-only result ports; no local corpus, browser profile, private data or filesystem writes are needed. The source-byte isolated V8 run and its limitations are recorded in `RUN-LEDGER.json`. Schemas are contract-compatible **by field naming and shape**, but no Ajv/draft-2020 validator was run here.

## ERA 3 — What this does not authorize
PREPARED_NOT_PROMOTED. Not an approved W1-G/Phase2 merge, not a real Host screen observer, not an accepted TierFlow UX, not a disk quota governor, and not permission to reach private chat history. Read SOURCE-MAP, CONTRACTS, STORAGE-GUARDS and W2-OWNER-HANDOFF before integration.
