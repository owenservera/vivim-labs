# Second-pass VIVIM source, user-journey and index-wiring audit
**Date:** 2026-11-01  
**Trust:** source/structured-index inspection of remote `main`; *not* Windows, Android or provider live acceptance.

## ERA 1 — Repository breadth and index drift

This second pass inspected the GitHub recursive source tree; all 120 mapped user journeys; 39 activation gaps; the canonical atomic registry; the real roadmap, dependency topology, value crosswalk, source-file and source-symbol catalogs, document catalog, 26 context routes, and the executable query/validator/refresh scripts.

| Authority | Observed snapshot | Qualification |
|---|---:|---|
| GitHub recursive tree | 1,514 entries when checked | Includes directories, docs, tests and scripts; not 1,514 independent features |
| Atomic registry | 336 requirements: 232 PRD + 104 cross-project | The `352` in earlier narrative was stale; `AGENTS.md` already said 336 |
| Value map | 336 rows | Consistent with current atomic requirements |
| Roadmap | 146 task records | Sole authoritative execution-status ledger; not reclassified by this audit |
| Capability topology | 59 grouped nodes | Static dependencies; not 59 independently accepted runtimes |
| Journey graph | 120 atomic actions, 36 subjourneys, 12 chapters | 150 directed dependency edges |
| Activation gaps | 39 (17 P0, 20 P1, 2 P2) | Source-level UI/consumer/backend seams, not all independently reproduced |
| Journey/registry exact overlap | 25 linked, 95 unlinked actions | **Unlinked means missing index evidence, not missing code** |
| Old source file/symbol catalogs | 413 indexed | 15 new non-test source files missing at the original second-pass scan |
| Old document catalog | 431 indexed | 7 missing documents at the initial scan, increasing as this pass adds new files |

The source catalog missed the **actual** new first-run implementation and UI:
`apps/mobile-relay/{bootstrap-prompts,build-canvas,build-composer,pairing-link,welcome}.mjs`,
`apps/vivim-host/onboarding/{environment,service,studio-port}.mjs`,
`apps/vivim-host/public/qrcode.js`,
`harness/src/build/{build,cli,review-cli,review}.mjs`,
`tools/docs/check-user-journey.mjs`,
`tools/packaging/prepare-host-runtime.mjs`.
All of these paths were confirmed in the GitHub tree. Their omission is a clear source-index regeneration gap.

All 336 registry records lack completed `controlInterface` classification in the currently fetched registry snapshot. This is different from declaring that all 336 features ought to expose a UI control; many are internal requirements. The control catalog has ten `source_wired` action definitions and two non-executable definitions, including the global egress pause. Real execution acceptance still requires original receipts.

## ERA 2 — Cross-index repair and new discovery paths

The canonical query tool `tools/docs/query.mjs` now understands `--journey Jnnn`, `--gap Gnn`, `--phase A-L`, `--activation <state>` and `--flow F-...`, in addition to pre-existing capability ID, text, file, symbol and context route queries. **Journey mode returns only source-linked existing atomic candidates**, not unrelated first-page registry items. The result includes affected steps, source path and symbol, candidate atomic IDs, candidate roadmap tasks, topology nodes, gap observation and suggested repair.

The new `docs/capabilities/journey-capability-crosswalk.v1.json` is deliberately a *derived overlay*, not a replacement registry. Its 120 entries represent exact file overlap against currently indexed source candidates, and stronger symbol overlap only when an existing symbol candidate matches. A single high-fanout source file can link dozens of requirements; those matches are not behavior verification.

The new `tools/docs/journey-crosswalk.mjs` deterministically rebuilds that overlay after the main registry is refreshed. The enhanced `tools/docs/check-user-journey.mjs` checks all action source anchors, valid DAG directions, reciprocal edges, cycles, gap coverage and exact candidate link consistency with the atomic registry, topology and roadmap.

`tools/docs/check-index-health.mjs` independently walks non-test code and indexable documents and compares them to the source file/symbol and document catalogs. It also inspects the atom/value row counts, journey crosswalk, mandatory context route source paths and front-door Markdown relative links. Non-strict use reports existing index drift; `--strict` fails until all reported warnings are resolved.

The previous index front door had **three broken relative paths**, corrected:
- `docs/INDEX.md → ../ONBOARDING.md` (not a nonexistent `docs/ONBOARDING.md`)
- `docs/INDEX.md → ../test/run.mjs`
- `docs/INDEX.md → ../studio/android/build.ps1`

The source context router now has **26 routes** (previously 18), including dedicated journey, activation, installer, Android pairing, World reachability, Control reachability, Agent/context and source-index routes. All new route document and source paths were checked against the current GitHub tree.

Discovery now begins at `AGENTS.md` and `context.md` for agents, `docs/INDEX.md` for humans, `docs/agents/TASK-CONTEXT-ROUTING.md` for code work, `docs/CONTEXT-DISCOVERY.md` for query routing, and `docs/capabilities/README.md` for feature evidence semantics.

### What `index:refresh` actually does

```powershell
Set-Location 'C:\0-BlackBoxProject-0\vivim-web-harness'
git pull --ff-only
npm run index:refresh
npm run check:index
npm run check:index-health
npm run check:integration
```

`index:refresh` first invokes the existing `reindex.mjs --write`, which regenerates the **five existing derived authority-independent indexes** (atomic registry, source-symbol index, source-file catalog, document catalog and criticality). Then `journey-crosswalk.mjs --write` rebuilds source-to-journey candidate associations and `check-user-journey.mjs` validates the graph. The health report summarizes what still is missing.

**The regeneration commands have been committed, but this cloud GitHub session does not execute the user's local Node checkout.** Therefore the large generated catalogs remain stale until the command is executed on a checkout with current source files. This is tracked explicitly; it is not hidden behind a green "integrated" label.

## ERA 3 — Prioritized source-to-user activation seams

The second pass reconfirmed these **directly observed code seams**, not general guesses:

| Gap | Real code that exists | Missing activation boundary |
|---|---|---|
| G22 / G29 | `studio/src/studio/ui/world-view.mjs`, `studio/src/studio/personal-agent-routes.mjs` and `studio/src/studio/integration-hub.mjs` | The separate desktop Host `apps/vivim-host/public/host.js` uses explanatory placeholders for Today, Talk, Agents, Self and Build |
| G26 | `apps/mobile-relay/control-phone.mjs`, `apps/vivim-host/control/dispatcher.mjs` and Direct `/mobile/api/control` routes | The phone's `renderTabs()` shows `#v-soon` for Control, never unhides `#v-control` despite the module being loaded |
| G08 | `apps/vivim-host/bootstrap/scheduler.mjs` declares `F02,F01,D03,D04,D05,D06,D07` | The guided onboarding service configures only `['F02','F01']` |
| G15 | Phone localStorage chat state, Studio conversation journal, Host `conversations/live-projection.mjs` | No single user-facing durable conversation read model across PC and phone |
| G25 | Studio `personal-app-routes.mjs` and phone `build-canvas.mjs` | Separate personal-app blueprints and UI-revision surfaces without one user-visible identity lifecycle |
| G19 | `apps/browser-companion` passive MV3 sensor, Studio ingress | Optional installation/credential pairing not integrated as a normal consumer journey |
| G30 / G31 | `harness/src/context/dcb-composer.mjs`, Studio agent and orchestrator routes | Backend routes exist but are not presented in Host/phone's primary feature navigation |
| G32 | `apps/vivim-host/data-intelligence/catalog.mjs` | Explicitly read-only aggregate, `dataAssetIndexState:'not_implemented'` |
| G35 | `apps/vivim-host/control/action-catalog.v1.json` | Global outbound pause is `enforcement_missing`; no claimed safety stop |
| G01 / G11 / G12 / G13 | MSI staging, installed Android shell, QR handoff, separate enrollment primitives | Physical Windows install, actual APK signer and Android native/Keystore binding not accepted |

### Wiring order, not a redesign mandate

1. **Activate the cheap UI edges first:** Control panel navigation; Host World/Agent read surfaces. Do not duplicate Studio's World storage.
2. **Consolidate provider and conversation evidence:** one authorized Studio browser-effect owner; one durable identity/journal; explicit readiness and coverage qualifications.
3. **Enforce trust:** device enrollment, origin-bound private Direct, exact grants and global outbound pause (before giving users an unsafe button).
4. **Connect existing product intelligence:** deep bootstrap, retained World, provider reply → Work, DCB context, multi-model workflows, personal app factory and Build revisions.
5. **Preserve the living index:** any new user-facing feature links to a `Jnnn` journey or new step, existing atomic requirement(s) and a `Gnn` gap until accepted. The roadmap remains the *only* task-status authority.

**Anti-claims:** None of these source maps demonstrate that Android or Windows is presently running, that the provider is authenticated, or that a user could successfully complete an external effect. Exact live receipts, preserved approval and no duplicate sends remain acceptance criteria.

## Index artifacts and ownership
- **Human audit:** `docs/ux/USER-JOURNEY-TREE-AND-ACTIVATION-AUDIT-2026-11-01.md`
- **User-action DAG:** `docs/ux/USER-JOURNEY-ACTIVATION-GRAPH.v1.json`
- **Gap list:** `docs/ux/ACTIVATION-GAPS.v1.json`
- **Atomic/roadmap candidate crosswalk:** `docs/capabilities/journey-capability-crosswalk.v1.json`
- **Snapshot of missing index paths:** `docs/capabilities/INDEX-HEALTH-2026-11-01.v1.json`
- **Query:** `tools/docs/query.mjs`
- **Crosswalk refresh:** `tools/docs/journey-crosswalk.mjs`
- **Source-symbol/doc refresh:** `tools/docs/reindex.mjs`
- **Journey acceptance-map integrity:** `tools/docs/check-user-journey.mjs`
- **Full index drift integrity:** `tools/docs/check-index-health.mjs`

This report is a source-index reconciliation artifact, not another execution queue, roadmap or user-private data archive.
