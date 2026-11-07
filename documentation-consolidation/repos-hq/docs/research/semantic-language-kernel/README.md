# Semantic language, inline code contracts and reprogrammable wiki — reference workspace

**Created:** 2026-11-02. **Status:** source-grounded reference/design only; not executable code, no accepted runtime changes. **Owner:** Repos HQ as future shared KB/semantic kernel for VIVIM Web Harness and TierFlow.

## Purpose

Centralize the existing deterministic natural/symbolic language, inline agent coding annotations, typed action execution, KB registries, project-agnostic TierFlow integration and the programmable wiki. **Do not create a fourth language/parser** or duplicate permission owners. This folder is the stable discovery home; implementation remains in its existing owners until a verified integration migration.

Read in this order:
1. [Architecture and design principles](ARCHITECTURE.md)
2. [Evidence-backed source map](SOURCE-MAP.md)
3. [Contract and delivery backlog](CONTRACT-AND-ROADMAP.md)
4. [Ten implemented optimization algorithms and conformance gates](OPTIMIZATION-ALGORITHMS-V1.md)
5. Canonical kernel architecture: [../.. /SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md](../../SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md) (relative parent link points to docs root).

## Rules

- Knowledge is not authority. Source annotations describe intentions and links; only owned code enforces permissions.
- Every user-facing capability must be discoverable and eventually accessible via both GUI and text through **one typed semantic action**.
- Wiki V1: selectors, menus, forms, inspectable canonical typed commands and reversible configuration. NL comes later on the **same** action route.
- HQ owns schema identifiers, provenance, type/semantic metadata and curation. VIVIM Host/Studio/Android and TierFlow Tier 3 retain their respective execution authority.
- TierFlow continues isolated and project-agnostic. VIVIM develops without a TierFlow runtime dependency.
- The existing `@hq:` protocol is v1; proposed extensions are not yet supported by its parser or linter.
- No change to installed `.hq-tool/` in TierFlow, no VIVIM HQ installation, no cross-repo data migration here.
- Agent messaging is HQ-owned via existing `scripts/sync.mjs` and its versioned coordination contract. Messaging never authorizes effects.

## Requested future outputs
- Source/semantic contract coverage and contradictions ledger.
- Machine-readable versioned annotation grammar with acceptance fixtures.
- Capability/semantic IDs and adapters with consistent authority and provenance.
- Click-to-typed-IR wiki V1; optional NL-to-same-IR V2.
- Three-repo conformance tests and explicit promotion gates.
