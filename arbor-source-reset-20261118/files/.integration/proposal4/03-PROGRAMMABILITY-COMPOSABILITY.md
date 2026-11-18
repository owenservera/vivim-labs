# Proposal 4 — Programmability, Composability, Extensibility and Configurability

**Status:** candidate architecture integrating inspected existing hooks and unratified Labs proposals. Not an installed plugin system.

## 1. One programmable substrate

Programming is not a separate advanced feature sitting beside ChatGPT-style chat. **Every important VIVIM operation is already a potential program node**: reading a project, gathering source evidence, creating a DCB, computing dependencies, previewing a change, rendering a view, submitting an approval, or checking an outcome.

A program is a typed and inspectable composition of existing capabilities, not an arbitrary string of LLM prompts or a workflow engine with its own privileges. It can be authored through:
- Natural language: compile candidate semantics into typed program nodes and request clarification before commitment.
- Exact command / CLI: reference canonical operation IDs and typed arguments.
- Visual app builder: connect source-backed ports and views with type/permission feedback.
- Wiki / screen context: select and parameterize already available program/view recipes.
- Declarative config file: versioned recipe for expert reuse, review and distribution.

All paths produce the **same IR and verification diagnostics**. The interfaces can vary; the operation identity and transition semantics cannot.

## 2. Primitive kinds (intentionally small)

| Kind | Input/output contract | Allowed behavior |
| --- | --- | --- |
| `Source` | Scoped resource/reference → source-pinned observation | Read only, actor/purpose/source authorization required |
| `Transform` | Typed values → typed values | Pure, deterministic where possible, bounded CPU/memory |
| `Query` | Typed predicate + source pins → result set | Source-bounded reads; provenance and staleness retained |
| `Branch` | Typed condition → chosen dependent edges | Explicit condition semantics; no hidden effect |
| `Join` | Multiple typed outputs → composite with version vector | Reject incompatible source claims or mark conflict |
| `Temporal` | Clock/source event → bounded scheduled *proposal* | Clock does not obtain permission to execute an effect |
| `Predict` | Explicitly accepted patterns + scope → ranked candidates | Inert hints and authorized pure prewarm only |
| `View` | Typed state + audience + profile → presentation model | No source-owned writes; no inferred effect success |
| `ProposeEffect` | Exact current target and arguments → Host candidate | Not an effect; caller cannot mint grants |
| `Verify` | Source observation + expected postcondition → verification evidence | Must distinguish observed, unknown and verified |
| `Checkpoint` | Program version + consented state → durable owner-scoped record | Only through authorized Host persistence |

In the initial migration, the exact program DSL is a proposal. Existing HQKG `typed_dag.rs` has source/namespace/revision/budgeted pure transformations, and existing Host actions/journal already have effect claims; **reuse those two capabilities at their boundary** rather than rewriting them as one unsafe generic `execute`.

Loops, arbitrary user scripts and remote code execution are **not required** for the first integration. A versioned DAG with explicit temporal reentry can express meaningful programs while remaining auditable and bounded. General program branching/looping requires additional termination, resource and policy contracts.

## 3. Example capability declaration

This is *illustrative schema*, **not** a current supported loader or compiled manifest:

```yaml
schema: arbor.capability.proposal/1
id: project.read-summary
version: 1.0.0
owner: host.world
kind: pure-query
ports:
  input:
    project: resource-ref(project)
    sections: list(enum(issues,decisions,next-steps))
  output:
    summary: source-evidence-set(project-summary)
bindings:
  read:
    adapter: existing-host-project-read-port
    observed_status: needs_runtime_verification
placement:
  allowed: [native-host]
privacy:
  required_source_classes: [owner-project]
effect:
  allowed: false
quotas:
  cpu_ms: 50
  result_bytes: 262144
projection:
  default_view: project-summary-view
tests:
  - contract-source-revision
  - wrong-principal-denial
  - revoked-project-denial
```

The port ID `project.read-summary` above is a **proposed new capability**, not a claim it exists in the Host catalog today. The current catalog v6 was source-observed with 19 entries and no canonical full-project-update action. Before exposing it, approve a source-owner read adapter and current evidence.

## 4. Example composition: "full update on this project — next steps"

```yaml
schema: arbor.program.proposal/1
id: example.full-project-update
version: 1
scope: current-workspace
inputs:
  project: resource-ref(project)
  focus: enum(next-steps,issues,decisions)
nodes:
  - id: active
    kind: Source
    capability: project.resolve-current
    with: { project: "$inputs.project" }
  - id: report
    kind: Query
    capability: project.read-summary
    with: { project: "$active.project", sections: ["$inputs.focus"] }
    after: [active]
  - id: output
    kind: View
    capability: view.project-update
    with: { data: "$report.summary" }
    after: [report]
outputs:
  primary: "$output"
effects: none
```

This is a **user-program candidate**, not current executable ARBOR configuration. The value is the architectural principle: the same program appears in chat, wiki, CLI, mission card and app builder. The visual interface does not have a separately implemented "project update" action.

If no verified `project.read-summary` port exists, compilation returns `UNAVAILABLE`, not an invented summary. For familiar users, a prediction reactor may fill the project/focus slots with explicit two-Tab selections; it never sends or writes on Tab.

## 5. Reuse verified extension seams rather than build a marketplace

Source-backed hook families (from VIVIM Labs `vivim/extension-points/VERIFIED-HOOKS.md`):
- **HQKG Rust `EngineRegistry`**: typed `ComputeEngine`, generation CAS and pinned leases; trusted in-process computation only, not an untrusted sandbox.
- **Studio `src/plugins/registry.mjs`**: existing fixed plugin kinds and registration API; not a general package installer.
- **Host action catalog/dispatcher**: stable action IDs, risk, owner, fixed execution and receipt guards; no general unsafe dynamic action evaluator.
- **TierFlow typed command IR / phone projection**: inert typed candidates and guard checks; not a Host grant.
- **HQ knowledge/context/wiki programming**: source identity, versioned wiki configuration and deterministic context read procedures; not a product-wide privilege owner.

**One logical extension contract** should describe an implementation's existing contribution and how it may be used. Do **not** add a new parallel "plugin manager service" to achieve that.

The extension contribution object describes `id,version,port,schema,implementation,sourceRef,placement,compatibility,requiredDataClass,requestedScopes,resourceLimits,health,conformanceProof`. It references existing owners for policy. A manifest is a *claim about capability*, not permission.

### Trust tiers

| Tier | Contribution | Execution location / admission |
| --- | --- | --- |
| 0 | Declarative views, themes, templates, schemas, localization | Data validated by existing Host/TierFlow contracts, no runtime code privilege |
| 1 | Trusted first-party pure transforms | Pinned Rust graph engine or reviewed deterministic JS adapter with bounded resources |
| 2 | Third-party or user-authored pure transforms | Only after a real sandboxed WASM or restricted worker design is proven; default unavailable |
| 3 | Actions, OS tools, provider sends, memory writes, device operations | **Never plugin-owned authority**; typed requests admitted and journaled by Host |

Not every tier has to ship in the first integration. Tier 0 and source-backed Tier 1 are enough to establish future extensibility without building a risky generic loader.

### Extension family

A "project management" extension family may contribute a view, a typed parser, a source reader, context recipe, test pack and action proposals. Each contribution has separate source access and version pins; the family is a convenient composition, not a permission bundle. Revoking one capability invalidates its dependent programs and projections.

## 6. Configuration as owned, reversible data

| Configuration scope | Examples | Source of authority |
| --- | --- | --- |
| Person | shorthand, command suggestions, privacy, UI density, default context budget | Host owner configuration, never inferred silent approval |
| Workspace | active project, participating sources, enabled program recipes, visual layout | World/workspace accepted version |
| Capability | binding to a verified engine generation, resource quotas, result projection | Signed build + Host-approved install/activation |
| Device | offline cache quota, display density, paired transport, background rules | Native/device grants; no permission cloning |
| Program | typed nodes, bindings, parameters, version and retention | Owner-accepted program configuration, checked by compiler |
| System | low-resource mode, storage bound, diagnostic level, provider profile availability | Local Host configuration and versioned release defaults |

Config lifecycle:
`proposed → validated → previewed → approved(if needed) → activated(revision CAS) → observed → rolled back/superseded`.

**Configuration must never change Host effect permissions simply by changing a YAML file, view mode or program parameter.** Any new requested scope is separately admitted and owner-reviewed.

Two presentation modes share semantic state:
- **Simple:** "I want concise updates", "make this my default", visually configure cards.
- **Advanced:** inspect exact program, source nodes, privacy disclosures, typed ports and runtime budgets.

Both compile to the same configuration change. Simple users don't see manifest structures unless they want to.

## 7. Capabilities are self-describing and inspectable

If a capability exists, VIVIM should be able to answer:
- What is it, what does it do, and where does it run?
- What source data does it read, write or disclose?
- Which other operations depend on it?
- Which versions and data schemas is it compatible with?
- Is it `declared`, `source-wired`, `tested`, `installed`, `available-now`, `authorized`, `failed` or `revoked`?
- How is it configured? Can I preview, disable or replace its implementation safely?
- What exact evidence and tests justify its current status?

This information powers:
- CLI help and program introspection.
- A self-generated capabilities Wiki.
- Visual app-builder port libraries and validation feedback.
- Context-aware hints and the Node 0 composer.
- Developer HQ source/implementation dashboards.
- Human-readable "why didn't VIVIM do that?" explanations.

**The catalog is a materialized view over source-owned descriptors and verified implementation records**—not a second manually edited action truth. The Host owns actual executable actions; HQ's semantic source IDs are evidence and crosswalk inputs; Lab proposals remain marked candidates.

## 8. A view is a program node

TierFlow's visual language is not forced to become a Rust UI. Instead:
- Rust emits typed view data, source spans, status, selectable capability references and current revisions.
- TierFlow builds presentation/interaction geometry, glyphs, motion and editor focus behavior.
- The same `View` node can be mounted in desktop, phone, AutoWiki or an app built by the user.
- A view may emit `AcceptCandidate`, `EditConfig` or `SubmitIntent` input events; it **cannot** mark a Host effect verified.
- View configuration can be edited and versioned, but actual World data stays with its source owner.

This is the architectural interpretation of "no frontend/backend distinction": **one operation and data model, with platform-specific execution and rendering mechanics**.

## 9. Predictive and autonomous composition

Prediction should be allowed to optimize **which pure computations to prewarm** as well as which slots to suggest, subject to CPU, source-permission and confidentiality budgets. It must not turn a likelihood into an executable effect.

An agent may propose a program graph or revisions to it. The compiler validates types, source availability, dependency cycles, resource usage and effect declarations. An authorized user then inspects/commits the plan; Host authorizes any effects. The agent never edits Host grants, hides effects, or silently installs its own runtime.

"Unlimited AI coding budget" should be used for **contract generation, port adapters, conformance suites, and safe retirement of duplicate implementations**, not for inventing a new service every time a program needs one more node.

## 10. Design acceptance

1. Same capability ID can be invoked from text, CLI, a visual selection and an app-builder program with the same risk/source semantics.
2. An added view of an existing source requires no independent "backend" CRUD route or new data writer.
3. A new pure first-party transform can replace an implementation generation while preserving its stable interface and existing in-flight version pins.
4. An untrusted contribution cannot inspect hidden private source data or call a Host effect.
5. A user edits a program; VIVIM shows a deterministic capability/permission/dataflow diff before activation.
6. If a source is revoked, dependent programs and views become unavailable or stale without losing unrelated work.
7. A stored program is not runnable with higher grants merely because a later version asks for broader scope.
8. Existing signed-in Chrome provider worker remains the only browser send implementation until any tested replacement is approved.
9. The same project-update program produces a chat answer, AutoWiki section and mission card without divergent definitions.
10. A missing/ambiguous provider, project or capability produces a verifiable clarification—not a silently invented program node.
