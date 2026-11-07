# VIVIM plugin system — platform-wide architectural design proposal

**Status:** candidate architecture, November 7 2026. Not implemented, ABI frozen, or approved production authority. This consolidates rather than replaces existing Harness [plugin research](https://github.com/owenservera/vivim-web-harness/tree/main/docs/plugins). The source atlas and [153 candidate surfaces](extension-points/PLATFORM-SURFACE-INVENTORY.md) are the discovery input, not an assertion that 153 working loaders exist.

## Product question

Which existing VIVIM subsystem contracts can accept third-party/user-authored capability, and which should be split into independently installable/replaceable components? The plugin system spans *all surfaces* (Windows, Android, Host, Studio/provider, knowledge/Rust, TierFlow, Oracle, data, integrations, security, DX) without changing core authority. A family can compose several plugin contributions; a standalone plugin must be independently versionable and revocable.

## Three-layer control plane (one logical system)

1. **Platform ports** — stable semantic contract for a capability: immutable contribution descriptor, typed schemas, version range, placement compatibility and negotiated availability. Ports are owned by canonical subsystem source; plugin manifests are not executable permissions.
2. **Extension manager** — one *logical* composition/installation state, projected onto Host and device through existing owners: package verification, dependency/compatibility resolution, user grants, activation state, generation pins, disable/revoke, migration and rollback. This is a design goal; do not create a second World writer or split-brain installer. Device-local state is valid only for explicitly local data, not a copy of Host authority.
3. **Execution adapters** — declarative/pure contribution by default; effectful actions only through existing Host dispatcher, Studio browser-effect owner or audited Android/desktop native broker. HQKG Rust trusted engine slots may host first-party algorithms; untrusted code requires a separately proved sandboxed process or WASM path.

```text
             Owner text / Oracle proposal / Studio / PC UI / Android UI
                                    |
                          Typed semantic capability
                                    |
                Extension discovery + policy-scoped projection
                                    |
           verified manifest -> dependency lock -> owner admission
                                    |
                     installed, versioned contributions
                    /               |                  \
           view/data/skill      pure compute          action proposal
                |                /    \                  |
        TierFlow UI/Repos HQ   Rust  isolated worker   Host / Studio / native broker
                |                |                        |
          canonical read      bounded result       owner policy, receipt, journal
```

**Core vs plugin boundary:** identity, scope/grants, owner policy enforcement, approvals, effect journals, canonical World writer, browser profile lease and signed device-pairing trust stay with canonical owners. A plugin may *strengthen* constraints or propose policies, not redefine who can consent. Oracle can design and orchestrate plugin proposals but cannot self-grant.

## Minimum contribution kind vocabulary (independent axes, not one class inheritance)

`command`, `action-adapter`, `view`, `ui-template`, `editor-tool`, `language-pack`, `knowledge-source`, `ingestion`, `parser`, `transform`, `schema`, `index`, `retrieval`, `ranker`, `memory-policy`, `rust-engine`, `agent-skill`, `agent-orchestration`, `workflow`, `trigger`, `verification`, `provider-adapter`, `device-native`, `storage-adapter`, `localization`, `theme`, `privacy-policy`, `self-healing-diagnostic`, `telemetry`, `test-pack`, `developer-tool`.

For EACH contribution separately represent **where code runs**, **where state lives**, **what authority it requests**, **how it is delivered**, **which device/build supports it**, **whether it works offline**, and **its trust tier**. A family can have one module exposing several distinct ports; a plugin may offer a Host implementation and Android declarative fallback.

## Versioned package/manifest: minimum conceptual schema

```json
{
  "schema": "vivim.extension-manifest.proposal/0",
  "id": "publisher.namespace.plugin",
  "version": "0.1.0",
  "digest": "<package-content-sha256>",
  "publisher": {"id": "publisher.namespace"},
  "compatibility": {"contracts": {}, "host": "*", "android": "*", "desktop": "*"},
  "contributions": [{"id":"example.view","kind":"view","port":"vivim.ui.view","portVersion":"1","implementation":{"placement":"declarative"}}],
  "dependencies": [],
  "requestedPermissions": [],
  "dataSchemas": [],
  "offlineMode": "none",
  "resources": {"maxMemoryBytes": 0, "maxCpuMs": 0}
}
```

Provisional only: no claim this JSON can be installed. Effective permissions are intersection of supported build + actual runtime readiness + owner grants + device/world scope + effect-owner policy. A verified signature establishes publisher/integrity, **not** execution trust or consent.

## Contract interfaces that need actual design/ratification

- `ExtensionDescriptor`: id, contribution type, port, semantic capabilities, schemas, publisher, digest, version.
- `ExtensionAdmission`: compatibility, trust, permissions requested and explicitly granted, World/device placement.
- `ExtensionAvailability`: supportedByBuild, installed, enabled, online/offline, provider readiness, owner permitted, stale/revoked.
- `ExtensionCall`: immutable operation, typed arguments, correlation, context scope and version pins, provenance.
- `ExtensionResult`: verified outcome vs proposed/cached/stale/uncertain, source freshness, receipts, errors.
- `ExtensionState`: owned data vs derived caches/drafts, schema migration CAS, governed export/purge.
- `ExtensionLifecycle`: inspect/install/enable/suspend/quarantine/update/disable/uninstall/rollback.
- `ExtensionConformance`: normative semantic, security, accessibility, effect, failure/replay and cross-device tests.

Bind to existing actual Host/HQ/TierFlow contracts before creating a new runtime protocol. Never use arbitrary `plugin.execute()` as a shortcut to OS or browser privileges.

## Implement by port category, not by one universal loader

**Tier 0: data/declarative** — UI templates, themes, vocabularies, skill descriptors, document schemas, read-only projections. Low execution risk; can be installed to PC and projected to existing APK if client supports schema.

**Tier 1: pure computation** — deterministic transform, parser, ranking, verifier; isolate by contract and strict resource bounds. Trusted HQKG compute slots may support built-in Rust implementations; third-party native Rust requires confinement rather than trusting in-process `ComputeEngine`.

**Tier 2: broker-mediated effects** — automation, typed actions, provider/site integrations, device native actions. Plugin contributes a typed candidate, and existing effect owner validates, approves, executes, records and reconciles. Revocation during run and unknown effects are hard stops.

## State and deployment constraints

- World/knowledge objects: canonical owner only, versioned, with provenance, tombstones and access checks.
- PC plugin data: namespaced, quota-bound, backed up with explicit export/retention; independent plugin caches cannot become core truth.
- Android: declarative views and scoped local drafts/offline projections first. Native OS capabilities exposed only via APK-bundled audited broker; no assumption of runtime downloaded DEX/.so installation.
- Windows: Tauri plugin/ACL builds are distinct from runtime user extension grants. A desktop extension cannot edit Tauri permissions or browser session without owner-controlled release.
- Studio: existing deterministic plugin registry remains an implementation seam; browser CDP lease remains singular.
- Rust: slot generation swaps are genuine for trusted in-process engines but need crash isolation, durable swap reconciliation and cancellation for hostile/third-party code.
- Distribution: local file/peer first; remote catalog optional. Signed source and reproducible bytes required before shared distribution.

## Migration strategy and decision gates

1. **Read-only audit**: inventory all exact callers/registries/handlers, with SHA and evidence. No registry consolidation by name alone.
2. **Semantic port experiment**: three contributions (Host-only read, offline mobile view, reversible Host change), same identity from PC/mobile/text; two implementations and replaceable version.
3. **Loader/admission prototype**: verify package integrity, dependency lock, versions, user grants and install-disabled default; test disconnect and revocation.
4. **Isolation prototype**: first untrusted pure compute extension, compare isolated worker vs WASM; test time/memory/CPU/file/network bounds with Windows/Android evidence.
5. **Owner-authorized effect prototype**: manifest cannot create/override action risk or claim; Host/R2 denial and Studio uncertain browser effect remain protected.
6. **Lifecycle prototype**: update, interrupted write, crash/restart, rollback, device skew, uninstall and governed data export; no hidden auto-retry.
7. **Production promotion**: only integration owners change source and state machines; validate canonical typed command, UI and KB authority plus physical PC/APK runs before claiming release.

**Open decisions:** exact universal package format, native code trust model, WIT/WASM adoption, which state is truly device-local, package signatures, per-owner versus per-World composition pins, and whether plugin catalog discovery is integrated into existing Host capability descriptors. Hold until experiments.

**Non-goals:** a parallel personal AI, new browser controller, second Android app, independent KB/World, alternate UI publisher, extra Host law engine, or pricing/tier policy in these architecture files.
