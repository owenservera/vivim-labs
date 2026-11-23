# PY-001 — Explicit PWA Build / Change intent admission: implementation-ready R003

**Claim:** PY-001 Planning-only, scoped to this Labs packet. **Arbor source observed:** `dfb5480b17f5daa8861352e0cb81dd97e01e9f9d`. **Labs source observed:** `6840fa86db4426254ede07ace5dfcdbd070cd1e0`. The source SHA is an observation, not a long-term pin. **Do not edit ARBOR application source until Dev accepts a nonoverlapping claim.**
**Authority:** [master issue #33](https://github.com/owenservera/ARBOR/issues/33) → existing `P1-VLANG-WIKI-ORACLE` / `P0-RELEASE-SYNC` packets; [owner claims](../../../TASK-CLAIMS.md). The prepared packet does **not** claim an actual implementation, receiver ACK, queued job, provider message, or phone UI.

## Outcome and blocking decision

A person must be able to type a desired Build/Change outcome in the **one** canonical PWA/GPUI surface, explicitly choose **Create Work / Submit Build request**, receive a genuine Host-authorized durable request ID with status, and revisit or cancel it without unintended AI sends, visual effects or competing task stores.

**Currently blocked at PY-002:** there is no demonstrated *owner-authorized Host → persistent Work writer binding*. Studio's existing Work CAS is a reusable candidate, **not automatically a product authority**. Until Dev resolves it, **do not add a PWA POST directly to Studio or a new Work database, job runner, provider dispatcher, Host effect catalog entry or fake completed status**.

## Source-level reuse, no parallel engines

| Existing verified source path | Existing behavior | Scope of legitimate reuse |
| --- | --- | --- |
| `apps/mobile-relay/app.js` around `mountCompose` (line ~2519 at inspection) | Build already has explicit Preview + separate Activate; `send` queues one hidden `relay-chat` request in existing conversation route. | **Preserve** existing provider-design flow. New Create Work must be visually separate and must not silently call current hidden-provider send. |
| `apps/mobile-relay/build-canvas.mjs` | `ask` creates preview, `activate` requires exact Host visual plan and committed revision; canceled/stale preview does not effect. | Reuse explicit event/preview/current-source checks. Do not equate Activate visual UI with dispatching an agent task. |
| `apps/mobile-relay/build-composer.mjs` | `resolveBuildRequest` uses existing VEC; deterministic theme/visual preview is inert. | Classify intent without generating a new command parser or model requirement. |
| `apps/mobile-relay/control-client.mjs` | Direct/paired checks, existing Host `interpret/plan/execute`, exact digest/receipt verification, uncertain-response no-retry. | If Dev selects an existing Host-registered action, route through it only after confirming **actual** catalog and owner permission. An invented work action ID is prohibited. |
| `apps/vivim-host/control/dispatcher.mjs` and `action-catalog.v1.json` | Existing sole action planner, grant and effect authority. | Dev must decide/implement any necessary admitted writer within this authority; Planning does not claim such a writer exists. |
| `studio/src/agents/personal-agent-work.mjs` | Pure Work reducer: idempotent key → deterministic `work-<digest>`; queued/active/blocked/completed/cancelled; no provider call. | Reuse reducers and invariants rather than a second Work schema. |
| `studio/src/agents/personal-agent-service.mjs` | Existing `VersionedStore.compareAndSave` CAS with local scoped identity. | Candidate durable store only if Dev proves Host actor/session owner mapping, device revocation and cross-viewport semantics. |
| `studio/src/studio/personal-agent-routes.mjs` | Local `GET/POST /personal-agent/work`; explicit version/key, no tool dispatch; Work preview from stored provider reply requires user confirmation. | Host-private, not a public mobile API. Direct PWA bypass prohibited. |
| `hq/lib/channel-transport.mjs` | Existing ZCode session wake / ACP evidence. | **Not an authorized production Dev dispatch ACK** until seat, session, cost, ownership and durable outcome proven. Do not invoke `--resume -p` as a read-only test. |
| `studio/test/unit/pwa-work-ingress-reuse.test.mjs` | New ARBOR test added at `dfb5480b`; authored six source-level reducer tests covering idempotency, refusal, transitions and no effect. | Extend rather than duplicate; **authored ≠ executed**. No proof of PWA/Host admission or physical phone. |

## Required user-visible states and transition rules

`draft` → (explicit Preview request) → `preview` → (explicit distinct Submit Work action) → `admitting` → `queued/blocked`; optionally `active` only with an independently acknowledged worker claim, then `completed` with verifiable evidence; `cancelled` is terminal. `uncertain` is a **transport observation**, never a fabricated Work state or proof that an effect did not happen.

- Typing, IME composition, paste, clipboard, voice partial, autosuggest, an agent-inserted draft, viewport re-render or reconnection **must not submit**. Ask-AI-to-design and Create-Work are separate user choices; each requires its own explicit event.
- Host verifies current paired device/user scope, current source revision, permitted action and idempotency key; no browser-generated claim grants authority.
- Duplicate explicit tap must either return the same already-committed Work identity or refuse a changed-payload key. Never create two Work objects or two provider turns.
- Timeout **before** an acknowledged request may report definite failure only with evidence; lost response **after** possible commit is `uncertain`, status/readback required, not auto-resubmitted.
- Device revocation, changed actor, stale epoch/revision and host restart invalidate stale admission. No silent refresh of old tokens.
- Status/receipt projection must be Host-authenticated and source-backed on both GPUI and phone. User-attested evidence is not a verified developer execution result.
- Cancellation must use existing authorized status transitions; a cancelled item cannot re-awaken a worker. A queued Work record has `execution: not-dispatched` until an independently authorized dispatch/ACK path exists.
- The production feature must not consume a model just to classify a deterministic Build/Change command; if escalation is needed it is a **proposal only**, with explicit budget/provider permission.

## Ready Dev integration order (never leapfrog PY-002)

1. **PY-001 (this packet):** retain Build's two distinct choices. Specify explicit Submit Work admission and separately observable pending/uncertain/blocked UI. No product code claim in this packet.
2. **PY-002:** source-owner choice of a **Host-authorized durable Work writer**. Compare reuse of Studio CAS with incumbent Host journal/Control model. Record exact identity/grant/ownership and canonical persistence. Block if no producer.
3. **PY-003:** Host-bound idempotency, stale source/epoch and device revocation, no duplicate after reconnect. Refuse conflicting same-key payload.
4. **PY-004:** one existing LiveSystemState read projection for work status, with source digest/revision and actual GPUI/phone parity. Do not add another polling engine.
5. **PY-005 / PY-006:** optional actual Dev work dispatch only after separate owner-approved agent/session/channel and receipt semantics, with atomic OFFERED_TO → ACCEPTED_BY in existing `TASK-CLAIMS.md`. Default = MANUAL.
6. **PY-007:** release/source/phone acceptance follows existing [#31](https://github.com/owenservera/ARBOR/issues/31), never a new release workflow.

## Executable source/regression recipe

Current no-effect precursor (source checkout **must** be freshly pinned to a commit containing the new test):
```powershell
node --test --test-concurrency=1 studio/test/unit/pwa-work-ingress-reuse.test.mjs
node --test --test-concurrency=1 apps/mobile-relay/test/build-composer.test.mjs apps/mobile-relay/test/universal-control-chat.test.mjs
```
**Both existing browser test paths were independently confirmed in ARBOR's current test directory**; runner outcome still UNRUN in this packet. The first source test's six authored cases are currently **UNRUN in this packet**. No executable code was added here.

Dev must add an explicit Host/API/phone regression suite after choosing the writer; acceptance matrix (all **UNRUN** today):
1. draft/IME/multiline/voice partial does not submit
2. Preview alone does not create Work or ask provider
3. separate Submit Work explicit UI event creates exactly one admitted request
4. no paired device/direct connection → denied before effect
5. revoked grant/current actor mismatch → denied, no private Work projection
6. same key/same payload → stable ID, no second record
7. same key/different payload → explicit conflict
8. concurrent phone + GPUI submit → one winning CAS; loser sees honest conflict
9. lost POST response → uncertain status + readback; no automatic retry
10. API 429/503/offline/restart → no ghost Work or auto resume
11. editing draft after preview → invalidates approval source digest
12. Cancel queued/blocked → terminal and never auto dispatches
13. provider design route does not also enqueue Work
14. user-confirmed Work does not imply agent actually dispatched
15. Host revision + phone render + GPUI render + explicit source ACK are correlated on real devices

## Final acceptance and handoff

**PREPARED/OFFERED_TO Dev, not ACCEPTED_BY.** Dev must read newest `main`, claims, source files and release state; confirm no other active claim of `app.js`, `build-canvas.mjs`, Host Control or Studio Work before taking ownership. Use Rust for Host/semantic core where sensible; browser glue TS7+ only as needed. No direct source edit via Desktop Commander or ZCode. Preserve all user data, provider sessions and rollback.

**Evidence required:** `PY_ID | RECEIVER | SOURCE_SHA | CLAIM | HOST_WRITER | OWNER_GRANT | CASES_RUN | EFFECTS | PROVIDER_SENDS | WORK_ID | HOST_RECEIPT | PC_SOURCE_REV | PHONE_SOURCE_REV | DEVICE_ACK | BLOCKER`. Missing actual provider/phone/Dev ACK remains UNRUN.
