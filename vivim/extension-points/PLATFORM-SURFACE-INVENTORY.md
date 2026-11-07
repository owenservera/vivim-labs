# VIVIM complete candidate plugin-surface inventory (first source-linked pass)

**Date:** 2026-11-07. **Scope:** VIVIM codebase, mobile, desktop, Host, Studio/browser, HQ/Rust, TierFlow, cross-cutting platform services. This is an *inventory of potential standalone plugins*, not a claim that every hook exists or a completed symbol-by-symbol whole-repository audit.

## Evidence vocabulary

- **I — implemented registry/interface:** a directly inspected general-purpose registration or engine slot supports the named kind (but is not necessarily an installable user plugin).
- **S — source-backed seam:** an inspected codepath exists and could be adapted; not currently proven as a general user plugin hook.
- **D — designed/documented:** a documented design describes a related interface, but code-level runtime extensibility is not yet verified.
- **N — needed new plugin port:** capability could stand alone, but no safe existing user-facing port was confirmed. The linked file is a starting inspection anchor, not proof of the exact capability.

All entries in this first pass use S/D/N conservatively except specific separately confirmed interfaces in the [verified hooks](VERIFIED-HOOKS.md). Other source paths are *anchor candidates* (one or more per family), not line-level proofs. Each row requires follow-up: inspect exact symbol/callers; test install/uninstall, scope, data owner and effect boundaries.

| ID | Standalone plugin candidate | Surface | Maturity | Starting evidence |
|---|---|---|---|---|
| VP-001 | Android share-sheet ingress | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-002 | Android file picker and storage access | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-003 | Android camera/scanner capture | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-004 | Android notifications | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-005 | Android alarms and reminders | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-006 | Android contacts/calendar bridge | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-007 | Android voice capture and speech output | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-008 | Android accessibility/assistive interaction | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-009 | Android deep-link and app intents | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-010 | Android biometric/unlock gate | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-011 | Android offline storage and drafts | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-012 | Android connectivity/transport adapter | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-013 | Android permission broker | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-014 | Android background jobs and battery policy | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-015 | Android device health and diagnostics | Native mobile and APK | S | [H:MainActivity.java](https://github.com/owenservera/vivim-web-harness/blob/main/studio/android/src/inc/vivim/app/MainActivity.java) |
| VP-016 | Mobile command/consumer category | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-017 | Mobile NLCL vocabulary | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-018 | Mobile text command palette | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-019 | Mobile touch/gesture bindings | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-020 | Mobile declarative UI components | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-021 | Mobile screen templates | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-022 | Mobile theme/iconography | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-023 | Mobile context help | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-024 | Mobile home tab and navigation | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-025 | Mobile offline read projection | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-026 | Mobile relay event subscriptions | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-027 | Mobile paired-device capability discoverability | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-028 | Mobile per-plugin settings | Mobile UI and relay | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/mobile-relay/consumer-commands/registry.mjs) |
| VP-029 | Desktop file chooser/import/export | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-030 | Desktop clipboard and share | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-031 | Desktop OS notification | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-032 | Desktop shortcuts/hotkeys | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-033 | Desktop tray and menu integrations | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-034 | Desktop deep links | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-035 | Desktop voice input/output | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-036 | Desktop native device capabilities | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-037 | Desktop window/pane layouts | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-038 | Desktop hardware integrations | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-039 | Desktop startup/background service | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-040 | Desktop installer/version channels | Windows desktop and Host shell | S | [H:main.rs](https://github.com/owenservera/vivim-web-harness/blob/main/src-tauri/src/main.rs) |
| VP-041 | Host typed action descriptors | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-042 | Host executor adapter binding | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-043 | Host intent-planner lexicon | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-044 | Host action permission policies | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-045 | Host action preflight/readiness | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-046 | Host receipts and observability adapters | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-047 | Host reversible local configuration | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-048 | Host World object schemas | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-049 | Host World projection/query | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-050 | Host World import/export | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-051 | Host background triggers | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-052 | Host authentication/device-scope projections | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-053 | Host restore/recovery workflows | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-054 | Host extension lifecycle coordinator | Host control and World | S | [H:dispatcher.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/apps/vivim-host/control/dispatcher.mjs) |
| VP-055 | Studio format readers | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-056 | Studio codecs | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-057 | Studio secret-pattern detectors | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-058 | Studio migration handlers | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-059 | Studio data transforms | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-060 | Studio capture adapters | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-061 | Studio provider site profiles | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-062 | Studio DOM/structure extractors | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-063 | Studio recipe definitions | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-064 | Studio provider thread selectors | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-065 | Studio conversation projections | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-066 | Studio attachment formats | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-067 | Studio output parsers | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-068 | Studio redaction filters | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-069 | Studio browser event observations | Studio, browser and providers | S | [H:registry.mjs](https://github.com/owenservera/vivim-web-harness/blob/main/studio/src/plugins/registry.mjs) |
| VP-070 | Knowledge ingestion connectors | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-071 | Knowledge document types | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-072 | Knowledge extraction strategies | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-073 | Knowledge entity resolvers | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-074 | Knowledge ontology/schema packs | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-075 | Knowledge graph edge predicates | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-076 | Knowledge provenance validators | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-077 | Knowledge retrieval engines | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-078 | Knowledge ranking/rerankers | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-079 | Knowledge temporal reconciliation | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-080 | Knowledge citation processors | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-081 | Context-bundle policies | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-082 | Context compression/selection | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-083 | Personal memory policies | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-084 | Memory consolidation engines | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-085 | Embedding/index backends | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-086 | Database read-only adapters | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-087 | Knowledge security/redaction projections | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-088 | Knowledge export/import adapters | Knowledge, graph and memory | D | [Q:README.md](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/README.md) |
| VP-089 | Rust ComputeEngine slots | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-090 | Rust AlgorithmDriver implementations | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-091 | Rust database engine adapters | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-092 | Rust algorithm pipeline compositions | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-093 | Rust graph traversal algorithms | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-094 | Rust graph diagnostics | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-095 | Rust journal snapshot/compaction strategy | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-096 | Rust ingestion sidecars | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-097 | Rust cache/index strategies | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-098 | Rust resource budget policies | Rust algorithms and engine substrate | S | [Q:engine.rs](https://github.com/owenservera/repos-hq/blob/main/research/rust-knowledge-graph/src/engine.rs) |
| VP-099 | Semantic vocabulary packs | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-100 | Semantic parser strategies | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-101 | Capability descriptors and discovery | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-102 | Command validators/argument codecs | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-103 | Oracle domain specializations | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-104 | Oracle tool/capability brokers | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-105 | Agent persona/skill packs | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-106 | Agent coordination patterns | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-107 | Multi-agent routing strategies | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-108 | Model/provider selection strategies | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-109 | Workflow/plan templates | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-110 | Workflow triggers/schedules | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-111 | Workflow verification checkers | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-112 | Workflow compensations and recovery | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-113 | Context selection by task | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-114 | Self-healing diagnosis/repair proposals | Semantics, Oracle and orchestration | S | [Q:CORE-ENGINE-ADAPTER-MATRIX.md](https://github.com/owenservera/repos-hq/blob/main/docs/integration/CORE-ENGINE-ADAPTER-MATRIX.md) |
| VP-115 | Canonical UI component definitions | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-116 | Workspace layout templates | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-117 | UI data-binding projections | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-118 | UI authoring grammars | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-119 | Studio editing adapters | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-120 | UI theme/design tokens | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-121 | Accessibility semantic adapters | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-122 | Device input normalization | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-123 | Device render adapters | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-124 | Cross-device session reconciliation | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-125 | Typed command transport adapters | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-126 | UI preview/diff tools | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-127 | UI undo/history tools | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-128 | Effect status/receipt projection | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-129 | Experience/onboarding steps | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-130 | UX contextual explainability | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-131 | Evaluation fixtures and acceptance cases | TierFlow UI and execution | D | [T:README.md](https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-5/README.md) |
| VP-132 | Plugin manifest schema | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-133 | Plugin trust/signature verifier | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-134 | Plugin package resolution | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-135 | Plugin dependency solver | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-136 | Plugin permissions and grants | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-137 | Plugin installation and updates | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-138 | Plugin activation/revocation | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-139 | Plugin state namespace and migrations | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-140 | Plugin backup/export/import | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-141 | Plugin telemetry and quotas | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-142 | Plugin sandbox execution | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-143 | Plugin crash/restart supervision | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-144 | Plugin compatibility negotiation | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-145 | Plugin capability discovery projection | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-146 | Plugin test/conformance packs | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-147 | Plugin localization/language packs | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-148 | Plugin privacy and retention restrictions | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-149 | Plugin audit/evidence readers | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-150 | Plugin marketplace/discovery catalog | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-151 | Plugin authoring from natural language | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-152 | Plugin policy contribution restrictions | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |
| VP-153 | Plugin developer SDK and diagnostics | Cross-cutting platform services | N | [H:ARCHITECTURE-DECISION-FRONTIER.md](https://github.com/owenservera/vivim-web-harness/blob/main/docs/plugins/ARCHITECTURE-DECISION-FRONTIER.md) |

## Mandatory second-pass evidence for every row

Record: exact repository/commit/path/symbol/caller, implementation role, registration/activation API, deployment placement, data read/write ownership, identity/actor/world/device grants, native/platform permissions, offline modes, API and state versions, package type, failure isolation, resource limits, uninstall/revoke, tests, and visible user value. Classify **ALREADY-PLUGIN / CONVERTIBLE-SEAM / DESIGNED / MISSING-PORT / UNSAFE / DUPLICATE**. Maintain a machine-readable source map and deduplicate against existing Harness plugin and atomic catalogs.

**Critical:** research inventories and architecture documents are not effective code authority; do not infer availability from a feature name, view rendering, type declaration or historical implementation.

**Total: 153 distinct candidate plugin surfaces across 10 platform areas.**