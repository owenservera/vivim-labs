# Capability dependency map — actual Host vs Android boundaries

Date: 2026-11-06. This is a source-anchored first-pass dependency audit, NOT a fully enumerated implementation census.

## Evidence used
- `vivim-web-harness/apps/vivim-host/control/action-catalog.v1.json` (v6, 2026-11-01): **19 source-catalogued** actions, categories control/system/devices/personalization/appearance/privacy/network/machine; targets local-host, multi-component-local and operating-system. Source-wired does not imply real-device acceptance.
- `vivim-web-harness/docs/design/UNIVERSAL-TEXT-CONTROL-V1.md`: any user action from mobile and desktop must compile to one typed, Host-controlled plan, with risk, policy, approval, journal, verification, receipt.
- `vivim-web-harness/docs/USER-JOURNEY.md`: connect AI via signed-in webapps, Talk, Build, provenance, undo and universal text.
- `vivim-web-harness/docs/design/APK07-mobile-runtime-world-offline.md`: native shell, Host connection, device-local capture exception, no offline provider-effect queue.
- `repos-hq/docs/architecture/MOBILE-ONE-AUTHORITATIVE-PATH.md`: only `apps/mobile-relay` is phone; one Host canonical World writer and effect send path.
- `repos-hq/research/unified-platform-architecture/README.md`: cross-platform graph/engine/context goals are not production proofs.
- `repos-hq/docs/architecture/MACHINE-001-host-effect-authority.md`: additional policy design, marked documented/spec only.

## Dependency graph (functional, not repository graph)

```text
Owner intent (text / tap / accessibility)
     |
     v
[1 identity + device-scoped authorization]
     |
     v
[2 unified action and readiness catalog]
     |
     v
[3 deterministic intent / AI candidate from signed-in webapp]
     |
     v
[4 typed bounded plan + scoped approval]
     |
     v
[5 single effect owner + send-once journal]  <--- canonical Host policy DEC-MOBILE-001
     |
     +--> [6 browser adapter] --> signed-in provider DOM / navigation / send / evidence
     |         +--> desktop Studio/CDP (existing route)
     |         +--> Android extension (candidate, not accepted executor)
     |         +--> embedded WebView/GeckoView (candidate)
     |         +--> Accessibility (UI fallback, NOT DOM/CDP)
     |
     +--> [7 device adapter] --> PC OS control OR approved Android system API
     |
     +--> [8 revisioned World/KB and event provenance] (PC canonical)
     |
     v
[9 verified receipt/status + projection to one VIVIM phone UI]
```

## Core requirement mapping

| ID | Requirement/blocking dependency | Current local PC Host mechanism | Android feasible replacement | Blocker type | Phone-only without PC today? |
| --- | --- | --- | --- | --- | --- |
| C01 | Authenticate and identify executor/owner/device | Host enrollment and device grants | Keystore + enrolled local identity + Host check | Existing authority contract | Partial: local identity only |
| C02 | Accept text/tap and deterministically resolve typed actions | Host UTC parser / dispatcher / catalog | Portable shared parser/UI + APK ingress | Port/wiring | Yes for local proposals; effect remains Host |
| C03 | Interpret ambiguous requests with subscribed AI | Desktop signed-in AI websites via Studio | Browser extension/controlled engine where login works | Browser compatibility & lifecycle | Research only |
| C04 | Observe signed-in provider page, DOM and conversation | Studio dedicated Chrome/CDP | Android WebExtension content scripts / controlled WebView | Browser engine | Conditional, unproven |
| C05 | Navigate tabs and control pages | Studio browser-owner/CDP | Extension tabs/scripting; embedded browser runtime | API scope and lifetime | Conditional, unproven |
| C06 | Exactly-once external send with receipts | Studio provider-effect owner and Host journal | Browser adapter could physically click but not become an independent send owner | VIVIM DEC-MOBILE-001 | No |
| C07 | Run multi-provider orchestrator/context exchange | PC browser sessions, Host orchestration and DCB | On-device local compute possible; provider sessions rely on browser adapters | Multiple: browser, memory, lifecycle | Not validated |
| C08 | Canonical Personal World/KB and revision/undo | PC Host durable state, Rust/SQLite target substrate | On-device DB technically possible; *forbidden as second canonical writer by DEC-MOBILE-001* | Architecture decision, not Android | No |
| C09 | Capture explicit user-shared URLs/docs/audio | Host ingress + device capture design | Android Share Sheet, SAF, mic with permission, app-private durable local store | App implementation & permissions | Permitted locally; verify runtime |
| C10 | Operate other installed Android apps | Not identical PC feature; native app integrations needed | Explicit intents/IPC + Accessibility UI when enabled | App sandbox, permissions & policy | Limited; no arbitrary private app control |
| C11 | Desktop process, filesystem and OS networking | Host OS adapters | Android APIs for *own* process/storage/network, limited settings | OS boundary | No PC-equivalent |
| C12 | Persistent autonomous background service | Host Windows service/processes | WorkManager/scheduled work and eligible foreground services | Android lifecycle | Only conditional/bounded |
| C13 | Notifications, owner approvals, device UI | Direct phone + Host policies | Native notifications/permission prompts/local UI | Android permissions | Yes for UI; Host validates effects |
| C14 | One consistent interface and provenance | Host World revisions projected to phone | Mobile-relay + APK, device-labeled local captures | Wiring/state sync | Partly |
| C15 | Runtime evolution/plugin programmability | PC local modules and typed actions | Bundled/sandboxed runtime, declarative UI + extension adapter | Native permission changes need APK release | Conditional |
| C16 | Recover after crash/restart/revocation | Host journal, durable canonical state | APK local recovery + Host idempotent receipts | Architecture + lifecycle | Local-only subset |

## What other work depends on (critical path)
**Foundation:** C01 identity -> C02 catalog/plan -> C04 browser observations -> C05 action control -> C06 send-once authority -> C08 canonical state -> C14 same UI/status. AI context/orchestration C03/C07 depends on browser observations and trustworthy source/receipts. Cross-app integration C10 and background operation C12 are separate native adapter tracks, not something browser extensions automatically solve.

## Three distinct ceilings
1. **Technical:** extension can access permitted browser tabs; APK can run native Android APIs; neither gives unrestricted other-app filesystem, shell, browser private profile or desktop CDP.
2. **Operational:** Android can suspend or kill background processes; mobile provider sites, tab state and extension APIs may differ from desktop.
3. **Current policy:** even a technically capable phone executor remains read/capture/proposal-oriented until Host/effect owner contracts are explicitly revised with tests.

## Next source-audit steps (do before selecting browser)
A. Extract exact current Studio/CDP action and capture API inventory from `vivim-web-harness/studio/**`, `harness/src/**`, Host dispatchers and tests; compare to each extension/Android adapter.
B. Verify actual APK Java entrypoints/build/manifest rather than inferred path; distinguish built, wired, test-passed and device-verified.
C. Publish executable feature-by-feature dependency and negative tests; separate platform impossibility, permission barrier, provider behavior and current VIVIM decision.
D. Rank browser engines only against that required API inventory, not extension counts. No MiniMax delegation until evidence is sufficient.
