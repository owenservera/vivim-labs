# Android developer instrument — owner-device requirements
2026-11-26. Owner correction: the APK is both product surface and development tool for ChatGPT, not only WhatsApp UI. Preserve existing Product Host, Rust SDK and 100-task plan. Termux remains build/compute worker; APK provides Android-framework execution identity. No duplicate scheduler or generic remote shell. No private third-party content in diagnostics.

## Capabilities and acceptance
| Capability | Intended implementation | Actual acceptance |
|---|---|---|
| Build identity | package version, signer digest, source/release seal | installed identity matches candidate |
| Native ABI | load library and run actual positive/negative JNI calls | in-APK results, not Termux-only |
| Framework probes | PackageManager, intent resolvers, supported feature flags | typed available/denied/unavailable results |
| Resource health | own heap, storage, battery/thermal state where public | measured device values |
| App-owned lifecycle | bounded event journal, process start/stop, exception type | actual transitions without message text |
| Diagnostics | structured status and reviewed export, no private data | bounded JSON readback |
| Test runner | fixed owned fixtures, deterministic assertions, timeouts | physical device receipt for each test |
| UI verification | owned Activity and view state, authorized instrumentation | real observed state; no synthetic passes |
| Display verification | owned private display and release | actual frame/ID evidence |
| Capture lifecycle | explicit MediaProjection session, stop/revoke/timeout | user-authorized fixture test |
| Permission diagnostics | actual current permission state and revocation | denied/allowed cases; never grant remotely |
| IPC | authenticated, versioned, scoped local request/reply | verified peer identity, expiry and replay denial |
| Runtime evidence | request ID, source pin, observed state and outcome | distinguish dispatch from effect |
| File handoff | user-selected SAF receipt, content digest | exact selected artifact, no broad shared directory scan |
| Task execution | existing Termux native sessions, not APK arbitrary shell | preserve existing receipts and scope |
| Release staging | same signer, version, digest, package installer session | user-approved install and readback |
| Recovery | retain known-good build, supported reinstall/update | documented failure and safe recovery |
| Performance | measure heap, latency, capture resources and battery | before/after actual results |
| Privacy | diagnostics never include chat data, tokens or notification text | inspect exported schema and denial tests |
| Stop/revoke | visible control, close channel and active operations | actual stop and no unauthorized restart |

First increment: developer diagnostics inside existing APK, with native ABI tests, package identity, public Android resource checks, own private cache roundtrip, native WhatsApp launcher/share resolution, capture/notification connection state without accessing chat content, and user-selected diagnostic JSON export. Do not equate this with remote APK control. Existing build verification may read only the reviewed artifact explicitly selected for that purpose.

Next: investigate narrow authenticated IPC under distinct UIDs. Prefer Android-bound service if both endpoints are signed/eligible; Termux is a separate distribution and signature permission is not assumed. Evaluate explicitly paired local protocol with small allowlisted diagnostics before any app effects. Do not expose an unauthenticated socket, generic shell, arbitrary capture or send. OS-required permission/install prompts remain user-owned. Instrumentation is appropriate for owned app tests when authorized debugging is available; no bypass. Android package installer may require user action. See https://developer.android.com/privacy-and-security/security-tips , https://developer.android.com/training/testing/instrumented-tests , https://developer.android.com/reference/android/content/pm/PackageInstaller .
