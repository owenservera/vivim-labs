# R002 — On-device APK build
2026-11-26. AND-008 closes at research-toolchain scope; cumulative parity research count 10/100. AND-009 foreground app-launch fixture and actual app installation/virtual display acceptance remain open. R001 prototype implements owned presentation only, not third-party app launch.

Termux install session a4075fcf5b2545ec987a0c41e10ad002 exited 0. Installed native aapt2 16.0.0.4-4, d8 and apksigner package 37.0.0, OpenJDK 21.0.13.1, libzopfli. SDK platform jar verified in R001-EVIDENCE.md. zipalign not present on PATH; pure Java fixture uses no native libraries or uncompressed native payload. Build session a4075fcf5b2545ec987a0c41e10ad008 exited 0. AAPT2 -> javac -> d8 -> zip -> apksigner. APK Signature Scheme v2 and v3 both verified, one local debug signer. Obsolete Java 8 source/target warnings only. Local key not published.

APK: surface-probe/out/vivim-surface-probe.apk inside android-linux-lab, 12,685 bytes, SHA-256 7eec3dea1efe82f399215db16817a90acb86ad97b10ae8accb5792d4ca489946. No PC, Gradle, Android Studio, ADB or root used. Native Rust JNI probe compiled separately, not embedded (Termux-specific RUNPATH and in-app loading not validated).

Install via user-initiated Termux foreground command:
```sh
termux-open --view "$HOME/vivim-workspace/android-linux-lab/surface-probe/out/vivim-surface-probe.apk"
```
Android installer may require permission to install apps from Termux. Grant only if owner chooses, revoke after test. Open VIVIM Surface Probe, tap Start private display, change fixture color twice, inspect display ID/frame count/center RGB/digest changes, Stop and release. No actual install or runtime capture confirmed yet. No personal app data or permissions requested by APK. No hidden input injection or unauthorized third-party capture.
