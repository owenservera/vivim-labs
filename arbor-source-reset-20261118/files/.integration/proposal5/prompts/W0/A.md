# W00-A — Native Rust and Tauri

## Role and launch
You own **Wave 0, lane A** of ARBOR Proposal 5. Begin from the exact shared integration branch `integrate/proposal5-beta-r001` HEAD pinned by the wave orchestrator. Create your independent branch `work/p5-w00-a-native`. **W0 is source-read-only; no product code edits.**

**Shared plan:** Read `.integration/proposal5/README.md`, `01`–`09` documents and `08-BETA-GATES.json`. ARBOR is a single self-contained product; no external donor repo/service runtime wiring. Preserve working JS Host, Studio, PWA, HQ operational JS, HQKG Rust and TierFlow; no unnecessary features or UI redesign. Existing Host is sole effect/World writer. Do not commit secrets or user data. Workers never merge into integration/main; only Wave G merges.

## Mission: Source Truth and Import Freeze
Owned source inspection: src-tauri/, core/programmability/, native/ (read-only in W0).

1. Pin exact Harness native Tauri and core/programmability sources, their Cargo manifests and current native R001 branch/PR; identify compile-ready vs source-only work.
2. Pin HQKG Rust donor tree and inspected unmerged Rust lanes; map typed DAG, engine, revision/namespace, scheduler, importer, tests and source-level Cargo dependencies.
3. Record Tauri pinned versions, IPC remote origin scopes, Node Host process health/bootstrap token transfer, and the owner/session trust boundary. Label source risk, not exploitable incident.
4. Enumerate native Windows build/test commands, target hardware disk footprint and required file/path closure; recommend only essential beta native patches.

## Acceptance
Exact file/commit/path evidence, dependency/test list, P0 native risks and import/retire decisions; no code modified.

## Required output
Commit exactly one evidence report `integration/worker-reports/W00-A.md` (explicit disjoint report-path exception) with donor repo/branch/SHA, source files and ownership, tested-vs-proposed status, copied/not-copied decisions, security/privacy class, verification commands (mark UNRUN), blocker list and instructions to successor lane W1-A. Provide HEAD SHA. **Do not call anything successfully integrated.**
