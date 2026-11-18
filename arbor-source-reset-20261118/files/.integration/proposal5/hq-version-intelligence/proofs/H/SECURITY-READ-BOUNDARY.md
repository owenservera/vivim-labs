# HVI H — native catalog and typed local read boundary

Branch base: da338c0025e878ff9ee4eb9958dc8bd215dfb748

GO_FOR_W2_TOOLING is a development-only override. G1 has no accepted main SHA; Node24 and security acceptance remain UNRUN.

Canonical Host NL discovery: apps/vivim-host/control/action-catalog.v1.json, intent-planner.mjs, command-intent.mjs and dispatcher.mjs. This H change registers a single inert versions.compare.pinned action with enforcement_missing and null executor. The canonical dispatcher therefore still refuses actual execution. No second dispatch table, HTTP/phone endpoint, background scan, provider API or source authority was introduced.

Native local read: control/hvi-capability.mjs requires Host-injected actor/session, per-repository allowlist, live verifySession and verifyGrant callbacks and an authorized Git-object adapter. It uses Lane A resolvePin and snapshotAt with per-page revocation checks, G1 runHviTool and a final pre-report grant recheck. It projects safe summary only: no raw paths, manifest entries, sourceGrantRef, actor/session IDs or private text. Synthetic fixtures only.

**Blocker:** inspected Host source contains owner token/browser session and device/onboarding grants, but no attested live development-source grant service for this HVI action. Production NL invocation is AGENT_DISCOVERY_UNVERIFIED; H native catalog availability is installed-but-disabled. No real source comparison, remote access or beta promotion.

Test file: apps/vivim-host/test/hvi-capability.test.mjs. Command: node --test apps/vivim-host/test/hvi-capability.test.mjs. **UNRUN, no executable exit**: connected GitHub offers source/commit but no checked-out Node24 runner. Cases: 3 same-repo pins, per-page revoke, pre-report revoke, missing grants, unapproved repo, secret scope, moved SHA and redaction.

Next owners: Host security grants + local trusted caller; Node24 CI; Z2 independent authority review; G2 merge reconciliation after accepted G1.

**PREPARED_NOT_PROMOTED.**

Additional boundary: createHviLocalOwnerGate reuses the existing Host owner-token verifier or live UI session, checking again on every read. It separately requires verifyDevelopmentSourceGrant, which must come from a real Host grant owner and is NOT provided by the owner token. Synthetic test verifies revoke and phone-proxy denial. No production grant store has been discovered, so normal invocation remains disabled.
