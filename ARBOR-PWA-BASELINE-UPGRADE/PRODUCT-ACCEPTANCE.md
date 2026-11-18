# Real product acceptance
Run on actual current ARBOR main, persistent disposable development data directory (never reset user data), authenticated Host and paired PWA. Use Bun and existing Node/SQLite/Cargo tests, Chrome/CDP if installed; no Playwright/Puppeteer. Assert actual DOM/computed state and canonical Host readback. Report UNRUN if full checkout/Host/phone not available. Do not present conceptual layout or mocked receipts as proof.

| Scenario | Required observation |
|---|---|
| Fresh install and first launch | Clean functional baseline with real navigation, composer, connection and settings; no fake data, no implicit provider sends |
| All destinations | Today/Talk/Build/World/Wiki/Me/Control, Pocket, media, pairing, real actions reachable; desktop and mobile responsive |
| Instance inspection | Every visible owned instance has stable ID, source type, applicable/installed capabilities and permitted binding; dynamic identity survives rerender |
| Full catalog | 417/461 snapshot discoverable or current source-derived counts; distinguish declared, applicable, source-mapped, installed and authorized. No fake controls |
| Global color | NL 'set my app background to #20362d', typed config and direct control produce equivalent candidate; preview real app, no pre-approval write; committed state updates whole shell and Build where intended |
| Component style | Choose actual existing card, set reviewed radius/border/text/spacing setting; other scopes unchanged; computed output matches committed value |
| Navigation | Move Wiki first, actual click/keyboard goes to same Wiki; protected recovery and approval still reachable |
| Composer | Move mic among allowed slots; actual mic uses existing capture and permission flow. Native session/provider targets and NL control still route correctly |
| Construction | Add Research card; actual VEC typed candidate and TierFlow preview; no state change until Activate; Host revision persisted; real card visible |
| Rich types | Actual collection, timeline, chart, buttons, profile/project cards where supported; data bound to authorized sources; unavailable data clearly identified |
| Wiki | Existing live Wiki and NL edit works through actual typed Host plan + approval + journal, source/section and revision correct |
| Persistence | Reload/reopen actual Host ledger, restore layout/settings and user data without mock/fallback; paired phone/desktop receive matching authorized revision |
| Undo | Restore prior verified content through later revision or incumbent proof; wrong revision/unavailable inverse refuses |
| Responsive/accessibility | Narrow phone, wide desktop, keyboard/IME, touch, focus, labels, contrast, reduced motion; no loss of required controls |

Adversarial cases: unknown/ambiguous target; uninstalled engine; invalid token/enum/dimension; missing data ref; private data redaction; invalid authority or revoked grant; unsafe CSS/HTML/JS/URL; stale preview, same-revision conflict, rollback, concurrent edit, oversized/cyclic document; no approval; double tap, lost result, pending/uncertain, failed persistence/readback, no silent retry. Test real command route and claims, not just compiler. Preserve existing data and theme migration; test old persisted revisions and restart. Do not claim all 461 capabilities are installed.
Completion: all currently visible instances registered; old feature regression and safe default; P0 adapter set explicitly installed and tested; end-to-end natural language + direct controls + typed equivalence; committed real screen, persistence and rollback safety. B09 returns actual commit, test commands/results and limitations in chat. No reports or dashboards in target repository.
