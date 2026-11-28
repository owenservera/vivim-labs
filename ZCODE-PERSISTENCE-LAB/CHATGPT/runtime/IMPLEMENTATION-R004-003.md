# R004 — source checkpoint 003

Source pin: 3496f0b829b652488cdfdc2ed39da996c2c6e425. Scope: CHATGPT/runtime on main; GitHub connector only. No compilation, build, test execution, live agents, or experiment modifications. Prior source checkpoints 001 and 002 retained.

## New mechanisms

- E26/E47: service.rs adds an independent bounded controller supervisor, durable per-launch reservations, restart and backoff, immutable exit records, stop signals, and uncertain-exit accounting. Controller is parent-pipe bound; workers retain their controller lifetime and lease fencing. The outer service itself still needs an OS-level launcher if unattended restart is required. Restart budget is conservative: reserved but unused work is not refunded.
- E38: custody.rs adds private cohort commitments, distinct fingerprints, training-only candidate synthesis, immutable candidate class fingerprint, independent signed verifier verdict, one-shot cohort admission, mandatory custody per configured family, frozen-class scheduling fences, prior public regression and evidence-backed publication. Legacy public capability admission is denied once custody is enabled. Private cases remain outside the canonical state. The trusted evaluator must be independently administered and the store/authority file OS-protected. Fingerprints are not secrecy for low-entropy inputs. The trusted host remains the final authority; this is not adversarial cryptographic proof of evaluator honesty.
- E31: ingress.rs implements a bounded JSON-lines broker with a fixed startup authority, signed scoped requests and sanitized errors; replay checks remain scope-aware. local_effect.rs implements one host-owned content-addressed JSON publication adapter with no arbitrary path or command, prevalidated receipt oracle, read-only reconciliation for unknown outcomes and durable effect resolution. All host-only APIs require trusted process and filesystem scope; no OS permissions or network listener provisioned.
- E42: durable outer reserved job/learning/start budgets and measured independent cohort evaluation counts augment previous execution and synthesis receipts. Full CPU/memory/I/O and regression-cost accounting remains incomplete.
- E34/E44: qualification.rs contains test-only pre/post-head crash pause hooks, actual owned subprocess kill/recovery source, eight concurrent independent writers, external-anchor deleted-tail fixture, and sixty logical agents competing for four lease slots. Hooks are excluded from non-test builds.
- E47: tests/r004_process.rs contains an ignored opt-in integration fixture that launches and kills an owned controller after observing a bound worker, restarts the controller, and checks all 12 outputs, goal completion and generation replacement. This does not independently verify simultaneous OS worker death; add explicit process-death observation and fault schedule qualification later.

## New command interfaces (not executed)

Under kernel <store>: serve <workspace> <ledger> <service-config>, ingress <trusted-authority> <request-limit>, local-effect <effect-id> publish|reconcile, private-evaluate <candidate-id> <private-cohort-file>. New durable actions: RegisterCohort, FreezeCandidate, FreezeLearned, AttestCandidate (signed verifier only), AdmitFrozen. The dedicated kernel-controller entrypoint is for an owned parent-pipe process.

## Tracker

Original 50 gates, priorities, weights and historical evidence preserved. 40 source-implemented/unverified, 6 partial source, 4 deferred. R003 historical acceptance 28/100; R004 verified acceptance 0/100. Source coverage is not an acceptance pass.

Remaining partial gates: E31 (OS authority boundary), E34 (actual crash/concurrent-write qualification), E41 (generalized semantic knowledge discovery), E42 (full resource accounting), E44 (actual multiprocess population qualification), E47 (actual total-death recovery qualification). Deferred: E45/E46 comparative policies and fault runs, E48/E49 real 36h/72h endurance. E26 and E38 now have source mechanisms sufficient to enter later qualification, not proof that they work.

Known limitations: complete World materialization per transaction remains; source is uncompiled and may contain compile or logic errors. Windows directory durability and crypto vectors require validation. The GitHub-only iteration did not sync the local checkout. No new crate dependency. No test execution or build attempted.

Next: finish general-purpose semantic specialization and accounting, strengthen OS authority boundary and process observation, then perform source-level invariant review before an explicitly authorized compile/test cycle.
