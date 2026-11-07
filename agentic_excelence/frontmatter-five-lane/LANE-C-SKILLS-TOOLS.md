# Lane C — Skills, best-practice methods, tools and JSON configuration

**Owns:** 79 files: family 09 (67), family 08 (5), family 10 (7). Input: A's schema, 217-file matrix, source bodies and file flavors. No installed skill, daemon or tool handler may be created just by metadata generation.

## Type-specific design
- `SKILL.md` 52: native agent-skill `name` + `description` in YAML where supported. Metadata sidecar maps exact trigger, task/problem, prerequisites, arguments, execution mode, safety, output, verified scripts, fixtures, failure modes, dependencies and observation receipts.
- `BEST-PRACTICES.md` 15: domain, applicable conditions, evidence date, alternatives and anti-patterns. Research advice ≠ binding owner policy.
- `TOOL-REGISTRY.json` 1, `TOOLS.json` 3: versioned JSON schema, current action IDs, actual wiring/handler status, effect class, authentication/owner permission boundaries; **no YAML frontmatter**.
- `TOOLS.md` 1: descriptive, source-grounded human view of registered capabilities with dynamic readiness data referenced, not copied as permanent truth.
- `claude.json` 4, `gemini.json` 2, `agent.json` 1: exact target schema/version and known supported keys; all universal metadata external unless native schema explicitly permits the fields.

## Work product
For each file: exact original hash and role, native/sidecar format, required native keys, 13-type section crosswalk, unclassified headings, unique trigger or tool IDs, verified compatibility evidence, examples, refusal rules, and whether a valid current invocation/handler receipt exists. Run syntactic JSON/YAML tests before calling a draft valid.

## Key traps
52 skills cannot share one generic description; a native skill trigger must be discriminative. A source snippet that mentions a tool does not prove it is installed; 2,000 command catalog entries do not imply 2,000 wired handlers. Invalid JSON or unknown keys must be refused, not auto-corrected by stripping meaning. README-style best practices are not executable recipes.

## Gate
**79/79** distinct file IDs specified; 52 skills with individualized trigger plans and native schema constraints; 11 JSON configs/registries preserved as JSON; no fabricated runtime status; independent Lane E parser and recall tests.
