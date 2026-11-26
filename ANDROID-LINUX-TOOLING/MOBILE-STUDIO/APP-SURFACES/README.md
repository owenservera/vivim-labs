# Android application surfaces — R001
Owner goal: interact with supported native apps through VIVIM PWA/native client even when the target is not on the physical display. Keep Product SDK/Host as authority and existing one-renderer policy. This is a Labs fixture, not production UI.

## Mechanisms and boundaries
1. Public DisplayManager private own-content-only VirtualDisplay + Presentation: render our own content into an app-owned Surface, without mirroring the device. Test first with an owned, colored fixture. This proves independent rendering, not third-party app streaming, arbitrary input, or hidden WhatsApp access.
2. Public ActivityOptions.setLaunchDisplayId and ActivityManager.isActivityStartAllowedOnDisplay: evaluate eligibility on owned fixture. Android applies restrictions on launches onto virtual displays; never escalate privilege or route another app into a hidden capture surface.
3. MediaProjection: system user consent each session for target SDK 34+, visible foreground service, capture stop/revocation, resize and visibility callbacks. Measure frames when selected fixture is visible, partly visible and hidden. Do not infer background live frames; no protected content or unattended capture.
4. Companion app streaming: Android 13+ VirtualDeviceManager/CDM provides virtual displays and remote input, but COMPANION_DEVICE_APP_STREAMING is for preinstalled privileged OEM apps. No sideloaded permission workaround. Research existing OPPO-supported integrations only if publicly documented and owner-selected.
5. App-native contracts: intents, explicit shares, user-selected content and app-provided notification reply affordances. No arbitrary personal WhatsApp history API or assumption that off-screen apps remain interactive. Prefer supported app interfaces; user confirms effects.

## Prototype
P1: no-permission Java Activity; user taps Start, creates a 320x480 private own-content display backed by ImageReader, shows own Presentation and measures visible frame changes using SHA-256. Tap Change to update fixture; verify distinct digests; Stop or leave activity releases display and reader. No Internet, capture permission, third-party access, hidden persistence or image export. This is a tiny OS boundary fixture; Rust and Product integration follow after native surface feasibility is measured.
P2: owned Activity on private display using eligibility check, explicit user action, verify display ID and deny unsupported launch.
P3: user-consented MediaProjection fixture, foreground service, visibility/resize/stop, bounded capture.
P4: native companion + Rust JNI, typed Host descriptor, authenticated local IPC; no duplicate renderer or execution authority.
P5: app-specific bounded WhatsApp read (three recent chats and at most three messages each), no send before owner acceptance. If unsupported, report gap. PWA sends scoped requests to Host, not direct Android privilege.

## On-device build
Native Termux aapt2, d8, apksigner and JDK; API 35 platform jar from Google's SDK metadata with archive checksum verified. No Gradle, PC, model or guest Linux required in principle. Actual tools, APK build/sign and physical install/runtime must be verified separately. Keep private signing material device-local, no key in repo. User installs APK through Android's normal installer. No silent install.

## Sources
- https://source.android.com/docs/core/permissions/app-streaming (role eligibility, remote input)
- https://source.android.com/docs/core/display/multi_display/activity-launch (launch policy)
- https://developer.android.com/reference/android/hardware/display/DisplayManager
- https://developer.android.com/reference/android/app/Presentation
- https://developer.android.com/reference/android/app/ActivityOptions
- https://developer.android.com/reference/android/media/projection/MediaProjection.Callback
- https://developer.android.com/media/grow/media-projection
- https://developer.android.com/tools/aapt2
- https://dl.google.com/android/repository/repository2-1.xml (official platform archive metadata)
Context7 Android Developers /websites/developer_android resolved and queried for private virtual displays, own-content flags and surface lifecycle. Source and device evidence are separate.
