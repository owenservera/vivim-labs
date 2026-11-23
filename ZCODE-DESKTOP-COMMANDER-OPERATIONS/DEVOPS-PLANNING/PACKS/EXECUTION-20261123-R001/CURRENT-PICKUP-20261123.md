# Current-source pickup addendum — 2026-11-23 (planner reconciliation)

**Scope:** updates the existing R001 five-package intake. This is NOT another backlog, source owner, job runner, or claim. Canonical outstanding tasks and acceptance remain [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33), AI integration [#32](https://github.com/owenservera/ARBOR/issues/32) and release/parity [#31](https://github.com/owenservera/ARBOR/issues/31). Every recipient must fetch `main` and [TASK-CLAIMS.md](../../../TASK-CLAIMS.md) before editing or acknowledging scope.

## Source and exact evidence at this reconciliation

- Observed local ARBOR `main` clean and safely fast-forwarded to `0cd77a2185afe204e41e04fe3d3d588133a3eac6`; direct GitHub issue receipt confirms that exact head and source. Do not assume it remains newest.
- Independent native Windows tests at this source: `cargo test --manifest-path native/gpui-desktop/Cargo.toml --lib --offline --locked` **61/61 PASS**; `cargo check --manifest-path native/gpui-desktop/Cargo.toml --features desktop --offline --locked` **PASS**. These are tests/compile, not provider or phone delivery.
- Existing source-pinned no-effect observer: `local-logs/native-verifier/latest.json` `state=PASS`, `sourceSha=0cd77a2...`, hash-bound `nativeSha256=389C67DB9FA6913E1B257A5339846BCEA5BA96FB4429DC168059A0D2B33CEC14`, `releasePromoted=false`, `hostEffects=0`, `providerSends=0`, `sourceWrites=0`. Matching `vlang-iterations/latest.json` run `vlang-2026-11-23T19-13-20-587Z`: **22/22 VLANG, 160/160 SAFETY, 12/12 AUTOWIKI, 21/21 VISUAL = 215/215 PASS**.
- An earlier simultaneous four-lane run failed 3/215 at older SHA `84ca3c36` and a two-concurrent attempt later also failed. Individual failing source tests passed isolated, supporting a 1.2s native subprocess / Windows fanout contention hypothesis. Current Labs `scripts/vlang-parallel-cycle.ts` serializes **four intact scopes** with `maxConcurrentLanes=1`; no assertions dropped, no model routing rebuilt. Keep the earlier failures and complete future receipts; a single clean run does not prove arbitrary high fanout safe.
- Actual source-pinned GPUI was separately promoted and physically accepted at `0cd77a2` with native palette changes, Host-approved Dusk→Tide→Undo, and `wiki` producing three source-backed sections at revision 1. Previous local [live UI receipt](../../../local-logs/GPUI-LIVE-COMMAND-SITREP-20261123.md) distinguishes these from unverified Android PWA parity, provider delivery, full typography/layout customization and compound NL commands. GPUI PID or binary hash are snapshots, not future liveness guarantees.
- **Not verified:** a real fresh ChatGPT provider reply, single-send settled readback, native GPUI conversation continuity, Android phone rendering/ACK, bidirectional source-view sync, source-closed same-SHA Host/PWA/native release, six genuinely working ZCode model routes, GitHub Actions zero-job startup repair.

## Next execution decisions — existing packages, nonoverlapping file ownership

| Order | Package / canonical task | Primary owner | Next safe, source-bounded operation | Evidence for completion / stop boundary |
| --- | --- | --- | --- | --- |
| 1 | P0-AI-CHAT / A01-A03, #32 | **ChatGPT Dev** on GitHub ARBOR main | Read-only classify old ChatGPT `CREATION_UNPINNED/TYPING_UNCONFIRMED` key, current Studio session ownership and signed-in Chrome controller; inspect existing `studio/src/studio/relay-chat-startup-fence.mjs` and `studio/bin/relay-chat-mvp.mjs`. Pin worker and thread; do not replay old key | Pinned no-effect gate, exactly one fenced worker and source-readiness receipt. Old uncertain request never retried; busy session not disturbed |
| 2 | P0-AI-CHAT / A04-A08/A10/A12, #32 | **ChatGPT Dev**; DC only actual device evidence | On a **new explicitly owner-submitted** request, prove typed composer readback, one send, same-thread settled provider reply, Host ledger and native/phone projection using incumbent Studio→Host→GPUI/PWA | Separate PROVIDER_VERIFIED, HOST_RECEIPT, GPUI_RENDERED, PHONE_RENDERED states. No claim from HTTP 200, browser sign-in, or worker heartbeat |
| 3 | P0-RELEASE-SYNC / S02-S14, #31 | **ChatGPT Dev**, DC local verification | Use existing `tools/packaging/prepare-host-runtime.mjs` and single immutable SHA; close lock/runtime imports in isolation before reversible production promotion; maintain exact paired URL and live Host sessions | Same-SHA manifest/binary/source receipts; actual PC→phone and phone→PC revision/ack + rollback; Android remains UNRUN until observed |
| 4 | P1-GPUI / U01-U04/U09/G02-G05 | **ChatGPT Dev**; DC observer | Build on current deployed Host-owned native theme and Wiki recovery; test actual menu/keyboard/IME/review/cancel/edit fences, layout/density/typography options and gesture adapter parity. Preserve GPUI as the sole Windows product surface | Physical native interaction evidence vs source; phone parity separately. No duplicate TierFlow/PWA desktop or second Host controls |
| 5 | P1-VLANG-WIKI-ORACLE / U05-U08/O01-O06 | **ChatGPT Dev** integration; **Planning** only Labs prep; **Oracle** audits | Reuse existing deterministic NLCL/Host plans and Rust Wiki/HQKG. Examine source grant producer before accepting staged R003/R005/R006 Labs Rust precode. Bind real authorized ChatGPT conversation ingestion to existing Oracle seven passes and provenance/revisions | Complete grant/epoch fences, no private graph leaks, real authorized corpus + correction/delete receipts; never fabricate provider/semantic-verifier evidence |
| 6 | OPS-OBSERVERS / Z01-Z07/S12/R04 | **ZCode** test observer; **ChatGPT** source/CI owner; **DC** Windows evidence | Keep existing hourly `VIVIM-VLANG-Observer`, compare SHA + native hash + runner hash before crediting 215 checks; investigate Actions `startup_failure` with an actual allocated-job receipt; model scouts only via existing verified registry/cost admission | No effect, spend or source writes. Distinguish deterministic Windows test PASS from ZCode live model worker availability and actual screenshot grading |

## Planner-only work available without colliding with the Dev source owner

1. **PY-002 / PY-005:** identify the existing authenticated durable PWA build-request writer and *supported* coding agent dispatch/ack boundary. Oracle's PWA_BUILD_INGRESS signal is **AMENDED / mapped to PY-001–PY-007, not accepted as already implemented**; manual dispatch remains labeled MANUAL until proved. Do not create tasks from draft/IME/voice partials.
2. **AW-010 / AW-013:** repin the existing Host Wiki/HQKG read port and trusted actor/source-ref grant producer; test revoked/incomplete-scope refusals. Existing R003/R005/R006 offer is **PREPARED/OFFERED**, no Dev ACK or rust runtime proof of those Labs-only modules. No second graph, ACL or permission engine.
3. **VF-001 / VF-002:** keep the SOURCE→TEST→PACKAGE→SERVE→HOST_ACK→GPUI→PHONE ladder and screenshot/receipt correlation precise. The GPUI physical theme/Wiki pass applies to the exact 0cd SHA; it does not satisfy PC↔phone.
4. **PL-003:** source/collision refresh is already named in older AW-R003 preparation; do **not** claim it a second time without reconciling the existing sender-owned Labs task claim.

## Source-reuse discovery: existing Work and agent dispatch seams (same current ARBOR main)

**PY-002 is still a Host/PWA integration blocker, but the "no existing Work source" assumption is too broad.** Source inspected on ARBOR `main`:
- [`studio/src/agents/personal-agent-work.mjs`](https://github.com/owenservera/ARBOR/blob/main/studio/src/agents/personal-agent-work.mjs): existing local Work reducer, deterministic `work-<digest>` ID, input idempotency, queued/active/blocked/completed/cancelled transitions, evidence references and finite caps. It explicitly **never dispatches** tools or calls providers.
- [`studio/src/agents/personal-agent-service.mjs`](https://github.com/owenservera/ARBOR/blob/main/studio/src/agents/personal-agent-service.mjs): incumbent `VersionedStore` with `compareAndSave` CAS at `personal-agent/primary`, explicitly scoped to one local data directory/identity. This is **not** proven to be the owner-authorized Host durable task writer nor a cross-device identity.
- [`studio/src/studio/personal-agent-routes.mjs`](https://github.com/owenservera/ARBOR/blob/main/studio/src/studio/personal-agent-routes.mjs): local `GET/POST /personal-agent/work`, `GET /personal-agent/outcomes`, and explicit status/next-action transitions. Provider-reply work preview/apply requires an already-stored settled reply and human confirmation; it never sends a provider message. **Do not expose these Studio APIs directly to untrusted mobile callers** or mistake their existence for a completed PWA→Host admission path.
- [`hq/lib/channel-transport.mjs`](https://github.com/owenservera/ARBOR/blob/main/hq/lib/channel-transport.mjs): incumbent ZCode session wake/liveness integration exists, but the source is not an authorized current Dev-agent acceptance channel. Session IDs, seats, run receipts, deduplication/uncertainty and cost admission still need proof. Do not create another dispatcher or run `--resume` on a personal session as a read-only probe.
- [`studio/src/studio/chat-channel.mjs`](https://github.com/owenservera/ARBOR/blob/main/studio/src/studio/chat-channel.mjs): guarded provider-neutral channel has request ID/revocation logic, but its static `SERVED_SITES` currently covers ChatGPT, Claude and Gemini; other known scopes (including Qwen) are *reserved but not served by this channel*. Even these three are not proof of currently signed-in/settled-provider delivery.

**Planning directive:** map an *explicit, owner-authorized PWA submit* to the existing Host authority and suitable durable Work record only after identity/session/write ownership is demonstrated. Provide an idempotent request ID, status/readback, cancellation/deny/offline fencing, one Dev ACK. Treat Studio Work as a reusable candidate, not as an automatically promoted canonical database. Dev retains integration; Planning supplies the source seam and refusal tests.

### Next ten claimable Planning proposals (from existing 37-candidate catalogue; none automatically claimed)

| Rank | Existing ID | State before checkout | Precise preparation / dependency |
| --- | --- | --- | --- |
| 1 | PY-001 | SCOPED | Explicit PWA Build/Change action; drafts/IME/speech remain inert |
| 2 | PY-002 | BLOCKED_SOURCE | Choose the authoritative Host-owned work writer; assess Studio Work candidate and avoid duplicate durable stores |
| 3 | PY-005 | SCOPED | Supported Dev dispatch/ACK vs manual bridge; inspect existing ZCode wake without executing it |
| 4 | PY-003 | SCOPED | Cross-device idempotency/deny/offline/cancel policy once writer known |
| 5 | PY-004 | SCOPED | Revision-accurate PWA work status readback using existing LiveSystemState |
| 6 | PY-006 | SCOPED | Planning OFFERED_TO → Dev ACCEPTED_BY atomic claim/ack contract |
| 7 | PY-007 | SCOPED | Source→test→package→serve→phone release receipt chain |
| 8 | VF-001 | SCOPED | Separate evidence gates (SOURCE/TEST/PACKAGE/SERVE/HOST_ACK/GPUI/PHONE) |
| 9 | VF-002 | SCOPED | Actual screenshot, source revision and Host receipt correlation |
| 10 | VF-004 | SCOPED | Revoked grant, stale epoch, IME, human-agent race and no-send negative cases |

These are **Planning preparation candidates**, not independent ARBOR implementation claims; any unmet dependency stays blocked. The existing R003/R005/R006 OFFERED_TO sender-owned claims stay untouched, and Dev-owned #32/#31 remain the live product priority.

## Pickup/receipt contract

Before any file edit: fetch current remote `main` / file blob SHA and the existing claim; record a disjoint file scope. ChatGPT alone edits ARBOR application source, Rust by default and TS 7+ for browser glue; ZCode/desktop do source-pinned read-only/local verification. No provider API, Playwright/Puppeteer, second browser owner, user-data scraping without authorization, retries of uncertain sends, model paid fallback, or uncontrolled ZCode `arbor-life` workflow.

`TASK_ID | EXECUTOR | CLAIM_STATE | ARBOR_SHA | LABS_SHA | FILE_SCOPE | TEST_COMMAND | RUN_ID | PASS/FAIL/UNRUN | HASHES | HOST_EFFECTS | DEVICE_RECEIPT | BLOCKER | NEXT_ACTION`

**Delivery semantics:** This committed file plus a #33 comment is a shared intake *for agents that read it*. It is not automatic cross-session delivery or an assertion that a parallel agent has started or accepted work. Sender remains accountable until explicit receiver ACK in the existing claim surface.
