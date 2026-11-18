# Proposal 5 — Phase 2 Decision Matrix and Integration Dependencies

**Working policy:** allocate by architectural leverage and safe promotion cost, not visual excitement. This is a preliminary ranking based on Labs branches, not a verified whole-repository runtime audit.

| Workstream | Category | Beta priority | Current W1 owner overlap | Safe to research now | Safe to merge during W1 | Promotion window |
| --- | --- | --- | --- | --- | --- | --- |
| Observatory schemas/tests | E | High for truth | F | Yes | **No** | W1-G/W2-G |
| Verified extension ports | E | Medium | F/A/B/C/D/E | Yes | No | G approval |
| R005 pure semantic lens fixtures | E | Medium | E/F | Yes | No | W2-G or later |
| Chronos pure read tests | E/C | Medium-low | A/C/D/E | Yes | No | W2-G with owners |
| HQ wiki/provenance quality tests | E | High | D/F | Yes | No | W2-G |
| Personal knowledge ZIP streaming intake | C | Potentially high | B/D/A | Yes | No | W3+ only, after source/auth proof |
| Native Node 0 protocols | C | Critical, already in Proposal 5 | A/F/E | Yes | No | Existing W2/W3 lanes own it |
| HQKG Chronos Rust code | C | Optional | A/D | Yes | No | W2-G after graph import |
| Voice-to-tasks | C/H | Conditional Day-0 | B/E | Yes | No | Source audit then owner scope decision |
| R007 visual hypergraph | H | Post-beta | E/A/F | Yes | No | Post-beta unless no-cost |
| Screen AutoWiki | H | Post-beta | D/B/E | Yes | No | Post-beta |
| General user app builder | H | Post-beta | B/E/F | Yes | No | Post-beta |
| Rock Core advanced compactor | H | Post-beta, safety quotas now | B/D/F | Yes | No | Simple retention gates during beta |
| Automation Queen | H | Post-beta | B/F | Yes | No | Post-beta |
| 100 predictive composer capabilities | H | Post-beta | A/E/F | Yes | No | Post-beta |

## Recommended investment sequence

**Immediate low-risk leverage:** Observatory test/QA + source-status model, semantic regression fixtures, HQ wiki provenance coverage and small pure chronology projection. These improve beta verification without adding new authorities.

**Core critical integration leverage:** identity/consent/checkpoint/provenance bridge for personal knowledge, Node 0 actor/revision handling and graph namespace safety. Prep now, implementation assigned to W2/W3 real owners.

**Highest future value but highest uncertainty:** auto Wiki mirror, advanced adaptive input, general app builder, Oracle/Zero Intent, automation/meta-agent. Specify contracts and source gap lists without installing a competing runtime.

## GO / HOLD for each component

GO only if all of these are true:
1. Exact Labs source commit and path are pinned.
2. Target ARBOR owner already exists.
3. Import does not add a second catalog, engine, personal DB, effect writer, visual publisher, scheduler or phone app.
4. No external Labs repo/CLI/server/source path is needed after import.
5. Synthetic test fixtures are privacy safe.
6. Regression test exists or is added and the owner accepts it.
7. Required Windows/source/phone acceptance is reported honestly.
8. Work does not defer existing Proposal 5 beta hard gates.

If any condition is unknown, status is `PREPARED_NOT_PROMOTED`; if directly contradictory, `REJECTED_FOR_BETA`.

## Suggested owner handoffs

- **A:** HQKG Chronos source, Node 0 native/event invariants, actor scopes.
- **B:** personal intake consent/Host ledger/World data; effect and task commits.
- **C:** Studio Chronos read projection, ChatGPT signed-in fast lane observation.
- **D:** HQ context/search/wiki source quality, semantic source taxonomy.
- **E:** R005 read-only UI, tiny Chronos read view, recorder-on-phone source audit.
- **F:** Observatory, contracts/tests, Labs status ledger.
- **G:** sole import/promotion owner and root lock/release/capability index integrator.

Do not independently tell all A–F to install the same Labs code. Give G a single ownership map, and let each owner apply its own slice after the current wave's merge gate.
