# VIVIM / ARBOR — Five Parallel Lanes (R001)

A ready-to-run **five-lane operational handoff** using existing `ARBOR/.zcode/workflows` and canonical issue/claims authority. The five lanes are organizational workspaces and starting prompts, not new ZCode DWF registrations or automatic provider/model sends.

**Directory:** `ZCODE-DESKTOP-COMMANDER-OPERATIONS/FIVE-LANE-OPERATIONS-R001` in `owenservera/vivim-labs`.

| Lane | Mission | Existing workflow references | First state |
|---|---|---|---|
| [L01 Signed-in AI Conversations](01-chatgpt-provider/START-PROMPT.md) | A01–A08 first; A09–A16 only after ChatGPT is provider-verified. | `arbor-area-c.dwf.ts`, `arbor-chatgpt-bridge.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L02 GPUI + Android PWA Convergence](02-gpui-mobile-surface/START-PROMPT.md) | S02–S14 release/device; native menu, IME and GPUI parity after safe packaging. | `arbor-area-d.dwf.ts`, `arbor-area-e.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L03 Authorized Work + Oracle AutoWiki](03-work-oracle-autowiki/START-PROMPT.md) | PY-001/U07 before O01–O06; do not count staged Labs Rust code as integrated. | `arbor-area-b.dwf.ts`, `arbor-oracle-corpus-verify.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L04 VLANG + Visual Feedback + Gestures](04-vlang-visual-gestures/START-PROMPT.md) | U06/U07/Z01 failure isolation first, then U08/U09/G02-G05. | `arbor-vlang-iterative-observers.dwf.ts`, `arbor-product-verification-parallel.dwf.ts` | `READY_NOT_LAUNCHED` |
| [L05 QA, ZCode Admission + Release Evidence](05-verification-models-release/START-PROMPT.md) | Z01/Z02/Z03/Z07 and R04 first; other R/Z tasks once blockers close. | `arbor-product-verification-parallel.dwf.ts`, `arbor-area-a.dwf.ts` | `READY_NOT_LAUNCHED` |

## Quick start

1. Read [AGENTS.md](AGENTS.md), [DIRECTIVE.md](DIRECTIVE.md), [LANE-BOARD.md](LANE-BOARD.md), and up-to-date GitHub #33/#32/#31 + Labs TASK-CLAIMS. The root `AGENTS.md` is a local lane contract, not a grant to run product effects.
2. Start **five independent ChatGPT/ZCode planning sessions** only through supported session tools. For each, copy the corresponding `START-PROMPT.md` into that lane's session. Start no more than two **model-driven** read-only ZCode workers until verified runId/liveness/quota/cost.
3. Use incumbent `ARBOR/.zcode/workflows` by reference; inspect their current implementations and permissions before invoking them. Do **not** run `arbor-life` or enable new always-on execution. An independent ChatGPT session can follow a lane prompt without any ZCode workflow execution.
4. Every session produces its own handoff using `HANDOFF.md`. One primary Dev source writer applies claimed ARBOR code; other lanes can verify/prepare without source conflict.
5. Consolidate completed receipts into existing [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33); do not create a second task ledger. The existing hourly ChatGPT continuous-dev automation remains the only ChatGPT scheduler.

**Validation:** `node --experimental-strip-types FIVE-LANE-OPERATIONS-R001/validate.ts` when run from the parent operations folder (Node 24+). It checks only the local package structure and source references; it does *not* start sessions or prove model execution.

**Current baseline:** master #33 recorded 7/71 verified, 64 open; source-level ChatGPT/Studio tests and prior native verifier are distinct; no genuine provider/phone reply or five live model workers established by this folder package. Refresh the evidence at run time.
