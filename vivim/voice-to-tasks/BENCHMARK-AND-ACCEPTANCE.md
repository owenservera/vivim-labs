# Experiment protocol and acceptance gates — voice→tasks Day-0

**State:** READY FOR MEASUREMENT; NO TESTS EXECUTED; no Android/Windows PASS. Requirement authority: [Repos HQ issue #11](https://github.com/owenservera/repos-hq/issues/11). These numbers are **engineering targets to validate**, not recorded measurements.

## A. Read-only prerequisite map — do not spawn another subsystem
- Discover the **existing** Android/desktop record and import handlers, source-file lifecycle, pending drafts, authenticated phone relay C43, Host binary upload/claim/receipt and task/command persistence. Record exact SHA/paths; do not copy private contents to public Labs.
- Determine whether current Host has task object schema, task mutation command, owner grants, compare-and-swap and idempotency, current device projection and deletion propagation. Missing ≠ permission for new datastore. Align to existing repos-hq graph identity/revision and TierFlow view contracts only where appropriate.
- Confirm installed engines and model assets already present; test with **already licensed** binaries before requesting any new download, model bundling, provider consent, or install.
- Verify clock/timezone and Android speech-service availability on target **physical** phone. Desktop baseline machine/OS and active runtime versions must be read from the device, not inferred from source or memory.

## B. Candidate ordering; isolate one axis per test
**STT:** (1) Android on-device speech recognizer as separate live route; (2) Moonshine Tiny Streaming en/es on Host; (3) whisper.cpp tiny/base multilingual on Host, then smaller quantizations; (4) sherpa-onnx if needed; (5) phone offline model pack only after baseline. M4A decoder required; no hidden cloud.
**Text extraction:** (1) deterministic segmentation, date-phrase grounding; (2) GLiNER2.5-small en; (3) NuExtract-1.5-tiny en/es; (4) GLiNER2.5-multi ca/es; (5) Qwen3 0.6B Q4 local Host if smaller approaches miss quality. Benchmark exactly one at a time and note model SHA/license.
**Run at least:** warm and cold, 10/30/60/180 s audio, quiet/noisy and mic distance, en-GB/en-US/es-ES/ca-ES + mixed-language; an empty non-task memo; negation; ambiguous dates; multiple actions; repeated retransmit.

## C. Corpus and measurement
Keep private owner recordings out of public GitHub. Public synthetic/consented fixtures or generated speech in a test-only private corpus are fine **only with provenance and licensing**. Prepare at least 100 short utterances per selected language for a meaningful initial comparison; include 30+ multi-action, 20+ ambiguous/negation/non-task, 20+ relative-time/DST and 20+ interruptions/error cases per aggregate suite. Mark sample counts and demographics/distribution; do not publish one number implying universality.
- STT: WER and critical-token error (dates, negations, person, place, priority); **not** WER alone.
- Task: span-attributed action precision/recall/F1, **zero fabricated tasks** target on no-task set, deadline intent accuracy, date grounding accuracy, unsupported/ambiguous rate, review-edit rate.
- End-to-end: stop-to-draft p50/p95, review-to-commit p50/p95, cold-start model initialization, peak/steady RSS, CPU time, battery drain, APK/Host install delta, model downloaded bytes, storage growth, network bytes and replay/duplicate count.
- Device/OS: actual Android phone make/model, ABI, API, total/free RAM, available storage, audio backend and thermal class; Windows Host CPU/RAM/OS and exact source/app build SHA.
- Repetitions: >=30 executions per meaningful config after warm-up; publish median/p95 with confidence or observed range and failure denominator. Randomize order to reduce thermal/cache bias.

### Preliminary budgets (to accept or adjust after measured baseline)
| Metric | Proposed target | Rationale/status |
|---|---|---|
| Phone base model payload | **0 MiB mandatory** | Architectural target; only existing UI/recorder/transport diffs |
| Default phone extra idle RAM | **< 20 MiB incremental** | Target, not current runtime measurement |
| Default CPU while idle | **~0%** | No continuous capture/VAD/LLM |
| 30 s saved-audio payload | **≈0.24 MB at 64 kbps** | Compression math only, container/metadata extra |
| 30 s note stop→draft on paired Host | **p95 < 8 s** | Candidate user-experience acceptance budget |
| Captured-note loss after process kill | **0** | Require persisted recovery proof |
| Duplicate approved tasks after 3 retries | **0** | Host idempotency acceptance |
| Unapproved task mutations | **0** | Security invariant |
| Ambiguous/unspoken due dates committed | **0** | Deterministic date + approval invariant |
| Phone model download in lean profile | **0 bytes** | Architecture target; only when offline add-on approved |
| APK increase | **< 2 MiB** | Aspirational; reusing existing native recorder should help |

These are intentionally **not** a claim of benchmark completion or hard model-accuracy numbers. The product release decision should prefer a trustworthy measured 8-sec pathway over a faster false-date path.

## D. Release journey fixtures

### D1 — desktop with partial approval
Record/import: “Email the supplier tomorrow morning; send the invoice to María by Friday; check the pump filter sometime next week. Also I enjoyed the beach.”
- Ground exactly three proposed actions and no beach task.
- Present original text/source spans. Time phrases remain intent until deterministic resolution with locale, timezone and reference instant; ask about ambiguity. Approve 2, reject 1, restart app; only 2 canonical tasks persist.
- Confirm no calendar/email send, no external effects.

### D2 — physical phone
- From a real Android build, owner grants mic; record 20–30 sec with 2–3 action items; stop; prove durable local file hash and queue. Pair using existing authorized C43 route; Host verifies owner grant, transcribes and returns proposals; approve/correct on phone; reopen on both surfaces.
- Denied mic permission, offline Host, revocation while processing, killed app after recording, lost ACK after commit, duplicate retry and device re-pair must not silently lose a memo or duplicate tasks.
- If offline intelligence pack absent, visibly show `Waiting for paired Host` (not `Processed`). That is a **partial/offline degradation**, not full offline inference proof.

### D3 — time, identity and provenance
- Test Europe/Madrid DST spring/fall dates, “next Friday”, “tomorrow morning”, no deadline, ambiguous “Monday”, Spanish “pasado mañana”, Catalan “demà”; offset-aware reference instant fixed for each test.
- Same note re-uploaded by another paired device cannot claim owner identity. Same `noteId` and same approved revision repeated 3+ times yield exactly one Host receipt/task set. Revoking scope between proposal and approval refuses commit.

### D4 — transcript critical errors
- ASR mishears “don't order” as “order”; threshold/uncertainty highlights review and never auto-commits.
- Mixed English/Spanish/Catalan, names and numbers; unsupported locale is explicit, no fabricated translated task. Audio should remain recoverable with owner consent.

## E. Measurement work orders (lease-safe)
1. **Lane R, read-only:** map exact current file paths and contract schemas for record→intake→propose→approve→task commit→device projection. Return source SHA and `EXISTS | PARTIAL | MISSING | UNKNOWN` per step.
2. **Lane S, isolated benchmarks:** benchmark on hardware outside app effect path; record OS/device/CPU, model hashes/licensing, package delta, RSS/latency/WER and 100-sample task F1. No provider sends, no graph/task writes.
3. **Lane I, bounded integration proposal:** only after R/S evidence, select smallest patch in the current ownership seams. Single Host writer, exact scopes, retries and rollback. **Do not develop or ship from this public Labs folder.**
4. **Lane V, independent verifier:** replay negatives and on-device checks, link raw redacted evidence **in approved private location**. Mark Android PASS only for the actual physical build and exact flow.

## F. Decision rule
Choose the *lowest total installed bytes + runtime memory* configuration that meets task accuracy, critical-token safety, supported languages, privacy, authorization and stop→review latency. If a smaller model violates quality or privacy, prefer a larger **Host-only** or explicit human-correction workflow. A model compressed into 50 MB but delivering wrong deadlines is not performant for this product.

**Promotion stop:** Host auth/send-once or Android C43 gate unverified; ambiguous task write ownership; out-of-scope private material; no hardware tests; wrong language license; background audio without consent; silent auto-commit; missing original recording recovery.

## Result format
```
date | sourceSHA | device+OS | buildSHA | inputFixtureHash | lang | ASR/modelSHA |
extractor/modelSHA | APK_delta | downloaded_bytes | RSS_peak | p50/p95_stop_to_draft |
WER | critical_token_error | task_P/R/F1 | due_date_accuracy | task_duplicates |
source/receipt_verified | status PASS/FAIL/BLOCKED/UNRUN | rollback
```

### Research sources
Android [SpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer) and [RecognizerIntent](https://developer.android.com/reference/android/speech/RecognizerIntent); Moonshine [models](https://moonshine-voice.readthedocs.io/en/latest/models/available-models/), [accuracy](https://moonshine-voice.readthedocs.io/en/stable/models/accuracy/), [license](https://moonshine-voice.readthedocs.io/en/stable/license/); [whisper.cpp](https://github.com/ggml-org/whisper.cpp); [GLiNER2.5](https://github.com/fastino-ai/GLiNER2); [NuExtract](https://huggingface.co/numind/NuExtract-1.5-tiny); [Catalan comparison](https://models.handy.computer/languages/ca/).
