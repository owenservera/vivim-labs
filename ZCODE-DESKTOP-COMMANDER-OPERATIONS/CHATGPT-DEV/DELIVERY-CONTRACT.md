# Dev receipt
Report: OW ID; canonical repo; refreshed base SHA; changed paths; ownership/collision check; cloud test commands and actual results; remotely verified main commit SHA; risks; deployment state; physical Android proof state; next owner/action.
Never merge CODED, PUSHED, DEPLOYED and DEVICE_VERIFIED into one status. Update the shared registry per ../OWNER-WORK/PROCESS.md, respecting single-writer coordination and append-only history.

## Required structured evidence for live deliveries
Use separate fields, never one aggregate “done”:
- **SOURCE:** canonical repo and inspected pre-write SHA; exact modified paths; fresh GitHub commit SHA re-read after push; collision/race handling.
- **CLOUD:** actual commands and environment, PASS/FAIL/UNRUN with counts and exit code; note if Rust/WASM cloud toolchain unavailable.
- **LOCAL BUILD:** explicit local authorization; immutable source SHA; native binary resolver/package location; WASM asset; Bun executable and frozen-lock state; tests exact to that source; ZCode 429 or executor issues recorded separately.
- **RELEASE:** previous PID and release SHA, current listener PID and release SHA, unchanged URL, HTTP asset status, Origin/auth route response, HTTPS/TLS/Tailscale Serve/DNS evidence and rollback outcome.
- **PRODUCT:** human typing->Wiki and authorized agent typing->same Wiki both marked PASS/FAIL/UNRUN individually; source revision/receipt/approval, no-Send, real Android screenshot/readback and whether the physical phone was available.
- **TRUTH:** distinguish local-only documents from connected GitHub; a simulated phone HTTP roundtrip is not device verification. Never describe an attempted rollback or startup as successful without readback.

See [the failure-recovery playbook](RELEASE-AND-INTEGRATION-PLAYBOOK.md) for the exact incidents that motivated these gates. Keep delivery receipts concise and link them instead of duplicating full logs in the SITREP.
