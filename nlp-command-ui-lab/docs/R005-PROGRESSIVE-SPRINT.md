# R005 — Progressive Composer Sprint: Semantic Density Without Authority

**Date:** 2026-11-12. **Scope:** `vivim-labs/nlp-command-ui-lab/` only.

## Product hypothesis

The composer can communicate substantially more meaning without enlarging the input or creating a competing command interpreter. Test a **small, reversible, source-backed semantic strip** before building exotic visualizations.

This is a progression on the earlier Omega and Final work. Existing Host, HQ and TierFlow registries remain authoritative in their own domains. The Lab is a source-derived, read-only UI projection.

## Completed increments and evidence

| Wave | Result | Code | Verification | Decision |
| --- | --- | --- | --- | --- |
| A — Truthful semantics | Quoted text respected; `don't`, `do not`, `never` and other negative cues preserved; Host `refused` and `clarification_required` take precedence over the email fixture | `src/semantic-lens.mjs`, `src/compiler.mjs` | Unit tests for quotation, negation, Host precedence | GO, for Lab prototype |
| B — Symbolic compression | Composer strip with context-sensitive chips, generated source-backed per-command sigils, semantic source cues, dual meaning/authority readout | `src/affordances.mjs`, `public/index.html`, `public/app.css`, `public/app.js` | Test determinism/uniqueness and no authority claims | GO, for Lab prototype |
| C — Menus | `/` command discovery over the current **actual** source inventory; existing fictional contact prototype retained; keyboard Tab/arrow/Esc; within-session selection ranking; one-step undo; stale-span protection | `src/semantic-lens.mjs`, `public/app.js` | Pure insertion tests, in-memory HTTP checks | GO, with runtime browser test still needed |
| D — Density | Quiet / assisted / X-ray selectable inside chat box; token-local focus, generated meaning delta and compact trace | `public/*`, `src/semantic-lens.mjs` | Static DOM and protocol tests; **no device/browser visual acceptance** | PROTOTYPE only |
| E — Source bridge | Source-registered Host, HQ actions/CLI and TierFlow commands shown separately; source-level `verified` flag no longer elevated to runtime `liveVerified` | `src/bridge.mjs`, `tests/inventory-live.test.mjs` | Isolated simulated checkouts, current local live checkout still not mounted | GO, for adapter contract only |
| F — Bench | 100 synthetic input probes with no external effects and semantic checks | `scripts/semantic-bench.mjs` | Executed under Node 22.16 | GO, for in-process synthetic bench only |

## Test record

- `node --test tests/*.test.mjs`: **28/28 passing** at this sprint closeout (up from 13/13). This is a local Node test result, not a device claim.
- `node scripts/semantic-bench.mjs`: **100/100 synthetic probes passed**; in-process timings are diagnostic only. No browser, HQ index or production Host latency conclusions follow from them.
- Read-only endpoint contract: `/api/interpret`, `/api/inventory`, `/api/context`, `/api/health` and static assets only. `/api/execute` is absent; content must be JSON for POST and requests are loopback restricted.
- Chrome headless screenshot attempt could not complete in this environment; do not claim visual verification.

## Explicit mapping to the 100 R004 inventions

**Implemented in the Labs UI to prototype level:** #1 (generated per-command sigils), #3 (semantic ink), #8 (icon explanations via labels), #13 (negation indication), #15 (missing slot display), #19 (source provenance), #23 (density-dependent compression), #30 (advanced mode trace), #33 (pixel budget), #35 (verb-driven contact menu), #37 (within-session menu ranking), #40 (Tab selection), #42 (arrow navigation), #43 (Esc), #44 (prefix matching), #47 (clarification outcome shown), #55 (meaning-based menu only via current source metadata), #58 (plain-text editable representation), #67 (in-composer strip), #74 (provenance metadata), #75 (one-step reversible suggestion), #79 (meaning delta), #80 (meaning vs availability), #84 (immediate local lexical hints), #86 (density choice), #96 (micro trace).

**Partially supported:** #2 (composite icon + sigil, not visual grammar DSL), #4 (truth border hints, not exhaustive), #31 (source vs runtime distinction), #49 (slot views but not in-line breadcrumbs), #72 (project lexical hints, not grounded project scope), #85 (race-safe response rejection, not stale-evidence persistence).

**Not implemented or verified:** the remaining research ideas, especially personalization across sessions, mobile haptics, grammar-driven recursive morphing, full command composition and bidirectional AST editing. Do not overstate coverage.

## Critical boundaries and deficiencies

1. **No real complete platform census here.** Local machine command inventory and HQ SQLite were not mounted in this execution environment. The built-in `/` menu becomes source-backed when the appropriate repo paths are available on the developer machine.
2. **No real contacts**: email suggestions are fictional fixtures unless a separate, appropriately authorized read-only source is supplied. No lookup grants a send capability.
3. **No real Host-authorized operations**: `planText()` is used as interpretation only; all previews have `executionPerformed:false` and `authorization:'not_assessed'`.
4. **Truth gap:** a Host catalog field `readiness:'verified'` alone is not a live availability receipt. `liveVerified` must remain false until a provenance-bearing live check is wired through Host authority.
5. **A slash reference is inventory discovery, not execution language.** Typing `/ui.publish` shows the source identity and is never interpreted as a grant or runnable action.
6. **Lightweight lexical parser is deliberately conservative**: not a universal multilingual grammar, not a replacement for Omega, and not sufficient to authorize commands.
7. **No automated browser acceptance**. Before merging to product, test text composition, IME, keyboard and cursor stability, Tab behavior, dark/paper/contrast palettes, phone reach and screen-reader labels with real devices.
8. **No personal data in Labs.** Neither raw private conversations nor session history is stored in this artifact.

## Next progressive sprint (R006) criteria

- Mount actual HQ/Host/TierFlow checkouts, run `node scripts/command-space-report.mjs --full`, reconcile exact source SHAs and render fidelity. Do not equate counts with coverage until commands across Studio/Host/HQ/CLI/MCP/skills are reconciled.
- Run a browser accessibility + mobile acceptance matrix on the owner machine. No Playwright/Puppeteer is required: use the existing production UI acceptance tools and human-verified checks.
- Add a versioned **presentation-only `CommandAffordance` descriptor** sourced from real registry metadata, with no new command registry and no risk of setting Host policies from UI controls.
- Experiment with a true bidirectional semantic text/chip editor only after proving lossless source spans, selection, IME and undo semantics.
- Learn from repeated *explicit* user choices through locally bounded per-user preferences, never implicit permission escalation.
- Reconcile R004 catalog with measured signal-per-pixel, error rates, keyboard-efficiency and correction accuracy; graduate only tested ideas into TierFlow.

## Run

```powershell
cd C:\0-BlackBoxProject-0\vivim-labs\nlp-command-ui-lab
$env:VIVIM_HARNESS='C:\0-BlackBoxProject-0\vivim-web-harness'
$env:VIVIM_HQ='C:\0-BlackBoxProject-0\repos-hq'
$env:VIVIM_TIERFLOW='C:\0-BlackBoxProject-0\vivim-ui'
node --test tests/*.test.mjs
node scripts/semantic-bench.mjs
node scripts/command-space-report.mjs --full
node src/server.mjs
```

Open `http://127.0.0.1:4387` when the port is free. `bun run test` / `bun run bench` are equivalent conveniences where Bun is available; no install step is necessary.