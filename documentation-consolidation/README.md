# VIVIM documentation consolidation

Central repository for relocated **human-authored documentation**, separated by source repository and original relative path. Working source, agent instructions, executable prompts, schemas, task-state registries, test fixtures, installed wikis and live contracts stay in their owner repositories.

## Source layout

- `tierflow/` — relocated TierFlow historical documents and blueprints.
- `vivim-web-harness/` — relocated historical/design documents from the VIVIM product repo.
- `repos-hq/` — relocated research and dated documents from the HQ repo.
- `INVENTORY.json` — document-by-document source SHA, original path, move/retention decision.

This is **not** an alternative runtime or source of execution authority. Source code and deployed system behavior are unchanged by a document migration. Moved Markdown retains its original content and relative path under its source prefix. Relative links to code in the source repository may need follow-up repair.

**Safety:** Every deletion requires a successful destination publish and blob SHA verification, then a non-force, expected-head update to source `main`. Never delete `AGENTS.md`, `SKILL.md`, developer agent configs, JSON/CSV datasets, operational instructions, or runtime files.
