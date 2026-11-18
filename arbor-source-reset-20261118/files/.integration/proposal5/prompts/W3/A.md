# W03-A — Node 0 Prime native minimum

Wave 3 of ARBOR Proposal 5. Start from exact W2-G GO SHA of `integrate/proposal5-beta-r001`, branch `work/p5-w03-a-native`. Own **native/, src-tauri/, core/programmability/** plus disjoint report `integration/worker-reports/W03-A.md`. No shared branch merge or root file edits.

## Mission — Real Beta Journeys and Minimal Native Node 0
Read Proposal 5 (01–09), previous G gates, canonical source-lock and release compatibility matrix. **As-is first:** migration of existing working features and truthful statuses; avoid new UI/features. Fully self-contained ARBOR, one actual Host effect writer; signed-in Chrome browser (no APIs/Playwright/Puppeteer); scoped HQ Development vs Personal data; no private user data in Git. G is only integration/root lock owner.

1. Build a small real native **read-only** draft/input interpretation path from authenticated Tauri context to source-pinned graph/Host capability candidates and back to UI; preserve current existing Host parser for actual effectful submitted commands until verified semantic parity.
2. Replace R001 per-keystroke scoped threads and whole-session Mutex join with bounded native work, replace-latest drafts, short snapshot/CAS and stale source/result rejection. Reuse HQKG's existing scheduler where it fits.
3. Only use source-registered command IDs and verified project/source refs; do not ship synthetic fixture.full.update as an available action. Accept ordinary conversations even without a command.
4. Test input 500 rapid changes, stale revisions, cancellation and source conflict with editor never waiting synchronously on native compute; report actual latencies, not promises.
5. Keep text/caret/IME local to UI; transport typed source revisions and evidence, no native effect writers.

## Required proof
Measured native read-only user-event→bounded graph→source-linked projection with no effects, plus denied/stale tests.

Commit changes and detailed report with base/head SHA, test commands/exits, actual supported source IDs, runtime/environment, availability/auth and P0 blockers. If real owner login or physical phone needed, report a **specific pending acceptance** rather than mock success. G will reconcile and authorize next wave.
