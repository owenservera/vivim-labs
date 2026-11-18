# ARBOR HVI B — original HQ identity crosswalk and pinned lineage

**Status:** SOURCE_PRESENT / PREPARED_NOT_PROMOTED on the isolated worker branch. No Host/runtime acceptance.

The original repos-hq/hq/catalogue-identity C4 identity/orphan package was ported from pinned donor main 9b922ea049dbadc94e504b5b9432743995c020ca. Original C4 code retains schema hq.catalogue-crosswalk/1; new ARBOR identity projection uses arbor.hvi.identity/1. Neither creates a source registry, operational graph or effect authority.

## Original pure components
- crosswalk.mjs: buildCrosswalk(input), source-native stable IDs, explicit many-to-many links, review-only overlap and orphan findings. No inferred tombstones.
- labs-adapter.mjs / source-adapters.mjs: pure source-owned legacy catalogue inputs. Labs records are claims, not product installations.
- observer-export.mjs: deny-by-default source allowlist and redacted Labs observer output.
- cli.mjs: opt-in local developer snapshot JSON reader/writer, **not** a Host API or browser/network integration.

## New versioned adapter
Under hq/version-intelligence/identity/, index.mjs exports buildPinnedIdentityCrosswalk, projectPinnedIdentities, sourceScopedRevisionId and stableLineageId. Every input pin is resolved to an immutable 40-hex commit plus 64-hex manifest digest. Repository + commit + source-native record ID distinguish exact versions; repository + source-native record ID connects lineage without collapsing separate repos.

Source-declared capability and semantic IDs remain claims until owner review. Equivalence decisions only annotate pairs: never merge records, retire code, authorize an effect or assert runtime proof. Conflicting pin, blob, ID or replay digest is refused. Missing or partial scans remain explicitly unobserved.

legacy-adapters.mjs provides separate adaptC4CrosswalkToHvi and adaptCatalogueDeltaToHvi bridges, preserving hq.catalogue-crosswalk/1, vivim.catalogue-export/1 and hq.semantic-registry/1 without mutating those pre-existing HQ contracts.

## Local tests for G1 (UNRUN in this connected-GitHub worker)

    node --test hq/catalogue-identity/test/crosswalk.test.mjs
    node --test hq/version-intelligence/identity/test/identity.test.mjs
    node hq/catalogue-identity/test/benchmark.mjs

Developer CLI only accepts operator-selected offline inputs and output; do not point it at browser profiles, credentials, personal corpora or runtime donor checkout. G1 must run Node 24 tests after serial integration; no Windows/Android/Bun/Cargo/Host grant/security acceptance exists here.

Provenance, changed blob map, nonported historic donor files, compatibility and test ledger: .integration/proposal5/hq-version-intelligence/proofs/B/. Historical C4 HANDOFF.md and old execution logs are intentionally excluded, not newly certified HVI test receipts.
