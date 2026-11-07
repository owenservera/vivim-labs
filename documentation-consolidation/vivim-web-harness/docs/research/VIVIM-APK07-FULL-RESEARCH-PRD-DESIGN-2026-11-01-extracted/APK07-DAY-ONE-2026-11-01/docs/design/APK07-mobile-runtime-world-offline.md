# APK07 Android runtime, mobile Personal World and offline architecture

## 1. Starting evidence and change boundary

Current `MainActivity.java` implements a Java-only WebView and pairing/recovery; no bundled web UI. Current `apps/mobile-relay/app.js` persists a localStorage conversation list and has Talk/Build, but not an accepted canonical offline World replica. Current `apps/mobile-relay/generative-ui` provides safe declarative renderer/semantic projections; Studio World/Host hold separate durable data. **Do not replace these wholesale.** Create a thin native platform layer around the existing shared presentation/control contracts, and verify browser-to-native interfaces on a device before activating them.

## 2. Runtime layout

```text
Android VIVIM APK
 ├─ Native shell (MainActivity, lifecycle, permission prompts, identity gate)
 ├─ Trusted bundled offline UI assets + safe virtual HTTPS origin
 ├─ Direct remote UI client (Tailscale Serve; explicit trusted Host)
 ├─ Device identity (Android Keystore, session verifier, revocation view)
 ├─ Native service adapters (Share, file picker/export, notifications, deep links)
 ├─ Local data store (revisioned cached World projection, settings, drafts, receipts)
 ├─ Sync/reconciliation engine (bounded, authenticated, no uncertain replay)
 ├─ UI engine (shared declarative catalog/renderer, Talk/Build/World)
 └─ Diagnostic/recovery screen (identity, Host reachability, storage schema, release)

PC VIVIM Host
 ├─ Owner/auth/device-grant registry
 ├─ Canonical World/SQLite/VersionedStore
 ├─ Universal typed action catalogue + effect journal
 ├─ One Studio/CDP webapp owner
 ├─ Device-scoped privacy projection & version change feed
 └─ Signed artifact registry / APK installer
```

## 3. Offline capabilities (real, not simulated)

| Offline feature | Phone-local authoritative state? | Host reconciliation |
|---|---|---|
| Launch/shell/selected theme | Yes (signed packaged assets and local choice) | Sync only accepted shared theme changes |
| Browse cached World and provenance | Local **copy** only | Host canonical; stale banner and revision freshness |
| Local search within authorized cached records | Local index of approved projection | Rebuild on approved changes/revocations |
| Draft Talk, text actions, UI proposals | Yes, **DRAFT** only | Explicit review/send on reconnect; never automated browser send |
| Local-only notes/view preference | Yes with local object IDs | Optional reviewed sync to Host if authorized |
| Offline object edits | Pending change proposal; not canonical | Rebase/review CAS conflicts; no silent memory overwrite |
| Export selected local data | Yes via SAF and user intent | Provenance includes cache revision/missing omitted records |
| Host provider/harness job | No | Status `host-unavailable` / `uncertain` instead of success |
| Full personal memory, credentials, D06 sensitive | No by default | Only specific owner-approved scoped replica |

**Do not promise offline on-device model reasoning** unless a separate explicit model/size/privacy/benchmark acceptance exists. APK07 includes robust deterministic local action preview where data exists; PC/browser AI actions remain Host-bound.

## 4. Store design

Suggested local tables (logical; select SQLite/Room/SQLiteOpenHelper after local toolchain evaluation):

- `device_meta`: appId, releaseId, protocol min/max, enrolled Host fingerprint, lastSync, local-schema version.
- `world_projection`: opaqueId, type, contentDigest, ownerStatus (`confirmed`, `proposed`, `rejected`), provenanceRef, sourceRevision, privacyClass, encrypted payload, tombstone, expiry.
- `drafts`: draftId, kind (`talk`, `ui`, `control`, `note`), bounded payload, created/updated, target/source, status, intended effect, never default `sendNow`.
- `local_preferences`: key, value, revision, deviceOnly/shared designation.
- `receipts_cache`: requestId, status, observedAt, source Host timestamp, limited display body/digest, freshness.
- `sync_cursor`: Host ID, projection scope, change cursor, last synced revision, schema, last acknowledged device grant.
- `change_proposals`: local ID, canonical target ID, baseRevision, requested patch, conflict status, owner approval state.

Minimize private cache, encrypt sensitive entries if implemented with tested keys, avoid localStorage as the only durable draft store. Use Android Keystore for key protection, not as if it automatically encrypts arbitrary SQLite files. A native bridge cannot expose raw key material or arbitrary SQL. If using `WebViewAssetLoader`, account for dependency/toolchain; otherwise implement safe `shouldInterceptRequest` equivalent with well-defined HTTPS virtual origin and no insecure file privileges.

## 5. Sync algorithm (proposal)

```text
On launch:
   load signed local UI → load device-lock status → show local drafts and approved cache
   attempt Direct/Host readiness without changing local drafts
   if host reachable: validate Host identity and device grant
   negotiate schema/version/capabilities
   fetch bounded projection changes since trusted cursor
   verify revisions/tombstones and commit local transaction
   show new live state + lastSync, or explicit conflict/unavailable

On local edit:
   change only an offline-draft or device-only object
   assign unique localId and baseRevision; persist before rendering success
   when Host returns: preview diff + confirm explicit mutation
   Host CAS executes and returns receipt → phone marks canonical
   stale/revoked/privacy-changed target → conflict/refusal; no overwrite
```

**No generic write-back queue**. Queueing a draft is not queueing a future irreversible effect. Provider messages, system control, token/private-field changes and externally visible actions must be freshly approved once online. Only idempotent local changes can be automatically reconciled if the Host's policy explicitly allows them.

## 6. Native integration specs

**Share**: maintain `ACTION_SEND text/plain` behavior and extend to `ACTION_SEND_MULTIPLE`/URI documents only with explicit user-selected files; show editable import preview; no automatic send; size/MIME limits, SAF grants and URI lifetime/revocation.

**Notifications**: generate device-native notifications for Host events that require decisions or show verified completion. Declare/ask `POST_NOTIFICATIONS` on API 33+ at first relevant use; categories configurable through text and GUI; provide fallback inbox when denied. Avoid promising push while PC offline without an approved transport/background delivery mechanism.

**App intents**: `vivim://pair` currently exists; reject credentials in deep links; use Host fingerprint/owner confirmation, validate schemes and path, show raw expected origin, never silently replace paired PC. Prefer verified app links only when `.well-known/assetlinks.json` can be controlled for the actual hostname and signing cert; local agent must prove ownership and behavior.

**Camera/file/clipboard**: system picker over broad storage permission; avoid camera/audio/contacts by default. When invoked, show data destination/preview first, with permission revocation path.

**Android lifecycle**: preserve forms on back, rotation, renderer death, Android force-kill, screen lock and update. Distinguish `unavailable`, `offline-draft`, `queued`, `claimed`, `uncertain`, `verified`. Never show green from presence of a fallback provider or a stale cached heartbeat.

**Accessible touch**: 44–48 CSS px design targets on dense mobile controls; all drag/long-press/swipe actions have tap alternatives, keyboard/screen-reader labels, safe-area/IME behavior, reduced-motion support. Respect text selection and system back.

## 7. Generative UI and World data boundaries

- Reuse `vivim.ui-surface/1`, `vivim.ui-proposal/1`, `vivim.ui-revision/1`, GUI01–GUI07; do not ship a new untrusted HTML/JS interpreter for generated apps.
- The same F02/F01/D03–D07 bootstrap data renders on one stable Build surface. D06 remains private unless explicit approved projection.
- Knowledge object ID, UI instance ID and action/intent ID are separate; spatial placement must not duplicate knowledge facts.
- Phone may locally preview style/edit without AI. Host commit requires `baseRevision`/CAS and privacy-aware authorization. Undo restores a local UI revision, not an irreversible provider send.
- Use `docs/ux/schema/mobile-interaction.schema.json` as a proposed mapping catalog and validate which interactions are actually executed; a gesture is never a grant.
- Dynamic Context Bundles are Host-scoped read/projection/approval features, not direct dumps from the phone cache to arbitrary webapp prompts.

## 8. Privacy and data retention

- Owner chooses which sensitive World classes appear on Android; default phone profile should exclude D06, full provider raw conversations, owner tokens and system secrets.
- Display source (`owner confirmed`, `AI reported`, `tentative`) and age. Deletion tombstones must wipe local cached copy when online. Offline revocation cannot erase already downloaded data instantaneously; PIN/biometric app lock and storage encryption mitigate but do not eliminate physical compromise.
- Export JSON with schema, manifest, checksums and source/provenance; do not export private signing key as World data. Signing key backup is a separate explicit encrypted user action.
- Import validates schema, max size, IDs and privacy class; preview changes/conflicts; no implicit execution/memory promotion.

## 9. Acceptance scenarios

1. Install APK, turn Wi-Fi/Tailscale off, kill process, reopen: local UI + cached data + drafts available, timestamped stale banner; no web error shell.
2. Draft a Talk message offline, reconnect: draft remains unsent until explicit owner tap; on send a unique idempotency key is minted once.
3. Edit a World object offline, meanwhile change it on PC: conflict shown, PC canonical untouched, owner can rebase or discard.
4. Revoke phone on PC while phone offline: old cache remains behind local device lock; reconnect denies all Host mutations and triggers bounded wipe/redaction per policy.
5. Update APK without changing signer/app ID: local cache/schema/drafts survive and migrate transactionally.
6. Share text from a third-party app: VIVIM opens draft in existing Talk surface; no network send; no leaked text query parameter.
7. Enter Build, create direct-manipulation proposal and Undo: stable UI revision, unchanged Talk draft and no raw provider resend.
8. Confirm small-screen, keyboard, landscape, Talk scroll, WebView crash, OS Back and permission-denied fallbacks.
