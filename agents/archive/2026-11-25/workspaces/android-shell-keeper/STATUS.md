# STATUS — android-shell-keeper standing workspace

**Phase:** scoped  
**Seat:** android-shell-keeper (permanent seat, this conversation)  
**Identity:** inc.vivim.app, versionCode 1, versionName 1.0.0, minSdk 26, targetSdk 34

## Baseline (read-only, NO build run)

| Check | File | Result | Evidence |
|---|---|---|---|
| Build pipeline (bash) | studio/android/build.sh | EXISTS, unchanged | aapt2 -> javac -> d8 -> zip -> zipalign -> apksigner; JDK 17+; android-34 + build-tools 34.0.0; security gate rejects addJavascriptInterface/@JavascriptInterface |
| Build pipeline (ps1) | studio/android/build.ps1 | EXISTS, unchanged | Same pipeline; same integration-2 source enroll; same release monotonicity + signing-profile schema |
| Manifest identity | studio/android/AndroidManifest.xml | EXISTS, unchanged | package="inc.vivim.app", versionCode="1", versionName="1.0.0", minSdk 26 / targetSdk 34; INTERNET + ACCESS_NETWORK_STATE + POST_NOTIFICATIONS + RECORD_AUDIO; `.MainActivity` with pair/send intent filters |
| Security gate | build.sh:45 / build.ps1:99 | CLEAN | grep for addJavascriptInterface / @JavascriptInterface in MainActivity.java returns empty |
| MainActivity loadUrl | studio/android/src/inc/vivim/app/MainActivity.java:328 | VERIFIED | `web.loadUrl(url)`; full-screen WebView around Studio mobile PWA; no PWA assets bundled |
| NativePortBootstrapped NPE blocker | historical | RESOLVED | Historical D8 8.2.2-dev NPE on NativePortBootstrapped.class; toolchain state to be re-checked before any build claim |

## Claims

| Claim | Outcome |
|---|---|
| bootstrap.mjs claim <session> --agent android-shell-keeper | NOT CLAIMED — bootstrap.mjs is a disabled `BootstrapScheduler` draft; running `claim unknown-session --agent android-shell-keeper` exited 1 with `BOOTSTRAP_SCHEDULER_DISABLED: scheduler is a disabled draft`. No `claim` subcommand exists. Journalled in LOG.md per instructions. |

## Deliverables

| Deliverable | Status | Where |
|---|---|---|
| Baseline verification | complete | this file |
| Reflection | complete | C:\Users\VIVIM.inc\.zcode\oracle\bootstrap\android-shell-keeper.reflection.md |
| Claim journal | complete | C:\Users\VIVIM.inc\.zcode\research\android-shell-keeper\LOG.md |

## Blockers

- none

## Top three contracts (shell keeper)

1. **Identity**: inc.vivim.app; versionCode 1; org.vivim.phone rejected — never fork identity. AndroidManifest.xml is the source of truth parsed by build.sh/build.ps1.
2. **Build pipeline**: NO Gradle — build.sh/build.ps1 drive aapt2→javac→d8→zip→zipalign→apksigner against platforms;android-34 + build-tools;34.0.0; JDK 17+. Debug keystore auto-minted; release requires user-owned signing profile + existing keystore (never mint a new release identity).
3. **Architecture / acceptance**: full-screen WebView shell around the live Studio mobile PWA (web.loadUrl :328); no PWA bundled; server-side JS changes need NO Android change unless tests demonstrate it. Compilation ≠ device acceptance; physical-device evidence required for device claims; emulator evidence labelled emulator-only.
