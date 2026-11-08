# Comparative architecture research — maximum utility per shipped byte

Research date: **2026-11-08**. Product requirement: [Repos HQ issue #11](https://github.com/owenservera/repos-hq/issues/11). All quoted accuracy figures are **the original publishers' dataset numbers**, not VIVIM measurements, and MUST NOT be compared as if benchmarked on the same corpus, CPU or language. Model sizes vary with conversion and release; check exact artifact hash/license before download.

## Executive decision: minimize the mandatory install, not just model parameters

**Rank 1, recommended for release prototyping:** thin phone + existing paired Host processing + explicit offline queue. Reuse Android native recorder and existing storage/sync/approval transports; ship **0 MB of mandatory ASR or LLM weights in the APK**. One Host-side, lazy-loaded, per-language compact ASR pack transcribes the saved audio. Compare CPU extractors; run deterministic validators and owner review. Phone can capture while disconnected, but **automatic conversion is deferred until a capable, authorized engine is available**. A phone-only offline task-generation guarantee would require a separately approved optional local speech and extraction pack.

**Rank 2, supplementary fast-path:** Android built-in on-device SpeechRecognizer for live dictation, only after feature + installed-locale tests; no app-bundled STT model. Cannot be assumed universally present or to accept saved M4A. Do not silently fall back to potentially networked generic `createSpeechRecognizer`: only `createOnDeviceSpeechRecognizer` where verified.

**Rank 3, premium stand-alone offline mode (opt-in):** downloadable phone speech pack plus small extraction model, with device RAM/battery limits. This costs tens to hundreds of MB or more and is **not a Day-0 default dependency**; benchmark before choosing. A separate 0.5B local text model alone consumes around 0.45 GB of Q4 weights.

## Existing VIVIM integration evidence and gaps
- The Android `PocketRecorder` in Web Harness already provides foreground-only app-private AAC/M4A capture, 16 kHz, 64 kbit/s, max 10 minutes and 25 MiB hard file bound, recovery of uncommitted audio, and microphone permission refusal. Raw audio recording is NOT identical to a successful `PocketStore` commit or Host delivery. Existing source: `studio/android/src/inc/vivim/app/PocketRecorder.java`.
- At **64 kbps**, theoretical mono audio payload is ~0.48 MB/min (30 sec ≈ 0.24 MB) before container overhead; this is an arithmetic estimate, not a recorded benchmark. The 10-min configured maximum is ordinarily ~4.8 MB of payload, far under the safety byte cap; use actual sampled output for sizing. Do not assume M4A can be handed directly to a PCM-only ASR; decoding/resampling costs and permitted codecs must be tested.
- Android native `MainActivity` contains PocketRecorder integration and local permission lifecycle. Existing extension-point inventory identifies Android voice capture, phone transport, Host actions and desktop voice as seams, but an inventory row is not implemented output.
- Host authorizes mutations; duplicate task/SQLite store would violate the product architecture. Locate the **actual current owner task collection and command** before coding: whether the canonical task object exists, ACL/write route, CAS revision, replay journal, provenance, and sync surface are **UNVERIFIED**. If none exists, surface a precise Day-0 gap for the existing owning team, not a Labs-built alternate store.
- Android and desktop physical E2E acceptance remain **UNRUN**.

## Pipeline options by capability and smallest-footprint strategy

| Stage | First choice | Fallback / research challenger | Why not load by default? |
|---|---|---|---|
| Audio capture | Existing Android PocketRecorder; current Studio/Host record/import UI | Browser `MediaRecorder` only where already owned | A second recorder introduces audio-loss/privacy paths |
| Storage | Existing app-private pending note + authorized Host ingest and provenance | Existing offline draft queue, if proven | No second store/sync service |
| Speech-to-text | Verified Android on-device recognizer for *live* mode; compact on-Host ASR for **recorded M4A** | Moonshine Tiny Streaming en/es; multilingual whisper.cpp tiny/base/quant for ca | Language packs only when required, never bundled per APK |
| Voice activity detection | Record on explicit press/stop; batch files don't require live VAD | ASR-native VAD/segmenter for streaming long audio | An extra constantly active VAD network/model is wasteful |
| Task extraction | Rules + source-span-grounded extraction with measured small Host model where required | GLiNER2.5-small/multi, NuExtract-tiny, Qwen3 0.6B Q4 as baseline | Task semantics, not benchmark rhetoric, determine model |
| Dates | Deterministic locale/timezone reference + DST tests | Present ambiguity for owner resolution | Generative due-date guesses are an integrity defect |
| Commit/sync | Existing Host typed intent, approval, CAS/lease, idempotent receipt | None until authority proven | Replays must not create duplicates |
| UI | Existing phone and desktop surfaces; review-only proposal cards | Read-only TierFlow projection when Gate 5 passes | Don't block on a new publisher/renderer |

### Speech recognition comparison
1. **Android framework recognizer — extra shipped model weights: 0.** Android 12/API 31 introduced `isOnDeviceRecognitionAvailable` and `createOnDeviceSpeechRecognizer`. Availability differs by device; separately query language support, model availability and permission behavior. The API does not promise that phone's audio-file transcoding is available to it. Android 13/API 33 `EXTRA_AUDIO_SOURCE` is optional, may be ignored. Test actual phone locale and session first. Sources: [SpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer), [RecognizerIntent](https://developer.android.com/reference/android/speech/RecognizerIntent).
2. **Moonshine Voice Tiny Streaming — 34M parameters/language, MIT code + all streaming STT checkpoints.** Published quantized streaming Spanish Tiny WER **6.2%** and Small **4.9%** on a 400-sample Common Voice/FLEURS/MLS panel, not a comparison to English LibriSpeech. English Tiny Streaming **12.00%** eight-dataset leaderboard average, not the same test. Memory-mappable 8-bit ORT files, native Android/Windows. Exact disk/RSS/warm-start per configuration are **UNMEASURED for VIVIM**. Framework's default download may select a *larger* model; specify architecture, controlled downloaded assets/hashes, consent and Wi-Fi policy. **No Catalan streaming model listed**; test alternate pack. Sources: [model catalogue](https://moonshine-voice.readthedocs.io/en/latest/models/available-models/), [license](https://moonshine-voice.readthedocs.io/en/stable/license/), [download behavior](https://moonshine-voice.readthedocs.io/en/latest/using/downloading-models/), [accuracy methods](https://moonshine-voice.readthedocs.io/en/stable/models/accuracy/).
3. **whisper.cpp — multilingual, no provider APIs, broad cross-platform C/C++, CPU/NEON/SIMD/Vulkan etc.** Published unquantized model Disk / Memory: tiny **75 MiB / ~273 MB**, base **142 MiB / ~388 MB**, small **466 MiB / ~852 MB**. Integer quantization reduces space, but quality/runtime after quantization require evaluation. Catalan is supported by Whisper multilingual, but low-capacity accuracy is problematic: independent *Handy* FLEURS ca table reports Whisper tiny **46.33%** WER, base **29.48%**, small **14.19%** on its own hardware/pipeline. This makes tiny/base risky for accurate Mallorquí/Catalan task deadlines. Sources: [whisper.cpp](https://github.com/ggml-org/whisper.cpp), [OpenAI Whisper languages/models](https://github.com/openai/whisper), [Catalan FLEURS comparison](https://models.handy.computer/languages/ca/). The Handy figures are **not** a VIVIM performance result.
4. **sherpa-onnx — mature cross-platform streaming/offline adapter ecosystem, Apache-2.0 engine; model licenses differ.** Small Zipformer options include 20M English and multi-language alternatives, but don't assume English-only checkpoints can handle Spanish/Catalan. Good third candidate if Moonshine native integration or licensing/network-download footprint underperforms. Source: [Android ASR catalogue](https://k2-fsa.github.io/sherpa/onnx/android/apk.html).
5. **Larger ASR** (Whisper small/medium/large; Moonshine Medium) can help accuracy but are **not** base-install choices. Their model/memory/load costs need a documented exception against user quality.

### Semantic extraction comparison
A transcript is not a task list. Segmentation must distinguish instructions, tentative ideas, negation ("don't book"), references to other people, quotes, lists, multi-task conjunctions, dates and reminders. Keep **verbatim source spans and spans-to-fields mapping**, proposal only.

| Candidate | Published footprint evidence | Language | Inference caveat / test |
|---|---|---|---|
| Deterministic rules + chrono/date parser | Small dependency, no weights; actual packaged bytes TBD | implement en/es/ca tokens | Excellent for explicit intents; insufficient alone for open-form implicit tasks; evaluate task recall |
| **GLiNER2.5-small** | **74M parameters** (unquantized ~296 MB weights at FP32 arithmetic, excluding tokenizer/runtime; NOT a measured package) | **English** | CPU encoder/structured spans; multi-task boundary extraction; task splitting may need rule postprocess |
| **GLiNER2.5-multi** | **287M parameters** | Multilingual | More weight than small; evaluate es/ca and export/ONNX portability |
| **NuExtract-1.5-tiny** | Based on **Qwen2.5-0.5B**; on-disk size depends quantization | English/Spanish and others | Source/template-constrained extraction, not inherently a full task planner; test repeated/implicit actions |
| **Qwen3-0.6B Q4_K_M** | **~0.45–0.48 GB GGUF weights** from community quant distributions; may add runtime/KV RAM | Multilingual, specific ca unknown | Strong useful baseline if already installed locally; NOT a lightweight phone default; test hallucinations, invalid fields, performance |

Sources: [GLiNER2/2.5 model families](https://github.com/fastino-ai/GLiNER2), [GLiNER2.5 architecture/benchmarks](https://fastino.ai/blog/gliner2-5-span-free-information-extraction), [peer-reviewed GLiNER2 2025 paper](https://aclanthology.org/2025.emnlp-demos.10/), [NuExtract 1.5 tiny](https://huggingface.co/numind/NuExtract-1.5-tiny), [Qwen3 Q4 artifact sizes](https://huggingface.co/tensorblock/Qwen_Qwen3-0.6B-GGUF).

**Licensing:** verify each **exact checkpoint** and the libraries, dependencies and model source; don't rely on project umbrella license. Moonshine **legacy Spanish Base** is **non-commercial** despite MIT streaming Spanish models—exclude it from proprietary release unless a proper license is obtained. [Moonshine license matrix](https://moonshine-voice.readthedocs.io/en/stable/license/). Picovoice Rhino is attractive for fixed command grammar but requires its own account/access/licensing and is not a safe assumption for unrestricted freeform notes; do not introduce it as a default platform dependency. Source: [Rhino docs](https://picovoice.ai/docs/rhino/).

## Performance architecture: no wasted work
1. Compress audio using the *existing* AAC/M4A recorder, never hold entire audio as base64 or JSON; send authenticated bounded chunks or existing binary attachment when reconnecting, with payload hash/size and existing claim/receipt.
2. Decode/transcribe **once per (source_hash, ASR_model_hash, ASR_config)**; cache only under current owner/revision, revoke on delete. Do not leave redundant WAV and M4A copies by default. Handle failure as UNCERTAIN rather than repeated external effects.
3. Extraction once per **(transcript_hash, locale, date_reference, parser/extractor_version)**; bound length and token budget. No open-ended agent loop.
4. Prefer batch 20–60 sec owner-ended notes initially: press/stop avoids always-on mic and live VAD cost. Optimize live streaming only if time-to-review evidence requires it.
5. One model loaded at a time; lazily load packs, bounded CPU threads/RAM, release on idle; keep full speech/extraction stack off the phone base install. Bench startup/RSS and temperature alongside latency.
6. Generate immutable proposal IDs; **not** task writes. Source ref = hash, recording device/owner, timezone at capture, transcript offsets, extraction version, and approval receipt.
7. Reuse Host task command with stable idempotency key e.g. `owner + noteId + proposalId + approvedRevision`; do not treat a client-generated task ID alone as protection against concurrent Host writes.
8. Shared typed UI states: `CAPTURED_LOCAL` → `QUEUED_FOR_HOST` → `TRANSCRIBED` → `PROPOSED` → `OWNER_APPROVED` → `COMMITTED`, with `REFUSED`/`FAILED`/`UNCERTAIN` and restart/replay semantics; existing product state machine/contract wins if available.

## Representative conceptual contract (NOT new canonical schema)
```json
{
  "noteId": "stable-content-scoped-id",
  "source": {"sha256": "hash", "device": "authorized-device-id", "locale": "es-ES"},
  "capture": {"at": "ISO-8601 with offset", "zone": "Europe/Madrid"},
  "transcript": {"text": "...", "engineDigest": "...", "spans": []},
  "proposals": [
    {"proposalId": "stable-revision-scoped-id", "title": "...", "sourceSpan": [0, 0],
     "duePhrase": null, "dueInstant": null, "priority": null, "status": "REVIEW_REQUIRED"}
  ],
  "approval": {"approvedProposalIds": [], "ownerGrantRef": null},
  "hostReceipt": null
}
```
The real implementation must map to **existing** Host/World schemas and reject unknown fields; this illustration confers no permission to introduce a shadow task ledger.

## Architecture trade-offs and what would reverse the choice
- If phone/Host pairing is not reliably available for the Day-0 user journey, the **thin phone + Host** profile is insufficient for automatic task extraction while offline. The requirement can be interpreted as eventual conversion on phone UX after reconnect (recording itself works offline). If product owner requires **fully offline phone intelligence**, budget optional packs and benchmark 4 GB RAM devices, file-space, battery and downloads explicitly.
- If Android on-device STT proves high quality, locale-available and reliable across Android targets, favor it for **live dictation** without any shipped ASR model. Never sacrifice retained-audio provenance silently.
- If GLiNER2.5-small or deterministic rules fail the **actual task-level F1 and false-commit** gates, choose a heavier **Host-only** local text model or already-authorized intelligence route; do not lower the acceptance gate for a smaller download.
- If Catalan/Mallorquí quality is a Day-0 requirement, treat multi-language/code-switching as a separate quality gate. Moonshine currently lacks a listed Catalan model. A 40 MB whisper tiny pack is fast but may be *too inaccurate* for dates/names; test larger optional multilingual pack or human transcript correction.

## Data minimization, privacy, safety
Explicit foreground microphone; indicate active capture; keep file in private app storage; ask before Host transfer (pairing alone != consent for a particular memo), no provider/API transmission, encrypt in transit, device-scoped authorization, expiry, deletion and review of original note. Preserve source hashes/receipts without retaining raw audio beyond owner's configured retention; deleting a note must not resurrect cached transcripts/tasks. Mark uncertainty on ASR transcription of dates, people, durations and negations. No model may directly write tasks, send provider messages, schedule reminders, or create calendar events.

## Evidence tiering
**Source-reviewed:** Android Recorder implementation and published API docs; cited model cards. **Unverified:** exact current task backend, Host ingestion path, native Windows build/link of ASR, speech model performance on owner hardware, locale coverage and 4-GB phone, Android recording-to-task acceptance. **No test run, device install, code patch, model download, release or GO is asserted.**
