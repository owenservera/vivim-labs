# ARBOR / VIVIM Harness — Source Intake Lab

Date opened: 2026-11-22. Status: **research intake; no code approved for import**.

## Mission
Identify reusable code, patterns, and capability coverage from owner-selected GitHub repositories for the **existing ARBOR harness**. **Native integration is mandatory**: external repositories donate valuable behaviors, not additional services, MCP layers, or parallel applications. We inspect and compare first, then propose bounded imports into ARBOR's existing source. Do not build a second harness or make Labs a production authority.

**Canonical product:** [ARBOR/harness](https://github.com/owenservera/ARBOR/tree/main/harness), with adjacent `apps/`, `studio/`, `native/`, `contracts/` and `wiki/` consumers.  
**Research host:** [vivim-labs](https://github.com/owenservera/vivim-labs).  
**Source donor identified:** [DesktopCommanderMCP-VIVIM](https://github.com/owenservera/DesktopCommanderMCP-VIVIM), pinned at `9a101b17e44577f0baefa4c7120f087881c5cc56`. See [assessment](DESKTOP-COMMANDER.md).

## Current implementation constraints
- One canonical native **GPUI** desktop surface, with the mobile PWA representing the same surface/state and real-time parity; retire dependence on Edge as a PC application UI. Treat this as an owner design target, not verified code parity.
- Reuse TierFlow interaction/visual primitives through migration into GPUI where feasible; avoid duplicate renderers and competing state ownership.
- First demonstrable flow: user types in one command surface → AutoWiki visibly updates; the same state/feedback should be observable on desktop and phone.
- Host is sole authority for effects, with approvals, receipts, recovery, and provenance; no direct provider API assumption or synthetic success fallbacks.
- Rust-native first-class modules in existing ARBOR supervisor/Host contracts; TypeScript 7+ only where browser/platform interop actually requires it. No extra MCP daemon or remote transport.
- ChatGPT cloud development → direct GitHub canonical `main`; local Desktop Commander/ZCode limited to approved context inspection and post-push on-device verification.

## Intake workflow
1. **Identify** exact source URL, owner, default branch, full immutable commit SHA, license, and subdirectories.
2. **Inventory** real modules/symbols, dependencies, build/test commands, transitive binary or browser dependencies, and security/privacy implications.
3. **Map** each candidate to an existing ARBOR source path and owner contract. Record overlaps, missing capabilities, and migration complexity.
4. **Classify** as adopt/adapt/reference/defer/reject, with evidence, explicit constraints, and user-visible GPUI/PWA acceptance.
5. **Propose** minimal PR-free integration steps on canonical ARBOR `main`; preserve concurrent edits. No import without a traceable decision.
6. **Verify** with real commands and screenshots/receipts where available. Label cloud checks versus Windows GPUI and physical mobile tests separately.

## Starting documents
- [Candidate register](CANDIDATES.md) — what to examine and why; unverified candidates stay marked.
- [Inspection checklist](INSPECTION.md) — source-specific evidence and acceptance criteria.

## Scope and anti-goals
This folder is public-safe **research**, not the product tracker, another wiki/index or a third implementation surface. No credentials, browser sessions, local/private files, transcripts, or unreviewed recovered artifacts. See root [AGENTS.md](../../AGENTS.md). Historical Labs reports are not proof of current implementation. All status must be revalidated against pinned ARBOR commits.
