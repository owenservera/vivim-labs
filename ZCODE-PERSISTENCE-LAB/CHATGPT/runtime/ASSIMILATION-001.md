# Generalized persistent intelligence assimilation

Baseline 108fb9313978a5eb09e7461d2749634a4c5b8b7a. E41 source implementation.
- Versioned, bounded family policy; explicit disjoint training and single-use review goals. Freeze only examines training goals; training rejection becomes durable.
- Structural class identity normalizes names while preserving expression, behavior, types and graph wiring. A completed pure goal trace is independently replayed against its evidence/oracles. Context roles retain key and type, not old values. Literal/knowledge values become fresh parameters. DAG coordination survives agent loss.
- Scoped verifier submits a deterministic verdict through existing signed ingress; host replays the held-out trace evaluation and checks exact verdict. No claim of private holdout custody from a trusted host. Promote only after review, current policy, and frozen training recheck. Optional automatic freeze/promotion uses existing learning budget.
- Reuse plans list compatible classes; explicit host instantiation requires fresh parameters, context pins and per-task oracles. It creates a sealed goal and tracks outcome, never auto-admits artifacts or effects. Disabling blocks future reuse, not already authorized work.
- Conservative structural matching, not proof of arbitrary semantic equivalence. Topological ties are ordered by original task ID; differently ordered equivalent DAGs may not match. No arbitrary code synthesis, provider execution or network. Existing class-synthesis custody, repair learning and recipes remain separate and intact.
- CLI: kernel <store> assimilation; assimilation-review <candidate>; assimilation-plan <family>. Mutations use the existing typed request admission protocol. No compilation or tests performed.
