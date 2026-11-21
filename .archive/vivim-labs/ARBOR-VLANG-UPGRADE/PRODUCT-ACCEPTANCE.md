# Real product acceptance — integration checklist, not a new workflow
Run available actual ARBOR tests and report exact test command and result; don't create a second runner. Device or Windows tests not available remotely remain UNRUN. Candidate source unit tests are not a substitute.

| Case | Expected result | Lanes |
|---|---|---|
| Change only source meaning, then contract | Only affected semantic consumers invalidate; conservative fallback if no trustworthy granularity | U1/U3/U5/U6 |
| Remove source or conflict on identity | Stale/unresolved, no silent symbol merge or planning use | U1/U2/U3 |
| Observe a successful receipt, a failed receipt, a user statement | Narrow occurrence evidence only; not contract proof; failed/uncertain not success | U2/U6 |
| Request context with mandatory information over budget | Explicit bounded failure rather than silently dropped constraints | U3 |
| Request restricted or stale context | Excluded or visibly stale under correct scope; cannot enable effects | U2/U3 |
| Enter ambiguous Wiki request and choose target | Typed draft updates; late response cannot overwrite it; no effect | U7/U4 |
| Preview real Wiki mutation | Exact human-readable sentence/typed arguments/target revision round-trip and digest-bound | U4 |
| Change target, catalog or rule after preview | Existing dispatcher refuses stale or tampered approval; no commit | U4/U5 |
| Approve correct plan twice | One effect, exact persisted readback; existing replay/recovery behavior | U4 |
| Teach or accept safe command then reuse | Existing exact reuse preserved; new parameterized reuse only if freshly validated and owner-approved; no provider call on hit; no cached grant | U5 |
| Restart and rebuild derived source index | Durable owner rules, Wiki data and observations preserved | U1/U2/U5/U6 |
| Open same source anchor in Wiki and context | Same provenance/revision, distinct inferred/verified UI, no duplicate authored source | U2/U3/U6 |
| Try unauthorized source edit | No file changes; use existing approved writer only where available; otherwise read-only | U6 |
| Actual Talk/Build/Wiki/desktop invocation | Same typed semantics where supported, legitimate surface policy retained, final readback is current | U4/U7 |
| Existing regression suite | No broken compatibility in Host, native core, PWA or journals | all |

No new generic WASM runtime, schema rewrite, daemon, external tool API, source mutation subsystem, agent orchestration, or performance guarantee is required to complete these lanes.
