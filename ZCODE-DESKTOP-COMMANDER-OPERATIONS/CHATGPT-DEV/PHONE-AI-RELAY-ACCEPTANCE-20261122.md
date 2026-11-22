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
