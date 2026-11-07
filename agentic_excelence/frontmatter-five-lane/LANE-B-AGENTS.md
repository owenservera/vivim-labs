# Lane B — Agent identity, bootstrap and provider-native instructions

**Owns:** 60 files in families 01 (53) and 02 (7). Input: A's schema + `FRONTMATTER-217-WORK-MATRIX.json`, exact source bodies, original section atlas. Does not modify active `AGENTS.md` or provider config.

## Type-specific design
- `AGENTS.md` 26: project/path scope, inherited instruction precedence, current owner rules, constraints, effective consumer, conflict/refusal semantics; **sidecar-first** unless current provider-specific loader is proven to recognize selected header fields.
- `BOOTSTRAP.md` 20: first-run prerequisites, environment gates, exactly reproducible safe startup, readiness checks, recovery and provenance; avoid stale environment claims.
- `WORKSPACE.md` 4: workspace identity, project boundaries, entrypoints, source/ownership map.
- `AGENT.md` 3: role/mandate, input/output typed contracts, tool boundaries, handoff and escalation; not self-authorized permissions.
- OpenCode `.opencode/agents/*.md` 3: inspect actual YAML (observed `description`/`mode`), only loader-verified native keys; private role/authority metadata in a sidecar.
- `CLAUDE.md` 3, `GEMINI.md` 1: exact provider-native path conventions and instruction precedence; do not flatten across providers.

## Each file's work product
File ID → observed header status (native/plain/unknown), loader and version evidence, identity+scope fields, file-role fields, section links, unique content signposts, unknowns, forbidden auto-fields, target sidecar/native projection, fixtures. Attribute all instruction/status/permission claims to current authority; historical AION donor instructions are research, not VIVIM rules.

## Essential adverse scenarios
Root and nested instruction precedence, same-named files from different repos, provider switch, stale bootstrap, instruction injection in source data, owner revocation, multiple agents sharing a workspace, contradictory nested rules, and synthetic attempts to grant permissions via Markdown.

## Gate
**60/60** distinct file IDs documented, original scope preserved, no invented runtime version, zero unsupported native key emissions and no activation. At least five representative inert native/sidecar examples with side-by-side source/target differences; independent Lane E acceptance.
