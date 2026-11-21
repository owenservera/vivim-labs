# Proposed VEC SDK and program ABI (design, not accepted API)

## Minimal capability SDK contract
Every callable VEC SDK function has:
- stable registered capability ID and semantic version, exact compiled module digest and owner;
- supported utterance patterns/aliases by locale, declared slots and canonical intent frame;
- typed input/output schemas and composable edge kinds (event, condition, query, transform, proposal, action, view);
- effects/side-effect class and **real Host authority owner**, grant scope, confirmation policy;
- sandbox execution profile: pure/no-network by default, fuel/time/memory/IO limits and bounded cost;
- supported runtime versions, dependencies, idempotence and retry policy;
- test/oracle coverage and independently accepted evidence status; migration/rollback compatibility;
- availability state: catalogued, implemented, installed, enabled, denied, revoked, deprecated.

## Illustrative user program (data only; not runnable today)
```json
{
  "schema": "vivim.vec-program/0.proposal",
  "id": "personal.program.show-unfinished-tasks",
  "revision": 1,
  "owner": "current-authenticated-user",
  "state": "DRAFT",
  "trigger": {"kind": "local_schedule", "rule": "every_day_08:00", "timezone": "user-configured"},
  "steps": [
    {"kind": "query", "capability": "sdk.tasks.list@1", "arguments": {"status": "unfinished"}},
    {"kind": "view", "capability": "sdk.ui.show-list@1", "from": "steps[0]"}
  ],
  "requiredGrants": ["tasks:read", "ui:personal-view"],
  "onMissingCapability": "CAPABILITY_UNAVAILABLE",
  "onRevoked": "PAUSE_AND_REDACT",
  "source": {"utterance": "Every morning show unfinished tasks", "parser": "proposal-example"},
  "execution": {"modelRequired": false, "receiptRequired": true}
}
```
The SDK IDs, parser string and time in this example are invented demonstration values. They are **not** assertions of current ARBOR capabilities or actual user preferences.

## Compiler outputs
- `IntentResolution`: exact interpretation or needs-clarification/unsupported/denied; compatible registered capability candidates and missing slots.
- `ProgramProposal`: immutable canonical IR + source utterance provenance, capability versions, resource budgets, human preview, permission plan.
- `ProgramInstallReceipt`: Host approval reference, durable revision and enabled state; no claim of executed actions.
- `ProgramExecutionReceipt`: program revision, source/actor/scope, trigger instance, bounded step results, failures and affected projections.
- `ProgramDelta`: old/new version, changed triggers/permissions/behavior, verification and rollback plan.

## Developer Rust-capsule packaging
A first-party file may embed an opt-in fenced `rust` capsule with declared stable ID, pure typed ABI and test obligations. The meta-compiler extracts exact bytes and checks code under an isolated pinned toolchain, rejects arbitrary Cargo build scripts/net/filesystem operations, and produces an immutable registered module artifact. No standard user's NL utterance is translated into unrestricted Rust compilation. Runtime references vetted precompiled artifact IDs and supplied typed inputs.

## Existing VLang/SCF/SV interfaces
SCF's source passport can index developer capsule sources and their claims. Independent SV may verify pure capsule predicates and receipts, but does not self-approve developer specs or user grants. HVI can compare capsule/source program changes on authorized pins only; HVI native-tooling tests remain synthetic-only until later explicit real comparison. A user program is **personal World data**; do not silently store it inside HQ developer intelligence or source comments.
