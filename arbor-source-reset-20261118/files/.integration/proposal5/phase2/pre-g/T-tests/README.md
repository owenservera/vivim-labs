# P2-T — Source-truth and tests (pre-G)

**PREPARED_NOT_PROMOTED**. Branch work/p5-p2-preg-tests-r001. All writes stay in this directory. G does not wait.

Reviewed ARBOR pre-G work order, refreshed decision, intake and readiness mapping, Labs readiness report, Labs source files and ARBOR D/E frozen reports. No combined G runtime was inspected. F final freeze report was not retrievable when checked. Source presence is not activation.

## Exact source pins (repo owenservera/vivim-labs)
- main: 022608f5797e7b4a14ccc4c393bd3c5a4704a4be
- R005 branch: 56bb145ea1eb479e4d8ec629163a84322797367a
- Chronos staged branch (readiness): b87ea71c653c3833451bd0af19d9ef1c4327728e
- capability-observatory/lane-d/src/observatory.mjs blob 4b0284b209ca4bb45155a82aab02af1bc2f99101
- vivim/voice-to-tasks/workbench/core.mjs blob 24120e5214833e4365ef9ad76ec7fe40f6339ef2
- vivim/voice-to-tasks/workbench/workbench.test.mjs blob f36284e31f10dce0f47c0f955202a7780ffae185
- nlp-command-ui-lab/src/semantic-lens.mjs blob d5de11edf693eb67ffed23e1fd26e68240d4420c
- nlp-command-ui-lab/src/affordances.mjs blob 8f010ef7d0e1ac444e9855055f6f4881fc36c56a
- chronos-toolkit/wave2/vivim-web-harness/studio/src/studio/chronos-conversation-projection.mjs blob 94d43ae2a3c20dcc054d6c16f3b44ec664862d42

ARBOR D PREG-D report blob 939475cfc842e4fad1c281090ae65d7b25fccb81 on work/p5-w01-d-hq; E PREG-E blob a2944217c2a42c377c8aa3ee2aa4efe9d255295a on work/p5-w01-e-ui. D read adapter is Host-authorized; E TierFlow is imported on isolated branch, not G/runtime/device accepted. F must reconcile exact freeze source head. Readiness ledger: ARBOR work/p5-phase2-readiness-docs-r001, Labs work/p5-phase2-readiness-r001.

Observed source behavior: Observatory requires evidence, receipt and client proof before certifying live; voice workbench emits inert draft proposals and checks spans/consent; R005 lexical interpretation and risk sigils are display only; Chronos Studio timestamps are read-only, without effects. No second catalog, task store, scheduler, observatory server, UI renderer or permission broker is authorized.

Run staged fixture policy test locally: node --test .integration/proposal5/phase2/pre-g/T-tests/synthetic-contracts.test.mjs. This exercise is PROPOSED CONTRACT ONLY, not an upstream integration regression. GitHub connector cannot execute Node/Cargo. **CODE_WRITTEN / TEST_UNRUN** for new suite, all real integration tests and devices.
