# ARBOR development intelligence

This HQ package is **physically internal** to ARBOR. Existing JS indexing,
semantic IDs, FTS search, context, graph read, capability analysis and wiki
algorithms remain under hq/lib. See source-provenance.json for donor ancestry,
including the audited symlink-security amendment to hq/lib/walk.mjs.

## Application-facing boundary

Import createDevIntelligence from ./lib/arbor-service.mjs.

**Host mode (required for ARBOR product callers):**

    const hq = createDevIntelligence({
      mode: 'host', dataDir: hostUserDevDataDirectory,
      actorId: trustedActorId, sessionId: trustedSessionId,
      approvedRoots: hostApprovedDevelopmentSourceRoots,
      authorize: ({ actorId, sessionId, sourceId, sourceRoot, operation, domain }) =>
        hostCheckExistingGrant({ actorId, sessionId, sourceId, sourceRoot, operation, domain }),
    });

hostCheckExistingGrant is a Host-owned function supplied by Lane B and must
read LIVE Host permissions on every operation. No grant is stored in HQ.
Use exact approved project roots, not a shared parent folder.
A registry row is metadata only. HQ has no personal World or effect authority.

**Diagnostic CLI (operator-local only):**

    node hq/bin/arbor-hq.mjs help
    node hq/bin/arbor-hq.mjs register --name demo --source C:\code\demo --allow C:\code\demo
    node hq/bin/arbor-hq.mjs index --allow C:\code\demo
    node hq/bin/arbor-hq.mjs search indexer --allow C:\code\demo

It uses explicit mode=diagnostic, never to be exposed through phone/server
endpoints. The original broad hq/bin/hq.mjs CLI is preserved as donor source,
NOT a product-facing Host command router.

## Source contracts

Source root ID: dev-source:SHA256 of canonical root path (stable on same user
device/root); legacy sourceId project:path preserved in
arbor.dev-source-evidence/1. indexWatermarks reports Git-index revision and
an honest HEAD-only freshness class; sourceEvidence hashes actual bytes and
marks stale when its content or revision differs. Context verification discards
untrusted absolute-path hints and checks every source against the index.
Graph is a derived read projection only; native computation belongs to Lane A.

## Storage

Fixed per-user data root on Windows:
%LOCALAPPDATA%/ARBOR/development-intelligence

Index dataDir/.hq/index.db, registry dataDir/projects.json and generated
dataDir/wiki. No donor SQLite, source checkout, service or network needed.
SQLite FTS5 and node:sqlite require Node >=24 for the supported beta.
Git is optional for non-Git repositories but needed for revision stamps.
Do not bundle any user's cached data or index. The Host installer must
enforce owner-only ACL on the data root.

## Tests — pending execution by G / CI

    cd hq
    node --check lib/arbor-service.mjs
    node --check lib/walk.mjs
    node --test test/arbor-service.test.mjs test/arbor-boundary.test.mjs test/in-repo-closure.test.mjs
    node --test test/indexer.test.mjs test/search.test.mjs test/graph.test.mjs test/context.test.mjs test/wiki.test.mjs test/semantic-registry.test.mjs

Tests were authored and staged, **not executed** in the GitHub connector.
Windows ACL/junction, low-disk and full cross-lane integration remain blocked.
See W01D-FOLLOWUP-REPORT.md.
