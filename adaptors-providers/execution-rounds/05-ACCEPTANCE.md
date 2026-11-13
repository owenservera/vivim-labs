# R002 mock-complete acceptance and live exclusions

**Mock target**: one canonical in-ARBOR Hub backend, 10 provider packs, 4 normalized surface families (social/messaging/media/email), 20+ synthetic user scenarios, seeded multi-account fixtures, source-scoped stream pagination, four user surfaces (provider setup, unified inbox, feed/media, conversation/composer), in-ARBOR Host/PWA handlers and personal scoped graph proposal, non-bypassable mock intent→approval→receipt. No second owner or donor runtime dependency.

**Must-pass negative tests**
- Switch Facebook account after Messenger draft: refuse or invalidate.
- Change recipient/thread from preview to claim: refuse exact binding.
- Deny or revoke grant mid-import, before proposal, and after cached projection: refuse/invalidate.
- Simulate claimed send and crash before confirmed receipt: UNCERTAIN forever absent external independent evidence; never retry.
- E2EE WhatsApp/Messenger conversation not available in browser: explicit partial/unavailable state, no invented body.
- Import same archive and repeat same stream cursor: no duplicate evidence/graph units.
- Cross-personal World graph source references: rejected.
- Prompt-injection in untrusted source content: inert source text; no command execution.
- Unsupported provider capability: typed UNSUPPORTED, not pretend send.
- Provider origin/identity mismatch, rate-limited/locked session, revoked linked device, offline: scoped refusal.
- Synthetic source tries external fetch or credential read: trap/refuse.
- PWA offline draft never auto-replays an effect on reconnection.

**Status labels**: CATALOGUED; MOCK_RUNNING; ARBOR_SOURCE_WIRED; LIVE_READ_VERIFIED; LIVE_WRITE_VERIFIED; DEVICE_VERIFIED; BLOCKED; UNRUN. Mock coverage never upgrades a live state.

**Verification reporting**: per lane exact file SHA/branch, tests with command and exit, setup, fixtures, platform, negative failures and uncertainties. No honest basis for live provider support, Windows packaging or physical PWA acceptance in the document-only program.

**Future live gates, not part of these seven waves:** account-scoped user permission, current provider UI/terms, hand-verified browser read target, independent outbound readback, exact Host dispatch/receipt, E2EE exceptions, retention/revocation with real data, Windows signed package and physical paired phone.
