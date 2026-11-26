# R001 — Execution evidence and current position

2026-11-26. Research only. [PLAN.md](PLAN.md) contains exactly 100 tasks and an immutable creation snapshot of 8 verified / 92 open. This round closes AND-007 at the research-fixture scope, bringing the evidence-based count to 9 verified / 91 open. No integrated Mobile Studio acceptance.

| Task | Evidence | Result |
|---|---|---|
| AND-001 | ARBOR handoff blob 13da650699beaccba1957813d8658303fb50bad8; Windows parity blob 4cccc73a852e8933b3ae18649374e883118a22ce | Source inspected, not live Windows revalidation |
| AND-002 | Live MCP 0.3.0 ready, current Termux development grant | Verified |
| AND-003 | Physical device Android API 35, ARM64, JDK 17, 16 GiB available storage at observation | Verified inventory |
| AND-004 | Previous same-user package query 131 visible third-party packages; WhatsApp launcher/share/deep-link resolver exit 0 | Verified discovery only |
| AND-005 | Android system am returned shell UID permission denial; Termux am returned exit 0 but owner explicitly confirmed WhatsApp did not open | Functional launch failed; reconciliation complete |
| AND-006 | Native package query found aapt/aapt2/d8/apksigner/Gradle; JDK 17 installed; apksigner install preview adds JDK 21; no expected SDK root found | Toolchain gap identified; installation not performed |
| AND-007 | Native Rust receipt classifier, [source](experiments/launch_receipt.rs), source commit 45ccd8a2831aa3f18c19e5ae11f7ef85de537dfd, SHA-256 ee0aff315855359ebfb2ca0d995ecfe6935ed0f1042af84c6e3454502af25343 | 8/8 physical-device Rust tests passed, exit 0 |
| AND-021 | [Bounded recent conversations](../WHATSAPP/WA02-RECENT-CONVERSATIONS.md) | Acceptance scenario registered, content access not established |
| AND-051 | [Native smoke](../../experiments/native_smoke.rs), prior physical-device 3/3 tests | Native Rust execution verified |

AND-007 command: rustc --test launch_receipt.rs -o launch_receipt_test; ./launch_receipt_test --test-threads=1; remove temporary binary after success. Actual stdout: 8 passed, 0 failed, finished 0.01s. The test fixture distinguishes dispatch rejection, zero exit without evidence, target visibility, negative observation, wrong app, wrong request, stale and expired observation. It is a pure classifier with trusted-observation inputs; it does not itself capture screens or authenticate observations. Production correlation and authorization remain AND-017/073/074/084.

Next: AND-008 native APK toolchain. Prefer native aapt2, d8, apksigner and verified official Android API 35 platform jar; check compatibility, artifact provenance and size first; no full guest Linux or model download. If native SDK host tools cannot run, build the candidate in cloud with official SDK and validate APK on physical device. AND-009 foreground companion: explicit user tap, standard PackageManager launch intent, no exported arbitrary shell. AND-010 owner installation and visible launch confirmation. Then consented MediaProjection on mobile data, with foreground service and stop/revoke. Screen capture is observation only, not input privilege. No ADB pairing required for primary path. No new app permission, APK installation, screen capture, chat read or message send occurred this round.

Official references inspected: https://developer.android.com/guide/components/activities/secure-bal ; https://developer.android.com/about/versions/15/behavior-changes-15 ; https://developer.android.com/reference/android/content/pm/PackageManager ; https://developer.android.com/media/grow/media-projection . Context7 Android Developers /websites/developer_android consulted for foreground app launch and package visibility. Background launch restrictions are a plausible hypothesis for failed launch, not an established device-specific root cause.
