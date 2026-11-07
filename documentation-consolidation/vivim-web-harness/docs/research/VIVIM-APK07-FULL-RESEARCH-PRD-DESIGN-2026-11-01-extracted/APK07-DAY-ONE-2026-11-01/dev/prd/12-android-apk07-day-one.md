# PRD 12 — VIVIM Android APK07: complete Day-One personal app

**Date:** 2026-11-01  | **Status:** owner target DECIDED; design proposed, implementation not accepted | **Priority:** launch-critical | **Target repository:** `owenservera/vivim-web-harness` | **Canonical roadmap authority:** `docs/roadmap.json` | **Purpose:** an implementable, evidence-aware product contract for the next local agent.

> **Owner decision**: the *first releasable Android APK* must pass the complete APK07 contract. Do **not** substitute a QR-only installer, hosted PWA wrapper, or limited Android MVP and call it finished. Coding may be staged in dependency order; release acceptance is unified. **No Vercel dependency**. Tailscale Direct PC ↔ phone is default. **No parallel Chrome controller, Playwright, Puppeteer or unauthorized provider send**. The PC Host retains canonical authority over PC data, provider actions and browser effects. The Android device must have genuinely useful, private, user-owned local capability without the Host online.

## 1. The product in one paragraph

From a working, owner-controlled VIVIM PC, a person opens **Create My Android App**, chooses an app identity/branding and signing ownership, verifies the build, scans one QR entry-point to install, follows Android's unavoidable installer permissions, launches the app, approves pairing with their PC, and immediately gains Talk, Build, World, natural-language settings and commands, native share/notifications, transparent job receipts and meaningful offline browsing/editing/export. The same codebase also produces a standard VIVIM-signed package. Updates preserve data and identity; the user can revoke devices, restore signing credentials with clear warnings, import/export their data and recover after network or PC loss.

## 2. Decision hierarchy and definitions

- **APK07 product** (this PRD): full personalized/installable/independent Android experience. **Different from existing `GUI07`**, which is a narrower Android bootstrap→Build acceptance suite. Extend GUI07 rather than overwrite it.
- **User-owned**: the user's PC controls generated artifacts, signing choice, personal data and release exports; recovery documented. *No claim that VIVIM can guarantee all future sideloads work without Google's verification or OS prompts.*
- **Personalized APK**: can personalize app label/icon/color/splash, configurable package/application ID, default non-secret Host discovery hints and bundled offline assets. Sensitive memories and irreversible permissions must **not** be compiled into the APK.
- **Independent operation**: install/launch, local browse/edit/search/export and queued **drafts** while PC disconnected. PC-hosted harness/browser execution remains unavailable until Host online and authorized. No fake offline AI send.
- **One scan**: one QR launches the install/enrollment *journey*, not a promise that scanning an APK link automatically transfers installer state/credentials into the installed app. The phone may need a short-lived match code or second explicit PC approval; OS installer confirmation cannot be bypassed.
- **First release**: all requirements in §§4–7 satisfy acceptance across real phone and Windows Host. Optional future experiments (on-device LLM, broad OS automation, cloud relay) are not Day-One gates unless explicitly authorized.

## 3. Personas and entry cases

| Persona/case | Baseline | Outcome |
|---|---|---|
| PC owner, first Android install | VIVIM setup complete; Tailscale on PC and phone | Customizable, release-signed QR installation; device pairing; Talk/Build/World |
| PC owner with no Android SDK configured | Normal non-developer Windows user | Standard prebuilt option; customization build service **on user's PC** only if dependencies can be managed automatically and securely; no command-line keystore ritual |
| Technical owner with custom signer | Wants independent application identity and key | Local key generation/import, export/restore and versioned APK factory; explicit signing/developer-verification consequences |
| Paired phone temporarily offline | No PC/Tailscale/Internet | Open familiar locally cached World and Build, edit local drafts, view job history as stale, export, queue only reversible/explicitly staged changes |
| Owner replacing phone/PC | Compromised/lost/new device | Revoke old credential, enroll replacement, restore verified data; no forged identity or silent send |
| Existing Studio LAN-paired user | Old HTTP URL and Studio cookie | Guided migration to Direct enrollment; preserve drafts, Talk continuity and owner-approved state; no surprise cloud fallback |
| Android install/update edge cases | OEM browser, Android 8–latest, unregistered app | Clear verified installer instructions, signed artifact hash/cert, honest Google verification/advanced flow guidance |

## 4. Requirements — all are first-release gates

### A — Custom APK Factory / Identity

**FR-A01** Single source tree builds standard and fully personalized APKs without source hand-edits. Inputs validated against a strict configuration schema; no injection into manifest, Java or aapt commands.

**FR-A02** User can set display name, icon, splash, theme/accent, app description and generated build profile; preview exactly what is compiled. Minimum acceptable custom identity is label/icon/theme; a custom application ID must also be supported in advanced mode.

**FR-A03** Package ID is unique and stable across subsequent updates; distinguish standard shared app (`inc.vivim.app`) from custom namespace. Changes to package ID create a separate installed app, not an in-place update; require explicit warning and migration plan.

**FR-A04** User chooses standard publisher signing OR user-owned signing. User-owned signer must support local generate/import, certificate display, secure backed-up recovery, no logging of passwords/private key, and clear consequences for lost keys. Never use test `.keystore` for production release.

**FR-A05** Version code monotonic, semantic release channel/version, min/target Android API compatibility, deterministic artifact metadata, signed SHA-256 manifest, signing certificate fingerprint and explicit release provenance. Rebuild equivalence should be defined and tested (byte-for-byte if feasible; otherwise normalized content/hash evidence).

**FR-A06** Windows-first and Linux-compatible build entry points. Capture JDK/SDK detection and controlled installation; do not demand global Android Studio installation from end users unless unavoidable. Build failure produces actionable diagnostics, not a partially released APK.

**FR-A07** SDK/toolchain choice resolved after audit. Retain Gradle-free for simple Java-only shell if it can support all requirements; prefer an Android-standard managed build for dependable AndroidX/notifications/background tasks/assets if evidenced. No needless rewrite.

### B — Distribution / Install / Updates

**FR-B01** PC Host serves an explicit install landing page and versioned APK over authenticated Tailscale Serve HTTPS with correct MIME, Content-Disposition, Content-Length, no-store, restricted path and safe file handling; never serve arbitrary filesystem path.

**FR-B02** Show QR from desktop and optionally terminal; QR encodes canonical short-lived install/enrollment landing URL, not APK bytes or permanent owner credentials. Support phone without camera by copyable HTTPS link.

**FR-B03** Installer landing identifies app, version, package ID, signer fingerprint and SHA-256 checksum, and presents correct Android unknown-app/developer-verification instructions. Must not claim Android can auto-install without OS consent.

**FR-B04** Release manifest supports current/past versions, compatibility bounds, expiration/revocation, immutable digest and measured artifact size; no unsigned/unchecked APK is offered as current.

**FR-B05** Updates are installed with Android's package manager and user consent as required, preserve package/signing lineage and local data. Distinguish UI asset updates from binary APK upgrades and roll back invalid content without replaying Host actions.

**FR-B06** Support download resume/range only if safely implemented and tested; otherwise reliable fresh retry for immutable artifact. Expired install enrollment must not break already-installed valid package or leak a permanent token.

### C — Connectivity / Pairing / Device Trust

**FR-C01** Tailscale Direct HTTPS default, with no Vercel requirement and no silent cloud fallback. Document tailnet setup, HTTPS cert/MagicDNS requirements and Serve configuration ownership; do not use Funnel for private enrollment.

**FR-C02** Secure onboarding: Host issues short-lived single-use challenge; phone creates local device keypair (Android Keystore when appropriate), proves key possession and presents a pairing code; owner explicitly approves device/role/scope on PC or a securely validated reciprocal flow.

**FR-C03** Host stores revocable device ID, public key fingerprint, enrollment status, scope set, created/last-seen, expiry and revocation timestamps; no blanket authority from matching Tailscale user's email.

**FR-C04** Every mutating request has authenticated device proof (mechanism selected and tested: challenge signing/mTLS or proof-of-possession session), replay protection, freshness window, scoped permission, idempotency key and Host receipt. Tailscale identity remains network perimeter, not application authorization.

**FR-C05** Reconnect after Host/Tailscale restart and PC hostname change using an explicitly approved rebinding; never silently accept a different origin due to a deep link or broad `http(s)` parser.

**FR-C06** Per-device revoke, sessions expiry, app lock and reset. Host detects unknown/deleted device and gracefully shows re-enroll without writing. Trusted provisioning must not depend on hidden Android install referrer behavior that direct APK sideloading does not reliably provide.

### D — Native Android Integration

**FR-D01** Safe WebView shell with signed bundled offline pages + remotely served Direct UI; same-origin navigation and strict intent/URL validation. Disable broad WebView file/content access and unsafe JavaScript bridges; use HTTPS asset loading for local static content.

**FR-D02** App lifecycle: cold start, resume, rotate, keyboard, back, process death, WebView renderer crash, screen lock, Android app update, Host outage; no loss of unsent drafts.

**FR-D03** Native share ingress text and attachments; review/edit before sending. Use Android picker/SAF for user-selected files; no ambient storage scanning and no auto-upload of camera/photos/documents.

**FR-D04** Deep link / pairing action requires explicit origin and identity confirmation; disallow arbitrary schemes, redirect to installed untrusted apps or injected owner commands.

**FR-D05** Notifications for consent-needed/completed jobs, opt-in categories and Android 13+ permission flow; locally generated notification from securely delivered events, not an insecure push token. Must still work reasonably without always-on background service.

**FR-D06** Permissions requested at feature use, with graceful no-permission fallback; no accessibility-service, root, device-admin or screen-capture permissions for first release without separate specification.

**FR-D07** Support privacy screen behaviors (sensitive preview, app lock, recent-app thumbnails policy where needed), Android backup/export policy and power/foreground constraints without misleading reliability promises.

### E — Personal World / Generative UI

**FR-E01** Retain one shared Talk/Build/World UX and existing declarative renderer. Respect `vivim.ui-proposal/1`, Host `vivim.ui-revision/1`, stable IDs, preview/CAS/Undo and private-scoped projections.

**FR-E02** BF00 immediate first paint, grounded F02/F01 and optional D03–D07 proposals use the existing validated bootstrap compiler and Studio owner; no second AI send for conversion. Complete prior `GUI07` real-device acceptance.

**FR-E03** User can design/arrange a personal interface on phone via text, tap, accessible gesture alternatives, safe UI component catalog and undo; preserve Talk drafts and protected Personal AI entry.

**FR-E04** World lists/cards/search/inspect/edit show provenance, owner-confirmed vs model-proposed distinctions, privacy scope and current vs cached indicators. No sensitive D06 defaults on phone.

**FR-E05** User-created views and theme updates are versioned; local previews are immediate, trusted Host mutations only after authorized CAS. Local-only changes have explicit owner and reconciliation semantics.

### F — Harness / Natural-Language Control

**FR-F01** Every user-facing action and setting is discoverable and operable by text and GUI through the **same Host action IDs**, per `docs/design/UNIVERSAL-TEXT-CONTROL-V1.md`. `unavailable` shown when no verified executor.

**FR-F02** Deterministic parser for known typed commands; AI interpreter only proposes untrusted `actionId + args` candidates via existing signed-in webapp harness. No model-granted privileges, arbitrary shell/SQL, or duplicate browser owner.

**FR-F03** Risk R0–R3 permissions, previews, scoped approval, freshness and revocation; exact effect owner executes; journal differentiates queued/claimed/applied/verified/failed/**uncertain**. Never auto-resend uncertain provider sends.

**FR-F04** Phone Talk surfaces real provider capabilities/readiness derived from current harness owner—not fallback `signedIn:true`—including model and thinking-level support only when verified executable.

**FR-F05** Phone can initiate/monitor/cancel where safe: approved Studio provider conversation, OpenCode existing session, World edit, UI revision, personal context preview, settings; distinguish a remote plan from an executed effect.

**FR-F06** Every completed phone effect has correlated device ID → request ID → Host action → journal/send-once receipt → provider thread/World revision → phone rendering acknowledgment. Sensitive body stays out of logs/QR/telemetry.

### G — Offline / Independence / Export

**FR-G01** Android includes offline-capable UI assets (not solely WebView to PC). Explicit cache version, migration/version fallbacks; last known-good UI available after PC disappearance.

**FR-G02** Local encrypted or platform-protected data cache/storage scoped to approved phone projections. Device-owned drafts, settings and explicitly offline-editable World subset survive process death. The PC remains canonical for PC-owned data.

**FR-G03** Local reads/search/export and staged edits work without network. Side-effectful PC execution is not queued for automatic replay; offline action requests remain **drafts awaiting fresh authorization**.

**FR-G04** Reconciliation uses versioned object revisions, CAS/conflict states, tombstones and idempotency. No last-write-wins overwrites of canonical memory or owner hard NOs; deletion/revoke honored on reconnect.

**FR-G05** Export JSON-based documented archive with schema/version/manifest/checksums, optional encrypted sensitive payload and explicit file picker; import validates limits, schema, privacy and conflicts. PC/phone data ownership documented.

**FR-G06** Local key loss, app reinstall, signing change, phone theft, backup restore, incompatible schema and expired credential recovery guides. Explain what data is unrecoverable without backup.

## 5. UX acceptance journey (physical Android; required)

1. **Setup PC**: Host/Studio/owner profile/Tailscale Serve healthy. UI shows exact service readiness and no phantom signed-in providers.
2. **Customize**: user chooses profile (publisher-signed or user-owned), icon/name/theme/package, sees estimated download and signing/recovery policy; cannot hide key ownership consequence.
3. **Build**: PC produces signed APK + digest/cert/version manifest, verifies SDK/keystore, forbids debug signing for final release, preserves version lineage.
4. **Scan**: desktop QR opens Tailscale HTTPS landing on phone; Android unknown-app and, where relevant, verification steps are explained; first-time install explicitly approved by OS.
5. **Launch**: offline splash/local UI works before any Host contact; app identity and signed package match expected.
6. **Enroll**: device generates key; Host asks owner to approve *this* phone via match code; scopes and expiry visible. QR alone does not authorize.
7. **Talk**: show verified available provider; send once; exact provider/thread and response displayed on phone. Offline/unavailable/uncertain are distinguishable.
8. **World + Build**: first paint fast; retrieve allowed World claims with evidence; apply approved UI edit, Undo, context-scoped AI design proposal; preserve draft.
9. **Control**: natural-language preview of a setting, approve and execute through same typed action as GUI; verify durable receipt.
10. **Native**: Share from another Android app populates a draft; opt-in notification delivered; revoke device blocks subsequent commands.
11. **Go offline**: airplane mode or Host down; local World/drafts/export remain; no server effect silently executed; reconnect resyncs conflicts honestly.
12. **Upgrade/restore**: signed vN+1 installs over vN, data survives and app still pairs; backup exported/imported into a clean authorized install; incompatible signature updates are refused with explanation.

## 6. Quantified product/service targets (proposed budgets, measure not promises)

- Cold offline launch to usable local UI: p95 <= 1.5 s on selected reference Android device; BF00 local provisional paint < 500 ms when feasible.
- Immediate local tap acknowledgment: p95 < 100 ms; no AI dependence.
- Direct authenticated Host read (warm, healthy local tailnet): p95 <= 1 s; record actual cellular+DERP path separately without invented latency.
- Verified Host-approved UI revision→phone visible: p95 < 500 ms under stable connection, as inherited from GUI07 spec; log assumptions.
- No unconfirmed provider send auto-replayed (zero tolerance), zero unapproved device actions and zero sensitive fields in public/Cloud status (zero tolerance).
- Artifact reproducibility: identical inputs/toolchain yield consistent content and signing evidence; distinguish deterministic unsigned payload vs nondeterministic signing timestamps if needed.
- Compatibility: minSDK remains 26 unless explicitly changed by a documented compatibility study; test at least Android 8/9, 13, 14 and current reference OEM/Android available locally; no untested universal device support claim.
- Availability: Host offline clearly labeled; local World and drafts remain accessible; no fabricated live completion.

## 7. Non-functional security, quality and release constraints

- **Threat model mandatory** for install token capture, intercepted deep links, compromised phone, stolen keystore, Tailscale shared users, forged proxy header, malicious HTML/model output, replay, Host compromise, origin hijack, lost key and cache leakage.
- **Boundary:** phone never gets PC owner-token or unrestricted raw Host filesystem access. Tailscale identity and HTTPS alone do not grant VIVIM mutations. HTML renderer cannot access a powerful JS bridge by default.
- **Privacy:** privacy tiers and D06 restrictions enforced on Host projection and local storage; no blanket sync of user's personal memory to phone or provider.
- **Release:** signed manifest and artifact, deterministic test/verification receipts, install/update/revoke/restore smoke results; archived last-known-good APK and secure signing backup.
- **Observability:** redacted structured receipts, no raw prompts, private notes, QR tokens or owner capability in telemetry. Source SHA and Android version included.
- **Compatibility:** Windows-first with Linux build/Host parity. No deployment prerequisite to Vercel. No separate PC Chrome owner/Playwright/Puppeteer.
- **Regression:** existing Talk, Studio exact-thread, Studio profile ownership, bootstrap, JSON World, GUI07 and UTC invariants unchanged unless intentionally, safely migrated.

## 8. Dependencies and architecture owners

| Area | Canonical source/service | Existing roadmap links | APK07 addition |
|---|---|---|---|
| Build | `studio/android/` | P01/PACK01 | factory, release signer, custom resources, reproducibility, toolchain |
| Distribution | `apps/vivim-host/direct-phone.mjs` | DIR01 | signed artifact registry, installer landing, QR, immutable download |
| Android native | `MainActivity.java` | P01/GUI07 | keystore identity, offline assets, Share/notification, lifecycle |
| Control | `apps/vivim-host/control/*`, `docs/design/UNIVERSAL-TEXT-CONTROL-V1.md` | UTC01–UTC07 | enrolled device ingress, typed execution, mobile text parity |
| Browser actions | Studio/CDP and harness send-once | H02, H07, P05 | exact phone correlation; no additional browser owner |
| Personal World | Studio World/Host/bootstrap | W01–W04, DATA*, DCB* | scoped phone replica, offline rights, export/import |
| UI | `apps/mobile-relay/generative-ui/*` | GUI01–GUI07, BF00–BF03 | Android packaged renderer and native acceptance |
| Runtime/lifecycle | `apps/vivim-host/server.mjs`, startup suite | START01–START03, HOST01–02 | Android onboarding checks, package update, connection recovery |

## 9. Release governance and blockers

**All seven lanes A–G are P0 for first release.** Use `docs/roadmap.json` as **the single authoritative execution ledger**: map the machine-readable APK07 subtask list in this package to those existing rows and add only truly new rows. Do not create a parallel master TODO. The machine-readable task file is a *draft import manifest* and is not another status authority.

A release is **BLOCKED** if any of these are unproven: production signer/update lineage; custom build actually installs; Host-approved per-device enrollment/revocation; natural-language action execution with verified receipt; real Android Talk/Build/World; packaged offline UI/data/edit/export; upgrade and restore; security negative tests; signed-in provider readiness; draft continuity. Missing physical-device access must remain an explicit `awaiting_device_acceptance` rather than be silently waived.

## 10. Decisions to resolve through local investigation

1. Android build toolchain: Gradle-free vs modern Gradle/AndroidX, weighed against native feature requirements and maintenance.
2. Package identities: immutable `inc.vivim.app` standard vs custom application ID namespace; ensure no collision/signature overwrite.
3. Signing choices: publisher-signed standard and user-owned custom, plus optional custom-brand-but-publisher-signed (only when policy permits); account for Android developer verification globally by 2027.
4. Enrollment bootstrap: install landing → launch with non-secret Host hint + short match code vs verified app-link; direct install cannot assume an install referrer.
5. Local data store: Android SQLiteOpenHelper/SQLite or library, keystore/SQLCipher only if dependency+threat tradeoff proven; never store keys as plain SharedPreferences strings.
6. Offline Host UI assets: packaged HTML/JS assets with secure virtual HTTPS origin, and compatibility with remote Direct `apps/mobile-relay` content.
7. Notifications: local polling/periodic reconciliation vs permitted system background work; avoid unsupported always-on assumptions.
8. Scope of external custom branding: package IDs and icons feasible; guarantees of Google registration, trademark or Play approval are *not* included.

## 11. Related documents in this package

- `docs/research/APK07-current-state-evidence-2026-11-01.md` — source audit, evidence grades and contradictions.
- `docs/design/APK07-architecture-and-protocols.md` — component diagram and schema contracts.
- `docs/design/APK07-factory-signing-distribution-updates.md` — custom build, signing and updates.
- `docs/design/APK07-security-threat-model.md` — trust and adversarial test plan.
- `docs/design/APK07-mobile-runtime-world-offline.md` — native, storage, data, local UI.
- `docs/design/APK07-ux-and-real-device-acceptance.md` — user journeys and physical phone test matrices.
- `docs/agents/APK07-LOCAL-AGENT-INVESTIGATION-AND-EXECUTION.md` — **mandatory agent handoff**, precedence, inspection, commands, decisions, receipts.
- `docs/roadmap/APK07-task-ledger.v1.json` — proposed subtask/acceptance import, subordinate to canonical roadmap.

## 12. Definition of success

A real owner can get a uniquely branded, appropriately signed personal Android app from their own PC without operating Android dev tooling, install via Tailscale-hosted QR, explicitly authorize the device, independently access/correct/export a meaningful local Personal World while disconnected, remotely command the PC via typed, scoped natural language when online, safely experience native Android features and user-generated Build UI, and update or recover without broken signer identity or lost data. Every step must have actual-device evidence, not only checked-in source or unit tests.
