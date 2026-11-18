# W03-C — Supported signed-in provider conversation

Wave 3 of ARBOR Proposal 5. Start from exact W2-G GO SHA of `integrate/proposal5-beta-r001`, branch `work/p5-w03-c-studio`. Own **studio/, harness/** plus disjoint report `integration/worker-reports/W03-C.md`. No shared branch merge or root file edits.

## Mission — Real Beta Journeys and Minimal Native Node 0
Read Proposal 5 (01–09), previous G gates, canonical source-lock and release compatibility matrix. **As-is first:** migration of existing working features and truthful statuses; avoid new UI/features. Fully self-contained ARBOR, one actual Host effect writer; signed-in Chrome browser (no APIs/Playwright/Puppeteer); scoped HQ Development vs Personal data; no private user data in Git. G is only integration/root lock owner.

1. Validate one currently supported real signed-in Chrome webapp provider workflow using the user's authorized session when available; if interactive login unavailable mark LIVE_LOGIN_BLOCKED and provide exact manual test steps, no fake pass.
2. Prove exact user-confirmed text, intended provider/thread, send-once lease and streamed/captured response to Host with source evidence.
3. Test provider UI changes and error states: disconnected browser, thread mismatch, slow DOM, permission revocation and uncertain send after claim; never blindly resend.
4. Preserve existing Studio Chrome profile and tested browser logic; fix only path/packaging/runtime defects necessary for internal ARBOR beta.

## Required proof
Real provider E2E if login available, plus synthetic fault tests and source/receipt evidence; no provider API.

Commit changes and detailed report with base/head SHA, test commands/exits, actual supported source IDs, runtime/environment, availability/auth and P0 blockers. If real owner login or physical phone needed, report a **specific pending acceptance** rather than mock success. G will reconcile and authorize next wave.
