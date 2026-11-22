# Desktop Commander → VIVIM Development Learning | 2026-11-22

**Disposition:** CODED + PUSHED in `owenservera/ARBOR` main; ISOLATED HOST VERIFIED with real local archive; production Host/GPUI restart and phone UI acceptance **NOT VERIFIED**.

## Canonical source and authority

The existing PC collector remains authoritative: `.private/desktop-commander-stream` under this operations program. Its scheduled task `VIVIM-DesktopCommander-Archive` keeps the private raw/delta, hash dedup index, receipts, and `reports/learning-latest.json`. No new collector or competing database was added. Raw Desktop Commander arguments/output remain local; source retention and upstream omissions remain explicit.

ARBOR has an additive Host read-only adapter at:
- `apps/vivim-host/data-intelligence/desktop-commander-learning.mjs`
- `apps/vivim-host/server.mjs` (owner-private existing `/api/host/control/wiki-context` field `developmentLearning`)
- `apps/vivim-host/public/pages/wiki.js` (AutoWiki owner view)
- `apps/mobile-relay/app.js` (Host-preview Wiki **only**, never paired Direct phone)
- `apps/vivim-host/test/desktop-commander-learning.test.mjs`
- `apps/vivim-host/test/oracle-wiki-wiring.test.mjs`

The schema is `vivim.development-learning/1`. The only projected data: trusted capture freshness, daily learning report timestamp, aggregate observation counts, upstream omissions, process exit counts, a fixed allowlist of pattern counters, and explicit provenance/authority labels. It does **not** copy private commands, output, tokens, account content, inferred personal traits, or executable commands. It neither mutates the Oracle personal corpus nor auto-promotes a correction.

## Verified results

- Cloud isolated adapter test: 5 passed / 0 failed (`node --test`).
- Windows read-only post-push Host tests on checkout `3500dfc2c75fba1e02e1e028503720837a7aad68`: 6 passed / 0 failed. This checkout already contained the new implementation; it did not yet contain the final integration-test-only commit.
- On real PC archive, new adapter reported `state:ready`, `capture.fresh:true`, 2,922 analysed records and 352 upstream large-output omissions. These are daily-report counts, **not** the entire current archive total.
- Isolated owner-authenticated Host HTTP route reading the **real PC archive** returned 401 unauthenticated, 200 authenticated, correct source/schema/state, 2,922 observations, `privateContentIncluded:false`; the public Wiki did not expose the dev-learning data. Test server was closed.
- Main GitHub integration-test commit: `9a54ff048def82a10e29ec5ae3c53f4be69cb611`.

## Remaining device gate

The live Windows Host listener on port 43187 was observed running from **older release** `.arbor-pwa-releases/a767921ea4dc`, started 21:03 local. Therefore live production UI display and GPUI/native viewport acceptance are **not proven**. Do not claim the live Host was restarted or the phone rendered the panel. A safe, owner-approved release integration must preserve ongoing Studio/Host sessions, apply the exact tested GitHub source, and verify owner-private Wiki readback without exposing logs to paired Direct phone.
