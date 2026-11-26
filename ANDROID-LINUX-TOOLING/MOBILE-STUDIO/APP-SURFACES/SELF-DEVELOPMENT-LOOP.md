# Android self-development loop — owner requirement, 2026-11-26

The Android APK is both product companion and development instrument. It must eventually run its own real physical-device tests, stage/install its own software and upgrade itself through supported Android mechanisms. The default development loop should need no repeated owner intervention except where Android or explicit policy requires it.

## Existing machinery
Reuse native Termux worker, existing APK DevActivity, Host authority, Product SDK/Factory receipts and current signing/build tooling. Do not create another scheduler, execution authority or unbounded remote shell. Preserve concurrent features in prototype main.

## Release lifecycle
Source identity -> native build/test -> exact signed APK and signer digest -> candidate registration -> authorized installation -> actual installed identity -> live APK-context tests -> durable receipt -> diagnose/repair or promote. Promotion is a statement of observed results, never a fabricated success. Keep prior signed artifacts and versioned, compatible app state; Android downgrade may be forbidden. Recovery may require a higher-version repair built from last known good source, or owner-approved reinstall with explicit data impact.

## Installation modes
- Private sideloaded self-update: Android PackageInstaller supports USER_ACTION_NOT_REQUIRED for eligible self-updates on API 31+; on Android 15 the target must be API 33+; REQUEST_INSTALL_PACKAGES granted and UPDATE_PACKAGES_WITHOUT_USER_ACTION declared, and applicable installer/update ownership rules. Always handle STATUS_PENDING_USER_ACTION, failure and actual success. First install and permissions may need owner interaction. No silent-install assumption, root, permission bypass, or device-owner conversion on personal phone.
- Authorized development device: supported ADB pairing enables normal package install/update and instrumented tests. Currently ADB device list empty; do not assume pairing or escalate privileges.
- Managed dedicated test device: Android Enterprise device policy controller can install packages without interactive approval in eligible managed environments. Not proposed for current personal phone.
- Play distribution: use Play-supported update mechanisms and applicable distribution policy.

## Trust and scope
Updater handles VIVIM-owned package identities only; verify current signer, candidate signer, package, increasing version, exact digest and authorized release identity before staging. Debug signer is development only; no production trust claims. Owner-approved scoped policy, revocation, visible status; no arbitrary downloads executed, no remote permission grants, no third-party app installs, no generic update commands. PackageInstaller callbacks plus installed package readback required. An APK process replacement is normal; persist minimal non-sensitive release journal and resume evidence on next supported execution. Never auto-promote on installer status alone.

## Live tests
Automatic own-app startup checks: actual JNI invocation, private filesystem write/read/delete, own APK identity and digest, declared permissions, resource health, owned UI/display fixture tests with timeout. Extend through Android instrumentation on authorized development channel. Permission-dependent external-app tests remain blocked until approved; never automate consent dialogs or send test messages to real contacts. All receipts distinguish pass, fail, unavailable, unrun. No private WhatsApp content in diagnostic artifacts. Reuse existing Factory verification and index; receipt export initially explicit, paired IPC later.

## Immediate implementation increment
Add automatic startup self-test using existing DevActivity diagnostics, retaining a private latest receipt and explicit export. Release metadata verification and narrowly scoped PackageInstaller integration after actual candidate installation; do not claim unattended updates until physically verified. Concurrent native download feature introduces INTERNET permission; report actual permission state rather than requiring it absent from all future releases. Build identity must capture the actual candidate, not only source docs.

Sources: https://developer.android.com/reference/android/content/pm/PackageInstaller.SessionParams#setRequireUserAction(int) ; https://developer.android.com/reference/android/content/pm/PackageInstaller.Session ; https://developer.android.com/work/dpc/dedicated-devices/cookbook ; https://developer.android.com/studio/test/command-line ; https://support.google.com/googleplay/android-developer/answer/12085295 .
