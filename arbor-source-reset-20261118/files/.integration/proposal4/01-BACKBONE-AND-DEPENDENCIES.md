# Proposal 4 — Exact Capability-First Backbone and Dependency Laws

**Status:** proposed root tree. Not scaffolded, compiled, or migrated.  
**Architecture mode:** integrate-first / composition-first / one product language.

## Decision: structure by *system capabilities*, not frontend/backends

A subsystem's source identity, typed ports, rendered presentation, configuration, program contributions, proof and compatibility adapter belong **together in a capability module**. A module is a *development/package organization unit*, not necessarily its own process, dynamic plugin, Rust crate or database.

The top-level **kernel** consists of shared computational primitives only; it must not turn into a god object that contains all product behavior. The top-level **surfaces** are delivery mechanisms for the same system, not separate truth owners.

## Full proposed ARBOR tree

```text
ARBOR/
│
├── .integration/
│   ├── proposal1/                          # independent alternative: infra-first
│   └── proposal4/                          # this alternative: composition-first
│
├── Cargo.toml                              # one Rust workspace + pinned Cargo.lock
├── Cargo.lock
├── rust-toolchain.toml                     # pinned and Windows-verified
├── package.json                            # one Bun workspace
├── bun.lock
├── AGENTS.md
├── README.md
├── .gitignore                              # secret, runtime data, build exclusions
├── .github/workflows/                     # Rust, Bun, Windows, security, migration
│
├── kernel/                                 # native *mechanisms*, not product features
│   ├── model/                              # Rust crate: IDs, claims, types, refs, status, scope
│   │   └── src/{lib,identity,types,source,revision,evidence,policy_ref}.rs
│   ├── graph/                              # pinned HQKG source promoted/adapted, not rewritten
│   │   └── src/{lib,engine,typed_dag,scope,invalidation,incremental}.rs
│   ├── composition/                        # Rust crate: typed ports, program IR, verifier, planner
│   │   └── src/{lib,port,program,compile,resolve,validate,manifest}.rs
│   ├── runtime/                            # Rust crate: actor-bound input sessions + bounded work
│   │   └── src/{lib,ingress,session,subscription,schedule,projection,trace,health}.rs
│   └── authority-port/                     # Rust crate: HOST effect/source read interface
│       └── src/{lib,source_port,command_port,effect_port,receipt_port}.rs
│
├── modules/                                # first-class capability definitions
│   ├── interaction/                        # Node 0 Prime: text, voice, touch, selection
│   │   ├── module.yaml                     # typed capability/port/owner declarations
│   │   ├── schemas/                        # semantic trace, input and view projection
│   │   ├── bindings/                       # existing NLCL, Rust prototype and TierFlow bridge
│   │   ├── programs/                       # reusable input→interpret→confirm compositions
│   │   ├── views/                          # declarative composer/VIEW ALL bindings
│   │   └── tests/                          # latency, IME, multilingual, privacy, effects
│   ├── conversations/
│   ├── workspaces/
│   ├── knowledge/
│   ├── context/                            # DCBs, retrieval, destination disclosure
│   ├── missions/                            # goals, progress, recovery, state
│   ├── personalization/                    # owner-accepted preferences / prediction
│   ├── scheduling/                         # Chronos, time, reminders
│   ├── voice-to-tasks/
│   ├── onboarding/                          # ChatGPT-first provider/session bootstrap
│   ├── media/
│   ├── app-builder/
│   ├── automation/
│   ├── device-pairing/
│   ├── diagnostics/
│   ├── observability/
│   └── extensions/                         # verified extension *ports*, not arbitrary loader
│
├── surfaces/                                # physical input/output deployment wrappers
│   ├── desktop/
│   │   ├── src-tauri/                       # Tauri Windows shell + native Rust host bootstrap
│   │   └── bootstrap/                       # exact-process IPC/session authority
│   ├── web/
│   │   ├── src/
│   │   │   ├── shell/                       # one responsive app
│   │   │   ├── composer/                    # consumes modules/interaction projections
│   │   │   ├── worlds/
│   │   │   ├── missions/
│   │   │   ├── wiki/
│   │   │   ├── visual-runtime/              # TierFlow pinned renderer integration
│   │   │   ├── transports/                  # Tauri IPC / authenticated Host PWA
│   │   │   ├── local-editor/                # source text/caret/IME, no native wait
│   │   │   └── accessibility/
│   │   └── public/manifest.webmanifest
│   ├── cli/                                 # native commands from same operations/IDs
│   └── browser-worker/                      # isolated existing Studio/Chrome CDP adapter
│
├── bridges/                                 # transitional *source-backed* adapter owners
│   ├── host-control/                        # existing Host JS planner, grants, journal (ONE WRITER)
│   ├── studio-browser/                      # signed-in site lease/capture/send-once
│   ├── hq-development/                      # source-index/project/wiki read only
│   ├── tierflow/                            # versioned renderer/inert UI command contract
│   ├── personal-knowledge/                  # stage existing intake/World read ports
│   ├── media/                               # existing Host read/stream/cache
│   └── migration/                           # source schema translation/legacy ID aliases
│
├── protocol/
│   ├── schema/                              # one versioned IDL/JSON schema source
│   ├── generated/rust/                     # generated glue only, not a separate domain model
│   ├── generated/ts/                       # generated TS types and validators
│   └── compatibility/                      # version negotiation, fixture corpus
│
├── configuration/
│   ├── profiles/                            # owner-selected capability and view profiles
│   ├── defaults/                            # safe, low-resource default configuration
│   ├── resource-budgets/                   # bounded CPU/RAM/storage/stream limits
│   ├── privacy/                             # references to Host-enforced scope, not new grants
│   └── schema/                              # config migrations and version contracts
│
├── programs/
│   ├── builtins/                            # composable source-owned program recipes
│   ├── templates/                           # examples/app-builder starter recipes
│   ├── tests/                               # programs compile, view, replay, no implicit effect
│   └── README.md
│
├── fixtures/
│   ├── synthetic/                           # strictly fake users/projects/provider state
│   ├── differential/                        # existing Host/TierFlow/HQ output comparisons
│   └── privacy-redacted/
│
├── tests/
│   ├── contracts/
│   ├── graph/
│   ├── composition/
│   ├── authority/
│   ├── install-upgrade/
│   ├── desktop-pwa/
│   ├── failure-recovery/
│   ├── low-storage/
│   └── security/
│
├── tools/
│   ├── intake/                              # source inventory, checksums, licensing, lineage
│   ├── codegen/                             # source schemas → TS/Rust
│   ├── doctor/                              # inspect runtime port health/provenance
│   ├── migration/                           # snapshots, shadow-read compare, rollback
│   ├── performance/
│   └── release/
│
├── docs/
│   ├── architecture/                        # accepted AFTER owner decision
│   ├── reference/                           # generated from verified module/source truth
│   ├── security/
│   ├── migration/
│   └── operations/
│
└── .github/
    ├── CODEOWNERS
    └── workflows/
```

The pseudo paths `src/{...}.rs` represent *planned* modules, not actual current files. Do not mass-generate them merely to make the repository look mature. During first integration, some capability module directories may contain only a tested descriptor and references to existing implementations.

## One module's internal anatomy

Example: `modules/context/`

```text
modules/context/
├── module.yaml                          # stable capabilities, version, source owner
├── schemas/
│   ├── context-request.schema.json
│   └── context-packet.schema.json
├── bindings/
│   ├── host-dcb.adapter.ts              # existing Host authority/data ports
│   └── hq-context.adapter.ts            # scoped HQ source/algorithm contribution
├── programs/
│   ├── assemble-context.program.yaml    # typed pure transformation DAG
│   └── preview-disclosure.program.yaml  # never effects by itself
├── views/
│   └── context-preview.view.yaml        # UI intent, no CSS/runtime authority
├── configuration/
│   └── budget.profile.yaml
├── tests/
│   ├── contract.test.ts
│   ├── no-unapproved-disclosure.test.ts
│   └── source-revision.test.ts
└── README.md
```

**Physical implementations remain source-owned.** A module descriptor can bind a Rust engine in `kernel/graph`, an existing JS Host read port in `bridges/host-control`, and a TierFlow view from `surfaces/web`. The descriptor does *not* create duplicate engine instances. Modules with stable native features may later own a `native/` crate if justified, rather than making every manifest a Rust crate.

## Five native crates — deliberately fewer than Proposal 1's eight

| Kernel crate | Stable contract | Explicit non-ownership |
| --- | --- | --- |
| `model` | IDs, provenance, source revisions, authority references, capability and result states, text span types | Database, application features or policy decisions |
| `graph` | Existing HQKG-derived graph-node computation, typed DAG, engine leases, selective invalidation, bounded incremental cache | Host personal source writer, new graph DB |
| `composition` | Declarative program validation and type checking, port linking, version negotiation, effect-typed boundaries, program-plan explanation | Permission granting, browser sends or arbitrary shell |
| `runtime` | Authenticated event admission, session and projection lifecycle, bounded subscriptions and scheduling, partial result convergence | Physical UI rendering, new event database, semantic "master AI router" |
| `authority-port` | One typed interface to current Host authoritative World/approval/effect journal with compatibility tests | Competing journal or any additional effect executor |

**Long-term:** collapse the compatibility port into the native Host owner only after the source-backed JS journal has been migrated with one-writer proof. Never keep a second permission authority alive to "future-proof" architecture.

## Directional dependency graph

```text
          signed-in user/UI/PWA/CLI, TierFlow renderer
                          │
                          ▼
                  runtime ingress
                       │
              versioned projection
                       │
          ┌────────────┴────────────┐
          ▼                         ▼
    composition engine         module descriptors
          │                         │
          ├──────> graph engine     └──> bound existing source ports
          │             │
          └───────────> model <─────────── every type/schema
                        ▲
                        │
                authority-port
                        │ (effect/World only; never fabricated)
                        ▼
              canonical Host writer
```

Mandatory dependency properties:
- `model` depends on no runtime/component package.
- `graph` depends on model only through a thin adapter if required; HQ Rust kernel functionality is reused.
- `composition` depends on type-only/graph read interfaces; `graph` must not depend on `composition`.
- `runtime` owns scheduling/invocation of composed nodes, never computes semantically by a global winner.
- `authority-port` is a **trait/contract** used by the Host assembly; pure modules must not import concrete Host implementation.
- All module descriptors and view bindings validate against the same canonical protocol.
- No cyclic crates, no public plugin handler with ambient OS or provider credentials.

## Why this is *one system* despite multiple languages and processes

The system boundary is the **typed identity, source-owner and transition model**, not the operating-system process boundary. One program graph can contain native Rust transformations, trusted existing JS read adapters and a TierFlow view projection without creating separate definitions of a "task", "command" or "done". Trust and persistence stay with the actor actually authorized to write.

The browser editor owns its unpublished draft. The PC Host owns committed user intent, owner World, device grants and effect receipts. HQ owns its developer index. The **same reference model** connects them; **one truthful state projection** exposes the join. It would be dangerous to erase these authorities in the name of "no frontend/backend distinction."

## Immediate scaffold only; not a rewrite

The first actual ARBOR code should contain just:
1. one valid Cargo workspace with the existing tested Rust graph crate pinned by source and minimal `model/composition/runtime` interfaces;
2. one Bun workspace with a working reused UI and bridge adapters;
3. source-generated interop protocol/validator;
4. a handful of **real source-backed module descriptors** for interaction, status, one DCB read and one visual projection;
5. a runnable read-only end-to-end demonstration;
6. compatibility and security negative tests.

All other directories appear as architecture inventory with owners and blockers until their first real source integration. **The scaffold is a map of the product, not a manufactured implementation.**
