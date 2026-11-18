# Proposal 1 — Exact Scaffold, Package Boundaries and Dependency Laws

Status: CANDIDATE DESIGN, not current ARBOR source. Date 2026-11-12.

## 1. First principles and why this shape

The product's principal reliability requirement is simple: **a person must always be able to express an intention, inspect how VIVIM understands it, choose/clarify it, and know exactly what happened.** The true implementation needs three independent time scales and trust levels:

1. **Edit/interaction plane:** 0 network/native dependencies to paint user's own text, IME composition, caret and undo. TypeScript/DOM under a single UI codebase; optional JS-only deterministic speculative hints. Visual decoration is disposable.
2. **Interpretation/knowledge plane:** authenticated Rust snapshots and parallel bounded read-only derivation. Multiple independent facets and provenance; no central semantic router or single interpretation winner. Rust graph is *computation*, not a second personal data authority.
3. **Commit/effect plane:** exact user-selected committed intent, current source/grant checks, one durable exclusive claim/receipt writer, browser adapter for permitted effects, truthful uncertain recovery. A prediction never enters this plane by itself.

**Physical process topology** (minimal): one resident Windows Rust Host executable (Tauri shell integrated, headless entrypoint sharing crate), one isolated Node/Bun browser worker for signed-in Chrome, one paired PWA running remotely in phone browser. TierFlow does not have to run its own product server. Headless CLI is another front door into the same Host—not a concurrent authority.

## 2. Proposed exact top-level tree

\`\`\`text
ARBOR/
  .integration/proposal1/              # review-only, not packaged
  Cargo.toml                           # [workspace] members = ["crates/*","apps/desktop/src-tauri","apps/cli"]
  Cargo.lock                           # committed exact product lockfile
  rust-toolchain.toml                  # pinned stable
  package.json                         # bun workspaces apps/web, apps/browser-worker, packages/*
  bun.lock                             # committed exact product lockfile
  bunfig.toml                          # optional; only if reproducible need
  .gitignore                           # mandatory deny-list: secrets, app data, browser profiles, target, node_modules, dist
  .gitattributes
  AGENTS.md                            # repo guardrails and authority boundaries
  README.md                            # product launcher, architecture graph, safety truth
  apps/
    desktop/
      src-tauri/
        Cargo.toml                     # binary only; imports host crate
        build.rs
        tauri.conf.json                # fixed, audited supported patched Tauri v2
        capabilities/                  # exact webview windows/origins, no wildcard localhost ports
        src/main.rs                    # window/tray lifecycle, native command transport
        icons/
      web/                             # optional desktop-specific boot wrapper; shared UI from apps/web
    web/
      package.json
      index.html
      vite.config.ts                   # or justified existing equivalent; one bundler
      src/
        entry-desktop.ts
        entry-pwa.ts
        editor/                        # local lossless text, IME and accessibility
        screens/                       # home, threads, missions, World, setup, app builder
        composer/                      # semantics presentation / VIEW ALL; not an authority
        tierflow/                      # pinned adapter, renderer mounts
        transport/                     # desktop native vs paired HTTPS/WS
        stores/                        # view caches only; no authoritative World mirrors
        accessibility/
        service-worker/                # versioned offline PWA draft strategy, no provider sends
      public/manifest.webmanifest
    cli/
      Cargo.toml
      src/main.rs                      # doctor, inspect, export/backup, trace, migration diagnostics
    browser-worker/
      package.json
      src/
        process-entry.ts               # isolated worker
        chrome.ts                      # existing dedicated Chrome profile via CDP (not Playwright)
        sessions/
        providers/                     # ChatGPT, Claude, Gemini etc read/send adapters as VERIFIED
        capture/                       # provenance / streams / DOM evidence
        effects/                       # executes *admitted* signed-in-browser operations only
        contract/                      # generated bridge types, not new action catalog
  crates/
    contract/
      Cargo.toml
      src/{lib,identity,event,revision,source,evidence,privacy,context,mission,effect,error}.rs
    node0/
      Cargo.toml
      src/{lib,session,input,unicode,subscribe,fabric,converge,trace,projection,predictive_hints}.rs
    graph/
      Cargo.toml
      src/{lib,scope,graph,engine,scheduler,dag,changes,invalidate,retrieve}.rs
    intelligence/
      Cargo.toml
      src/{lib,language,interpret,entity,temporal,prediction,context,dcb,semantic_evidence}.rs
    world/
      Cargo.toml
      src/{lib,owner,storage,personal,claims,ingestion,lifecycle,revisions}.rs
    host/
      Cargo.toml
      src/{lib,bootstrap,service,ingress,config,supervisor,observability,health}.rs
    effects/
      Cargo.toml
      src/{lib,admit,grant,plan,claim,journal,receipt,recovery}.rs
    devices/
      Cargo.toml
      src/{lib,pairing,transport,session,reconcile,offline,capabilities}.rs
  packages/
    protocol/
      package.json
      src/generated/                  # generated from crates/contract only
      src/clients/{tauri,phone,worker}.ts
    ui/
      package.json
      src/{tokens,components,semantic,focus,accessibility}.ts
    browser-bridge/
      package.json
      src/generated/                  # exact runtime bridge schema; no plugin API by accident
  schemas/
    protocol/                         # versioned JSON/IDL shape and contracts
    storage/                          # versioned owner World migrations
    provider-evidence/                # source/capture evidence, not credentials
  assets/{icons,illustrations,themes}/
  tests/
    contract/
    node0/
    security/
    full-stack/
    migration/
    performance/
  tools/{dev,codegen,integration,release,migrations}/
  docs/{architecture,contracts,security,operations,user-journeys}/
  .github/workflows/{validate,windows-release,security,compatibility}.yml
\`\`\`

**File paths within crate \`src\` lists are candidate concrete file names**, not mandatory pre-created files. Prefer cohesive modules and fewer crates over a ceremonial directory explosion. A crate is justified only by a stable compilation, privilege, test or reuse boundary.

## 3. Eight Rust crates — precise domain ownership

| Crate | Owns | Explicitly forbidden |
| --- | --- | --- |
| \`contract\` | Stable identity types, schema versions, source class, actor/session, event IDs, source spans, revisions, epistemic status and typed effect statuses | IO, state stores, grants, worker execution |
| \`node0\` | Pure/ephemeral interaction state; bounded input event subscriptions, reactor manifest, partial interpretation trace, CAS acceptance of displayed candidates | Command catalog ownership, authorizations, browser actions, persistent memory writes |
| \`graph\` | One reusable Rust graph-node kernel, DAG compute engine, engine registry, invalidation and fair bounded scheduling | Owning personal or HQ source data, effect grants |
| \`intelligence\` | Pure command/semantic language, prediction ranking, source-safe knowledge/context/DCB assembly, temporal source reconciliation | Durable World writes, model-generated authority, hidden outbound provider calls |
| \`world\` | Owner-scoped source knowledge, preferences, provenance, user-private graph persistence, ingestion lifecycle and revocation | HQ development intelligence database; provider browser sends |
| \`host\` | Running assembly, authenticated native ingress, bounded lifecycle orchestration, health and supervisor, native command/service endpoints | Another semantic interpreter or effect journal |
| \`effects\` | Exact Host-owned effect request/admission/claims, grants and receipts, uncertain recovery | Independent effect writer outside Host; UI granting permissions |
| \`devices\` | PC↔PWA authenticated pairing/session transport, device grants and revisioned reconciliation | Phone-owned native effect authority, alternate mobile app |

### Note on effect ownership
The **logical Host authority** is a combination of the \`host\` assembly, \`world\` source owner and \`effects\` one writer. Splitting them into three crates is *not* three authorities. They are compiled into the same resident process, with a single owned state and single admission/write path. If it is easier to prove single-writer invariants by merging \`world\` and \`effects\` into one crate, that is allowed after review; do not make a ceremonial boundary that weakens security.

### Graph package
The graph implementation is based on **HQ's existing Rust \`research/rust-knowledge-graph\`** rather than rewriting from scratch. Source identity/ABI, licensing and dependency lineage must be reviewed. Choose one canonical shipped crate; HQ is a development consumer of the versioned artifact. Preserve Development and Personal namespace isolation even where algorithms are shared. This is a **release ownership proposal**, not a unilateral change to currently documented HQ ownership.

## 4. Allowed dependency directions

\`\`\`text
                 ┌────────────────────────┐
                 │ apps/desktop, apps/cli │
                 └────────────┬───────────┘
                              ▼
                           host ────────────► devices
                            │
              ┌─────────────┴─────────────┐
              ▼                           ▼
            node0                       effects
          /   |   \\                       │
   intelligence graph  contract         world
       /      |      \\                   /  \\
     graph   world    contract        graph contract
        \\       \\      /
          └────► contract ◄─────────────┘
\`\`\`

The diagram indicates conceptual dependencies, not a generated graph from Cargo. **Strict rule:** \`contract\` has no project-runtime dependencies; \`graph\` does not depend on \`host\`, \`world\`, \`node0\`, \`effects\` or browser worker; \`intelligence\` cannot invoke \`effects\`; browser-worker depends only on generated bridge protocol, not Host implementation details; UI depends on generated read/command contracts and presentation package, never native internal structs.

Potential cycles in the illustrative dependency diagram must be resolved at code review by extracting pure contracts/interfaces into \`contract\` and using dependency inversion (e.g. \`world\` implements an \`intelligence\` read trait, not \`intelligence\` importing the concrete \`world\` owner). No cyclic Rust crate dependency is permitted.

## 5. One user request: precise control and data paths

- **Desktop read:** UI native bridge → authenticated \`host::ingress\` → \`node0\` session snapshot → independent \`intelligence\` / \`graph\` pure subscribers → versioned projected evidence → generated \`packages/protocol\` client → TierFlow/UI view.
- **Paired phone read:** PWA DOM → mutually authenticated (device-scoped) PC HTTP/WS ingress → same Rust session system → read-only projection stream → local UI view. Device cannot claim current permission via cached browser data.
- **Committed request:** explicit submit (not Tab prediction) → durable Host intent record (with actor, selected IDs and version pins) → exact \`effects\` plan/read or approval path → existing effect owner; no implicit browser send.
- **Browser effect:** \`effects\` claims exact operation with recipient/thread/content digest → isolated Node browser worker with signed-in Chrome tab → observed evidence and exact uncertainty status → Host receipt → user-facing result.
- **Knowledge bootstrap:** source consent → Host-owned intake → immutable source evidence + lineages → extraction as candidates → user-private source acceptance/write → graph computational projection → DCB only with destination/source policy.
- **Developer HQ:** independently indexes the repository from Git/FS; can query reusable graph algorithms without requiring product data. HQ is not mounted into the product by default.
- **UI TierFlow:** renders versioned read-only semantic/visual documents; no independent send/approval/World write from rendering.

## 6. Why not one Rust crate / 30 crates / separate servers?

- One giant crate creates unsafe compile-time privilege sprawl and slows independent safety verification.
- Thirty microcrates increase dependency and versioning overhead without meaningful isolation.
- Eight stable crates follow actual domain boundaries, can be collapsed/expanded only with evidence, and ship as **one resident process**.
- A distributed broker/queue or standalone "Oracle server" is wrong for local-first low-latency I/O and increases process failures.
- Isolated browser automation is justified: unstable external pages and session state are not allowed to block the native event core.
- TierFlow and HQ remain independently evolvable source repos but are **not extra mandatory live services** for the user's installed application.

## 7. Packaging/release and local data layout

Release artifact:
- Windows MSI Tauri native shell + Rust Host + selected isolated browser worker runtime/resources + bundled signed-in Chrome profile launcher configuration (not Chrome profile data).
- One static PWA build served only through authenticated direct Host path for private features, with honest offline static shell.
- No APK required at first launch.
- One version manifest covering Rust binary, JS bridge worker, UI bundle, PWA revision, protocol version, graph schema, data migration and device pairing.

Local on-disk source owned data (not in Git):
\`\`\`text
%LOCALAPPDATA%/VIVIM/ARBOR/
  config/
  credentials/                  # OS-bound / restricted local owner access; no plaintext Git storage
  world/                        # authoritative personal and owner-approved data
  evidence/                     # source-backed lineage; retention controlled
  effects/                      # exact plan claims and durable receipts
  sessions/                     # opt-in recovery of committed input, not covert raw-draft logging
  indexes/                      # rebuildable search/materialized graph projections
  cache/                        # disposable source- and actor-scoped caches
  backups/                      # encrypted, tested recovery
  logs/                         # local redacted logs, limited retention
\`\`\`

This is a **conceptual owner data topology**, not a mandate for separate DB files or novel stores. Reuse existing Host data formats when migrating; choose actual SQLite/flat journal implementation based on durability proof. Data paths are never bundled in the repo or installer.

## 8. Build and developer experience

- Bun is the JS workspace/package manager. Use no second top-level npm/yarn/pnpm lockfile; external pinned dependencies can be built separately where justified.
- Cargo owns Rust workspace and pinned lockfile. Rust stable toolchain with verified MSVC Windows/Windows SDK.
- One schema/codegen operation produces TS bridge types and runtime validation.
- All branches CI: \`cargo fmt --check\`, \`cargo clippy\`, \`cargo test\`, Bun typecheck/test, contract diff, dependency/cycle check, secrets scan.
- Merge gate: Windows packaged smoke test, cross-language contract tests, provider worker lease safety, data export/import/rollback, clean install and PWA pair/degraded mode.
- Product build must not require HQ, Labs or TierFlow development servers alive. TierFlow is imported/pinned at build time; HQ development index remains external.
