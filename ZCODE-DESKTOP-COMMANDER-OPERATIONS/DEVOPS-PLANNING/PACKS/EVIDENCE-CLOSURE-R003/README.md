# VF-001 / VF-002 — exact-source evidence closure and two-device acceptance pack R003

**Date:** 2026-11-23 · **Role:** Planning pre-integration only · **Status:** source code + deterministic tests authored, physical devices unverified by this pack.

**Canonical master:** [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33). Existing delivery issues [#32 AI](https://github.com/owenservera/ARBOR/issues/32) and [#31 runtime/phone](https://github.com/owenservera/ARBOR/issues/31). This packet is a reusable reviewer of proof, **not** another task ledger, executable effect engine, deployment script, screenshot recorder or agent dispatcher. It neither replaces the existing source-pinned VLANG observer nor changes its schedule.

## Source and verified work

- At creation, GitHub ARBOR `main` reached [`dfb5480b`](https://github.com/owenservera/ARBOR/commit/dfb5480b17f5daa8861352e0cb81dd97e01e9f9d); that commit adds 6/6 Windows-passed pure Work reducer tests, **not** mobile PWA ingress.
- The already-installed `VIVIM-VLANG-Observer` had a genuine isolated Windows regression PASS at exact **older** `f5623943b746daac3d55a0ae4896430083a568ba` with 215/215 checks, run ID `vlang-2026-11-23T19-24-07-866Z`, native SHA256 `389C67DB9FA6913E1B257A5339846BCEA5BA96FB4429DC168059A0D2B33CEC14`, runner SHA256 `5D929411261DB2D365F422AE80E5F2E13848F24130000F351F8277F1C569D6CB`, zero source writes, Host effects or provider sends; no release promotion. It is NOT a 215-case PASS for subsequent `dfb5480b` or later.
- Local Studio read-only `probeStudioRelayReadiness` returned `ready:true`, `reason:STUDIO_OWNER_IDLE`, permitted route `chatgpt` on this inspection. The script exited with a nonzero code because it intentionally maps ready to nonzero; do not label this a provider send, settled reply, active worker guarantee or phone reply.
- R001/R002 cross-agent intake already exists at [execution pack](../EXECUTION-20261123-R001/README.md) and [wiring delta](../EXECUTION-20261123-R001/WIRING-DELTA-R002.md). **No duplicate source owners or new model router.**

## The narrow addition

- [`receipt-gate.ts`](receipt-gate.ts) is an offline, side-effect-free **classification** function over caller-supplied receipts. It validates internal type/identity consistency for eight evidence gates: SOURCE_PINNED → SOURCE_TESTED → PACKAGED → LIVE_SERVED → HOST_ACK_VERIFIED → GPUI_RENDERED → PHONE_RENDERED → CROSS_DEVICE_VERIFIED. Only the requested gate supported by supplied fields is admitted.
- [`receipt-gate.test.mjs`](receipt-gate.test.mjs) contains six deterministic, nonlive unit cases (including one fake/simulated phone that must be rejected). Fake test inputs are *tests of the evidence checker*, never real phone evidence.
- [`observed-verifier-20261123.json`](observed-verifier-20261123.json) is a bounded, public-safe input transcribed from the genuine existing local observer. It claims only `SOURCE_TESTED`, and deliberately sets `currentMainSha` to the newer ARBOR head to surface `currentHeadMatches:false`.
- This validator **cannot cryptographically authenticate an external source, screenshot pixels, a provider reply or the owner of the Android phone**. Independent source, device and Host receipts must be gathered by the designated agents before constructing stronger records. JSON shapes are not grants.

## Execute — no product effects

```powershell
$root='C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS\DEVOPS-PLANNING\PACKS\EVIDENCE-CLOSURE-R003'
node --experimental-strip-types --test "$root\receipt-gate.test.mjs"
node --experimental-strip-types "$root\receipt-gate.ts" "$root\observed-verifier-20261123.json"
```

Expected from the observed record: `highest=SOURCE_TESTED`, `state=EVIDENCE_ADMITTED`, `currentHeadMatches=false`, `PACKAGED=false`, `PHONE_RENDERED=false`. CLI exit codes: 0 when the requested level is structurally supported, 3 for unsupported claim, 2 for malformed/unreadable input. It does not create, modify or upload any application file. Never run real provider or phone actions simply to satisfy an offline validator.

## Agent-specific work packets (use existing claim registry)

| Existing task | Source owner | Safe next action | Required real evidence |
| --- | --- | --- | --- |
| Z01 / Z07 | ZCode deterministic observer, Desktop Commander local verification | Re-run existing no-effect `VIVIM-VLANG-Observer` at *actual current* source SHA; do not spawn another daemon | Matching `native-verifier/latest.json`, `vlang-iterations/latest.json`, full exact 40-hex SHA and hashes, zero effects |
| R05 / R06 / VF-001 | Planner + Oracle audit | Compare new source/test/package/serve/Host/device receipts to this monotonic ladder; never automatically check #33 | Link original source receipts, dates, hashes and separate status per gate |
| S02–S06 / #31 | ChatGPT Dev source, Desktop Commander post-push operations | Build single-SHA immutable candidate and reversible isolated serve; preserve phone URL | Locked source artifact + manifest/native hashes + served runtime identity |
| S07–S14 / #31 | ChatGPT Dev + authorized physical device verification | Test PC→phone and phone→PC revisioned changes independently | Host region epoch/source/view revision, actual GPUI and Android render/ACK, two independent screenshot hashes and ack IDs |
| A01–A08 / #32 | ChatGPT Dev and existing Studio browser owner | Preserve historical uncertain turn; verify current Studio owner, then only a freshly user-submitted message | One send and settled same-thread reply + Host receipt; device render separately |
| PY-001/PY-002 | **Other current Planning executor owns PY-001** | Coordinate with existing [TASK-CLAIMS](../../TASK-CLAIMS.md); reuse `studio/src/agents/personal-agent-work.mjs` | Explicit owner submission + current Host authorized durable writer + CAS + real Dev ACK; local Studio Work alone is not that proof |

## Negative evidence that MUST remain BLOCKED
- Browser account signed in, HTTP 200, Studio idle or worker heartbeat without an actual settled provider reply.
- Desktop screenshot alone as proof that a phone rendered the same state.
- Synthetic DOM/phone tests, screenshots with `simulated:true`, or test artifacts with no source SHA and Host receipt.
- Source tests on previous SHA treated as a newer head's PASS.
- Source proof from an incomplete runner, unknown action authority, a mismatched package manifest, a stale Host epoch, or duplicated correlation ID.
- A GitHub handoff comment treated as an accepted Dev task or running ZCode agent.
- Unsafe retry of old ChatGPT `CREATION_UNPINNED / TYPING_UNCONFIRMED`, model paid fallback, uncontrolled browser recording.

## Pickup / transfer
Before using the pack: fetch current ARBOR/Labs `main`, [TASK-CLAIMS.md](../../TASK-CLAIMS.md), the current owner-issued ticket and its authoritative source. A receiver may **explicitly** `ACCEPTED_BY` this Planning deliverable through blob-SHA CAS; until then Planning retains sender ownership. Never auto-assign another agent or mark phone acceptance merely because these files exist.

**Receipt format:** `TASK_ID | EXECUTOR | ARBOR_SHA | LABS_SHA | STAGE | COMMAND/TEST_RUN | PASS/FAIL/UNRUN | BINARY/MANIFEST_HASH | HOST_RECEIPT | GPUI_EVIDENCE | ANDROID_EVIDENCE | BLOCKER | NEXT`.

**Next source engineering target:** an explicit PWA Build-submit contract through incumbent Host authority and existing Work reducer (no extra database); currently claimed by another Planning lane under PY-001. This VF packet is intentionally separate and can be validated independently.
