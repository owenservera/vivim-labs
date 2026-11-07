# Past implemented — source-code evidence shelf

**Status:** initial source scan, 2026-11-07; not an adoption or runtime certification. Source-present means code identified, not that it builds, passes tests or is called by the current product. Rows tagged CURRENT are comparison baselines, not orphans. Cap-store is unresolved and is not asserted implemented.

## High-level platform domains

- [01-context-and-dcb](./01-context-and-dcb/README.md)
- [02-memory-and-personal-intelligence](./02-memory-and-personal-intelligence/README.md)
- [03-knowledge-graph-and-retrieval](./03-knowledge-graph-and-retrieval/README.md)
- [04-capability-intelligence](./04-capability-intelligence/README.md)
- [05-parsing-and-protocols](./05-parsing-and-protocols/README.md)
- [06-automation-and-orchestration](./06-automation-and-orchestration/README.md)
- [07-self-healing-and-reliability](./07-self-healing-and-reliability/README.md)
- [08-optimization-and-resource-intelligence](./08-optimization-and-resource-intelligence/README.md)
- [09-security-privacy-and-authority](./09-security-privacy-and-authority/README.md)
- [10-interface-and-device-intelligence](./10-interface-and-device-intelligence/README.md)

## Source-linked intake

| ID | Primary domain | Candidate / baseline | Historical source | Finding |
|---|---|---|---|---|
| IMP-CTX-01 | 01-context-and-dcb | Context thermodynamics (Rust) | [owenservera/vivim-platform:edge-pwa/backend/src/cortex/thermodynamics.rs](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/thermodynamics.rs) | Rust implementation present; current DCB feature parity unverified |
| IMP-CTX-02 | 01-context-and-dcb | Context thermodynamics (TS) | [owenservera/vivim-platform:server/src/context/context-thermodynamics.ts](https://github.com/owenservera/vivim-platform/blob/main/server/src/context/context-thermodynamics.ts) | Separate implementation; compare semantics and avoid cloning |
| IMP-CTX-03 | 01-context-and-dcb | Cortex assembly and context diff | [owenservera/vivim-platform:edge-pwa/backend/src/cortex/mod.rs](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/mod.rs) | Rust composition code exists; end-to-end consumers unverified |
| IMP-CTX-04 | 01-context-and-dcb | Canonical current DCB (comparison target) | [owenservera/repos-hq:hq/lib/context.mjs](https://github.com/owenservera/repos-hq/blob/main/hq/lib/context.mjs) | CURRENT implementation, not orphan candidate; CLI/MCP/OpenCode present |
| IMP-MEM-01 | 02-memory-and-personal-intelligence | Cortex predictive scheduler | [owenservera/vivim-platform:edge-pwa/backend/src/cortex/prediction.rs](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/prediction.rs) | Source catalog attests code; runtime integration unverified |
| IMP-MEM-02 | 02-memory-and-personal-intelligence | Cortex memory tier model | [owenservera/vivim-platform:edge-pwa/backend/src/cortex/memory_tier.rs](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/memory_tier.rs) | Source catalog attests code; investigate actual quantization separately |
| IMP-MEM-03 | 02-memory-and-personal-intelligence | Legacy memory backend compression hook | [owenservera/vivim-final:src/engines/memory/memory-backend.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/memory/memory-backend.ts) | Pre-compress and session switch hooks; runtime invocation unverified |
| IMP-MEM-04 | 02-memory-and-personal-intelligence | Current receipt-backed working memory | [owenservera/repos-hq:research/personal-intelligence/working-memory/index.mjs](https://github.com/owenservera/repos-hq/blob/main/research/personal-intelligence/working-memory/index.mjs) | CURRENT implementation, compare; not orphan candidate |
| IMP-KG-01 | 03-knowledge-graph-and-retrieval | ACU information-theoretic scoring | [owenservera/vivim-platform:acu/src/score/index.ts](https://github.com/owenservera/vivim-platform/blob/main/acu/src/score/index.ts) | Shannon entropy, diversity and compression proxy source present; quality validity unbenchmarked here |
| IMP-KG-02 | 03-knowledge-graph-and-retrieval | Code Intelligence Platform context and learning modules | [owenservera/code-intelligence-platform:index/lib/cipkg/context_manager.py](https://github.com/owenservera/code-intelligence-platform/blob/main/index/lib/cipkg/context_manager.py) | README claims implementation path; code path not individually verified in this pass |
| IMP-CAP-01 | 04-capability-intelligence | Capability automation pipeline | [owenservera/vivim-platform:edge-pwa/scripts/capability-automation.ts](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/scripts/capability-automation.ts) | AST annotation/checks, CDP monitoring, bus notification; check current callers |
| IMP-CAP-02 | 04-capability-intelligence | Legacy capability parity auditor | [owenservera/vivim-final:src/engines/capability-parity.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/capability-parity.ts) | Source present; compare to current command/Host contracts |
| IMP-PARSE-01 | 05-parsing-and-protocols | Structural parser induction | [owenservera/vivim-final:src/engines/onboarding/parser-synthesis-engine.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/onboarding/parser-synthesis-engine.ts) | Parser synthesis exists; provenance/accuracy must be validated |
| IMP-PARSE-02 | 05-parsing-and-protocols | Low-confidence parser repair | [owenservera/vivim-final:src/engines/parser-repair.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/parser-repair.ts) | Repair code source present; current adoption unknown |
| IMP-PARSE-03 | 05-parsing-and-protocols | Streaming semantic AST reconciler | [owenservera/vivim-final:frontend/src/lib/streaming-assembler.ts](https://github.com/owenservera/vivim-final/blob/main/frontend/src/lib/streaming-assembler.ts) | Documented harvested from Edge PWA; confirms prior cross-repo movement, current use unknown |
| IMP-PARSE-04 | 05-parsing-and-protocols | Typed stream parsing | [owenservera/vivim-final:src/engines/stream-parser.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/stream-parser.ts) | DB-configured parsers; compare with current provider UI/capture constraints |
| IMP-PARSE-05 | 05-parsing-and-protocols | Deterministic NL command parser | [owenservera/vivim-final:src/engines/nlcl/nl-parser.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/nlcl/nl-parser.ts) | Regex/named group approach; avoid new execution authority |
| IMP-AUTO-01 | 06-automation-and-orchestration | Edge PWA capability automation | [owenservera/vivim-platform:edge-pwa/scripts/capability-automation.ts](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/scripts/capability-automation.ts) | Reusable discovery process candidate; historical automation not current Host permission |
| IMP-AUTO-02 | 06-automation-and-orchestration | Protocol loop parser | [owenservera/vivim-final:src/engines/protocol-loop-parser.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/protocol-loop-parser.ts) | Typed event streaming for agent workflows; no live action assumption |
| IMP-HEAL-01 | 07-self-healing-and-reliability | Selector failure classifier/healer | [owenservera/vivim-final:src/engines/selector-healer.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/selector-healer.ts) | Harvested from Edge PWA; compare current browser tooling and consent |
| IMP-HEAL-02 | 07-self-healing-and-reliability | Oracle corrective actuator | [owenservera/vivim-final:src/engines/kernel/oracle-actuator.ts](https://github.com/owenservera/vivim-final/blob/main/src/engines/kernel/oracle-actuator.ts) | Source present; cannot reuse auto-effects without Host approval |
| IMP-HEAL-03 | 07-self-healing-and-reliability | Streaming AST self-repair | [owenservera/vivim-platform:edge-pwa/frontend/src/observatory/streaming-assembler.ts](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/frontend/src/observatory/streaming-assembler.ts) | Source exists and was harvested to Vivim Final; not necessarily orphaned |
| IMP-OPT-01 | 08-optimization-and-resource-intelligence | Cortex budget allocator | [owenservera/vivim-platform:edge-pwa/backend/src/cortex/budget.rs](https://github.com/owenservera/vivim-platform/blob/main/edge-pwa/backend/src/cortex/budget.rs) | Source catalog attests implementation; compare with deterministic DCB budget |
| IMP-OPT-02 | 08-optimization-and-resource-intelligence | ACU scoring function | [owenservera/vivim-platform:acu/src/score/index.ts](https://github.com/owenservera/vivim-platform/blob/main/acu/src/score/index.ts) | Do not equate high entropy with relevance or truth |

**Status key:** source-present; CURRENT comparison; uncertain migration; no production verification. Never infer absence from identifier searches alone. Deduplicate shared files when assigning future source records. See [research shelf](../past-researched/README.md) for proposals not demonstrated in executable code.
