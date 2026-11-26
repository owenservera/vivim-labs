# Research and experiment plan
Every phase is proposed research, not a production work order. Record exact source pin, OS version, grant, precondition, exit, redacted receipt and negative tests. Real owned fixtures first.

| Lane | Scope | Acceptance |
|---|---|---|
| M01 Discovery | PackageManager visibility, launcher activities, labels/icons, intent resolver | Local graph of user-approved visible apps and declared actions; no content collected |
| M02 Companion build | Android SDK/API 35, Gradle/JDK compatibility, minimal Rust JNI adapter | Build signed debug fixture APK; install only with explicit owner approval |
| M03 Interop | Intents, deep links, Android sharesheet, content URI | Fixture resolves and returns explicit selected action/result; denial is clean |
| M04 Documents | SAF and photo picker | Read/write only a user-selected test file; revoke/deny verified |
| M05 Visual | MediaProjection | Owner grants one test capture of owned fixture; stop and secure-window behavior verified |
| M06 UI semantics | Accessibility API eligibility and owned UI Automator fixture | App-provided nodes or authorized test hierarchy; no general autonomous accessibility agent |
| M07 Events | Notifications, usage, media session, battery, sensors | Permission-specific test data; no private notifications or personal records |
| M08 Work lifecycle | WorkManager, foreground service, stop/resume | Restart and revoke; bounded recovery without replay |
| M09 SDK convergence | Native descriptor admission, VLANG contracts, receipts, semantic projections | Reuse Product graph/task catalogue; exact source and end-to-end fixture evidence |
| M10 App Studio cards | User-selected app integration review | Per-app supported intents/shares/APIs, observation and action evidence, unavailable capabilities explicit |

First vertical slice: own fixture app -> discover -> launch through user-directed supported intent -> share test content -> receive result -> typed Host receipt -> graph update. No actual personal-app interaction required to validate core. Expand one user-selected app at a time. APK build-tool availability is observed; toolchain compatibility and installation are not established. Prefer native Termux, cloud build fallback if Android SDK host binaries cannot run natively.

## References (Context7 plus official online verification)
- https://developer.android.com/training/package-visibility
- https://developer.android.com/training/package-visibility/declaring
- https://developer.android.com/guide/components/intents-common
- https://developer.android.com/training/data-storage/shared/documents-files
- https://developer.android.com/media/grow/media-projection
- https://developer.android.com/reference/android/accessibilityservice/AccessibilityService
- https://developer.android.com/training/testing/other-components/ui-automator
- https://developer.android.com/reference/android/service/notification/NotificationListenerService
- https://developer.android.com/develop/background-work/background-tasks/persistent
- https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start
- https://developer.android.com/develop/background-work/services/fgs/timeout
- https://developer.android.com/privacy-and-security/security-tips
- https://support.google.com/googleplay/android-developer/answer/10964491
- https://github.com/termux/termux-api
- https://github.com/termux/termux-api-package

Termux:API is a possible interim capability adapter. Verify installed Termux distribution/signing compatibility before proposing add-on; CLI package alone is not a grant. ADB is for explicit developer testing, not production authority.
