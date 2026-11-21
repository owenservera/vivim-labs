# VLang — semantic engineering and verification program

**Date:** 2026-11-12 | **Status:** STRATEGIC_SITREP_SCAFFOLDED, SCF_PREDEV_CONVERGED_WITH_OPEN_GATES | **Authority:** Proposal 5 read-only program index, not runtime source or an approved code gate.

## Canonical placement and exact responsibility
This folder in `.integration/proposal5/vlang/` is the canonical **cross-program navigation, decision, maturity and readiness index** inside ARBOR. It does NOT own the SCF source implementation, a new compiler registry, Wiki writer, verifier kernel, HVI comparator, Host action owner, graph or UI publisher. Keep the detailed design where it already exists:
- [SCF / PR #13](https://github.com/owenservera/ARBOR/tree/work/p5-semantic-capability-fabric-r001/.integration/proposal5/semantic-capability-fabric/): source-level architecture, typed ABI, agent protocol, Semantic Verifier, near-code engineering package.
- [SEVP / PR #19](https://github.com/owenservera/ARBOR/tree/work/p5-semantic-engineering-program-scaffold-r001/.integration/proposal5/semantic-engineering-program/): umbrella semantic-engineering/verification program and R002 convergence; post-SCF extension direction.
- [HVI on main](https://github.com/owenservera/ARBOR/tree/main/.integration/proposal5/hq-version-intelligence/): native version intelligence governing policy, gate owner and synthetic-only comparison tooling.
- [HQ on main](https://github.com/owenservera/ARBOR/tree/main/hq): existing parser, index, registry, SQLite, source intelligence, Wiki read model.

**Naming:** VLang is the informal umbrella name, not a new accepted executable programming language. `@hq:semantic/3` is an *opt-in proposed* additive annotation grammar. SCF = Semantic Capability Fabric; SV = independently checked Semantic Verifier; SEVP = long-horizon program; HVI = native 2..N source/version comparison.

## Read in order
1. [00 — Strategic SITREP](00-STRATEGY-AND-VISION.md)
2. [01 — Architecture and authority](01-ARCHITECTURE-AND-AUTHORITY.md)
3. [02 — Four-era implementation inventory](02-STATUS-AND-EVIDENCE.md)
4. [03 — Pre-development readiness and gates](03-PREDEV-READINESS.md)
5. [04 — Delivery roadmap and handoffs](04-ROADMAP-AND-HANDOFF.md)
6. [05 — Decisions, risk and boundaries](05-DECISIONS-AND-RISKS.md)
7. [06 — Sources and source-lock](06-SOURCES-AND-PINS.md)
8. [VEC — natural-language user programming](vec/README.md): deterministic SDK-first programs, Rust capsules, low-LLM fallback and protected source-editing trajectory.
9. [HQ × Rust × TierFlow convergence research](vec/system-convergence/README.md): deep system research and machine atlas.
10. [machine/README](machine/README.md): status manifest, component register, gates, pins, schema.

Do not merge draft PR #13 or #19, modify HVI owners, infer runtime acceptance, or run real ARBOR/VIVIM/Labs comparison from this folder. This is a **planning/status-only branch**. Any future implementation starts with separate source and trust gate S0.
