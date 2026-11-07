# APK07 Factory — user-built packages, keys, QR distribution and updates

**Decision:** personal APK builds and signing ownership are *Day-One requirements*. Standard and user-customized output come from one audited build source. This design supersedes the debug-only QR first milestone from the original 2026-11-01 research, without discarding that code.

## 1. Three factory profiles (all supported)

| Profile | Package identity | Signer | Intended install / upgrade behavior |
|---|---|---|---|
| Standard | Stable `inc.vivim.app` | VIVIM publisher-managed release key | General distribution; one app identity, consistent upgrades |
| Personal-branded | Standard identity only when **signed by same standard signer**; otherwise custom identity | Explicit, policy-compliant selection | User's icon/name/theme/assets; avoids lying about key continuity |
| Fully independent | Stable, unique user-specific application ID | User-owned locally generated/imported release key | Independent application, versioned updates only with compatible signer; user bears signing backup/verification responsibility |

**Important:** Android uses package name + signing identity for compatibility. Two APKs with the same application ID but incompatible signers cannot update each other in-place. Avoid doing branded per-user variants with the same package name and random user keys. Google developer verification registration is separate from cryptographic signing; a valid APK signature is **not** proof a developer account has been verified. The 2026 region/participating-store rollout is not a universal ban on direct APK sideloading, but plan for 2027 expansion. See official Android verification and signing references at end.

## 2. Build profile schema (proposed)

```json
{
  "schema": "vivim.apk-build-profile/1",
  "profileId": "opaque-owned-local-id",
  "mode": "user-owned",
  "applicationId": "inc.vivim.personal.example",
  "displayName": "My VIVIM",
  "iconSource": "local-approved-asset-ref",
  "splashTheme": { "background": "#0f1218", "accent": "#6ba1b5" },
  "bundledUiRevision": "host-approved-immutable-asset-digest",
  "bootstrapHostHint": "https://desktop.example.ts.net",
  "signatureProfileId": "local-secret-ref-not-a-path",
  "buildChannel": "personal",
  "versionCode": 1,
  "versionName": "1.0.0"
}
```

`bootstrapHostHint` is **public routing metadata**, not session authorization; do not compile in owner token, personal memory, signing password, full Tailscale credentials, access grant or secret enrollment URL. Validate names and IDs; rasterize/validate icons (no remote fetch with implicit network); size budget and shape constraints. A manifest is emitted from normalized inputs, not direct string substitution of user fields into XML/Java source.

## 3. Factory pipeline and evidence

```text
create profile → lint/preview → verify prerequisites → prepare isolated build workspace
→ build static assets/resources → aapt2/javac/d8 or supported Android build system
→ align → sign with selected release key → apksigner verify
→ inspect package ID/version/min/target/permissions/cert → hash APK
→ record signed immutable release manifest → activate distribution endpoint/QR
```

**Build invariants**

- No release artifact may be signed with auto-generated debug `.keystore`, alias `vivim` with password `android`, or a key committed to source. Keep existing scripts as development build until release profile is in place.
- Never pass cleartext signing secrets on a command line or print environment in build logs. Prefer secured interactive entry, OS-protected secret reference or `apksigner` supported protected password input; audit processes/log access on Windows/Linux.
- New custom signer generation requires explicit recovery/export option and confirmation that lost key can make updates impossible. Prefer OS-secured vault for active signing, *encrypted* export with recovery verification. No unrequested export of private signing keys over network.
- Version code must increase for upgrades. Prevent publishing a different APK under an existing immutable release ID. Verify custom app label/icons and cert fingerprint from the final built APK rather than trusting input profile.
- Maintain package/channel/version/signing certificate lineage in a Host-owned artifact registry; do not store private keys in it.
- Timestamps, toolchain versions, input file hashes, version/manifest ID, output SHA-256, size, cert fingerprints, build log redaction and target device support must be captured in a redacted receipt.
- Reproducibility: compare content after normalizing known nondeterministic signing steps; investigate ZIP entry ordering, resource compilation and cross-platform `zipalign`. Don't promise byte-identical signed APKs before measured evidence.
- Source/script authority: currently `studio/android/build.ps1`, `build.sh`, `AndroidManifest.xml`, `res/*`, `MainActivity.java`; inspect their latest versions before modifying. If AndroidX/WorkManager/encrypted storage is selected, prove a sustainable toolchain and explain build-system migration.

## 4. QR installer page and verified artifact service

**Proposed routes** (not current):

- `GET /mobile/install` or `/mobile/apk/` — a minimal HTML landing with icon, version, size, package name, signer identity, verification notes and download action.
- `GET /mobile/apk/releases/current` — bounded metadata for current authorized profile, with opaque artifact ID and exact hash; never expose local paths.
- `GET /mobile/apk/download/<opaque-release-id>` — only frozen/verified file from local artifact registry, `Content-Type: application/vnd.android.package-archive`, `Content-Disposition: attachment; filename="vivim-personal.apk"`, `Content-Length`, `X-Content-Type-Options: nosniff`, `Cache-Control: private, no-store`, safe `HEAD`, if implementing ranges exact range limits.
- `GET /mobile/install/qr` — safe QR image/SVG from canonical URL, preferably rendered on **PC** UI and terminal. Avoid putting sensitive pairing codes into the QR URL (can leak via screenshots/history/referrer).

**Security**: Tailscale Serve HTTPS only, exact expected host/header guard, application-level device/purpose authorization for privileged routes; install bootstrap can be accessed pre-enrollment by tailnet user but must be read-only, rate-limited and time/bundle constrained. Permitlist signed release ID → immutable canonical local file, `realpath` containment, symlink and TOCTOU checks, size caps and stream cancellation. Do not accept `file=` query parameters or use guessed paths. Ensure download does not expose owner token or private data.

**Operator UX**: setup page checks Tailscale Serve and reachable phone before showing QR; never says *installed* from a GET/download. Browser may download APK or ask to allow unknown apps; Android installation must be completed by user. Provide fallback manual install + status diagnostics. QR quiet zone and contrast tested across phone cameras/sizes. Accessibility: copyable URL, readable version/cert text, one or more reviewable setup steps.

## 5. Enrollment after install — avoid a false 'single QR auto-pair' claim

Android package installation generally does not carry an arbitrary query parameter from the browser QR into the installed app. Strategies to evaluate:

1. **Preferred**: compile non-secret Host origin hint in profile; on first launch app contacts Host for device enrollment challenge; owner compares short code on PC and phone, then approves. This yields one install QR and no secret-bearing deep link.
2. **Fallback**: after install, a secure OS-mediated app link or second scan to prefill non-secret Host address; still requires explicit enrollment approval.
3. **Not allowed**: embed permanent Host bearer token, keystore/password or one-time owner-approved authorization into APK; APK can be extracted, shared and reused. No silent approval just because Tailscale user matches.

The local agent should choose based on realistic device behavior, not imagined install referral APIs.

## 6. Update contract

```text
ACTIVE vN → CHECK_SIGNED_RELEASE → VERIFY appId + signing lineage + versionCode
  → DOWNLOAD immutable APK → VERIFY package metadata + artifact digest
  → USER_REVIEW → ANDROID_PACKAGE_INSTALLER → ACTIVE vN+1
  → LOCAL_DATA_MIGRATE (transactional) → HEALTH_AND_HOST_COMPATIBILITY_CHECK
  → [SUCCESS | SAFE_RECOVERY]
```

- Users may refuse or postpone an update; app must show compatibility state without faking connection.
- Rollback to older APK may fail due to Android downgrade restrictions; treat recovery as **restore data/export** or compatible forward repair, not an automatic installation downgrade promise.
- Keystore rotation and signing lineage require Android-version compatibility research and a regression matrix. Don't assume v3 proof-of-rotation works identically on older API 26 devices.
- New appId means a parallel app. Offer explicit export/import migration; do not overwrite installed different-signer application.
- Retain previous signed APK and signed build manifest (excluding keys) for audit. Never serve artifact if signature verification fails or signing profile unexpectedly changed.
- Test same-app update with preserved local DB, drafts, owner grant, notifications and share intent; test wrong signer/version, interrupted download, Host offline, revoked device, bad bundled UI migration.

## 7. Android official research / pending verification

- https://developer.android.com/studio/publish/app-signing
- https://developer.android.com/developer-verification/guides
- https://developer.android.com/developer-verification/guides/faq
- https://developer.android.com/tools/apksigner
- https://developer.android.com/guide/topics/manifest/manifest-element

**Local investigation mandatory**: determine latest Android developer-verification behavior in the owner's jurisdiction and target users; separate developer registration, per-user signing, package registration, limited-distribution constraints and side-load UX. Verify against a physical phone and record OS version, Play Protect/installer messages; do not claim a verified rollout limitation not directly observed.
