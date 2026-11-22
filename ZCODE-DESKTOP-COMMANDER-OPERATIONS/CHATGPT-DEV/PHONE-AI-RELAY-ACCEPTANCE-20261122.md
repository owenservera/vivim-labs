# VIVIM phone ↔ signed-in AI — ChatGPT Dev / ZCode acceptance

**Date:** 2026-11-22 (Europe/Madrid)  
**Canonical implementation evidence:** [ARBOR — Phone signed-in AI relay](https://github.com/owenservera/ARBOR/blob/main/docs/integration/PHONE-SIGNED-IN-AI-RELAY-20261122.md).  
**Deployed PC Host:** ARBOR `10006454b433efd53affc74e5e7a3d6518f96432`; pinned rollback-protected deployment passed; mobile PWA URL unchanged.

**Completed:** unified first-contact composer and source-backed signed-in account handoff; per-provider validation; draft preservation; no silent conversation retarget; exact current worker-capability heartbeat; phone send gate rejects a missing, stale, unsupported or incompatible consumer. Focused tests and full Node PWA/Host suite passed. Studio and production Host both healthy at final checks.

**Observed current account readiness:** ChatGPT, Gemini and Qwen ready; Claude blocked by preserved existing draft; Z.ai and DeepSeek blocked by changed browser binding. These are **account-readiness observations only**, not message delivery.

**Blocking evidence:** exactly one new ChatGPT Studio HTTP submit attempt became `CREATION_UNPINNED` / `uncertain`, without a verified reply. Do not retry or report it sent. No active `chatWorker` heartbeat; queue inventory harness `queued=0, claimed=0`. The current Studio-mode chat relay still supports default ChatGPT text only. Physical phone → provider → verified phone reply remains **UNRUN**. See the ARBOR canonical source for the detailed proof and acceptance criteria.

**Next tracked slices (source owner: ChatGPT Dev, on ARBOR main):**
1. AIR-01: repair/reuse **one Studio-owned** reply transport with pre-effect receipts and new-thread support; never add a competing Chrome controller.
2. AIR-02: prove adapter-specific account send/read for ChatGPT → Gemini → Qwen, then resolve draft/binding gates for the rest; update actual worker capabilities only from evidence.
3. AIR-03: run one explicitly initiated physical phone roundtrip and capture single-command, settled-reply, render-ack, offline/revocation/uncertain outcomes.
4. AIR-04: recover persistent Host/Studio/worker startup without duplicate writers or blind replays.
5. AIR-05: separately close the private-sharing Rust package report `SHARE_RUST_NOT_PACKAGED`.

**Ownership:** ZCode can research, iterate and challenge evidence/test contracts. Desktop Commander may inspect and run local acceptance/deploy; it is not the primary source code editor. ChatGPT Dev controls ARBOR GitHub `main`. No provider APIs, no fabricated send evidence, no message-body copies in the tracker.

**Final acceptance amendment (same day):** The server now refuses stale/unsupported phone AI queue submissions in addition to the phone-side guard. Focused and full PWA/Host tests passed at `10006454`, rollback-protected cutover succeeded on the unchanged URL, and Host/Studio were healthy afterward. ChatGPT reply remains uncertain and no phone delivery is claimed. See ARBOR canonical record for the precise HTTP/heartbeat contract.

### AIR-01 typing readback R002 — 2026-11-22
The original one-time ChatGPT Studio request is **not sent/verified**: persisted `creationFailureDetail` identifies `type-step / TYPING_UNCONFIRMED`; the turn is `CREATION_UNPINNED / uncertain` and must never be retried. The source now aligns `turnObservationExpression` with the visible focused/remembered composer, and returns no acceptable draft for ambiguous editor choices. The failure journal records only count/boolean diagnostics, not typed message text. Commits: ARBOR `e64cc59c5623`, `d365723456b5`.

**Validated in an isolated checkout:** five focused DOM selection tests green; full PWA/Host suite passed at the initial Studio patch; D05 packaging regression fixed by separate canonical ARBOR agent, not reimplemented here. Four broader Studio unit failures were reproduced on the prior `10006454` baseline. This is a **source fix**, not an asserted live ChatGPT or physical phone roundtrip.

**Live owner still busy:** Studio reports a Z.ai conversation `busy`, capture `recording`. Do not restart/force-close that process, dispatch a second turn, or run a competing browser controller. The current production Host SHA remains `10006454b433efd53affc74e5e7a3d6518f96432`; new Studio code has not been activated in that live process. Canonical evidence: [ARBOR phone AI relay acceptance](https://github.com/owenservera/ARBOR/blob/main/docs/integration/PHONE-SIGNED-IN-AI-RELAY-20261122.md).

**Combined-main R002 verification:** source checkout `c67894f26c757c16881aab6ac2789cf293111d76` passed Studio-focused editor-selection tests, the corrected D05 packaging suite, and full PWA/Host/HQ integration. Still not a live Studio deployment: the owner was busy and a later read timed out. Production Host stays at `10006454...`. No new model send/phone acknowledgement was asserted. Continue with AIR-01 browser-owner reconciliation and end-to-end acceptance only when verified safe.
