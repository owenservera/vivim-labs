# R008 — Three-round owner-device candidate

2026-11-26. Three consecutive on-phone signed builds completed with no intermediate owner installation request.

| Round | Version | Bytes | SHA-256 |
|---|---|---|---|
| 1 | 0.5 | 29146 | 654ad837700926e817236fbf91d7dd70a5ce36d99848d3dcd2a03fcc3053acf3 |
| 2 | 0.6 | 29146 | 36a8dec56e76e1f6125dd6c01fa04dc904fc6093c9c8a07d9e174b9b6fc5fdae |
| 3 | 0.7 | 33242 | 8f6f6560a74236c9d028ae835e50d9e88fc8c7374cb844f4e63fd217baedefbb |

Build sessions f250e8511ae64cc79cc859f94616b501/502/503 (full IDs share prefix f250e8511ae64cc79cc859f94616b5 and end 01/02/03) exited 0. APK v2/v3 signing verification passed. Final local artifact out/round3-v07.apk matches Downloads/vivim-surface-probe-v07.apk. Java obsolete/deprecation warnings remain.

Round 1: visible app launcher catalogue, filter, scoped native launch requests, reviewed snapshot export via document picker, capture reader draining correction.
Round 2: optional notification listener, only reads active WhatsApp message previews on owner tap, max three conversation previews and three messages each, no event persistence or upload; explicit reviewed text share. Android notification access is broad even though sample implementation filters WhatsApp.
Round 3: explicit selection of fresh active notification's native content intent to request chat opening; bounded manual reviewed JSON form and reviewed preview JSON export; capability status. No automatic message sending, no autonomous remote control or unrestricted history.

Live verification session f250e8511ae64cc79cc859f94616b504 exited 0: aarch64, API 35, package manager reports installed prototype versionCode 1, WhatsApp MAIN launcher and SEND text/plain activity resolve, ADB has no connected devices. Real ARM64 ctypes invocation of native classifier returned expected positive and negative values. Round 3 native Rust tests 5/5 passed. These results are not APK-side runtime acceptance. No personal messages read, no permission change, no app launch or send during verification.

Final APK remains uninstalled at observation time. Owner must install v0.7 and locally authorize optional screen/notification access if desired; later verify actual runtime and revocation. Current bridge has Termux UID, not APK identity, and no screen observation tool. No automatic transport for private observations; explicit owner-reviewed share/export only. No parity task count increment claimed solely from build. Existing v0.1 physical virtual display evidence stands.
