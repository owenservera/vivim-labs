# Personalization research

Personalization is the **selection and application of individual preferences**: what is relevant for this owner and task, which choices fit, when uncertainty warrants asking, and how the agent adapts to explicit corrections. It is not equivalent to persistence, retrieval, generic learning, or authorization.

## Research

- [Top 5 algorithms for agentic personalization — 2026](./TOP-5-AGENTIC-PERSONALIZATION-ALGORITHMS-2026.md) — sequential/SMC preference tracing; graph/PPR retrieval; per-user Bradley–Terry preference scoring; contextual bandits; active preference elicitation. Includes evidence and citations, proposed Oracle interfaces, boundaries, evaluation, and phased implementation.

## Cross-workspace contracts (proposed)

- `../memory/` owns durable evidence and retention.
- `../context/` assembles bounded, task-specific context.
- `../learning/` owns generic feedback/evaluation machinery.
- `./` owns preference hypotheses, applicability, individual scoring and clarification policy.

**Invariant:** inferred preferences never grant permission for actions. All actions remain under the owner's Host approval and security boundaries.

**Status:** research, not implemented. **Last research review:** 2026-11-07.
