# Candidate contracts, lifecycle and acceptance

This is a **design sketch** for experiments, not an existing runtime schema.

## Family manifest shape (illustrative YAML)

```yaml
schema: vivim.extension-family.proposal/v0
id: es-legal-practice
version: 0.1.0
display_name: Spanish Legal Practice
locale: es-ES
jurisdiction: ES
modules:
  - plugin_id: es-legal-knowledge
    targets: [EP-03, EP-04, EP-17]
  - plugin_id: es-legal-language
    targets: [EP-06, EP-07]
  - plugin_id: es-legal-agents
    targets: [EP-05, EP-08, EP-09]
  - plugin_id: es-legal-workspace
    targets: [EP-12, EP-16]
  - plugin_id: es-legal-governance
    targets: [EP-14, EP-15]
requested_scopes: [] # explicitly enumerated per module after authority mapping
dependencies: [] # exact ID/version pins; cycles rejected
data_classification: confidential-client-matters
release_status: lab-only
```

A family can expose **many** extension points; each module declares only required ones and has independently revocable grants. The above fields are not endorsed for a shipping loader until validated against canonical runtime contracts.

## Lifecycle

discover -> inspect manifest/signature/source -> resolve compatible versions/dependencies -> compute requested scopes and data-access impact -> owner-review -> install disabled -> activate explicit capabilities -> monitor -> suspend/revoke -> update via staged migration -> uninstall with protected export/retention policy.

Default-deny on version, owner/world/device, signature, policy or consent mismatch. Uninstall disables execution, revokes tokens and clears derived caches without silently deleting original client matters. Unknown external effect remains unknown, with no automatic replay.

## Variant model

Compose capabilities through transparent, independent **profiles** such as Foundation, Practice, Advanced Analysis and Organization-Custom. Profiles are not privileges: each resolves to explicit plugin versions, enabled entry points, resource limits and approved permissions. Never silently change scope during profile switching. This document makes no pricing or commercialization assertions.

## Minimum acceptance matrix

1. Manifest validity and schema/version negotiation.
2. Proven origin and reproducible package hashes.
3. Dependency solving and cyclic dependency refusal.
4. Install with no permission yields no privileged operations.
5. Restricted owner/world/device/scope access denied.
6. Host R2 approval and R3 refusal cannot be bypassed.
7. Typed command equivalence text/touch/keyboard and es-ES locale.
8. Redaction, privilege handling and cross-client-matter isolation.
9. Provenance retained from retrieval through generated document.
10. Source freshness and citation verification; no invented case law.
11. Offline/revoked-device purge and no hidden-item inference.
12. Update compatibility, interrupted migration and rollback.
13. Crash, resource-exhaustion and malicious extension containment.
14. Uninstall clears derived state and preserves governed originals.
15. Same UI canonical revision across Windows/Android.
16. Auditable receipts for approvals, denials and verified effects.
17. Adversarial prompt injection from uploaded legal documents.
18. License/copyright/source terms enforced on legal corpora.
19. Human review before filing, signing, sending or other legal effects.
20. Evidence labels distinguish design, isolated test, integrated test and actual device verification.

Promotion requires traced source/interface pin, executable tests in owner repo, security review, actual customer-data confidentiality design and explicit integration acceptance. No client-facing claims of legal correctness without evidence.
