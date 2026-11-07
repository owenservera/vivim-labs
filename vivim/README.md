# VIVIM — Extension architecture laboratory

Status: **exploratory scaffold, not a production plugin API**. This folder sits at the root of VIVIM Labs. It is the research, design and compatibility-testing home for composable VIVIM product extensions; it is not a second runtime or marketplace.

## Vocabulary

- **Extension point** — a canonical host-owned contract in an existing VIVIM subsystem where an independently versioned capability may be attached.
- **Plugin** — a bounded implementation or declarative asset pack implementing one or more extension-point contracts.
- **Extension family** — a coherent domain-specific collection of plugins, data packs, policies, workflows and UX.
- **Profile** — a manifest/configuration selecting and configuring a family for an owner/world/practice.
- **Pack** — a versioned distributable set of declarative resources; packs never acquire Host execution authority.

Extension families are *composition*, not new system authorities. The same plugin may be reusable across families; dependencies are explicit and permission-scoped. Family IDs are stable and locale-independent where possible.

## Start here

- [Extension-point discovery](extension-points/README.md): candidate attachment surfaces across the whole stack, with existing authority and verification status.
- [Spanish legal practice](families/es-legal-practice/README.md): first research family and multi-entry-point test case.
- [Manifest and acceptance contracts](CONTRACTS.md): provisional schemas, installation, lifecycle, isolation, risk and promotion gates.

## Architecture principle

Existing owners remain canonical: **Repos HQ** for knowledge/context/semantic intelligence, **TierFlow** for portable execution and visual/command runtime, and **VIVIM Web Harness** for Host approval, signed-in provider/browser controls, Studio, Windows/Android and device pairing. Oracle coordinates proposals but cannot bypass owner consent, Host policy or external-action approval.

All plugin interfaces are **candidate** until an inspected upstream caller, API, version and corresponding test prove an extension hook actually exists. An adapter proposed here does not imply an implemented plug-in loader. Source pin, read-only inspection and parity testing are required before migration or runtime modification.

## Development sequence

1. Inventory actual entrypoints, owners, callsites, versions and data flows across three canonical repos and Rust core.
2. Classify each as EXISTING-HOOK / ADAPTABLE / REQUIRES-NEW-CONTRACT / UNSAFE / UNKNOWN; keep evidence.
3. Prototype multiple extension layers using es-legal-practice without changing Host effect authority.
4. Verify privacy, provenance, cross-device parity, version skew, revoke/uninstall, failure containment and upgrade/rollback.
5. Promote through owners of actual canonical subsystems only after evidence-backed gates; preserve one source of truth.

No production code, user data or permissions are modified by this scaffold.
