# Coordination board (instructions; not a stateful backlog)

| Lane | Source/evidence scope | Collision owner | Dependency / first gate |
|---|---|---|---|
| L01 | Signed-in AI Conversations | ChatGPT Dev (Studio/Host scoped) | Verify fresh ARBOR main, #32/#33, TASK-CLAIMS and current Studio worker/browser/thread readiness without making a provider send |
| L02 | GPUI + Android PWA Convergence | ChatGPT Dev (native/mobile scoped) | Refresh #31 and the canonical UI closure document |
| L03 | Authorized Work + Oracle AutoWiki | ChatGPT Dev producer + Oracle provenance reviewer | Read the active PY-001 claim and exact source tests |
| L04 | VLANG + Visual Feedback + Gestures | Dev source owner + ZCode deterministic verifier | Read native-verifier/latest.json and vlang-iterations/latest.json, capturing the tested SHA, named failing assertions and timeout vs actual test failure |
| L05 | QA, ZCode Admission + Release Evidence | Planning/QA coordinator + Desktop Commander observer | Refresh ARBOR #33, #24, current Labs claim registry, local observer receipt and actual ZCode dwf_run process/event metadata |

**Leadership:** ChatGPT orchestrator is the single owner-facing coordinator; ChatGPT Dev is the single ARBOR source integration authority; Planning prepares safe Labs handoffs; Oracle is an evidence/provenance reviewer; ZCode and Desktop Commander supply bounded verification. The live `TASK-CLAIMS.md` controls any changes in executor, not this board.

**DAG:** L01 Studio preflight and L04 VLANG fault isolation can be read-only in parallel; L03 Work producer and L02 PWA/GPUI promotion require Dev source-claim collision review; L05 checks all receipts, runner admission and CI. L01 and L02 meet at one Host provider-to-phone acceptance; L03/L04 feed shared Host semantic/visual proof; L05 never self-certifies another lane's device result.

**Status vocabulary:** READY_NOT_LAUNCHED / SOURCE_READ / CLAIMED / RUNNING_VERIFIED / TESTED / BLOCKED / DONE_VERIFIED. Changing a README does not change workflow state. Fill the handoff record for each real run.
