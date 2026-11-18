# PG03 — negative acceptance, expected refusals, observed-versus-unrun

**No cases below were executed against B Host, A Tauri, E PWA, D HQ or a real phone.** These are adversarial commitments for F and post-G code owners; see NEGATIVE-CASES.json for IDs and synthetic fixture descriptions. The Node fixture test checks only file structure and policies, not production implementations; it too is UNRUN.

## P0 identity/session/Host tests
AUTH-01/02: bind native launcher to a specific verified child, not a healthy response on occupied 127.0.0.1:43187; no owner bearer sent to spoof. Limit Tauri IPC remote URLs to verified exact origins and windows.
AUTH-03/05: stolen grant ID and active session ID must not permit fresh privileged GET/POST or second effect. Existing Host claim-once is necessary but not sufficient to stop fresh plans under a stolen session. Require per-request proof/fresh nonce (or demonstrably equivalent protected binding), actor/audience/method/path/body/plan-bound; atomic replay refusal.
AUTH-04: duplicate session-signature challenge nonce/timestamp is rejected (source contains usedSessionNonces, current proof still unrun).
AUTH-06: revoked/scope-changed grant invalidates existing session immediately; never leak private memory/status.
AUTH-07: exact Origin/CSRF on POST enrolment/start/prove/session as well as control/relay; Host proxy forwarding headers must not be client-forgeable.
AUTH-08/09: lost IndexedDB key forces pairing; same-origin malicious JS can still use nonextractable CryptoKey sign method, so CSP, rendering and Host authorization are independent protections.

## PWA and effect truth
PWA-01/02: SW receives POST /mobile/api/control/execute, GET /mobile/api/memory/recent, and range/private request while offline: all bypass cache and never replay. A shell may load; private source/effect may not.
PWA-03/05: offline or stale World/wiki/candidate never gains authority; UI shows offline_unverified/source_stale/reconnect_requires_current_world and refuses stale plan/revision.
PWA-04: timeout after ambiguous provider or Host execute yields UNCERTAIN and receipt reconciliation; no automatic resend or claimed success.
PACKAGE-01/02: Windows MSI builds without signed APK or placeholder binary; Android Chrome app manifest/same-origin icon and service worker load without Android bridge.

## Storage/consent/low disk
STORAGE-01/02: measured physical free bytes and OS reserve trump selected Full managed quota; exhausted volume triggers read-only protected refusal.
STORAGE-03/08: rebuildable flag alone cannot delete; sourceAvailable, current permission, stable source revision and active-lease check are required; deny rehydration if source unknown/revoked.
STORAGE-04: simulate ENOSPC/crash in SQLite WAL, VACUUM INTO, wiki atomic rename and graph journal; preserve original and claim/receipt chain; no blind cleanup.
STORAGE-05/06: never auto-evict original user ZIP, DCB, personal memory, grants, Host receipts, World or kernel store; .hq/index.db contains potentially durable note/receipt data, cannot blanket rm.
STORAGE-07: corrupted/replaced ZIP and missing digest bound inspection must not checkpoint or publish any graph memory; consent and erasure fence override stale import.
STORAGE-09: offline PWA shell may render but not claim live Host provenance, current approval or source verification.

## Acceptance contract for all cases
Record source G commit, test ID, actual command, runner+OS/device, exit code, execution log, artifact byte counts where applicable, before/after Host receipts, no-effect invariant, bug/owner and retest. Security accepted only with actual owner-bound proofs; no case PASS from reading source, mock return value or fixture self-consistency.
