# Experimental persistence authority gateway

Baseline ec3b4897c053d9374706a526c88de3d97b8c9e72. E45 source closure, no execution.

The four-arm plan now has a concrete separate host-owned durable vault. Create requires a fresh root and a planned run ID. Open binds the exact comparison spec, run ID, arm and plan entry digest. The vault admits typed persistence categories only when the arm allows them; session arm stores an opaque native session reference only, agent arm identity/checkpoint, goal arm goal/DAG/artifacts/evidence/registry, intelligence arm those plus learned class/operator/capability/repair. All operations are bounded, serialized, hash-chained and version-fenced; stale epoch writes and request ID conflicts are denied. Advance replaces the logical writer epoch while preserving allowed state. Recovery projects only admitted records.

CLI: comparative-vault create <fresh-root> <spec.json> <run-id> <limits.json>; recover <root> <spec.json> <run-id>; write <root> <spec.json> <run-id> <request.json>; advance <root> <spec.json> <run-id> <expected-epoch>.

The gateway is not an OS sandbox and does not prevent an unrestricted provider agent from using other filesystem paths or semantically encoding forbidden information into an opaque allowed checkpoint. The trusted host must enforce exclusive custody and route adapters through this gateway. A deleted tail requires an independently retained anchor for detection. No real provider sessions or four-arm runs executed. No test or build executed. E45 remains PARTIAL_SOURCE pending real adapter integration and isolation verification.
