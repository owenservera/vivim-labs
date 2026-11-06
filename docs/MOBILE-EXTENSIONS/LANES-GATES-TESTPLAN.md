# Four-lane lab execution and gated build plan

Status: Wave 4 prep; NO TierFlow W4/Harness/Android/KB source authority; NO MiniMax delegation. 4 lab lanes are not additions to TierFlow's eight W4 A–H lanes.

## M1 Host primitive source audit (research-now)
Inputs: SOURCE-PRIMITIVES.md; harness/src/core/{cdp,pinchtab,send,journal}.mjs; studio/src/{cdp,chrome,capture}; Host catalog/dispatcher/receipts. Deliver explicit Page/Browser method table: target origin/session identity; DOM; navigation; input; preflight; journal; observed reply; missing APIs; active consumers; deployment-only extras. Every row has source path and confidence.
Done when browser-facing P0 primitives and their upstream/downstream dependencies are enumerated, with source-only versus live distinction. W4 consumers G/H.

## M2 Browser extension and engines (research and isolated fixture)
Inputs: BROWSER-ENGINE-DECISION.md, current passive Browser Companion and Firefox/GeckoView/WebView/Quetta docs. Deliver browser/API/permission/lifecycle comparison and one synthetic per-browser DOM observation experiment. Preserve Companion passive mode. No live provider sends, page data, or arbitrary origin injection. W4 consumers B/C/D.
Done when supported/unavailable/unknown per primitive is evidenced by exact browser version + real device or honestly marked unrun.

## M3 Native APK capability broker (research and build plan)
Inputs: studio/android manifest, MainActivity, build scripts, PocketStore/Home, user consent. Deliver existing permission inventory, component packaging, lifecycle, background constraints, signed package update risks, and proposed broker interface. Prioritize normal signed APK features and explicit user-granted APIs; ADB/Shizuku/root/enterprise are diagnostic/optional tiers only.
Done when no OS permission is accidentally assumed and existing one-APK, Pocket capture and enrollment identity are preserved. W4 consumers B/D/E/H.

## M4 Integrity and acceptance engineering (independent fixture now; real devices later)
Inputs: single Host effect/send authority, current Browser Companion origin/ACK protocol, TierFlow typed IR, Android owner grant. Deliver test cases and evidence schema; synthetic contract tests; device/build plan. W4 consumers C/D/F/G/H and Labs boosters 04/05/06/08/09/10/11/12.
Done when every proposed capability has happy, denied, expired, offline, crash/unknown and revocation outcomes; no effectful case is executed without owner grant.

## Gates — lab-specific labels, not TierFlow GO
| Gate | Required proof | Allowed now? | Does it authorize live source? |
| --- | --- | --- | --- |
| R0 — Source truth | Exact source paths + versioned primitive dependencies, past Android research reconciled | Yes | No |
| R1 — Spec/fixture scaffold | Research-only typed capability proposal and synthetic tests; no production changes | Yes | No |
| R2 — Android environment | Exact browser/phone/API/permission/build IDs; measured passive sandbox tests | Only when device available and separately arranged | No |
| I3 — Wave 3 Integration 3 | Explicit published GO and source/test owner allowlists | External prerequisite | Yes, but only named W4 paths |
| W4 — Native adapter | B/C/D/G/H owned implementation + adverse tests + real Windows/Android | After I3 + grants | Only W4 authorized scope |
| I4 — Wave 4 Integration 4 | Cross-device Founder Workspace, physical phones, Host denials, p95/battery/offline | After W4 | Gate for W5 |
| W5 | One typed command/programmability path, no second effect owner | After I4 | Wave 5 scope only |
| W6 | Exclusive publisher cutover, rollback, release | After I5 | Final release acceptance only |

## Adversarial matrix (required for supported features)
- Wrong browser origin, frame and extension ID; browser permission denied or revoked; local Companion token leaked/expired; unsigned receipt; duplicate/missing ACK; stale tab target; page swapped after preflight; wrong provider thread; hidden tab; existing draft; auth wall; inaccessible composer; double dispatch after crash; extension worker killed; Android app killed; offline/restart; age/revocation of owner grant; metadata/credential leakage; KB tombstone/freshness; background work forced stopped; Android Accessibility unavailable; mismatched APK manifest/target SDK; keyboard/TalkBack focus path.
- For each test: requirement ID, source/ref SHA, fixture hash, environment and build, steps, expected observed/refused outcome, actual raw log and status. Blank evidence = NOT TESTED.

## Exact path proposals ONLY (not grants)
- Research and fixture source now ONLY: repos-hq/docs/MOBILE-EXTENSIONS/{fixtures,tests,evidence} plus lab docs.
- Future Wave 4 B owner may propose a *thin adapter* under TierFlow src/visual-runtime/platform/android/** and W4-owned tests, subject to actual integrator allocation (do not create now).
- Existing Android native app remains vivim-web-harness/studio/android/** and UI apps/mobile-relay/**; changes require separate Harness/native owner's path-specific grant.
- Existing browser companion apps/browser-companion/** is passive; actuator research must be separate from observation data/identity and must not obtain independent send authority.
- Any Host owner/grant/command edits in apps/vivim-host/** require separate Host authority; no lab worker changes them.

## Buildability assessment
- Existing studio/android/build.sh compiles plain Java with Android SDK 34 and JDK17+; GeckoView embedding needs additional packaging/dependency design before declaring it buildable. A source-verified WebView is not evidence of GeckoView installation.
- A consumer-grade local browser extension needs an install/update story and a test for background lifecycle, not just desktop manifest compatibility.
- Future physical test rig should cover Android 15 phone with OEM battery restrictions plus another Android version/build when available; record exact installed versions and Android permissions.

## Next execution order
M1 inventory -> M2/M3 independent candidate proof -> M4 evidence validation -> recommend architecture and owner's exact file grants -> implement Wave 4 only after I3 GO. Start from existing passive extension and existing APK, not from scratch.
