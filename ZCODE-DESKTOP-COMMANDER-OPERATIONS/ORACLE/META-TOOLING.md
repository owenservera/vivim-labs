# Oracle meta-goal tooling: what exists, what is missing, what to verify
**Snapshot:** 2026-11-21. **Venue:** direct connected GitHub, not the local PC. This is an Oracle capability/readiness map, **not a new backlog, task router or automatic monitoring service**.

## Mission and scope
One owner statement in the existing VIVIM PWA -> explicit Host-authorized development request -> Planning acknowledgment -> cloud Dev implementation -> CI/build/release evidence -> observable result -> tested deterministic Rust reuse. Oracle steers the **shape** and audits what is proven, while Planning owns task breakdown and Dev owns product code. Minimize avoidable user handoffs and recurring AI calls, but never at the cost of privacy, effect authority or truthful receipts.

## Tooling readiness matrix
| Link to prove | Reuse first (observed) | Evidence class today | Missing proof / next capability question | Oracle owner action |
| --- | --- | --- | --- | --- |
| Source context/navigation | GitHub connector; Labs role AGENTS/SITREPs/claims; ARBOR product tracker | SOURCE_OBSERVED | Fresh, task-scoped cross-repo pointers; stale source detection | Keep location map small and fetch HEAD per review |
| PWA explicit build-work ingress | Rust LiveSystemState, Wiki read, composer; Host Control approvals and receipts | Existing input/edit source, **not** a development-request producer | One intentional submission distinct from ambient typing/search, deduplicated and scoped | Steer Planning to first missing admission edge |
| Canonical work identity | Historical OWNER-WORK referenced in role docs; Labs TASK-CLAIMS.md ownership overlay | **REMOTE_SOURCE_UNAVAILABLE** for OWNER-WORK; claims overlay observed | Locate/verify existing authoritative record before adding a new store; identity, cancel, revision fence | Reconcile authority; do not fabricate OW statuses |
| Shared checkout and handoff | TASK-CLAIMS.md GitHub SHA-checked claim protocol | Documented protocol; one paused/unclaimed row in 2026-11-21 audit | Per-task executor check-in, recipient acknowledgment, conflict response | Audit collisions, never seize Dev work |
| Planning -> Dev dispatch | Role documents; direct GitHub file/change access | Documentation / async shared artifact only | **Supported, authorized active executor dispatch/ack**. A push/file/issue is not a running chat or an awakened worker | Require an end-to-end tested transport before claiming autonomy A2 |
| Cloud code and CI | ChatGPT cloud + direct GitHub; ARBOR PWA and GPUI GitHub Actions definitions | Source/workflow files observed; current runs not checked here | Exact CI run SHA, results, tests, compiler/environment; no default local coding | Require Dev receipts; reuse CI, no second build farm |
| Release and observation | Existing Host release playbook; PWA first-contact telemetry; GPUI desktop crate/workflow | Readback/telemetry source and historical HTTP reports | Stable URL current SHA, Host rev, rendered GPUI/PWA outcome, opt-in visual proof | Keep SOURCE/TESTED/DEPLOYED/VISIBLE distinct |
| Recovery and trust | Host Control approval/effect receipts, Rust revision fences; Dev release playbook | Source contracts observed | Cancel/retry/rollback receipt tied to same request; screenshot capture only with consent and redaction | Inspect one failure path per pilot |
| Reduced model reasoning | Rust-first contracts, semantic candidate infrastructure | Goal/architecture; no measured closed-loop baseline | Count eligible repeated requests, owner touches, observed model calls/cost, replay success | Promote only tested bounded patterns; no model API requirement |

## Existing GitHub delivery tools (do not overclaim)
- **GitHub read/write connector:** available to this ChatGPT for current-source reads and direct GitHub commits. It is not automatically callable by a running VIVIM PWA.
- **ARBOR CI:** .github/workflows/pwa-main.yml and .github/workflows/gpui-native.yml run source/contract and native checks on scoped pushes or manual dispatch. These workflow definitions are *not* proof that an arbitrary commit passed, deployed or rendered.
- **Labs reconciliation workflow:** .github/workflows/arbor-ledger-reconcile.yml operates the ARBOR-DEEP-DISPOSITION inventory, **not** the missing OWNER-WORK request queue.
- **Host/LiveSystemState:** current code admits observer and authorized composer edits. That is *not* yet explicit, durable development-work ingress.
- **Potential future dispatch candidates (UNSELECTED/UNVERIFIED):** a verified native coding-agent/Work invocation bridge, or a separate explicitly authorized worker listening to canonical work events. GitHub Issues, webhooks and Actions are candidate transports/CI only; none independently guarantees an active ChatGPT development session. Planning must compare to existing infrastructure before choosing anything.

## Smallest outcome-driven tool sequence
**T0 — Reliable orientation (available now):** On each Oracle turn, refresh GitHub main + claim overlay + source/receipt map. Record first unproved edge and one Planning signal, not another tracker.

**T1 — PWA-authorized request (first meaningful gap):** A deliberate owner action yields one durable, scoped, revocable identity; ambient AutoWiki typing must never do this. Planning first resolves the canonical record. Show status in PWA even if downstream dispatch remains manual.

**T2 — Supported agent activation:** Demonstrate that the real Planning/Dev executor can retrieve, accept, act and acknowledge the same ID, with access control and cancel. Until verified, declare MANUAL_BRIDGE.

**T3 — Verifiable result:** GitHub source SHA -> actual CI run/result -> controlled release SHA -> GPUI or browser observation -> optional real-phone observation. Android is **not required** for intermediate Rust/GPUI/PWA validation; never equate simulator/HTTP with physical-phone PASS.

**T4 — Cheaper repeat:** Identify one recurring request, freeze input/permission/version preconditions, implement tested deterministic Rust reuse through Planning/Dev, compare real call counts and owner interventions. Safe agent fallback if the replay cannot prove equivalence.

## Minimum records each edge must carry (outcome contract, not implementation spec)
A globally unambiguous request/correlation ID; owner authorization + scope (no raw secrets in public logs); source epoch/revision; authenticated actor/claim; state + timestamp; source/test/release artifact IDs; distinct verification surface; cancellation/refusal/rollback reason; next accountable role. Keep one Host effects authority, one Rust fabric, one canonical work registry. An agent acknowledgment must be observed, not inferred from a file.

## Scorecard (never fill missing data with zero)
| Measure | 2026-11-21 baseline |
| --- | --- |
| Owner requests closed from PWA without manual chat relay | UNKNOWN / end-to-end proof absent |
| Avoidable owner interventions per verified delivery | UNKNOWN |
| Verified model calls/cost per accepted increment | UNKNOWN |
| Repeated eligible work served by deterministic Rust recipe | UNKNOWN |
| Completed deliveries with exact source + test + release + visual evidence | UNKNOWN |
| First unproven link | PWA explicit build-work ingress to a real canonical identity, followed by executor dispatch |

**Decision filter:** Prefer one missing transport/proof edge over a new dashboard, queue, agent persona or prompt. Oracle may recommend adding a task or dependency, but Planning decides how to represent it in the existing workflow.

## Current steering handshake
The actionable, still-unacknowledged proposal is [TO-PLANNING.md](TO-PLANNING.md) (**PWA_BUILD_INGRESS**). Recipient disposition is **PROPOSED / ACK_UNOBSERVED** until Planning actually responds in a shared verifiable artifact. Do not tell the owner the other ChatGPT thread has read it without evidence.
