# Native ARBOR command space — comprehensive natural-language computer control

**Research checkpoint:** 2026-11-21, Europe/Madrid  
**ARBOR inspected:** `owenservera/ARBOR@66c60ed2fc15ae05bb30f0a1e0f15c695ad3fb6e` (`main` at inspection time).  
**Status:** Architecture, evidence and evaluation seed only. **No commands implemented or tested by this research package.**  
**Authority:** This Labs folder is non-authoritative; ARBOR owns executable capability registry, Host effect controls, persistence, approval, receipts, GUI and device grants.

## Mission

Build Desktop Commander-class *utility* and go beyond it: ARBOR understands everyday natural language, compiles it into a safe, typed, grounded, potentially multi-step plan, and executes each effect through its **single canonical Host**. This is native ARBOR, **not** an embedded dependency, fork, generic Desktop Commander wrapper, or second autonomous OS executor.

**Intended user goal:** resolve at least 90% of **real frequently attempted** natural-language personal-computer commands within supported capability scope. This target is **not measured**. A list of many commands cannot establish frequency-weighted comprehension, safe execution, or user satisfaction.

## Starting position

The existing production catalog `apps/vivim-host/control/action-catalog.v1.json` is schema `vivim.control-action-catalog/1`, version **15**, containing **38** actions: **30 source_wired**, **6 adapter_pending**, **1 enforcement_missing**, **1 unsupported**. The catalog explicitly says **source_wired is not live acceptance**. No universal file/process/desktop task catalog exists today. Existing Rust kernel is pure/inert; execution goes through Host `control/dispatcher.mjs`, preserving digest, approval, claim and journal invariants.

## Deliverables

- [01-SOURCE-INSPECTION.md](01-SOURCE-INSPECTION.md) — exact ARBOR files, existing boundaries and needed extension points.
- [02-NATIVE-ARCHITECTURE.md](02-NATIVE-ARCHITECTURE.md) — end-to-end Rust design, typed IR, adapters and structured composition.
- [03-COMMAND-TAXONOMY.md](03-COMMAND-TAXONOMY.md) — 23-domain command-space map and prioritized user journeys.
- [04-SECURITY-PRIVILEGES.md](04-SECURITY-PRIVILEGES.md) — threat model, R0–R3, path/process/browser containment and consent.
- [05-IMPLEMENTATION-WAVES.md](05-IMPLEMENTATION-WAVES.md) — sequenced source integration, thin vertical slices and acceptance criteria.
- [06-90PCT-MEASUREMENT.md](06-90PCT-MEASUREMENT.md) — operationally meaningful 90% metric and corpus construction.
- [07-AGENT-HANDOFF.md](07-AGENT-HANDOFF.md) — tightly scoped implementation mission for ARBOR main.
- [registry/command-space.v0.json](registry/command-space.v0.json) — **279 machine-readable candidate commands**, all status=proposed.
- [registry/existing-action-crosswalk.v0.json](registry/existing-action-crosswalk.v0.json) — subset of conservative mappings from proposed intents to inspected current catalog IDs, no readiness promotion.
- [eval/seed-utterances.v0.jsonl](eval/seed-utterances.v0.jsonl) — proposed positive coverage cases and adversarial/ambiguity cases, **not test results**.

## Three execution eras

1. **Source convergence:** support a few universal read-only file + process queries with real rust handlers, user-selected scopes, actual Host dispatch, PWA/GPUI preview and truthful receipts.
2. **Useful native actions:** scoped reversible file changes, application/process control, approved developer tools, compose multi-step workflow DAGs, recover using Host journal.
3. **Broad domain coverage:** Studio browser, media/docs/spreadsheets, device sharing, message/calendar integrations and automation, each bound to real permissions and effect receipts. Only promote to product commands after live evidence.

## Non-negotiables

- Rust first, TypeScript 7+ only where browser/PWA glue is necessary.
- Existing Host = sole effect/world authority; no second action registry, journal, OS broker, approval, memory or permission path.
- PWA first; GPUI desktop companion; Tauri remains fallback until demonstrated convergence.
- Studio alone owns signed-in Chrome browser effects; no Playwright/Puppeteer or provider APIs.
- No untrusted model output, webpage, note or learned synonym can grant an effect.
- Arbitrary terminal commands and privileged machine changes remain **R3 / blocked** until a separate sandbox + explicit authorization design is genuinely implemented.
- No synthetic success claims or fabricated coverage percentage.

## Reference

[Parent Desktop Commander research](../README.md). Upstream example tool categories: https://github.com/wonderwhy-er/DesktopCommanderMCP and official MCP: https://modelcontextprotocol.io/specification/2026-07-28.
