# R003 — ARBOR Studio Craft intelligence integration

## Actual product changes

- ARBOR on `main` now contains `studio/src/studio/craft-intel-routes.mjs`, integrated with `studio/src/studio/integration-hub.mjs` as `GET /api/v1/dev/craft-intelligence`.
- Studio's existing server exposes `/craft-intel` and `/ui/craft-intel.mjs`, and the Studio sidebar links to the new read-only view.
- The backend reads the existing Labs `vivim.application-surface-atlas/0.1` and optional `vivim.application-surface-reconciliation/1` from absolute operator-configured paths. No new graph/registry/persistence/Host action/Chrome profile.
- Added `studio/test/unit/craft-intel-routes.test.mjs` with six scoped authority/privacy/identity tests. These have been committed but NOT executed against a full ARBOR checkout in this ChatGPT environment.
- Local separately staged analogous prototype passed 5/5 Node contract tests. That is NOT equivalent to six checked-in tests passing.

## How to run locally

1. Generate a genuine Craft evidence bundle on an authorized first-party VIVIM MVP, using a separate Craft browser.
2. Convert with `craft-anything-arbor-adapter/src/cli.mjs import`, ideally with the actual native Craft verifier (`--native-bin`) and source-scoped capture folders.
3. Optionally reconcile against the existing Labs profile baseline.
4. In the same PowerShell session from which you launch ARBOR Studio:

    $env:VIVIM_CRAFT_ATLAS_FILE='C:\captures\report-01\atlas.json'
    $env:VIVIM_CRAFT_RECONCILIATION_FILE='C:\captures\comparison-01\reconciliation.json'

5. From ARBOR root run `node --test studio/test/unit/craft-intel-routes.test.mjs` then start the existing Studio service using `cd studio; node bin/studio.mjs start`. On the resulting Studio address visit `/craft-intel`.

## Verification status

- GitHub source changes: PASS (committed, read back by path and blob SHA).
- Staged local Node prototype tests: PASS 5/5.
- Checked-in ARBOR six tests: UNRUN here.
- Full Studio service startup and API readback: UNRUN here.
- Genuine Craft exported data: UNRUN here.
- Native Craft Rust evidence parity: UNRUN here.

**Continuation:** preserve this read-only seam. Next priority is a real first-party Craft capture and Studio readback, followed by verified capability-specific atlas reconciliation; only then select a pure Rust algorithm to harvest. Never use an observed UI control as a new executable Host binding without independent authorization.

## ARBOR commit references

- `5db84c00339810e00939342bf3ff56a11ba2c280` — projection module.
- `b34261762b3c1e0fd60aeae2d2e05ca8634a4009` — API registration.
- `07cf1079e4c24bb2866835bc972c89d06e5bee4b` — UI page.
- `1e0c569ce134845b40f26f0860962ab0b3fb2480` — UI client.
- `8f0f021f553dec3035b647458320024e7e2af653` — protected server routes.
- `dba02d491a1103010519a87d7848bcf07bb045aa` — Studio navigation.
- `23dce5c289bd96887eb0ee6b81d5fb66f6742e79` — checked-in tests.
- `a60d7bfdd2976e4502f69afa5ce8bbef6675d389` — operational docs.
- `721eb94dc33c24311fce509b89f357858bbf9ea6` — qualification of native verification label.
