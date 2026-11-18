# VIVIM Host — M1 read-only local diagnostics

M1 is an additive, **read-only** Windows-first Host UI. It does not install or start relay/chat workers, own Chrome, send to an AI, or claim a working synchronized Talk mirror.

From the repo root: `node apps/vivim-host/server.mjs`
Local address: `http://127.0.0.1:43187`
Targeted tests: `node --test apps/vivim-host/test/*.test.mjs`

Optional environment:
- `VIVIM_HOST_PORT`: loopback port, default 43187.
- `VIVIM_HOST_DATA_DIR`: explicit private Studio data folder; `CAPTURE_STUDIO_HOME` also honored.

## What M1 actually observes

- `studio relay` now records a private heartbeat after a successful or failed cloud sync cycle.
- The separate `relay-chat-mvp.mjs` consumer records its heartbeat from the queue polling loop.
- The Host reports `online` only for a fresh successful worker cycle; stale/error/missing receipts remain explicit.
- The local message journey distinguishes cloud-pulled, queued, claimed, terminal local results and cloud update acknowledgements by comparing the append-only bridge with `relay-state.json`.
- `/api/host/status` is GET/HEAD-only and shows metadata, never the prompt or reply.
- `cloud_update_accepted` proves the relay service accepted an update. It **does not** prove the phone UI displayed it.
- Harness and Chrome readiness remain `unknown` until an independent read-only verification exists.

**Existing running workers will not write heartbeats until their updated code is started.** Do not terminate unknown worker processes, force a second Chrome driver, or resend a queued/uncertain message to force status. Future M3 supervision will own safe lifecycle controls. A missing heartbeat today does not prove the worker is down.

## Privacy and safety

All receipts live under the Studio data folder, not in Git, with no credentials, prompts or replies. Host binds `127.0.0.1` and refuses non-local Host headers. No new provider API requests, browser actions, webapp sends, process kills, or mutation routes.

This is **not yet** the actual locally synchronized Talk mirror or the Windows-installed desktop app. Those are M2 and M4. Preserve the current Vercel relay and session cookie flow unchanged.

## Internal data intelligence (PDV01–PDV03, source slice)

An **owner-authenticated, localhost-only, read-only** metadata API is defined at:
- `GET /api/host/data-intelligence/registry`: thirteen machine-readable ingress types; none grants additional capture permission.
- `GET /api/host/data-intelligence/summary`: aggregate counts from the existing Host bootstrap ledger, not indexed assets or measured personal-data utility.

Both require the existing `Authorization: Bearer <owner capability>` header, refuse foreign `Origin`, and are **not** registered on Direct `/mobile` or any cloud relay. The [ingress registry](../../docs/data-intelligence/ingress-registry.v1.json) and [field crosswalk](../../docs/data-intelligence/field-crosswalk.v1.json) are included in the repository; packaged hosts must include the registry JSON with their runtime dependencies.

`data-intelligence/journal-adapter.mjs` is a pure, read-only journal-receipt normalizer. It has **no automatic source discovery, scanning, storage, or provider control**. Next wiring requires canonical workspace/journal resolution and safe World/source correlation. No personal-content migration or optional telemetry collection is enabled.

## Universal Text Control — initial read-only slice

The user-facing product invariant is [Universal Text Control](../../docs/design/UNIVERSAL-TEXT-CONTROL-V1.md): every user-facing configuration/action will use one typed Host action callable from desktop and paired phone text as well as UI. Current implemented slice is **non-mutating**. An authenticated owner can use:

- `GET /api/host/control/catalog` with the Host owner Bearer capability on loopback; returns six proposed/readiness-qualified actions.
- `POST /api/host/control/plan` with the Bearer capability, same-origin, `X-VIVIM-Control: 1`, JSON `{\"text\":\"Set my theme to dusk\"}`; returns a deterministic preview, never executes.
- Over the authenticated Tailscale Direct mobile proxy: `GET /mobile/api/control/catalog` and `POST /mobile/api/control/plan` (same-origin, `X-VIVIM-Control: 1`). The separate **Control** phone tab sends previews **only when Direct mode is verified**; it will not fall back to the cloud relay or provider Talk.

The read/plan API has **no execute route**, and `readiness=adapter_pending` or `unsupported` is not success. A phone can request plans today but cannot yet change a machine setting through this new universal path. Runtime executor, durable approval, privacy and network enforcement remain tracked in UTC03–UTC07.
