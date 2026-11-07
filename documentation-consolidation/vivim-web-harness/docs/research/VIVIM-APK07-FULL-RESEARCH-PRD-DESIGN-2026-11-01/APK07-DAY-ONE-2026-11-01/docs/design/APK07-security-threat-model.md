# APK07 security architecture and threat model

**Scope:** personalized native packages, artifact distribution, local Host connection, per-device authorization, data sync, WebView, offline cache, provider execution and recovery. Treat this as a required P0 design/gate, not security theater. Every mitigation needs a test and actual owner.

## 1. Protected assets and principals

**Assets:** release signing private key, Android device key, Host owner auth token, World/private claims/hard NOs, session credentials, browser CDP profile, provider threads, unsent drafts, local Android cache, immutable signed APK, user-exported archive. **Principals:** PC owner at console, authenticated tailnet user, enrolled Android device, installed APK/OS, remote WebView content, provider output (untrusted), third-party shared web content (untrusted), malicious/compromised phone, LAN adversary, OS/browser installer.

**Trust boundaries:**

1. Internet/LAN/tailnet ↔ Tailscale Serve ↔ localhost Host (`tailscale-user-login` is network identity only).
2. Host ↔ Android per-device cryptographic session and scoped grant.
3. Android native privileged methods ↔ WebView JavaScript (default no exposed privileged bridge).
4. Model output ↔ proposal compiler and policy engine (never privileged code).
5. Host canonical World ↔ privacy-scoped Android projection/local cache.
6. Build customization input ↔ local tools/signer (no injection of arbitrary shell or XML).
7. Installer-page metadata ↔ actual installed package (cert and artifact binding must be verified).

## 2. Threat → mitigation → negative acceptance

| ID | Threat | Required defense | Test failure mode |
|---|---|---|---|
| SEC-01 | QR screenshot/reused landing URL | QR contains no permanent secret; TTL/single-use enrollment challenge, short bounded release URL | Reuse expired QR cannot enroll/revoke or access private state |
| SEC-02 | Impersonate phone under same Tailscale login | Enrolled device public key + proof-of-possession + Host-scoped grant | Other device on same tailnet cannot mutate |
| SEC-03 | Forge `tailscale-user-login` header by bypassing proxy | Host loopback binding + exact proxy guards + no LAN raw access | Direct request with injected headers from LAN refused |
| SEC-04 | Unauthorized enroll by shared tailnet user | Owner approval, match code, rate limiting, role/scope | Forged approval or guessed code fails |
| SEC-05 | Replay/mutate stale request | Nonce or signed timestamp, idempotency, base revision, expiry | Duplicate request no double effect; stale rev denied |
| SEC-06 | Uncertain provider send mistakenly resent | Send-once journal, exact thread, uncertain reconciliation | Kill connection after send: no automatic resend |
| SEC-07 | Rogue app intercepts custom `vivim://pair` | Non-secret deep-link, exact origin/Host fingerprint confirmation | Crafted external `pairUrl` never silently swaps PC |
| SEC-08 | Cross-origin WebView navigation steals cookies | Strict origin allowlist, app-external browser, CSP, no unsafe bridge | Foreign page cannot call Host/native privileged methods |
| SEC-09 | Cleartext LAN token sniffing | HTTPS-only Direct default, scoped LAN compatibility; never broader `usesCleartextTraffic=true` without review | HTTP public origin denied; explicit LAN flow warns |
| SEC-10 | WebView unsafe file/content access | file/content access disabled; use HTTPS asset virtualizer, no `file://` local UI | `file://` / mixed content / external scheme blocked |
| SEC-11 | Personal memories in compiled APK or QR | Non-secret build profile; private data in authorized encrypted/scoped store only | Inspect unpacked APK/QR: no raw secrets or memory |
| SEC-12 | Debug/forged APK served | Release signer, immutable manifest, hash/cert compare, path allowlist | Changed bytes, wrong cert, bad version rejected |
| SEC-13 | Signer key loss/theft | Secure key storage, encrypted owner backup, recovery drill; never print passphrases | Loss/compromise documented; unauthorized use refused |
| SEC-14 | Compromised phone reads cached World | OS-backed lock/encryption, minimal projection, scoped retention and sensitive-data opt-out | Offline stolen phone risk acknowledged; no broad PC-secret replica |
| SEC-15 | Revoked offline device retains stale data | Local lock/TTL/visibility policies; fresh grant before write; tombstones on reconnect | Revoked phone cannot execute after reconnect, stale cache warnings |
| SEC-16 | Sensitive D06 leakage to phone/relay | Host privacy projection, explicit per-field consent, tests | D06 private fixture absent from phone and export by default |
| SEC-17 | Prompt injection executes OS action | Parser→candidate→typed Host plan→approval→exclusive effect owner | Malicious AI text cannot grant or execute |
| SEC-18 | Build profile injection into manifest/signing shell | schema-bound resource generation, isolated build, no exec string interpolation | XML escaping, shell metacharacters and path traversal rejected |
| SEC-19 | Export archive leaks credentials | Owner confirmation, manifest content class, encrypted option and SAF picker | No hidden CDP/owner/signing key in JSON by default |
| SEC-20 | Denial-of-service via download/enrollment | max body/file size, limited concurrent streams, rate limit, TTL | Exhausted/invalid challenge bounded |
| SEC-21 | Malicious content via share intent/attachments | explicit selection, MIME/size validation, sanitized preview and no auto-send | Shared crafted content cannot auto-run/send |
| SEC-22 | App update changes package/permissions unexpectedly | inspect final APK identity/cert/permission diff before offering | Wrong app ID/cert/permission jump blocked |
| SEC-23 | Competing browser driver causes duplicate sends | existing Studio exclusive Chrome owner; Android only Host typed actions | Second owner startup refused/never attempted |
| SEC-24 | Network toggle remotely strands owner | explicit recovery and local PC authorization for R3; unavailable until verified | Remote loss-of-control policy tested |

## 3. Consent rules

- R0 read: device authentication and privacy projection; safe bounded reads.
- R1 reversible local: typed plan and CAS, owner prior scope only when explicitly granted, durable receipt.
- R2 privacy/provider-service settings: fresh explicit consent, risk preview, expiry; no bootstrap blanket approval.
- R3 external sends, irreversible actions, credential/OS/network changes: exact scoped owner approval, independent Host effect-owner check, possibly physical PC confirmation. Phone text cannot transform an R3 into R0.
- Revocation blocks *new requests*. A phone that is offline cannot be remotely erased or forced to forget instantly; be honest and design local key invalidation/lock controls.

## 4. Safety tests and operational review

Run static checks, unit/property/fuzz tests, Host integration, Android instrumented and physical device negative scenarios. Trace request ID, source commit, device fingerprint redacted, platform build, grant version, artifact SHA and outcome. Never collect raw owner tokens, live Tailscale secret link, full provider content or signing passwords as test artifacts.

**Release condition:** independent reviewer finds no open critical/high threat with exploitable path; all SEC-01..SEC-24 mapped to code/test/real-device receipts or explicit documented platform limitation. Tests alone do not establish production compromise resistance, but missing adversarial tests are release blockers.

## 5. External standards

- Android Keystore: https://developer.android.com/privacy-and-security/keystore
- WebView file risks: https://developer.android.com/privacy-and-security/risks/webview-unsafe-file-inclusion
- Tailscale Serve identity: https://tailscale.com/docs/features/tailscale-serve
- Android signing: https://developer.android.com/studio/publish/app-signing
- Android SAF: https://developer.android.com/training/data-storage/shared/documents-files
