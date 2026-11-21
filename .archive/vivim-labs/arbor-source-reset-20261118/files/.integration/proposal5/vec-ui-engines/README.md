# VEC UI Engine System — canonical cross-runtime configuration foundation

DESIGN + SOURCE FOUNDATION. NOT 461 installed user features or accepted physical device behavior.

## Scope completed on this branch
- 37 engine families and 461 typed configuration capabilities, entirely independent of the old PWA theme preset limitations.
- Single new internal source package: packages/vec-ui-engines, pure ESM with no Host/DOM/renderer effects.
- Typed validators, reviewed enum choices, inert multi-engine proposal compiler, typed DTCG-style token subset, semantic applicability registry.
- 100 individually addressable agent work orders with unique source scopes, no implementation collisions.
- Acyclic 37-node design dependency graph and canonical precedence/property-ownership policy.

## Read in order
00-MASTER-ARCHITECTURE.md — product vision, executable interface contract and scope.
01-CONTRACTS-AND-OWNERSHIP.md — authority boundaries and precise schema/ID rules.
02-WAVE-AND-MERGE-GATES.md — gated integration of 100 parallel outputs.
03-100-AGENT-FLEET.md — 100 agents, exclusive paths, starter prompt.
04-DEPENDENCY-CASCADE.md — DAG, CSS ownership, design token layering and conflicts.
05-STANDARDS-AND-COMPATIBILITY.md — external standards and source adoption.
06-ENGINE-FAMILY-ATLAS.md — all engine families and example capability IDs.
07-REPROGRAMMABILITY-UX.md — the paired PWA user journey.
08-OWNER-INTEGRATION.md — PWA/Host/TierFlow/HQ and Rust wiring.
09-SECURITY-AND-TESTS.md — falsification and real device acceptance.
10-IMPLEMENTATION-STARTER.md — commands and agent launch protocol.

Machine: machine/engine-catalog.v1.json, machine/engine-dependencies.v1.json, machine/100-agent-fleet.json,
machine/gates.json, machine/implementation-status.json.

The existing paired PWA VEC UI reprogramming work remains the first product delivery lane.
The new package does not yet change its renderer catalog, Host publisher, service-worker or root workspace manifest.
