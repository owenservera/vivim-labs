# VIVIM KB / Data-Systems Consolidation

**Status: discovery scaffold, not an inventory or migration.**

Mission: find **everything that has a schema, transport, data transformation, storage, state, identity, event, index, engine or protocol**, including transient/in-memory systems, migrations and UI programmability. Scope is open-world across Repos HQ, VIVIM Web Harness, TierFlow, Labs, and connected legacy code.

## Ownership
Repos HQ: canonical code index, wiki, knowledge provenance and semantic crosswalk. Host: owner grants, operational effects, receipts and device identity. TierFlow: UI document and revision/CAS semantics. Labs: discovery workbench, audit evidence and observability. Do not introduce a second canonical graph, indexer or transactional owner.

## Contents
- [Discovery taxonomy](docs/TAXONOMY.md)
- [UI data model scope](docs/UI-PROGRAMMABILITY.md)
- [Scout launch specification](SCOUTS.md)
- [Source roots](data/source-roots.json)
- [Discovery record schema](schemas/discovery-record.schema.json)

Scouts must report the actual source path and SHA, producer, reader, writer, authority, transport, transformations, storage, schemas, trust boundary, data lifecycle and consumers. Output typed edges: READS, WRITES, TRANSFORMS, TRANSPORTS, PRODUCES, CONSUMES, DERIVES, VALIDATES, AUTHORIZES and MIGRATES.

A disposition is KEEP / CONNECT / CONSOLIDATE / MIGRATE / RETIRE / INVESTIGATE; during discovery this is a **proposal only**. Never infer runtime activity or production certification from source presence; missing data is unknown, not absent. No deletion or production cutover in this phase.
