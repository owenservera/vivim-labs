# Source baseline

Pinned reference: https://github.com/owenservera/ARBOR/commit/e07e61385a53f30890b4a7f7441fe25d22a79b1c

Prior inspection enumerated 2,948 tree entries and sampled executable sources. It was not exhaustive and did not run the product. The following are hardening design targets, not confirmed exploits. Recheck current main and actual reachability before implementation.

| Component | Pinned source | Disposition |
|---|---|---|
| owner authentication | [apps/vivim-host/bootstrap/owner-auth.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/bootstrap/owner-auth.mjs) | Reuse protected owner authentication; assess same-user process and credential isolation. |
| Host effect authority | [apps/vivim-host/control/dispatcher.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/control/dispatcher.mjs) | Reuse digest-bound plans, exact approval, effect fences and fixed executors; trace alternate paths. |
| scoped grants | [apps/vivim-host/control/hvi-source-grant.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/control/hvi-source-grant.mjs) | Reuse ledger-backed resource scope, expiry and revocation; generalize only where justified. |
| workspace provisioning | [studio/src/workspace-manifest.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/studio/src/workspace-manifest.mjs) | Reuse explicit roots and profile selection; map OS boundaries and protected references. |
| development file access | [harness/src/dev/workspace.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/harness/src/dev/workspace.mjs) | Evaluate handle-based containment, Windows path semantics and races. |
| browser target | [harness/src/interface/target.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/harness/src/interface/target.mjs) | Preserve dedicated profile verification; evaluate transport and process isolation. |
| device enrollment | [apps/vivim-host/apk/pairing.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/apk/pairing.mjs) | Trace legacy and active enrollment paths; converge on atomic, protected challenge redemption. |
| renderer exposure | [packages/tierflow/src/security.ts](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/packages/tierflow/src/security.ts) | Verify authenticated local and remote privileged paths; inherit Host scopes. |
| desktop IPC | [src-tauri/capabilities/default.json](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/src-tauri/capabilities/default.json) | Narrow origin and IPC scope to necessary verified application surfaces. |
| knowledge provenance | [native/hqkg/src/scoped_graph.rs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/native/hqkg/src/scoped_graph.rs) | Bind structurally checked source provenance to authenticated ingestion. |
| secret scanning | [studio/src/studio/system-secrets.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/studio/src/studio/system-secrets.mjs) | Report incomplete scans explicitly and prevent sensitive diagnostic output. |
| native program validation | [apps/vivim-host/control/native-program.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/control/native-program.mjs) | Preserve read-only validation; enforce permissions independently during actual execution. |
| browser session | [apps/vivim-host/control/browser-session.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/control/browser-session.mjs) | Reuse existing one-time exchanges and session authentication; test expiry, revocation and trusted UI. |
| Host route surface | [apps/vivim-host/server.mjs](https://github.com/owenservera/ARBOR/blob/e07e61385a53f30890b4a7f7441fe25d22a79b1c/apps/vivim-host/server.mjs) | Inventory reads, mutations, workers and local/phone routes; map authentication to each effect. |

Verify actual installed packaging and release signing independently. Treat traditional branch protection and rulesets separately; preserve direct-main workflow without destructive ref changes. Do not publish live security exposures or credentials in this package.
