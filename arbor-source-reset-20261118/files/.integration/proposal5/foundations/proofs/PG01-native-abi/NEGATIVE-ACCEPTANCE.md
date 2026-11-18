# Negative acceptance — PG01 (candidate tests / expected denials)

**Run truth:** Rust suites below are authored in rust/ but **UNRUN** (no cargo/rustc). Authenticated ingress, live manifest loader, actor checks and full UI integration are also **UNRUN**, and several interfaces are absent. No negative security gate may be recorded PASS.

| ID | Scenario | Source-grounded expected outcome | Actual status |
|---|---|---|---|
| N01 | install replacement with wrong expected generation | EngineRegistry::install Err Conflict; active generation unchanged | Rust harness UNRUN |
| N02 | replace interface or remove previous capability | Err Conflict; no active mutation | Rust harness UNRUN |
| N03 | empty slot, implementation, interface or capabilities | Err Invalid | Rust harness UNRUN |
| N04 | old pinned lease after v2 install | old Arc implementation v1 unchanged | Rust harness UNRUN |
| N05 | rollback to reviewed v1 | install v1 with Some(2), new generation 3; older v1/v2 leases unchanged | Rust harness UNRUN |
| N06 | too many Scheduler tasks / too little output budget | fail closed; no effect journal | Rust harness UNRUN |
| N07 | cancellation before DAG work | RunState::Cancelled, staged cache not committed | Rust harness UNRUN |
| N08 | blocked Execute ignores cancellation | NOT preemptible; must reject acceptance once returned; owner must bound external execution | **BLOCKED by existing API** |
| N09 | stale or foreign RevisionToken/Namespace | early Err Conflict, not necessarily DagReport::StaleGeneration | Rust harness UNRUN |
| N10 | changed source revision with same graph epoch | exact input stamp differs, recomputes; verified source existence remains owner's obligation | Rust harness UNRUN |
| N11 | stale preview races new input/policy/graph/source epoch | candidate read-only revision gate refuses without leaking source ref | fixture-only Rust harness UNRUN; E live UNRUN |
| N12 | forged actor, session, audience or owner | Host/Tauri ingress rejects before emitting trusted snapshot | **BLOCKED** no accepted bound actor ABI |
| N13 | caller-set provenance or forged digest | shape checks alone insufficient; verified source walker and current owner grant required | **BLOCKED** source authentication missing |
| N14 | source revoked while engine computes | cancel/fence output; no stale projected private ref/trace | **BLOCKED** sourceEpoch & verified revocation adapter absent |
| N15 | manifest owner=host, granted=true, unknown effect shell/provider-send | never admit to pure engine; no effect execution path from manifest | **BLOCKED** no manifest admission parser/runtime |
| N16 | arbitrary effectful ComputeEngine marked PURE_INTERFACE | static interface label does not sandbox Rust; reject at trusted compiled bundle review/engine registry instantiation | **BLOCKED** trusted-source enforcement absent |
| N17 | unknown ABI / outdated grammar pack / invalid manifest digest | owner deny/quarantine and preserve current generation | **BLOCKED** versioned admission absent |
| N18 | raw private source ref in durable DAG event | existing commit_report deliberately writes descriptors only; inspect audience projection for no leaks | Source inspection observed; runtime test UNRUN |
| N19 | replay of Submit/Approve or uncertain browser send after engine swap | B Host journal rejects duplicate claim/auto-retry; pure engine receipt is not Host receipt | Host security integration UNRUN |
| N20 | unbounded producer count, slow subscriber or queue flood | cap per-input producers/queue/task/output and report SKIPPED_BUDGET rather than dropping commit events | **BLOCKED** native fan-out not implemented |
| N21 | different source owner under same personal draft | namespace+trusted actor+audience check; do not reveal even denied source existence | A structural test UNRUN, trusted context BLOCKED |
| N22 | derived event claims effectPerformed=true / synthetic HostReceipt | F/N0 inert projection rejects or displays unverified; only B actual receipt may be referenced | Live parity UNRUN |

## Falsification rules
- Passing Rust swap unit tests is **not** proof of safe plugins, security, serialized ABI or runtime release.
- A timeout after ComputeEngine::execute completes is a discard fence, **not** execution termination. Never claim forced cancellation.
- SourceRef.digest, Provenance.digest, EngineDescriptor.implementation, a manifest source.path, and caller-supplied actor are not source-authenticity proof.
- Rollback is a **new generation** with reviewed old implementation, never restoring old registry counter.
- Each independent pure producer may offer candidates; Host approval/effect journal remains one canonical owner. No additional route/scheduler/graph permitted.
