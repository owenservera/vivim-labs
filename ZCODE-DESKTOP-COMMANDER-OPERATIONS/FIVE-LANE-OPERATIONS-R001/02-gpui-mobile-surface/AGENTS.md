# L02 — GPUI + Android PWA Convergence agent directive

**Lead:** ChatGPT Dev (native/mobile scoped). **Priority:** P0. **Master issue tasks:** S02–S14 release/device; native menu, IME and GPUI parity after safe packaging..

## Mission
Converge native Windows GPUI and the one existing Android PWA onto the same Host-backed semantic and session state, then safely release without changing paired phone URL.

## Allowed scope
Audit native/gpui-desktop, apps/mobile-relay, LiveSystemState and existing Host packaging. File edits require TASK-CLAIMS checkout and nonoverlap with ONE-UI/PY-001 owners. Legacy UI retirement only after capability parity.

## Existing infrastructure to reuse (read/verify before launching)
- `ARBOR/.zcode/workflows/arbor-area-d.dwf.ts`
- `ARBOR/.zcode/workflows/arbor-area-e.dwf.ts`

## First work packet
Refresh #31 and the canonical UI closure document. Check current package/native binary/lockfiles and actual served Host source SHA read-only; propose one immutable candidate and an isolated preflight/rollback plan. Do not promote yet.

## Non-negotiable stop boundaries
Never spin up a second Host, dashboard, browser-based Windows product renderer, separate event bus or unverified replacement URL. Do not restart production, switch service worker or interrupt live drafting/pairing for a test.

## Acceptance target
Exact source SHA = packaged artifact = served version; native Windows input and physical phone both reflect the same Host epoch/revision, reverse direction works, rollback available, screenshots and receipts match.

## Reporting and coordination
Read `../DIRECTIVE.md` and live #33/#32/#31, then current Labs `TASK-CLAIMS.md`; **never count a plan as a running agent**. Begin read-only; do not check out a source claim owned by another role. Follow the [handoff template](HANDOFF.md). Report in ERA 1/2/3 with real SHA and exact test/runtime evidence, unresolved blockers and a next action.
