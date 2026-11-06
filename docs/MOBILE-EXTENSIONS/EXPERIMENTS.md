# Mobile extensions — minimal first experiment wave

## Experiment 0 — environment
Record phone model, Android version, browser version/channel, extension ID/version, installation source, OS battery mode, and date. Dedicated test profile/accounts only; no real personal session data.

## Experiment 1 — installability
Install synthetic minimal extension on Firefox Android, Quetta and Edge stable/Canary. Verify content script injects into own local fixture, shows visible marker, and can be removed cleanly. Log errors.

## Experiment 2 — capability coverage
On a controlled synthetic multi-tab site, test DOM read, form-fill, click, navigation, messaging, tab enumeration and persistence. Use permission prompts and denied-permission negative tests.

## Experiment 3 — lifecycle
Screen off, app background/foreground, battery saver, browser killed/restarted, connectivity loss. Verify work is canceled/safely resumable without duplicate effects.

## Experiment 4 — security
Forbidden origin, iframe constraints, private/incognito, missing permission, popup blocked, untrusted page content, extension unload, stale session. Assert no secret or unauthorized action escapes.

## Experiment 5 — VIVIM adapter design (no live integration)
Define a browser-neutral capability handshake: `capabilities -> propose -> approval -> constrained action -> evidence receipt`. Compare Android extension adapter, native APK bridge, and desktop Host adapters; do not add alternative effect ownership.

## Outcome rubric
For every case record: supported / partially supported / unsupported / not tested, feature-specific reliability, browser and build, steps, evidence, and fallback. No unsupported percentage-parity claims.
