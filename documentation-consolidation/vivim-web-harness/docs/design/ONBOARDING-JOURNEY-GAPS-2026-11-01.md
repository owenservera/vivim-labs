# Onboarding journey: gaps found and what is now wired

**Date:** 2026-11-01 · **Scope:** first install → workspace → VIVIM browser → AI sign-in → main AI → background personalization → phone → Build canvas → guided tour.
**Method:** each claim below was read in source or reproduced; "fixed" means covered by a test or a rendered check in this PR. Nothing here was run against a real Chrome profile, a real ChatGPT account, Windows, Tailscale or an Android phone.

## Gaps found, and their state

| # | Gap (evidence) | State |
|---|---|---|
| 1 | The setup wizard could not run: `onboarding.js` used `btn-0`/`badge-step-N` ids the page never had, and `currentStep` never advanced past step 0. | Fixed: rebuilt page and script; steps derive from Host state. |
| 2 | Chrome detection was one hard-coded Windows path (no Edge, no per-user install, no macOS/Linux). | Fixed: `onboarding/environment.mjs`, tested per platform. |
| 3 | No step to open the VIVIM browser, sign in to AI apps, or see per-AI readiness. | Fixed: provider list with Open and Check, backed by a fresh owned Studio tab that is closed again. No send. |
| 4 | Nothing recorded the owner's main AI. | Fixed: stored in `<data>/onboarding/state.json`, shown to the phone. |
| 5 | The bootstrap scheduler (`bootstrap/scheduler.mjs`) was never instantiated; the Studio owner adapter was never imported by anything but tests; the `personalize` action was hard-disabled. | Fixed: `onboarding/service.mjs` connects scheduler, adapter, Studio port and ledger; a background loop runs the approved passes. |
| 6 | The scheduler passed raw replies to strict parsers, so a fenced or chatty ChatGPT answer would always fail ingestion. | Fixed: tolerant extraction before the unchanged strict parsers. |
| 7 | No owner-approval surface: the scheduler needs an approval bound to each exact prompt digest and no UI created one. | Fixed: the owner reviews the exact text and taps Approve; approval covers those two prompts for 30 minutes. |
| 8 | F02/F01 prompt text lived only in a disabled dev harness. | Fixed: one shared module; the harness imports it. |
| 9 | Direct mode blanked all cards, so personalised cards never reached the phone. | Fixed: card id, title and basis reach the owner's phone; preferences and hard limits stay on the PC. |
| 10 | No QR or address for pairing, and no "phone connected" signal. | Fixed: QR from the configured Direct address; connection shown once the phone calls the Host. |
| 11 | Phone after pairing landed on Talk; Today, Agents, Self were "coming soon"; Talk also showed the placeholder under it. | Fixed: welcome tour on first run, a real Start-here page, placeholder bug removed. |
| 12 | Build canvas had no way to say what you want: only taps on an existing surface; no path from words to a proposal. | Fixed: request → design prompt → one send → reply validated by the existing compiler → dashed proposal → Activate / Change it / Not now → Undo. |
| 13 | The Host crashed on `/` when no workspace manifest existed (`MANIFEST_*` not imported). | Fixed. |

## Still open (not done here)

- **Live verification.** Everything above is tested with fakes and rendered with sample data. A real run on Windows with Chrome, Studio, a signed-in ChatGPT and a paired Android phone is the acceptance test.
- **Personalization reads from ChatGPT only.** The adapter, thread check and scheduler are ChatGPT-specific. Choosing another main AI is accepted and the page says so. Phone Build requests also work with ChatGPT only, because the Studio chat adapter supports default ChatGPT text only (`UNSUPPORTED_STUDIO_TARGET`).
- **Deep passes D03–D07** have no pinned prompt templates yet; only F02 and F01 run.
- **No real signed-in probe (roadmap M1a).** "Signed in" currently means Studio opened a fresh tab for that site and reported it ready. A page that loads is not proof of login; Studio's own pre-send checks remain the final gate.
- **APK.** No build, signing or install flow from the wizard; the enrollment object exists in the Host but has no PC approve or revoke route. Until a release is registered the wizard offers the phone web address and "Add to Home screen".
- **Tailscale.** The Direct address comes from `VIVIM_DIRECT_ORIGIN` / `VIVIM_DIRECT_USER`; there is no wizard step to set it up.
- **"Start VIVIM" from the page is Windows-only** (PowerShell launcher). Other platforms are told to run `vivim start`.
- **Agents, Self and Control phone tabs** are still placeholders.
- **Draft handling.** The 2026-10-30 review found Build-to-Talk could overwrite an unsent Talk draft. That path was not re-audited here; the new Build requests do not touch the Talk composer.
