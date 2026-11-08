# VIVIM Labs — Day-0 voice notes → tasks (Windows + Android)

**Research status (2026-11-08):** architecture recommendation and test plan, **not** an implementation, measured local benchmark, launch approval, phone acceptance, or new product authority.

**Source of release requirement:** [Repos HQ Day-0 issue #11](https://github.com/owenservera/repos-hq/issues/11). [Production closeout #10](https://github.com/owenservera/repos-hq/issues/10) remains security/availability critical. Inspiration: [XDA local LLM voice-notes article](https://www.xda-developers.com/my-local-llm-turns-my-voice-notes-into-to-do-items-automatically/) (the publication's full article text was not retrievable at research time; do not attribute implementation details to it).

## Outcome, not technology
One owner-initiated spoken note on either phone or desktop yields **zero, one or several grounded draft tasks**; show transcript and source spans; edit, approve some, reject others; commit **once** under existing Host authority; read the approved tasks on either device after restart/reconnect. Exact deadlines require deterministic timezone/DST-aware parsing; never allow the model or phone to directly mutate authoritative task state.

## Provisional recommended architecture — thin capture, one owner, optional intelligence packs

1. **Reuse native capture now present in Android:** Web Harness `PocketRecorder` records owner-initiated foreground AAC/M4A into app-private files (16 kHz, 64 kbps, duration and capacity safeguards). `PocketStore` is its existing commit authority for a captured original; independently audit its actual callers and existing Host upload/retrieval path. Desktop should reuse the existing Host/Studio capture/import route; no new recorder framework.
2. **Phone remains a thin client:** preserve note and pending intent locally; when paired, submit the existing authorized transport envelope for an owner-approved Host-owned transcription/extraction job. Display transcript, proposal drafts and commit receipt on the phone. If offline, retain the durable draft with a truthful "waiting for Host" state, not invented completion.
3. **Optional speech adapter selected by benchmark:** probe Android's **on-device** `SpeechRecognizer` availability for a *separate live-dictation route* where supported, but do **not** treat it as a general converter for existing M4A or as an assurance that all OEMs/locales are offline. For stored audio, default to a measured, downloadable small Host ASR language pack. Candidate: Moonshine Voice **Tiny Streaming** English/Spanish (34M parameters per model; shipped as quantized ORT), tested against multilingual `whisper.cpp` tiny/base for Catalan and code-switching. Never embed all packs in the base APK.
4. **Task extraction by smallest trustworthy route:** run bounded deterministic segmentation, explicit action detection, source-span anchoring and relative-date **intent** parsing first. Compare a small extraction-specific CPU model (`GLiNER2.5-small` 74M English; multilingual variants larger), `NuExtract-1.5-tiny` (Qwen2.5 0.5B, supports English/Spanish) and existing locally available intelligence for genuinely free-form multi-task memos. No compulsory 0.5GB phone LLM. The exact selected backend is **undecided pending measured task F1, RAM and latency**; deterministic validation + user review remain mandatory.
5. **Single Host effect:** model outputs are drafts, not commands. Existing Host command/approval/single-writer/revision/receipt path creates tasks. No new task DB, sync engine, publisher, provider API, background microphone, or remote AI without explicit permission.

**Critical mic/API distinction:** Android's `SpeechRecognizer` normally captures its own microphone stream; the existing `PocketRecorder` writes M4A. Simultaneous recording/recognition must NOT be assumed. API 33 `RecognizerIntent.EXTRA_AUDIO_SOURCE` offers an optional external PCM stream but recognizers can ignore it. A saved file-to-text pathway must have an independently proven file-decoding ASR adapter. The safe initial path is recorded M4A → authorized Host → local ASR → reviewed task proposals.

## Navigation
- [Deep comparative architecture research](ARCHITECTURE-RESEARCH.md) — candidate runtimes, languages, licenses, quantified source claims, cost/footprint, authority and decision.
- [Benchmark, experiment and release-gate plan](BENCHMARK-AND-ACCEPTANCE.md) — independent tests, explicit budgets (targets, **not measured facts**), real-device gates and stop conditions.

## Verified source-side seams (code presence ≠ release readiness)
- Web Harness Android `studio/android/src/inc/vivim/app/PocketRecorder.java`: foreground recorder with `MediaRecorder`, private pending audio, M4A/64 kbps, bounded files, recovery semantics; **not** automatic transcription/task creation.
- Web Harness Android `studio/android/src/inc/vivim/app/MainActivity.java`: permission and `PocketRecorder` wiring; existence does not prove a voice-task product flow.
- Labs existing [surface inventory](../extension-points/PLATFORM-SURFACE-INVENTORY.md): voice integration points are inventory candidates, not accepted implementations.
- Web Harness [critical-path document](https://github.com/owenservera/vivim-web-harness/blob/main/docs/capabilities/CRITICAL-PATH.md): phone C43, approval/send-once C03, reply C06, owner/sync contracts need runtime evidence.

## Ownership and promotion rules
This folder is PUBLIC-safe, research-only. **Never commit personal audio, transcripts, credentials, private repository contents or hardware logs** here. Implementation proposals belong to the existing owned code paths in private repos, only after leases and integration review. Owner approval of this research **does not unblock** production gates, provider sends, model downloads, device installs or external transfers. **Phone acceptance: UNRUN.**
