# Directly inspected existing hooks and blockers

2026-11-07 selective direct inspection, remote default-branch sources. **Inspected** below means contents retrieved and reviewed; **NOT** native-device executed or full callgraph verified.

## Source-confirmed hooks

| Surface | Source evidence | Actually implemented | What is missing for standalone user plugins |
|---|---|---|---|
| Studio plugin registry | [registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) | `createRegistry` with fixed kinds: `formats`, `codecs`, `secretPatterns`, `migrations`, `transforms`; `register/get/has/list/unregister/snapshot` | No package installer, signer, permission admission or Android delivery shown; caller exposure requires further trace |
| HQKG Rust engine registry | [engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) | `ComputeEngine` trait, `EngineDescriptor`, `EngineRegistry::install/pin`, generation CAS and leases, operation whitelist | Trusted in-process only, no untrusted code sandbox; live deployment/integration not proven |
| HQKG algorithm driver | [algorithm_adapter.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/algorithm_adapter.rs) | Custom `AlgorithmDriver` implements operations/execute via `AlgorithmCompute` and existing engine registry | Research algorithm definitions remain inert; compile/register actual code; no third-party isolation |
| Mobile command descriptors | [consumer registry](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) | Declarative command entries with consumer, scope, input/output, mode and handler reference | Not a secure install or mobile native execution interface; handler names aren't grants |
| Host typed actions | [action catalog](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/action-catalog.v1.json), [dispatcher](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) | Versioned catalog and fixed action execution, scoped owner approval, journal/receipt fences (crosschecked with existing Harness atlas) | General dynamic action executor admission and plugin sandbox not shown; no R3 authorization |
| TierFlow phone projection | [wave4-phone-adapter.ts](https://github.com/owenservera/tierflow/blob/main/src/visual-runtime/device/wave4-phone-adapter.ts) | Inert projection from one canonical UI document/render plan, paired identity and readiness checks | Not publishing, installing, approving or executing; no second APK |
| TierFlow typed transport | [wave4-typed-transport-adapter.ts](https://github.com/owenservera/tierflow/blob/main/src/visual-runtime/command/wave4-typed-transport-adapter.ts) | Typed candidate envelope, scope/version checks and bounded deduping | No Host ingress, signature, approval, action registry or effect authority |
| Repos HQ context | [hq/lib/context.mjs](https://github.com/owenservera/repos-hq/blob/main/hq/lib/context.mjs) | Deterministic budgeted task-scoped context bundles with provenance and verification | No demonstrated external policy/strategy registration interface |
| ZCode team plugin | [plugin.json](https://github.com/owenservera/repos-hq/blob/main/plugins/vivim-team/.zcode-plugin/plugin.json) | ZCode plugin descriptor exposing skills, commands, hooks | Developer tooling plugin, not proof of consumer VIVIM install compatibility |

## Existing design work that MUST be reused

- [Harness full plugin research](https://github.com/owenservera/vivim-web-harness/tree/main/docs/plugins) (research, not installed user plugin loader).
- [Harness source-map.v1.json](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/source-map.v1.json) includes historical Omega/BCP/VIVIM Final plugin leads.
- [TierFlow Waves 4–6](https://github.com/owenservera/tierflow/tree/main/VIVIM-VISUAL-RUNTIME) define one UI, device adapters, authoring, migration and cutover boundaries.
- [Repos HQ core/engine/adapter matrix](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) distinguishes replaceable compute from durable core state.
- [HQKG Rust README](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) states the unsandboxed trusted-driver, journal and scheduling limitations.

## Audit limitations

This is a **selective live source check + existing source atlas synthesis**, not a repository-wide complete symbol/caller analysis. Some inventory anchors are design docs, not source. The current work did not run Bun/Cargo, install Windows/Android, enumerate all repository trees or prove device/plugin functionality. Treat uninspected exact entrypoints as **candidate** and execute second-pass file/symbol/caller verification before changing production API.
