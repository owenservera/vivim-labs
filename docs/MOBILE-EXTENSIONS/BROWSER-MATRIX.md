# Research scope note (2026-11-06)

This is the earlier vendor-install shortlist; **not** the final platform ranking. For controlled app-owned capabilities, embedded GeckoView with built-in WebExtensions is now a critical additional candidate. Edge Android stable's curated catalog is not a general custom-VIVIM-extension install route. Prefer [BROWSER-ENGINE-DECISION.md](BROWSER-ENGINE-DECISION.md) for the reconciled architecture decision and official sources. No browser/device parity tests have run yet.

# Browser candidates — research, not verified VIVIM compatibility

As of 2026-11-06. Confidence describes published support, **not** experimental success.

| Candidate | Extension model | Initial priority | Validation needed |
| --- | --- | --- | --- |
| Firefox Android | Official Android-compatible WebExtensions, subset of desktop APIs | P0 | MV3/MV2 availability, install/update, background lifecycle, tabs/scripting/messaging |
| Quetta | Chromium extensions, third-party support | P0 | Store install, MV3, content scripts, background suspend, permissions |
| Edge Android stable | Curated mobile extensions | P0 | Custom extension distribution/install and supported API set |
| Edge Canary | Developer-mode extension-by-ID path reported | P0 | Reliability, per-build flags, unsupported APIs |
| Yandex Android | Chrome-style extensions | P1 | Extension installation and site compatibility |
| Vivaldi Android | Extension support reported in recent 2026 guides | P1 | Confirm official implementation and version; don't assume |
| Lemur / Mises / Helium | Third-party Chromium extension support reported | P1 | Maintenance, trust, APIs, deployment |
| Chrome Android | No mobile extension install in normal release | Negative control | Confirm no production path |
| Kiwi | Archived 2025 | Exclude | Keep as portability precedent only |

References:
- Mozilla installation: https://support.mozilla.org/en-US/kb/find-and-install-add-ons-firefox-android
- Mozilla desktop/Android differences: https://github.com/mozilla/extension-workshop/blob/master/src/content/documentation/develop/differences-between-desktop-and-android-extensions.md
- Chrome extension team (2026): https://groups.google.com/a/chromium.org/g/chromium-extensions/c/tCUm3kceybk
- Kiwi archive: https://github.com/kiwibrowser/src.next/blob/kiwi/README.md
- Independent hands-on roundup: https://www.androidauthority.com/chrome-extensions-android-3325587/
- Newer Vivaldi claim (secondary source, unverified): https://www.web-highlights.com/blog/how-to-install-chrome-extensions-on-android/

Do **not** rank browsers by extension count alone: prioritize functional API coverage, security update cadence, device compatibility and deterministic test reproducibility.
