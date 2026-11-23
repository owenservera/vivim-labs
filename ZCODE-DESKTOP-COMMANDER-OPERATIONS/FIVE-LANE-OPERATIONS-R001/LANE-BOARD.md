# Five-lane coordination board — strategic reconciliation R002

This is a **role/ownership map and runbook**, not a second tracker or declaration that five agents are live. Follow [STRATEGIC-AUDIT-R002.md](STRATEGIC-AUDIT-R002.md), [DIRECTIVE.md](DIRECTIVE.md), live [ARBOR issue #33](https://github.com/owenservera/ARBOR/issues/33) and the existing Labs [TASK-CLAIMS](../TASK-CLAIMS.md). Canonical mobile PWA and native Windows GPUI remain one shared Host-backed product per newer source directive.

| Lane | **Sole primary owner of open #33 IDs** | First no-effect task | Dependencies and collaborating lanes |
|---|---|---|---|
| **L01 Provider + Studio capture** | `A01–A16; C01–C03` (19) | Current Studio/browser thread/composer/worker readiness and no-replay preflight | L02 renders the verified reply; L05 checks evidence. Real provider sends need distinct owner approval. |
| **L02 Native GPUI + Android PWA + release** | `S02–S14; U01–U04; C04–C05` (19) | Same-source package/served-state drift audit, isolated preflight and rollback readiness | L01 supplies provider proof, L04 semantic/visual behavior, L05 release receipt. L02 exclusively controls safe deployment handoff. |
| **L03 Work + Oracle + AutoWiki** | `U05; O01–O06` (7) | Current live `Work initialized` state and Dev-owned `PY-001` Host-reviewed Work producer integration seam | L04 owns `U07` plan-to-Host approval; L03 consumes it for authorized Work with provenance. No personal data bootstrap without approval. |
| **L04 Rust VLANG + visual + gestures** | `U06–U11; G02–G05` (10) | One exact-source native/Host VLANG safety parity negative repro, without duplicate observer | L02 consumes U10 visual/gesture surfaces; L03 consumes U07 Host approval semantics. L05 is independent verifier. |
| **L05 QA + ZCode admission + CI/evidence** | `Z02–Z04; Z07; R01; R03–R04; R06–R07` (9) | Current live `dwf_run` + model cost/liveness admission, CI startup and source claim snapshot | All lanes return receipts to L05, but **L05 is not a competing ARBOR source writer or production deployer**. |

**Coverage audited against #33 on 2026-11-23:** 19 + 19 + 7 + 10 + 9 = **64 open task IDs; no duplicate primaries**. Already completed `S01, G01, Z01, Z05, Z06, R02, R05` remain completed, not requeued. Refresh #33 on every new execution; ownership mapping is not product acceptance and cannot close the task.

## Cross-lane dependency graph

```text
L01 signed-in ChatGPT & Studio capture ──────┐
                                              ├── L02 native GPUI + phone acceptance
L04 VLANG/visual/Host approval ───────────────┤
                    └── L03 Work/Oracle/Wiki ┘
L05 independent run/model/source/release evidence observes all four.
```

**The single ChatGPT orchestrator** owns prioritization, latest-sha/claim reconciliation, role assignment, integration decisions and owner-facing updates. **ChatGPT Dev** owns product code on ARBOR `main` and may only edit an explicitly checked-out nonconflicting source scope. **Planning** may create Labs-only handoffs. **Oracle** is provenance/outcome reviewer. **ZCode** runs bounded and independently admitted tests/scouts; **Desktop Commander** checks authorized local runtime, devices and test receipts.

## Existing ZCode workflow admission

Do not infer that five existing `arbor-area-a`…`e` definitions can safely run five new product lanes. They are **historical technical builders**, some with source commits/ledger writes and hardcoded older provider models. They are **REFERENCE_ONLY** until rewired and reviewed. `arbor-integrate` similarly is not the new sole coordinator.

- **CONDITIONAL DETERMINISTIC:** `arbor-product-verification-parallel.dwf.ts` and `arbor-oracle-corpus-verify.dwf.ts` only on exact pinned, clean isolated source with inspected test effects.
- **CONDITIONAL MODEL:** `arbor-vlang-iterative-observers.dwf.ts` only after matching deterministic baseline, actual health/billing admission and read-only grants.
- **UNSAFE UNMODIFIED:** `arbor-life.dwf.ts` has explicit live Host 43187 restart behavior; `arbor-chatgpt-bridge.dwf.ts` contains a real Studio ChatGPT send; legacy product/roadmap/demo/integration workflows include writers/queue effects. **No automatic launches**.

Do not register another router, scheduler or Host. Local ZCode `dwf_run` labels may be orphaned; require real runId, owner-process liveness and increasing event receipts. A scheduled deterministic Windows observer is not a model-driven ZCode agent.

## Progress/status semantics

Use `READY_NOT_LAUNCHED / SOURCE_READ / CLAIMED / RUNNING_VERIFIED / TESTED / BLOCKED / DONE_VERIFIED`. Record `CODED, PUSHED, SOURCE_TESTED, PACKAGED, SERVING, HOST_ACK, PROVIDER_VERIFIED, GPUI_VERIFIED, PHONE_RENDERED` separately. Every handoff includes exact source SHA, task, owner, claimed paths, command/exit, verification scope, unavailable stages, next gate and actual receiver acknowledgment. A GitHub comment or published prompt is not a wake receipt.

**Priority:** L01 true signed-in message→reply first, L04 negative test/fix in parallel, L02 release/device proof, L03 authorized Work/knowledge, L05 enforces truth continuously. Branch archive and old UI retirement follow accepted product capability; don't confuse inventory with parity.


## Live critical-path sequencing

Before prioritizing or claiming the next item, read [R004 five-lane strategic execution sequence](STRATEGIC-EXECUTION-SEQUENCE-R004.md). It combines the actual L01/L04 claim scopes, L02 S11 delivery, currently enabled L01–L03 hourly recurrences, isolated ARBOR package/deployment gate, and bounded real ZCode/Windows verifier status. **L04/L05 ChatGPT hourly recurrences have not yet been verified** at this read; a claim is not an automation. This is a source-linked orchestrator note; defer to live #33 and TASK-CLAIMS for status.


> **Live lane coordinator R005:** [Current five-chat assignments and exact last proven deliverables](ORCHESTRATOR-NEXT-ACTIONS-R005.md). Recheck live claims and #33 on each turn. Existing L04/L05 chats have completed source/evidence increments, but their hourly ChatGPT tasks are not verified; do not confuse chat launch with a scheduler.
