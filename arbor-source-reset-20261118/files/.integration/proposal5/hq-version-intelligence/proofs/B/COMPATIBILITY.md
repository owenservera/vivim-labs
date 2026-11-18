# HVI B — compatibility, source authority and refusal decisions

## Pinned baseline and identity authority
- Work branch: work/p5-hvi-w1-b-identity-r001 from S0 workerBaseSha f9823a5131e73e7562db5a4ec14e1ec481557d69, NOT the old 370d18d9 planning baseline.
- S0 proof: work/p5-hvi-s0-source-lock-r001@2ef7a9f66419d681e040b459ba4762a63aa9564f, WORKER-BASE.json gate GO_TO_PARALLEL. Donor repos-hq main pinned at 9b922ea049dbadc94e504b5b9432743995c020ca.
- Source compatibility grounded in original hq/catalogue-identity code and ARBOR hq/lib/catalogue-delta.mjs@blob 6849e6df26bd136ee50ae08a260d8ab9e9445ac7, semantic-registry.mjs@blob ec982381c77147119f62e702a162a203df8e7d4e, capabilities.mjs@blob 342563729c6743e98870aa899d2d6876d7369d75.
- Labs public capability observatory input at vivim-labs main 022608f5797e7b4a14ccc4c393bd3c5a4704a4be, capability-observatory/data/capabilities.json blob 4c5876bd40d727b6e7464f71dbe23679f5558f8a. No personal corpora consumed.

## Schema decisions
1. Original hq.catalogue-crosswalk/1 remains unchanged in crosswalk.mjs and is the owned source-native identity/orphan projection. It is NOT a parallel graph database, source authority, or operational registry.
2. Existing vivim.catalogue-export/1 and vivim.catalogue-delta/1 in hq/lib/catalogue-delta.mjs are NOT modified. Versioned legacy-adapters.mjs validates the existing digest via the exported query function before converting records to ARBOR HVI input.
3. Existing hq.semantic-registry/1 has its own owner-promoted name@version identity contract and is not rewritten. New pinned adapter consumes explicit declared canonicalId, never guesses one from names or locations.
4. New schema arbor.hvi.identity/1 distinguishes (repo, immutable commit SHA, source-scoped record id), and a separate repository/source key connects versions. Same file name across repositories cannot collapse. A record may participate in many-to-many declared links without ownership transfer.
5. Candidate overlap under lineage or declared canonical IDs remains REVIEW_REQUIRED. Different implementation digests, source types or owners yield explicit conflicts. Reviewed owner evidence is an annotation; merger, retirement and runtime equivalence remain unproved.
6. Existing source C4 exports have sourceRevision in their own domain. The compatibility shim requires matching pinned SHA for C4; the catalogue-delta adapter retains sourceRevision as a legacy claim, not a replacement for a Git pin.
7. S0's 361 originally absent HQ paths remain a source-path census; the new B crosswalk does not certify them missing capabilities, does not generate mass imports, and does not infer tombstones.

## Safety and refusal scope
- All new identity modules are pure: no network, source checkout, browser sender, Host effect, graph writes, scheduler or new operational DB.
- HVI pin scope must be development; revision and manifest digests are explicit. Duplicate IDs, contradictory digest, malformed path, owner-decision without reviewed evidence, replay fork and secret-looking evidence paths fail closed with typed HviIdentityError codes.
- Observer source export defaults deny; the original donor observer is source-adapted to validate snapshot SHA-256 and reject non-array source allowlists.
- Explicit owner-confirmed equivalence is **not** a signature verification. Decision output sets receiptCryptographicallyVerified=false; G1/F must define trusted approval receipt validation before claiming equivalence in product UI.
- Donor crosswalk source still has bounded output but some repeated-name pairing work can be quadratic before cap; G1/F should benchmark and harden without assuming a throughput threshold.
- The original operator CLI can write an explicitly chosen local output file. It is not mounted as a Host route, and caller path/scope authorization must be reviewed before any product integration.
- No personal Labs records, browser profile state, signed-in session, credentials or provider APIs scanned.

## Coverage boundary
- B produces identity and candidate lineage only. A owns immutable snapshot acquisition, C N-way comparison, D semantic capability/source parity, E final reports, F independent conformance, H live Host grants, X real five-repo audit. Any runtime claim or merge decision beyond this is not authorized.
- Node24 and actual checkout execution was not performed by this GitHub connector session. Treat source presence / synthetic tests as PREPARED_NOT_PROMOTED, never RUNTIME_VERIFIED.
