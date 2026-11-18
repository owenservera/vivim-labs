# Proposal 5 / 01 — The Exact Self-Contained Beta Scaffold

**Decision:** The first ARBOR release physically contains all selected product and intelligence runtime code. This is not a set of adapters that call separate donor repositories. Source-preserving imports and a limited set of internal compatibility boundaries minimize implementation time.

## Exact proposed repository

```text
ARBOR/
├── .integration/
│   ├── proposal1/                    # alternative, preserved
│   ├── proposal4/                    # alternative, preserved
│   └── proposal5/                    # THIS proposal; review-only
├── Cargo.toml                       # single Rust workspace
├── Cargo.lock
├── rust-toolchain.toml              # pinned Windows-compatible stable
├── package.json                     # one Bun workspace
├── bun.lock
├── .gitignore                       # target, node_modules, user data, profiles, secrets, logs
├── AGENTS.md                        # no-external-wiring and one-effect-writer rules
├── README.md                        # one bootstrap, one release, current beta limitations
│
├── src-tauri/                       # EXISTING desktop app subtree, imported with paths preserved
│   ├── Cargo.toml                   # upgraded/pinned patched Tauri 2.x family
│   ├── capabilities/               # strict origin/webview command scopes
│   ├── tauri.conf.json
│   └── src/main.rs                 # secure native bootstrap; minimal Node0/HQKG commands
│
├── apps/
│   ├── vivim-host/                 # EXISTING Host retained internally
│   │   ├── server.mjs              # one Host process, not an external repo
│   │   ├── control/                # existing catalog, NLCL, dispatcher, journal, receipts
│   │   ├── bootstrap/              # Host lifecycle and status
│   │   ├── onboarding/             # provider/session connection
│   │   ├── conversations/          # captures/streams
│   │   ├── media/                  # secure range-streaming when verified
│   │   └── public/                 # existing working desktop UI
│   └── mobile-relay/               # EXISTING single PWA, unchanged paths at import
│       ├── app.js
│       ├── control-client.mjs
│       ├── ux/
│       └── ...
│
├── studio/                         # EXISTING signed-in Chrome/Studio engine
├── harness/                        # only needed dependency closure for Host+Studio
├── core/
│   └── programmability/            # EXISTING pure Rust validator, no competing authority
│
├── native/
│   ├── hqkg/                       # one imported canonical HQ Rust graph crate
│   │   ├── Cargo.toml              # original hq-knowledge-graph package identity
│   │   ├── src/                    # typed DAG, engine, change sets, retrieval, import
│   │   └── tests/                  # source-preserving regression suite
│   ├── protocol/                   # SMALL new typed Rust IDs/events/source/effect states
│   └── node0/                      # SMALL new bounded Rust semantic read/projection layer
│
├── hq/                             # IN-REPO operational HQ intelligence (not an outside bridge)
│   ├── bin/hq.mjs                   # same CLI entrypoint; internal
│   ├── lib/
│   │   ├── indexer.mjs             # internal development source index
│   │   ├── schema.mjs
│   │   ├── graph.mjs
│   │   ├── search.mjs
│   │   ├── semantic-registry.mjs
│   │   ├── context.mjs              # deterministic source-bounded context
│   │   ├── wiki.mjs                 # deterministic generated dev wiki
│   │   ├── capabilities.mjs
│   │   └── ...                     # audited transitive dependency closure
│   ├── tests/                       # retained/ported native and Node behavior tests
│   ├── config/                      # generated defaults only, never donor private config
│   └── README.md                    # operation under ARBOR, local source roots
│
├── packages/
│   ├── tierflow/                   # SELECTED upstream TierFlow source inside ARBOR
│   │   ├── package.json
│   │   ├── src/                    # renderer, visual compiler/projections, UI adapter
│   │   ├── tests/
│   │   └── ...                     # only its verified required dependency closure
│   ├── protocol-ts/                # generated type-safe Host/Rust/HQ/TierFlow bridge
│   └── view-adapters/              # thin compatibility view mounts, no second UI publisher
│
├── capability-spaces/              # LOGICAL organization, no new service or database
│   ├── interaction/                # Node0 + NLCL + UI R007 contract status
│   ├── conversations/              # signed-in provider browser capabilities
│   ├── host-control/               # source-owned catalog/action/approval+receipt
│   ├── dev-intelligence/           # imported HQ indexing/context/wiki
│   ├── graph-knowledge/            # shared native hqkg and source projections
│   ├── personal-world/             # Host-owned private World and consent
│   ├── context-dcb/                # source-linked context, memory-off assumptions
│   ├── mission-control/            # mission status, recovery, source truth
│   ├── visualization/              # TierFlow, UX, AutoWiki read projections
│   ├── pairing-pwa/
│   ├── onboarding/
│   ├── media/
│   ├── voice-to-tasks/
│   ├── chronos-automation/
│   ├── app-builder/
│   ├── personalization/
│   └── observability/
│
├── contracts/
│   ├── schema/                     # versioned canonical identity/event/effect/view types
│   ├── generated/                  # build-generated Rust↔TS interop; not a new catalog
│   ├── compatibility/              # legacy ID and risk status crosswalk
│   └── golden/                     # semantic and effect differential corpus
│
├── integration/
│   ├── source-lock.json            # exact imported SHA, path and content digest
│   ├── import-manifest.json        # physical source/asset import inventory
│   ├── retirement-ledger.json      # one destination and one owner per legacy path
│   ├── capability-index.json       # DERIVED view, never executable action owner
│   ├── release-manifest.json       # Rust, Host, PWA, HQ, TierFlow, Studio versions
│   ├── no-external-wiring.rules    # automated forbidden paths/endpoints check
│   └── scripts/                    # import, verify, package, local-only smoke
│
├── config/
│   ├── defaults/                   # owner-configurable UI, resource and storage defaults
│   ├── profiles/                   # safe local-first presets
│   ├── storage/                    # bounded Rock Core/cache retention policies
│   └── schema/                     # versioned config, no grants inferred
│
├── tests/
│   ├── no-external-repos/          # product works with donor checkouts absent
│   ├── rust-graph/
│   ├── hq-intelligence/
│   ├── host-action-parity/
│   ├── node0-concurrency/
│   ├── tierflow-projection/
│   ├── paired-pwa/
│   ├── browser-provider/
│   ├── storage-and-recovery/
│   └── security/
│
├── docs/
│   ├── accepted-architecture/
│   ├── beta/
│   └── operations/
└── .github/workflows/
    ├── cargo-and-bun.yml
    ├── integration-proof.yml
    ├── windows-beta.yml
    └── security.yml
```

### Intentional source-path preservation

Retain `src-tauri/`, `apps/vivim-host/`, `apps/mobile-relay/`, `studio/`, `harness/`, and `core/programmability/` initially. This preserves existing relative imports, fixture paths, HTML routes and Tauri Node Host spawn expectations, minimizing time spent on mechanical restructuring. **But these paths are all inside ARBOR and packaged from ARBOR.**

Retain the operational `hq/` path **inside ARBOR** to preserve the HQ JS module dependency graph and CLI invocation. Do not install an HQ service pointing at the old `repos-hq` checkout. Import only audited source dependencies, not the old HQ runtime database/corpus.

Import required TierFlow source into `packages/tierflow/`, not a Git submodule that requires the external repo to be present. A vendored source snapshot with manifest+checksum and periodic deliberate upstream updates is acceptable. One UI view publisher.

## Small Cargo/Bun workspace

Illustrative Cargo root:

```toml
[workspace]
resolver = "2"
members = [
  "src-tauri",
  "core/programmability",
  "native/hqkg",
  "native/protocol",
  "native/node0"
]
[profile.dev]
debug = 0
incremental = false
```

HQKG's source dev profile explicitly disabled debug symbols and incremental rebuild caches after a multi-GB local target observation; preserve effective low-storage behavior when importing it.

Bun root workspaces include the existing Host/mobile/Studio/HQ modules that actually define package manifests, plus `packages/tierflow`, `packages/protocol-ts`, `packages/view-adapters`. **Do not invent manifests for every folder until their runtime import graph has been inventoried.** One supported package/lockfile owner, no competing global npm/pnpm installs.

## Logical capability spaces, not capability implementations

Every capability space is generated from **in-repo source owners**, e.g. Host action catalog, HQ semantic registry, installed HQKG engine slots, TierFlow renderer contracts and validated Labs contributions. Do not write another executable registry or new graph store.

A capability entry contains:
`id, sourceOwner, implementationPath, inputSchema, outputSchema, effectClass, requiredSourceScope, version, runtimeHealth, sourceEvidence, UIProjection, tests, betaStatus, configRef, deprecation`.

Statuses remain distinct: `documented`, `source_present`, `build_verified`, `runtime_wired`, `device_accepted`, `available_now`, `unavailable`, `deferred`.

**Module organization is the future-proof seam.** Current code can remain at old internal paths while a module manifest points to its in-repo implementation. Future Rust replacements need only satisfy the unchanged contract and pass parity. This avoids a second mass refactor.

## Internal and external boundary policy

**Forbidden product wiring:**
- cross-repo `../repos-hq`, `../tierflow`, `../vivim-labs`, `../vivim-web-harness`, Git submodule dependency or `git+ssh` direct runtime source import;
- shelling out to a donor HQ CLI binary or talking to a donor development server;
- dependency on donor source data folders or Windows-specific development paths;
- separate TierFlow runtime service or HQ indexer external to ARBOR;
- duplicate Host effect journal or duplicated authoritative graph storage.

**Allowed external resources:**
- external signed-in web application websites, the user's browser and actual local file/project repos as user-requested data sources;
- Windows OS functionality/installed WebView2/Chrome according to a tested installation contract;
- normal locked package dependencies resolved during CI/build. The installed beta must not need source checkouts or unbundled developer package managers.

## Critical acceptance test

Build and install ARBOR **after renaming/removing access to every donor checkout** (and with old donor background processes stopped), then test local status, HQ source indexing of a test project, native graph query, TierFlow view, signed-in Chrome conversation, device pairing and Host read/effect receipt. The user must not need to know which donor repo each implementation came from.

**Status:** exact design only. No claimed source imports or passing tests.
