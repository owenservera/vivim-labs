# W01-B Follow-up — Host bootstrap and authority compatibility packet

**Status:** PREPARED / INTEGRATION GATE BLOCKED; **NO RUNTIME REPAIR APPLIED**.
**Date:** 2026-11-12.
**Repository / branch:** `owenservera/ARBOR`, `work/p5-w01-b-host`.
**Baseline commit:** `9c52c35d473e15c86939c786f0d5e3819da5c302` (135/135 imported Host blobs verified).
**Integration branch:** `integrate/proposal5-beta-r001` **not present** during current check. W02-B branch **not created**. Do not merge/rebase/modify root or other lanes. This packet is preparation, not a Wave 2 implementation or a GO.

## Exact review evidence

1. Lane A `src-tauri/src/main.rs`: fixed `127.0.0.1:43187` and `GET /api/host/status` success currently serve as readiness proof. Then `desktop_launch_url()` reads the durable `VIVIM_BOOTSTRAP_TOKEN` or protected `bootstrap/owner-token` and sends it as Bearer to `POST /api/host/control/ui-session`. A foreign local listener could answer health and receive that token. The spawned-child handle is not authenticated to the health response.
2. Lane B `server.mjs`: `/api/host/status` is loopback/unprivileged; `/api/host/control/ui-session` checks Origin, `x-vivim-control`, and durable owner Bearer. Existing short-lived UI code exchange installs `HttpOnly; SameSite=Strict` cookie; reuse this machinery.
3. Lane B `bootstrap/owner-auth.mjs`: `loadOwnerToken` verifies or creates protected 0600 token file and checks Windows ACL. Preserve **unchanged**, including file path, Windows security rules and existing CLI usage.
4. Lane B `control/journal.mjs`: `<id>.claimed` exclusive `wx` fence; a claim without a receipt is `uncertain`. `control/dispatcher.mjs` checks plan digest, expiry, catalog version, replan/risk, actor surface and exact approval; claim occurs before effect. No second writer or new journal required.
5. Source imports in W01-B are known to require C's `studio/` and E's `apps/mobile-relay/` plus G root assembly; see `W01-B.md`. D's `hq/lib/arbor-service.mjs` remains **not mounted into Host**, and cannot receive personal/World permission implicitly.

## Proposed native ⇄ Host bootstrap protocol (joint A+B sign-off required before implementation)

**Protocol ID:** `arbor.host-launch/1` (launch-attestation ONLY, not a replacement owner auth system).
**Transport:** dedicated loopback HTTP only after Host binds; launcher nonce/challenge and high-entropy per-spawn secret are supplied to the **spawned child only** via native-owned protected startup channel. Do not send the durable owner token over HTTP. On Windows, confirm the actual confidentiality and lifetime of the selected IPC/environment handoff with A; merely passing a predictable PID or reading a public file does not prove identity.
**Ownership:** A generates the per-spawn secret and retains the OS child handle; B checks launch messages, computes attestation and issues a scoped, one-time UI launch code through the existing session store. B MUST NOT create alternate grants, session cookies, owner token stores or effect admission.

Suggested startup order:
1. A checks any existing port-43187 listener before spawn. If an unrelated listener is detected, fail closed and **never** reuse it or send a durable capability. If an already-running trusted Host is desired, it needs a separately proven compatible attachment protocol, not the simple health check.
2. A starts the in-repo Host child with an OS-protected per-launch secret (at least 256 random bits), `launchId` and version. The Host only installs the launch attestation route when this authenticated child mode is enabled. Unlaunched CLI/test Host remains unchanged.
3. B binds exact `127.0.0.1:43187`; on `EADDRINUSE` fail closed, do not mark ready or call external listener. B never returns any bearer/launch secret in a status projection or log.
4. A submits fresh `challengeNonce` to `POST /api/host/native/attest`, with strict input length, content type and loopback validation. B returns `{schema,launchId,hostPid,challengeNonce,protocolVersion,proof}`. Proof should be HMAC-SHA256 over a canonical, domain-separated serialization binding protocol, launch ID, challenge, Host PID and purpose `attest`, keyed by the per-spawn secret. A verifies HMAC in constant time, exact expected launched child PID/handle still alive and full response fields.
5. Only after attestation may A request **once** `POST /api/host/native/ui-launch`, presenting a fresh HMAC-based proof with distinct purpose `launch`, `launchId`, nonce and bounded deadline. Host verifies, consumes the one-shot launch allowance atomically, and calls **existing** `uiSessions.issue()`. It returns only a short-lived UI code. No owner token or session cookie is sent to A, and the native launch endpoint must not grant arbitrary actions.
6. A navigates its trusted webview to the existing `/#control-session=<code>` flow. Host `ui-exchange` still consumes the one-time code and sets the same HttpOnly cookie. Existing owner-capability and dispatcher checks remain in force for subsequent actions.
7. A verifies the same bound Host connection/identity for launch and the child remains alive; close/revoke unexchanged codes when launch fails. Never silently fall back to unsigned `/api/host/status`, token-over-HTTP `ui-session`, or a foreign listener when native-launch mode was requested.
8. Existing `/api/host/control/ui-session` must remain for separately authenticated legacy CLI flows unless formally retired with compatibility tests, but native desktop MUST stop using the durable-owner-token Bearer path after the new protocol is accepted.

**Important:** Env-passed per-spawn secrets can be readable to sufficiently privileged same-user local attackers. A must threat-model that limitation and prefer a tightly scoped inherited pipe/OS channel where practical. Attestation alone is not a substitute for trusted child identity/OS ACL binding. Either use a single authenticated connection for proving and minting or otherwise prevent a listener-rebinding race between phases.

**Suggested schemas/errors:**
- `arbor.host-launch-attestation/1` fields: `launchId`, `hostPid`, `nonce`, `protocolVersion=1`, `proof`. No token, root path, World ID, raw logs or personal content.
- `arbor.host-launch-code/1`: existing `uiSessions.issue()` response fields; code single-use and short-lived.
- Fail closed with `HOST_LAUNCH_UNTRUSTED`, `HOST_LAUNCH_VERSION_MISMATCH`, `HOST_LAUNCH_NONCE_REPLAY`, `HOST_LAUNCH_EXPIRED`, `HOST_LAUNCH_PORT_OCCUPIED`, `HOST_LAUNCH_ALREADY_CONSUMED`, and no bearer disclosure. Report only status/code to external callers.
- A's native `HOST_HEALTH` must no longer be an authenticity gate.

**Proposed Host-only patch locations once integrated:** `apps/vivim-host/bootstrap/native-launch-session.mjs` pure attestation verification helper; `apps/vivim-host/server.mjs` minimal route mount and startup fail-closed behavior; `apps/vivim-host/test/native-launch-session.test.mjs` negative and normal fixtures. The module should call existing `createControlBrowserSessions().issue()` instead of creating another session store. These are **not yet implemented**. A owns edits to `src-tauri/src/main.rs`; F owns contract cross-surface validators; G owns root release and lockfiles.

## Host effect and World tests to run after G integration

| Test | Existing code/evidence | Required observed outcome |
| --- | --- | --- |
| R0 read | `control/dispatcher.mjs`, `control/intent-planner.mjs` | Source-backed read returns current Host status, no claimed effect |
| R1/R2 | catalog/planner/dispatcher | Risk maintained, exact owner approval, R2 mobile refusal |
| plan digest mismatch | dispatcher execute | Refuse before claim/execute |
| approval channel/source mismatch | dispatcher | Refuse, no effect |
| duplicate execution | journal `wx` claim | One winner; others `CONTROL_UNCERTAIN` or receipt replay |
| after claim crash | journal get | state `uncertain`, no automatic retry |
| after possible external execution | journal/dispatcher | outcome uncertain until owner reconciliation, no fabricated success |
| receipt replay | journal and dispatcher | recorded receipt returned without executor replay |
| restart | Host runtime/ledger/recovery | preserved status, no duplicate writer |
| unreadable/corrupt ledger | bootstrap/runtime | explicit unavailable, no reset of protected World |
| Studio/HQ unavailable | Host read/projection | degraded/unavailable without fake receipt or lost journal |
| phone revoked/stale | phone sessions and enrollment | refuse effect path before approval/claim |
| backup/restore | synthetic fixtures only | no live user data changed, protected World/receipts retained |

Run `node --test apps/vivim-host/test/*.test.mjs` plus component fixture tests under `bootstrap/` and `corpus/`, then execute stress/fault injection when native runtimes are available. GitHub source inspection is **not** Node execution: **all test commands remain UNRUN** and exit codes N/A.

## Internal dependency handoff

**C:** `studio/src/config.mjs`, `studio/src/workspace-manifest.mjs`, `studio/bin/studio.mjs`, `studio/src/studio/bridge-commands.mjs`, `bootstrap-owner-adapter.mjs`, `relay-journal-correlation.mjs`; exact send/receipt correlation source must remain Studio-owned.
**E:** `apps/mobile-relay/lib/relay-core.mjs`, `control-language.mjs`, `nlcl/index.mjs`, `generative-ui/catalog.mjs`, `generative-ui/compiler.mjs`, `bootstrap-f01.mjs`, `bootstrap-f02.mjs`, `bootstrap-pass.mjs`, `theme-plan.mjs`, `bootstrap-prompts.mjs`; phone never owns effect execution.
**D:** `hq/lib/arbor-service.mjs` may receive only Host-approved development roots and data directory. No authorization bypass, developer-data to personal-World conflation, or read from private World without explicit Host scope.
**A:** launcher child bound proof, port collision and process identity, version compatibility, no durable token to unverified listener. Co-design protocol before changes.
**G:** source-lock, worker merge sequencing, root manifest, cross-owner paths, runtime packaging, recovery/no-donor Windows acceptance.

## Recommended exact test cases for A+B

1. Existing impostor returns 200 JSON at port 43187 ⇒ A rejects, **no durable credential sent**.
2. Foreign 200 with forged `launchId`/PID/MAC ⇒ A rejects.
3. Native-generated challenge nonce replay ⇒ B rejects.
4. Protocol mismatched major version ⇒ both sides reject.
5. Host binds a different port or cannot bind ⇒ launch fails closed.
6. Crash after attestation before code ⇒ no fallback to token HTTP.
7. Crash after code before exchange ⇒ code expired/revoked and no effect.
8. Two concurrent native launches ⇒ only authorized per-spawn session can mint its one-time code.
9. Legacy CLI owner capability flow ⇒ unchanged behavior and ACL.
10. Exactly one Host writer and one journal after desktop+phone session starts.
11. Negative origin/CSRF and unauthenticated `/api/host/native/ui-launch` ⇒ no code returned.
12. Stale source/phone grant during pending plan ⇒ refused without execution.

**Gate status:** SOURCE_COMPATIBILITY_PREPARED only. No native handshake repair, G integration, Node test, Windows device test, effect fault-injection execution, or World backup/restore was performed. Next branch `work/p5-w02-b-host` can only be created from G's exact integrated HEAD after confirmation.
