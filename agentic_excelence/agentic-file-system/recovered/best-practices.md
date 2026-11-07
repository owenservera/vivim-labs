# Sections: shared names, scoped semantics

| Section type | Agent instruction file | Skill file | Architecture/schema file | SITREP/HANDOFF |
|---|---|---|---|---|
| Purpose | mandate, hierarchy, boundaries | task trigger and result | invariants and system boundary | current objective and state |
| Best practices | enforceable local rules | reproducible procedure | alternatives, tradeoffs and design principles | evidence-backed lessons, not new policy |
| Verification | exact commands and stop rules | success/failure fixtures | integrity assertions and threat model | test receipts and open failures |
| Memory | canonical reference hierarchy | on-demand supporting refs | identity, lineage, retention | decision and handoff context |
| Recovery | escalation rules | retry/rollback recipe | failure domains, durability | exact resume point and owner |

Guidance: [AGENTS.md](https://agents.md/) uses freeform Markdown and scoped instructions; [Agent Skills](https://agentskills.io/) favors progressive disclosure; [MCP](https://modelcontextprotocol.io/specification/) provides typed tool contracts; [SQLite FTS5](https://sqlite.org/fts5.html) provides indexed lexical retrieval. External best practices are **examples, not automatically authoritative instructions**. Each section variant must retain provenance and test evidence.
