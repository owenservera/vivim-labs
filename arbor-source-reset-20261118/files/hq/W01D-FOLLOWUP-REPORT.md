# ARBOR P5 — Lane D follow-up integration assessment and HQ-owned patch

**Gate result:** BLOCKED_ON_G_INTEGRATION as observed during this follow-up.
The required integrate/proposal5-beta-r001 ref was absent. The existing
work/p5-w01-d-hq was at 16c590dbd97dc617b783d949268ddc8fb77da2c5.
No work/p5-w02-d-hq branch was created. No merge, rebase or root change.

**This is a disjoint HQ patch on the existing lane branch, NOT a W2 GO.**
G must establish its own combined base and run cross-owner tests.

## Source-preserving internal dependency closure

Preserved original HQ JS/index algorithms, schema, search, semantic registry,
graph observations, context composition, deterministic wiki, capability scan,
support modules, original legacy CLI and focused regression tests.
Original donor: owenservera/repos-hq
9b922ea049dbadc94e504b5b9432743995c020ca.
The W1 physical import contained 315 exact Git blobs. In this patch 314 stay
byte-identical; one original HQ source file, hq/lib/walk.mjs, is adapted to
refuse Git-tracked symlink escapes. Provenance records both blob identities.

Runtime dependency imports are internal relative JS modules under hq/lib,
hq/aii/fixtures, hq/registry, and hq/capabilities/registry.
The ARBOR application-facing entrypoint is hq/lib/arbor-service.mjs, in the
same ARBOR repository. It has no repos-hq process, service, checkout, SQLite
index or external CLI dependency. Legacy hq/bin/hq.mjs remains imported as
source but is not the beta application-facing API; selected historic CLI
subcommands and legacy tests retain non-beta assumptions requiring G audit.

## Boundary contract (Lane B)

createDevIntelligence has an explicit required mode, diagnostic or host.
For host mode, Lane B MUST pass:

- actorId and sessionId from trusted authenticated Host context; never from
  model text, phone input or arbitrary HQ request bodies;
- approvedRoots from Host-owned consent/grant state (exact canonical local
  development project roots, not arbitrary parent directories);
- authorize callback that returns true only if the current Host actor/session
  has the corresponding source-operation permission and is not revoked.

Each request receives operation, domain=development, sourceId (a stable
dev-source:SHA256 of a canonical root path), and sourceRoot.
A stored project registration is NOT a grant. Callback denial rejects.
There is no new authorization database or effect writer in HQ.

Operations: source.register, source.index, source.search/source.read,
source.context, source.graph/source.wiki, source.status,
source.capabilities, source.revoke. Lane B owns exact mapping to its
existing Host action catalog and risk/grant model; do not invent new Host IDs.

The separate diagnostic-only CLI opts into mode=diagnostic and requires
explicit --allow project roots for each call. Do not mount this diagnostic
mode in a remote/phone API. Host must own dataDir and its per-user ACL.

## HQKG and Host read port (Lane A/B)

- indexWatermarks(): [{sourceId,project,domain,projectRevision,currentRevision,
  indexWatermark,files,symbols,freshness}].
  Freshness values are unindexed, revision_changed,
  unversioned_not_file_verified, head_only_not_file_verified.
  HEAD equality does not prove working-tree content currency.
- sourceEvidence({project,path,startLine,endLine}) returns
  arbor.dev-source-evidence/1 including sourceId legacy project:path,
  devSourceId (stable source root identity), projectRevision/currentRevision,
  indexWatermark, indexedHash/currentHash and bounded line-span text.
  status indexed means indexed bytes match current bytes at read time;
  stale means content hash or Git revision diverged.
- search/symbols/files/graph/context/verifyContext/wiki/semanticIds/semanticId
  wrap retained HQ JS algorithms. No native graph algorithm or graph store.
- graph observations are read-only. Native/hqkg Development namespace maps
  only Host-authorized project/source revisions. No Personal World ingestion.
- Wiki is the deterministic local HQ code/documentation projection in the
  per-user ARBOR HQ data directory, not universal/personal AutoWiki.

Source roots are realpath-checked at registration and reads. Legacy walk
filters symlink escapes before content sniffing; context verification cannot
follow caller-provided absolute-path hints. Fixed database path is
dataDir/.hq/index.db; mutable legacy config.db is ignored. Revocation clears
project file FTS, associated derived capability scan records, generated wiki
and semantic source-link projections; semantic identity records are retained.

## Runtime environment

Node >=24 with node:sqlite/FTS5, Git for revision-aware roots (non-Git
sources permitted), per-user writable dataDir; Bun is optional for retained
legacy tests. Windows LOCALAPPDATA default is used on Windows, no donor
absolute development path. No production third-party package or network
dependency introduced for the supported internal read port. G owns root
workspace, lockfiles and bundling the Node child process with the product.

## Real verification evidence

Confirmed via GitHub API: branch identity, source donor SHA and copied blob
identity. Source inspection and static compatibility reasoning performed.
No executable Node/Bun runtime or Windows installation was accessible through
the connected GitHub write tools; therefore NO runtime test was executed here.
Do not mark new tests passed.

New tests (unrun): hq/test/arbor-boundary.test.mjs. Prior source regression
tests preserved. Commands G should execute from ARBOR/hq:

    node --check lib/arbor-service.mjs
    node --check lib/walk.mjs
    node --check bin/arbor-hq.mjs
    node --test test/arbor-service.test.mjs test/arbor-boundary.test.mjs test/in-repo-closure.test.mjs
    node --test test/indexer.test.mjs test/search.test.mjs test/graph.test.mjs test/context.test.mjs test/wiki.test.mjs test/semantic-registry.test.mjs

Explicit unverified: Windows junction/ACLs, symlink race under concurrent
filesystem mutation, full denial/erasure audit of all optional legacy HQ
tables, disk-full recovery, fresh/dirty Git semantics under Windows, cross-
language schema tests, full G merged runtime, clean donor-unavailable
installation, physical phone/device acceptance. Original legacy CLI contains
optional tooling modes requiring separate retirement/mount audit. Beta GO
must not be inferred from the source-only patch.

## Acceptance handoffs

**G:** Cherry-pick the HQ branch commits onto its own latest combined base
after source tree integration. Root manifests/lockfiles belong to G. Run tests
and clean-packaging smoke with donor checkouts absent. Do not expose legacy
HQ full CLI in beta. Confirm secure installed node:sqlite and index path.

**B:** Implement actual Host callback and actor/session extraction, source
approval/revocation event handling and dataDir ACL. Add Host integration
test with granted/denied/revoked actor against real Host journal; no HQ
permission writer and no app-facing diagnostic mode.

**A:** Consume sourceEvidence/indexWatermarks in Development namespace;
version and validate JSON/TS/Rust bridges from actual Host read port;
cross-check IDs/hash and test stale source refusal. Lane D does not
modify native graph code.

**F:** Run security/low-disk/junction tests on Windows, donor-offline test,
cross-domain Personal World exfiltration test and audit semantic/capability
derived record purge on revoke. Do not enable all legacy tests until
cross-lane source and fixture paths exist.
