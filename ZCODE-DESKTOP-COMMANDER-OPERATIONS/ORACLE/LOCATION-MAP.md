# Oracle location map: lean retrieval and ownership
**Checked:** 2026-11-21 against connected GitHub main. Pointers are source paths, not claims of live sessions or runtimes.

## Start here — minimum context, in this order
1. [AGENTS.md](AGENTS.md): Oracle role and what **not** to do.
2. [SITREP.md](SITREP.md): dated/source-pinned truth and first unproven edge.
3. [STEERING.md](STEERING.md): North Star and autonomy ladder.
4. [META-TOOLING.md](META-TOOLING.md): capability matrix, evidence gaps and tool choices.
5. Open only the source/evidence file for the current bottleneck. Read [RECONCILIATION.md](RECONCILIATION.md) for audits and [TO-PLANNING.md](TO-PLANNING.md) when issuing a shaping signal.

## Labs: coordination, authority and handoffs
| Location | Purpose / authority |
| --- | --- |
| ../CLOUD-FIRST-CODING-POLICY.md | ChatGPT cloud/GitHub source venue; local/ZCode not primary coding |
| ../VALIDATION-SURFACES.md | Rust -> GPUI -> PWA validation; **no Android prerequisite** |
| ../TASK-CLAIMS.md | Shared ownership/check-in overlay; not a task registry; inspect current row before any implementation claim |
| ../DEVOPS-PLANNING/AGENTS.md, SITREP.md, BUNDLE-INDEX.md | Planning's prep authority, currently prepared bundles, no implied recipient acceptance |
| ../CHATGPT-DEV/AGENTS.md, SITREP.md, DELIVERY-CONTRACT.md | Dev's cloud code ownership, truthful test/release receipts |
| ../CHATGPT-DEV/RELEASE-AND-INTEGRATION-PLAYBOOK.md | Release/recovery pitfalls and evidence requirements |
| ../DESIGN/AUTOWIKI-FOUNDATION-R001/ | Rust-first staged AutoWiki components: **PREPARED**, not automatically integrated |
| ../DEVOPS-PLANNING/HANDOFF-GPUI-AUTOWIKI-20261121.md | Paused Labs-only GPUI/AutoWiki preparation; do not take over active ARBOR source |

## ARBOR: product reality and current proof
Repository: https://github.com/owenservera/ARBOR — fetch its current main SHA whenever reviewing an implementation claim.

| Location in ARBOR | Existing purpose / needed test |
| --- | --- |
| docs/FIRST-CUSTOMER-TRACKER.md | First-contact priorities and honest evidence matrix |
| docs/integration/OW-20261121-LIVE-FABRIC-STATUS.md | Rust/Host composer integration reports and known UNRUN evidence |
| docs/integration/OW-20261121-PWA-LIVE-AGENT-VISIBILITY-DESIGN.md | Progressive typing and authorized tap-dot requirements; not full implementation proof |
| core/programmability/src/live_system.rs | Rust version-fenced live system observation; **not** new backlog or permissions engine |
| core/programmability/src/wiki_read.rs | Rust Wiki read/search/source stamp |
| core/programmability/src/agent_composer.rs | Inert authorized draft candidate, **not** autonomous development dispatch |
| apps/vivim-host/control/agent-composer-bridge.mjs | Host Control-mediated composer transport |
| apps/mobile-relay/first-contact-composer.ts and agent-composer-adapter.ts | Real PWA field events/agent delivery; confirm current implementation before assigning code |
| apps/mobile-relay/build-composer.mjs and build-canvas.mjs | UI design preview/activation (different from compiling new application code) |
| native/gpui-desktop/Cargo.toml and src/ | Native desktop surface; actual Wiki live parity must be tested |
| .github/workflows/pwa-main.yml | PWA source/contract CI definition; run results require separate observation |
| .github/workflows/gpui-native.yml | Rust/GPUI CI definition; real native UI is a distinct proof |
| apps/vivim-host/ and docs/FIRST-CONTACT-LIVE-OBSERVABILITY.md | Host launch and bounded first-contact diagnostic evidence; deployment is not automatic |

## Deliberately unresolved / do not synthesize
- Labs ZCODE-DESKTOP-COMMANDER-OPERATIONS/OWNER-WORK/ was **404 on connected GitHub main**. Its historical local contents, authoritative OW order and statuses cannot be reconstructed from agent prose.
- The historical local ARBOR-LIVE-SURFACE-DOCUMENT/WIRING-LEDGER.md is not equivalent to a currently accessible remote work authority. Verify the remote before using it.
- ChatGPT role files are *instructions*, not persistent running workers. Shared GitHub files are *artifacts*, not a push notification or automatic cross-session state transfer.
- The current Host PID, stable public/Tailnet URL revision, screenshot stream, physical phone display and per-task CI run are **not verified** by this static map.

## Retrieval discipline
For a steering turn, read Oracle AGENTS/SITREP + TASK-CLAIMS; check both repo HEADs; inspect only the relevant product seam and evidence; identify **one** first unproven edge; update TO-PLANNING only if signal meaningfully changes. Do not reproduce Planning's task list here, do not invent a replacement owner registry, and do not use Desktop Commander to code.

## Next-phase Wiki ↔ command strategy
- [WIKI-COMMAND-SYMBIOSIS-R001.md](WIKI-COMMAND-SYMBIOSIS-R001.md): source-grounded five option comparison, explicit Explore/Act mode model, PWA acceptance sequence and Planning crosswalk. Read this only when discussing next-phase reprogrammability; it is **not** an implementation ledger.

## Native Oracle convergence (26 Nov)
- [ORACLE-RESIDENT-PRODUCT-MIGRATION-R001.md](ORACLE-RESIDENT-PRODUCT-MIGRATION-R001.md) — cross-repo SITREP, Oracle as graph-backed Product actor, O0–O5 gates and P01–P06 migration decision map. Read for the new native-agent objective; not another task ledger.
- Product [foundation](https://github.com/owenservera/vivim-product/blob/main/docs/FOUNDATION-R001.md), [backbone](https://github.com/owenservera/vivim-product/blob/main/docs/META-VLANG-DURABLE-COMPUTATIONAL-BACKBONE-R001.md), [typed primitives](https://github.com/owenservera/vivim-product/blob/main/crates/protocol-sdk/src/semantic_primitives.rs), [task planner](https://github.com/owenservera/vivim-product/blob/main/crates/protocol-sdk/src/task_planner.rs), [resident evaluator](https://github.com/owenservera/vivim-product/blob/main/crates/protocol-sdk/src/meta_resident.rs).
- Labs [P01–P06 preparation launchpad](https://github.com/owenservera/vivim-labs/blob/main/VIVIM%20V1.0/PRODUCT-MIGRATION-PREPARATION/README.md) — each lane retains independent source/test/authorization ownership.
