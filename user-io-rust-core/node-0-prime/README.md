# NODE 0 PRIME — The VIVIM Interaction Foundation

**Date:** 2026-11-12  
**Status:** architectural red-team and engineering reset; not a product implementation or acceptance receipt.  
**Workspace:** \`vivim-labs/user-io-rust-core/node-0-prime/\` on the existing Labs R001 branch. No new repository, competing registry, graph store, scheduler, effect owner or mobile app.

## The strategic thesis

Node 0 Prime is the **first causal boundary** between human intention and the machine. It is also the last bidirectional boundary through which the machine must explain status, uncertainty, blocked work, durable evidence and verified outcomes.

If this boundary is wrong, better language models, prettier interfaces and faster graph algorithms cannot rescue the experience.

We should treat it as **three independent service-level promises** rather than "chat":
1. **Human immediacy:** input/editing, caret, IME, keyboard, voice feedback, focus and accessible navigation are locally responsive even if the native Rust engine, PC Host, provider or phone network is unresponsive.
2. **Computational integrity:** one immutable input revision fans out to independently relevant, bounded Rust computations and converges into coherent, explainable, source-pinned, privacy-scoped candidate evidence without a central semantic router.
3. **Outcome integrity:** the exact user-confirmed intent may become a committed request, which separately enters Host-owned plan authorization and effect journaling. Unknown, partial, attempted, observed and verified outcomes are never conflated.

**The first byte goes to the OS/IME/editor, not to a model or a Rust semantic router. The first semantic snapshot reaches a thin authenticated Rust ingress; independent reactors observe it on a bounded event fabric.**

## Executive verdict

**RED for Node 0 Prime production cutover. YELLOW for promising existing source components.** No current source audit proves a complete, native, high-frequency, actor-authenticated, cross-surface I/O pipeline. Do not merge the R001 native prototype or port real personal context through its Tauri command until security (including child-process/loopback identity), runtime scheduling and contract gates are satisfied.

The present R001 Rust spike is a useful disposable proof of interface shape, **not the production runtime architecture**. The UI R007 Total Interpretation / VIEW ALL work correctly widens the scope: a user input may produce overlapping interpretations, DCB, memory and effect proposals, and should carry *producer coverage* and epistemic status. It is not enough to show one winning command.

## Immediate release blockers

| Priority | Decision | Why |
| --- | --- | --- |
| P0 | Upgrade and audit pinned Tauri Rust dependency family; eliminate wildcard remote origins in production; verify origin-bound custom command authorization | \`src-tauri/Cargo.toml\` pins Tauri \`=2.0.0\`; official May 2026 advisory patched >=2.11.1 and September 2026 advisory patched >=2.11.6. \`src-tauri/capabilities/default.json\` enables \`http://127.0.0.1:*\` remote URL pattern. Exposure not proven; urgent threat model warranted. |
| P0 | Replace per-request session-wide lock + thread-per-reactor with snapshot/commit phases and shared bounded compute executor | R001 \`user_io_event\` holds native \`Mutex\` around \`apply_json\`; \`UserIoCore::apply\` synchronously runs \`Fabric::react\` spawning scoped threads. One slow reactor can block all subsequent native calls. |
| P0 | Close actor, device, session and source-admission boundary before connecting real personal graph data | R001 wire lets caller supply session ID and source revision, but Tauri command currently has no real trusted source adapter or actor-scoped session binding. |
| P0 | Define one versioned interop protocol and semantic truth rules across Host JS, Rust and all UI | Existing independent parsers/projections remain, and the R001 Rust prefix matcher is not differential-parity accepted. |
| P0 | Separate ephemeral drafts, committed requests, admitted effect plans and verified outcomes in data lifecycle | A fully formed two-Tab candidate is not a submitted command, and a submitted command is not authorized or completed. |
| P0 | Stop making all subscribers consume all raw text; install explicit purpose/data-class admission and minimal-interest delivery | Parallel intelligence must not turn private partial typing into an involuntary user-data broadcast. |

## Architecture choices — what I would change with unlimited engineering capacity

| Topic | Decision |
| --- | --- |
| Primary product computation & safety | **Rust** for Node 0 Prime, session protocol, graph projection, typed candidate evidence and effect-admission coordination. |
| User-visible editor & TierFlow | **TypeScript/webview** remains appropriate: DOM/IME/accessibility/rendering must stay locally immediate. Do not render every glyph in Rust. |
| Windows shell | **Keep Tauri v2, upgrade it and harden origin boundary**; do not replace with Electron/Wails merely for novelty. |
| Node Host control hot path | **Retire Node/JS as mandatory intermediary for every semantic event** after parity and source migration; Rust must become the first *trusted semantic* endpoint. |
| Host effect-authority implementation | **Long-term migrate admission, claim fences and durable control journal into the native Rust Host**, retaining the Host's single effect ownership, on-disk compatibility and receipts. Do this only after exact behavioral/crash parity; never create simultaneous JS and Rust writers. |
| Node Studio/browser automation | **Retain JavaScript for signed-in provider browser execution at a narrow, versioned capability adapter** while evidence supports it; isolate failures from native session loop. |
| HQ JS index and operational dev tooling | **Keep as development-intelligence consumer**; not user-facing request authority or personal corpus store. Rewriting HQ indexing in Rust is not required for Node 0. |
| Existing HQ Rust graph research | **Promote a single canonical, packaged, versioned Rust graph crate into product delivery** after compatibility tests. No copy/pasted competing graph engine. HQ may consume that same artifact; product data stays Host-owned. |
| Language & command compiler | **One canonical typed semantic frame and crosswalk**; use differential goldens to replace redundant JS inference without losing Omega/NLCL behavior. |
| Scheduling | **One bounded Rust compute runtime using existing engine/scheduler work** with latency classes, fair priorities and cancellable pure tasks. No spawned thread per typed event in production. |
| Storage | Domain-owned canonical journals/data remain; session speculation lives in a bounded ephemeral projection. Extend existing durable Host request/effect records only where evidence requires. |
| Wire protocol | One versioned, generated Rust↔TS schema. Strict validation, explicit identity/revision/provenance/privacy, no \`serde_json::Value\` as the internal domain model. JSON remains an appropriate compatibility/debug wire representation initially. |
| Mobile | **PWA-first**, authenticates to the same PC Rust authority. Optional browser-side WASM can render speculative pure hints offline, clearly unverified; never create a second effect authority. |
| Models | LLMs propose hypotheses or answer content, not authorization. Prediction starts as local deterministic ranker; larger models are optional/budgeted and never needed for input responsiveness. |

## Documents in this round

1. [01 — Catastrophic Failure Register](N0P-01-CATASTROPHIC-FAILURE-REGISTER.md): failure modes with mechanisms, triggers, evidence status and acceptance gates.
2. [02 — Target Architecture, Languages & Dataflows](N0P-02-TARGET-ARCHITECTURE-AND-ADRS.md): end-to-end packet lifecycle, locks/scheduling, consistency, process topologies and explicit ADR verdicts.
3. [03 — Migration Plan & Verification](N0P-03-PRIME-ENGINEERING-DIRECTIVE.md): order of work, tests, ownership, rollback, and nonnegotiable production gates.
4. [04 — Source Truth & Security](N0P-04-SOURCE-TRUTH-AND-SECURITY.md): SHA-pinned current observations, advisories, UI/HQ branch progress, uncertainties.

## Product contract

**Node 0 Prime must never lose the user's words, fabricate the user's intent, silently leak or persist a draft, lie about what is verified, deadlock interaction behind a model, or execute an ambiguous consequential action.**

The engineering objective is not merely "fast autocomplete." It is a **lossless, accountable, interruptible, privacy-scoped, source-consistent dialogue with a computer** which happens to use predictive intelligence and a parallel Rust graph.

## Out of scope

This directory owns decisions, experiments and integration evidence. Production code remains in the four canonical repositories with their domain owners. No claim that any Red gate has been resolved; no security exploit has been demonstrated against a running VIVIM instance.
