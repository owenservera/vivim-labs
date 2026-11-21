# Recovered architecture — distinct layers, one meaning

**Design synthesis, not an implementation claim.** This represents the architecture discussed across VIVIM and HQ research and the repository source referenced in SOURCE-MAP.md.

```text
Source code / KB schemas / agent contracts / docs
  └─ annotations (@hq:*), symbols, references, version hashes, evidence
            │ parse, validate, link; NO execution
            ▼
       KB + Repos HQ kernel
  type and capability registry | authority labels | source graph
  decisions and CANON | context bundles | lexicon | trace provenance
            │ same canonical stable schema/action IDs
    ┌───────┼───────────────────────┐
    ▼       ▼                       ▼
 Wiki V1   NLCL symbolic/NL         Agent/API consumers
 selectors  17 glyphs               (TierFlow/VIVIM)
 menus      lex→ground→IR           typed actions
    └───────┴───────────┬───────────┘
                        ▼
             Canonical typed intent/plan
                 (preview/diff/receipt)
                        │
                   scoped approval
                        │
             native authorized effect owner
      (VIVIM Host / Studio / Android / TierFlow Tier 3)
                        │
             observations + durable receipts
                        └─────────► KB evidence projection
```

## Separation of concerns

1. **Comment-declaration language**: source-level `@hq:` tags next to an actual file/symbol anchor. Existing v1: `context`, `why`, `invariant`, `see`, `todo`, `owner`, `task`, `risk`, `deprecated`. Proposed additive v2: `type`, `capability`, `effect`, `evidence`, `action`, `emits`, `consumes`, `contract`, `test`. These **require** a parser/linter/schema change and should not be treated as supported today. They must be scanned in comments only; strings ignored. Generated/vendor files excluded. Evidence anchored by path, content hash, line/symbol and revision. A code comment does not create a registered executable action.
2. **NLCL symbolic/NL interface**: `vivim-omega/plugins/vivim-nlcl-pure` is the deterministic interpreter reference: `interpret(text, WorldModel) -> Interpretation`; scan, lex, frame-match, ground, resolve, project. Output includes semantic IR, candidate ambiguity, canonical symbolic reading, token feedback and effect preview; low-confidence AI help can only suggest, never authorize. The 17 lexical families are `/`, `@`, `#`, `!`, `?`, `+`, `-`, `&`, `$`, `*`, `%`, `Δ`, `✓`, `=`, `^`, `~`, `→`. Symbol `!` cannot force bypass of an approval.
3. **Action contracts**: VIVIM Host's real `apps/vivim-host/control/dispatcher.mjs` uses `plan` and `execute` with plan digest/approval and exclusive claim. All UI controls and textual forms should compile to the **same** typed action, with one validation/authorization/receipt path. TierFlow's `src/protocol.ts` and three-tier runtime are independent and generic; a shared kernel adapter provides semantic knowledge, not HQ-specific executable logic.
4. **Kernel and agent coordination**: HQ owns deterministic context, source meaning, schema/type IDs, provenance and a versioned agent messaging bus. Cross-agent task messages and claims are metadata, never effect grants. Existing KB+HQ target is in `docs/SHARED-KB-KERNEL-ARCHITECTURE-2026-11-02.md`; the shared **runtime** is not implemented merely by this decision.

## Wiki programmability design

**V1** delivers visually authored, data-only reprogramming: choose a project, entity/type, source, confidence, view, layout, filters, graph depth, context budget, retrieval strategy, and allowed rule configuration; preview the exact canonical typed IR and affected projections; activate a version, inspect result, rollback. All configuration writes are through an explicit scoped **kernel configuration writer**; preserve the existing read-only query/console command lane and refuse shell, provider messages, source modifications and privilege escalation from wiki controls.

**V2** adds natural-language and 17-symbol input over the *same* registry, planner, preview and permission path. Never fork the action implementation by input medium.

## Example (proposed v2 comment grammar; not yet executable)

```ts
/**
 * @hq:context Privacy retention setting.
 * @hq:type vivim.privacy.settings@1
 * @hq:capability vivim.privacy.configure@1
 * @hq:effect host:config-write
 * @hq:invariant Host validates revision and scope before an effect.
 * @hq:evidence test:apps/vivim-host/test/control-dispatcher.test.mjs
 * @hq:see decision:privacy-retention
 */
```

A wiki selector `Privacy > Retention > 30 days > Preview`, a symbolic phrase `/privacy %retention = 30d`, and NL `Set retention to 30 days` **should** yield equivalent candidate typed plans. These commands are illustrative and require registry/grammar implementations. No permission is inferred from the comment.

## Critical contradictions and constraints to investigate

- HQ's current `docs/WIKI-CONSOLE.md` is read-only by design. A v1 programmable wiki needs a **separate bounded configuration writer**; never silently expand a query endpoint to effects.
- HQ's current KB is a projection over docs/decisions/terms/sessions/notes/wiki; VIVIM's KB 44 type/30 source/33 job registries are **design inputs**, not deployed HQ durable kernel storage. Plan a verified migration; do not create dual canonical writers.
- Source comments, KB schema declarations and code implementation can drift: provenance + content-hash based lint needed; index presence is not behavior proof.
- The NLCL symbolic language and HQ comment syntax are related via stable semantic IDs, but **must remain separate grammars**.
- `VIVIM` must run without TierFlow; TierFlow must work on arbitrary repos. Shared kernel contracts, independent clients.
