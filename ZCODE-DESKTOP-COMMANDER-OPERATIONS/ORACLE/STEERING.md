# Oracle steering charter — PWA-first, progressively self-building

## North Star (persistent)
The owner speaks or types naturally **in the existing PWA**. One explicitly intended request is admitted, planned, implemented, tested, released at the **same stable link**, and shown back with trusted source/phone receipts. The owner supervises outcomes and approves only genuinely permission-sensitive decisions. Each verified repetition becomes cheaper through **deterministic Rust capabilities** and reusable contracts rather than repeated AI reasoning.

**Oracle governs the direction and shapes requirements; Planning owns tasks/packages; Dev owns code/tests/deploy handoff.** This is a steering charter, not another backlog, a running service, or permission to launch autonomous agents today.

## The single desired path

```text
PWA speech/text
  -> ambient Wiki assistance (read-only, never auto-creates a task)
  -> explicit build/change intent + scope/Host admission
  -> durable, revision-fenced request and trace ID
  -> Planning's dependency-ordered, source-pinned implementation bundle
  -> authorized cloud coding executor (ChatGPT Dev today)
  -> tests and source evidence
  -> controlled release + unchanged PWA URL
  -> observable progress, failure/recovery and real phone acceptance
  -> verified, reusable recipe / Rust capability candidate
```

**One Rust fabric, one Host effects authority.** Reuse existing LiveSystemState, Host Control, Wiki, HQKG and composer surfaces. Browser/voice glue can be minimal TypeScript 7+ if native Rust cannot handle that boundary. Do not introduce an Oracle scheduler, new competing graph, second tracker, second task engine, default provider API, or hidden agent acting as the owner.

**Validation policy:** Android/phone verification is **not a blocking prerequisite** for the initial Rust/GPUI/PWA milestones. Rust test receipts, a genuinely mounted native GPUI surface and browser/Host evidence are distinct legitimate interim results. Preserve real-phone proof as a separately labeled later mobile acceptance, never replace it with simulated Android or HTTP status. See [validation surfaces](../VALIDATION-SURFACES.md) and [tooling readiness](META-TOOLING.md).

## The autonomy ladder (progress measured by evidence, not optimistic labels)

- **A0 — assisted:** owner manually relays intent across ChatGPT sessions; PWA has read-only Wiki/typing features. Some source implementation exists; complete end-to-end PWA-to-code proof is **UNRUN**.
- **A1 — PWA admitted:** one explicit PWA request creates a durable, idempotent, authorized work identity and owner-visible status without copying it into ChatGPT manually.
- **A2 — routed:** Planning and Dev receive scoped work through a **verified supported bridge**, acknowledge it, and return progress/evidence. A GitHub file alone is not an active agent dispatch.
- **A3 — closed-loop release:** bounded authorized executor builds/tests and deploys, confirms real phone state with correlation + rollback; the owner stays on the same PWA link.
- **A4 — economical:** frequent tasks are replayed via tested deterministic Rust recipes, bounded by input/permission/source revisions, with measured AI savings and a safe agent fallback.

Do not jump levels by renaming a file or declaring an agent role. The first unproved edge determines the next steering signal.

## Owner-effort contract
- **One PWA statement** to describe an outcome, not a custom prompt or separate ChatGPT session; preserve voice/text parity.
- **No manual copy/paste, status polling or chat-to-chat handoff** as a desired steady state.
- **No repeated questions** for known architecture decisions or information obtainable from current authorized sources.
- **Ask the owner only** for genuinely ambiguous intent, a material product tradeoff, or required approval for access/external effects. Risk-tiered Host grants and an easy cancel/undo remain mandatory.
- **Show in-PWA receipts:** understood / preparing / implementing / verifying / deployed / phone-confirmed, with specific blocker and one recovery action. Avoid false green.

## Progress gauges (record with evidence, never invent a baseline)
1. **PWA-only closure rate:** owner requests accepted via PWA that reach real phone verification without any manual session relay / eligible requests.
2. **Owner interventions per closure:** manual prompts, copies and approval actions, disaggregated by *necessary security* vs *avoidable coordination*.
3. **AI reasoning cost per verified increment:** number of model calls and observable usage/cost per accepted product change; distinguish unavailable billing data.
4. **Deterministic reuse rate:** verified tasks executed through tested native Rust contracts/recipes without new model reasoning / eligible repeated tasks.
5. **Trust:** % of closed changes with exact source SHA, test, deployment and phone receipts; rollback outcomes when failure occurs.

**Steering rule:** prioritize the bottleneck that most reduces repeated owner intervention or AI cost **without losing safety or correctness**. Measure new infrastructure against one observable PWA-to-result step. Stop expanding context and adding agents just to appear autonomous.

## Handshake with Planning
Oracle publishes a **small outcome-level shaping signal** in [TO-PLANNING.md](TO-PLANNING.md): missing capability, evidence, why it blocks the North Star, acceptance and constraints. Planning decides whether to add, split, combine, reorder or reject tasks in its **existing authoritative workflow**. Oracle does not amend Planning's detailed task queue or assume acknowledgment. Dev receives actual prepared bundles only through Planning's normal handoff.

## Current evidence limit
The connected GitHub Labs operations tree did not expose the historical local `OWNER-WORK/` registry on 2026-11-21; do **not** claim that file is a remote shared queue. ARBOR has source-backed composer/Wiki pieces and integration reports; autonomous PWA-to-chat-agent dispatch, fully verified phone control and measured AI reduction are **not yet proven**.

## 2026-11-26 destination — native Oracle rather than an external coordinating chat
This charter's PWA-first owner loop remains valid, but Oracle's planned home is **inside VIVIM Product** as a persistent, graph-grounded accountable actor. Labs/ChatGPT Oracle today remains a **strategic preparer**, not an installed resident agent. A living Product actor means stable Identity, scoped graph lens, accepted goals, observable activity, versioned source/protocol, checkpoints and recoverable Host-owned receipts; it does **not** mean a second graph or a Host superuser. See [native Oracle migration SITREP](ORACLE-RESIDENT-PRODUCT-MIGRATION-R001.md). Run a Product-only pure actor self-application test first; respect the current intel-200/intel-210 migration gates before incorporating Labs/ARBOR donor code.
