# ARBOR Proposal 5 — PRE-G final freeze receipt — Lane D (HQ)

**Lane:** D / HQ development intelligence
**State:** FREEZE_READY (source/evidence freeze only; not product, security, build or device acceptance)
**Worker branch:** refs/heads/work/p5-w01-d-hq
**Pre-report verified HEAD:** aa6284812432cbc375bbe6b1f94bb867a0e72cfa
**FROZEN TIP IDENTIFIER:** refs/heads/work/p5-w01-d-hq at the commit introducing this exact file, integration/worker-reports/PREG-D.md. Its object SHA must be read from the Git ref after committing: embedding that SHA into its own committed content is circular. No later lane writes are authorized.
**Common worker architecture base:** dd245d78ff3c110ba8630cad557ea0479c51a635
**Donor repo:** owenservera/repos-hq
**Donor revision:** main at 9b922ea049dbadc94e504b5b9432743995c020ca
**Donor subtree:** hq/ at Git tree 48e48313187e3747ac70206276a1a286c94ab002
**Source truth:** hq/source-provenance.json (all 315 source paths, SHA pins, amended walk.mjs provenance); hq/MIGRATION-REPORT.md; hq/W01D-FOLLOWUP-REPORT.md.
**G status:** integrate/proposal5-beta-r001 was absent at the freeze instruction check. This lane has not created or updated it.

## 1. Complete commit chain; no omitted first follow-up

| SHA | Parent | Type | Evidence |
| --- | --- | --- | --- |
| d350a2d96dc8f9c5d60da87879f32ebe529dfb4b | dd245d78ff3c110ba8630cad557ea0479c51a635 | W1D physical import | 186 original HQ donor source blobs imported plus ARBOR-owned runtime/report/test scaffolding; no other lane paths |
| 16c590dbd97dc617b783d949268ddc8fb77da2c5 | d350a2d96dc8f9c5d60da87879f32ebe529dfb4b | W1D donor dependency-closure import | +129 original donor files: 3 hq/registry JSON and 126 hq/aii/fixtures files; total 315 original donor blobs |
| aa6284812432cbc375bbe6b1f94bb867a0e72cfa | 16c590dbd97dc617b783d949268ddc8fb77da2c5 | Focused follow-up and final implementation commit | Host-mode source authorization interface, revocation/source-evidence/watermarks, symlink hardening, test cases and provenance/report updates |
| **FROZEN TIP (this report commit)** | aa6284812432cbc375bbe6b1f94bb867a0e72cfa | PRE-G closeout only | Adds exactly integration/worker-reports/PREG-D.md, no product, manifest, lockfile or other worker edits |

The last implementation commit changed these nine HQ-owned paths only:
hq/README.md; hq/W01D-FOLLOWUP-REPORT.md; hq/bin/arbor-hq.mjs;
hq/lib/arbor-service.mjs; hq/lib/walk.mjs; hq/package.json;
hq/source-provenance.json; hq/test/arbor-boundary.test.mjs;
hq/test/arbor-service.test.mjs.

Of those, **code**: hq/bin/arbor-hq.mjs, hq/lib/arbor-service.mjs,
hq/lib/walk.mjs, hq/test/arbor-boundary.test.mjs,
hq/test/arbor-service.test.mjs, hq/package.json (packaging/scripts).
**Documentation/evidence**: hq/README.md, hq/W01D-FOLLOWUP-REPORT.md,
hq/source-provenance.json. The freeze commit adds this report only.

## 2. Complete physical source inventory and exclusions

**Original donor file classes, exact per-file identities in source-provenance.json:**

| Donor class / source paths | Files |
| --- | ---: |
| hq/lib/** operational JS: schema, indexing, Git, registry, search, symbols, graph, context, wiki, capability/source analysis, semantic registry, CLI support | 127 |
| hq/bin/** existing CLI and supporting sources | 9 |
| hq/test/** selected original regression tests | 47 |
| hq/capabilities/registry/** versioned data contracts | 2 |
| hq/types/** original typing support | 1 |
| hq/registry/** canonical taxonomy, binding and agentic registries | 3 |
| hq/aii/fixtures/** selected source and test fixture harness | 126 |
| **Imported donor-tracked files** | **315** |

**Verified Git object inventory:** at implementation HEAD aa628481..., all
315 entries in hq/source-provenance.json resolve to the same blob SHA in the
ARBOR HEAD tree. This is *Git identity verification* of the declared
ARBOR inventory, not an executable test or assertion that every file remains
byte-identical to the donor. **314 donor blobs remain byte-identical**;
**one is SOURCE_PATCHED:**

- hq/lib/walk.mjs: donor blob
  c3fb10ada3b078f9f6f2e0429d0c5829a7b155fe;
  patched ARBOR blob 38f8bfe82220909cbd456d3115ecad350656fe1e;
  rationale: realpath boundary refusal for Git-tracked symlink escape before
  stat/sniff/index. Both blob identities are retained in source-provenance.json.

**Additional ARBOR-owned HQ files (11, not counted as original donor blobs):**
hq/.gitignore; hq/MIGRATION-REPORT.md; hq/README.md;
hq/W01D-FOLLOWUP-REPORT.md; hq/bin/arbor-hq.mjs;
hq/lib/arbor-service.mjs; hq/package.json; hq/source-provenance.json;
hq/test/arbor-boundary.test.mjs; hq/test/arbor-service.test.mjs;
hq/test/in-repo-closure.test.mjs.
Total files under hq/ at implementation HEAD: **326**.

**Deliberate source exclusions:** old hq/.hq/ or other developer SQLite
indexes and cache, node_modules, credentials and local browser profiles,
personal corpora, user-specific session dumps, generated logs/reports/results,
hq/aii/assessments, other donor tests targeting out-of-scope agent/media/
automation tooling, source roots and runtime checkouts outside the selected
hq/ scope. Donor research/rust-knowledge-graph was not imported: Lane A owns
native/hqkg. Retained source code may contain historical cross-repo assumptions
and legacy optional CLI modes; they are not automatically shipping/runtime
dependencies. This is a selected-source inventory, not a claim that every
optional donor CLI/test path is product-ready.

## 3. Internal source authority, privacy, G root-linker requirements

**Single ARBOR application-facing read/cached-projection boundary:**
hq/lib/arbor-service.mjs exports createDevIntelligence(), with mode='host'
or mode='diagnostic' explicitly required. Host mode requires a trusted
actorId, sessionId, *exact canonical approvedRoots* and authorize callback
supplied by Lane B. On every relevant operation, HQ calls the Host-provided
callback with { actorId, sessionId, operation, domain:'development', sourceId,
sourceRoot }. Only a literal true grants that operation. Registration in HQ
projects.json is metadata, not permission. Host must reconcile exact operation
names with its existing action/grant IDs, not create a second permission owner.

**Observed current implementation labels:**
- source.register for registerSource.
- source.revoke for removeSource, able to remove a previously revoked read root
  only when Host authorizes the distinct revocation operation.
- source.index for index.
- source.context + per-file source.read for context and verifyContext.
- source.wiki + per-file source.read for wiki.
- source.status for indexWatermarks and part of status.
- source.read for sourceEvidence, search, symbols, files, graph and semantic
  reads; current default withDatabase also requests source.read for
  scanCapabilities/status in addition to their top-level scoped checks.
- source.capabilities for scanCapabilities.

**G/B must not infer** these internal strings are already cataloged Host action
IDs; B must map existing consent/grant model and actor/session claims. No
effect writer, new Host grant journal, second dispatcher or browser credential
broker belongs to HQ. Do not expose mode='diagnostic' or broad donor
hq/bin/hq.mjs through the product server/phone routes.

**Path/data boundary:** roots canonicalized by realpath and checked against
explicit approvedRoots; path traversal and source/data overlap rejected;
walk.mjs rejects symlink escape before reading. Context verification overwrites
caller-supplied absolute path hints using indexed source records. HQ-owned
storage is per-user, fixed SQLite at dataDir/.hq/index.db, source registry at
dataDir/projects.json and generated wiki at dataDir/wiki, defaulting to
%LOCALAPPDATA%/ARBOR/development-intelligence on Windows. Legacy mutable
config.db does not set the internal adapter's DB path. Host owns user-scoped
dataDir and Windows ACL setup. No Personal World records belong in this
development index by default; private World/HQ domain isolation remains to
be tested under realistic Host grants and filesystem state.

**Revocation behavior in current source (unexecuted):** delete indexed project
rows and file FTS rows, remove associated capability scan rows, invalidate
semantic source links and generated wiki, unregister source. Complete privacy
deletion across all optional legacy derived tables, races and NTFS ACL/junction
behavior have NOT been verified.

**G root linker:** add hq as a single internal Node package/runtime,
resolve root package/Bun lock/CI and Windows installed runtime; do not spawn or
depend on donor repos-hq CLI/service, checkout, database or hard-coded developer
path. The selected beta read port has no donor process/API dependency in the
inspected source. Optional legacy CLI/tests have historical donor or research
source assumptions requiring individual compatibility classification before
exposure. Include only narrow arbor-hq diagnostic command for local checks;
never mount it as a remotely callable authority.

**Runtime prerequisites not exercised:** Node >=24, node:sqlite DatabaseSync,
SQLite FTS5 migration/FTS DDL, Git when source revision stamps are expected,
per-user writable dataDir, Windows %LOCALAPPDATA% path and ACL, optional Bun
1.4.x for legacy donor suites. No root Cargo/Bun or deployment file changed
in Lane D.

## 4. Native HQKG development read handoff (A/B/F)

The source-backed methods exposed in the current boundary are:
- indexWatermarks(): sourceId, project, domain, projectRevision,
  currentRevision, indexWatermark, file/symbol counts and one of
  unindexed/revision_changed/unversioned_not_file_verified/
  head_only_not_file_verified. A matching Git HEAD is NOT a working-tree
  freshness proof.
- sourceEvidence({project,path,startLine,endLine}):
  arbor.dev-source-evidence/1; status indexed/stale/unavailable/oversized;
  sourceId=legacy project:path, devSourceId=dev-source:SHA256 canonical path,
  projectRevision, currentRevision, indexWatermark, indexedHash, currentHash,
  bounded line range + source text. Text from a stale result MUST NOT be
  treated as approved current evidence by native consumers.
- search/symbols/files; context/verifyContext; graph (projects/deps/impact);
  wiki; semanticIds/semanticId; status; scanCapabilities.

Lane A owns native/hqkg computation, Development graph ingestion and
interoperability; F owns cross-language contract/negative tests. No duplicate
native graph implementation or Personal World ingestion was committed by D.
Source IDs and hash fields must be reconciled with F's typed interop at G/W2.
This is an exposed JS interface, **not a live Host or native transport**.

## 5. Actual tests and evidence (do not promote UNRUN)

| Method | Actual execution | Exit code | Result |
| --- | --- | --- | --- |
| Connected GitHub REST ref/commit/parent/tree reads | Performed in this ChatGPT worker | N/A: data read, not shell process | Confirmed branch ancestry, exact implementation HEAD and donor/source inventory |
| Git blob-map identity audit of 315 source entries against ARBOR implementation tree | Performed using GitHub tree + source-provenance | N/A: tool/data comparison, not test executable | 315/315 manifest paths match declared ARBOR blobs; donor equality 314/315 |
| Static inspection of key JS boundary/source modules and direct relative import locations | Performed using GitHub source reads | N/A: not a Node command | Found no missing direct relative imports in 10 inspected principal HQ modules; transitive/legacy closure not established |
| node --check lib/arbor-service.mjs / lib/walk.mjs / bin/arbor-hq.mjs | **UNRUN** | N/A | G/CI required |
| node --test test/arbor-service.test.mjs test/arbor-boundary.test.mjs test/in-repo-closure.test.mjs | **UNRUN** | N/A | Source-auth, evidence, symlink, revocation, corruption, deny cases not executed |
| node --test test/indexer.test.mjs test/search.test.mjs test/graph.test.mjs test/context.test.mjs test/wiki.test.mjs test/semantic-registry.test.mjs | **UNRUN** | N/A | Legacy selected core regressions not executed |
| Full 47 selected original donor tests, Bun-specific tests, optional legacy CLI | **UNRUN** | N/A | Selected cases require other source/test paths and separate compatibility selection |
| Native Rust Cargo cross-language integration; Host actor/grant integration | **UNRUN** | N/A | A/B/F/G required |
| Windows Node24/SQLite FTS5 and packaging, ACL, NTFS junction/symlink race, ENOSPC, clean donor-offline install | **UNRUN** | N/A | G/F acceptance required |
| Browser/Android PWA physical tests | **UNRUN** | N/A | E/B/F owns device acceptance, not a D claim |

No passing Node, Bun, cargo, native, Host, Windows security, packaging or
physical-device test result is claimed by this lane.

## 6. Blocker classification and actionable G/W2 handoff

| Classification | Meaning / status | Responsible |
| --- | --- | --- |
| SOURCE_IMPORTED | 315 donor-tracked in-repo HQ files imported and manifest-identified; selected beta runtime has internal relative source modules, not donor checkout | D DONE; G verify packaging |
| SOURCE_PATCHED | hq/lib/walk.mjs donor-byte variation plus ARBOR-owned Host-scoped adapter, tests and internal diagnostic CLI; changes committed on existing D branch | D DONE; G/F execute |
| BUILD_UNRUN | Node24 node:sqlite+FTS5, full graph/FTS/context/wiki flows, symlink and revocation regression, root linker, lockfiles and clean installed Windows/CI build not executed | G/F BLOCKING validation |
| DEVICE_UNRUN | No physical device/Windows Host-origin/browser PWA acceptance performed in Lane D | B/E/F/G external release gate |

**G (sole integrator):** take frozen D ref with exact commit SHA (this report
commit), never the stale 16c590db or pre-follow-up head alone. Resolve root
workspace/lockfiles, import and package internal Node>=24 runtime. Verify
source module closure in combined tree and run executable tests with donor
repositories *absent*. Decide legacy HQ CLI/test inclusion deliberately. Do
not create a separate HQ process/graph authority. Emit G source lock and W1
merge gate with real test results, not inferred success.

**B (sole Host source/permission authority):** implement actual host-mode
createDevIntelligence caller, trusted actor/session and current source grant
lookup, exact project root allowlist, separate source.revoke permission,
revocation race/fail-closed contract, and per-user dataDir ACL. Confirm source
identity lifecycle under renamed/moved projects and live sessions. Test
unapproved read, stale source, malicious path, source revoke and no-Personal-
World data mixing. Map operation labels to existing B Host catalog/receipt
contracts. No second source auth registry in D.

**A (sole native HQKG owner):** implement/validate typed Development graph
read input from Host-scoped evidence/watermark/semantic-source outputs only.
Reject stale content if current evidence is required. Preserve native graph
authority and domain separation; no imported donor Rust graph in D.

**F (independent tests and contracts):** run real boundary/security regression,
negative actor/session cases, missing/broken Git, corrupt/rebuilt index,
disk-full, Windows junction and Personal World leakage tests. Verify
JSON/TypeScript/Rust field compatibility and source evidence contract.

**C/E:** no D-owned edits. G/B must keep single authorized browser sender,
one direct PWA and inert TierFlow; do not route HQ diagnostics through mobile
or create a second UI publisher. Release-critical S1 native launch and S2
browser PWA enrollment remain external, unpassed G/W2 gates, not D fixes.

**Phase 2 Labs:** PREPARED_NOT_PROMOTED. No new AutoWiki, personal knowledge
pipeline, scheduler, application builder, or Labs promotion in this lane.

## 7. Freeze conclusion

FREEZE_READY for **source and evidence handoff only**. No more writes to
work/p5-w01-d-hq after PREG-D is committed. G must use the final report-commit
ref as the authoritative frozen tip and must not mistake source presence for
HOST_WIRED, BUILD_PASS, SECURITY_ACCEPTED or DEVICE_ACCEPTED.
