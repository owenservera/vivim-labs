# Sprint 02 — Language V0.1: Command, Comment, Interactive Wiki

Date: 2026-11-04. Three parallel High-effort agents, direct commits to main. This folder is the launch package; source work is not implied complete.

## Era 1 — Three workstreams
1. **COMMAND** — JS/TS: improve the *existing* NLCL and Host natural-language interpreter, settle `vivim.command-intent/0.1`, preserve negation/ambiguity/refusals, support desktop/phone.
2. **COMMENT** — **Rust only here**: design and implement the V0.1 inline comment programming language compiler; comment programs compile to a safe, deterministic intelligent interactive wiki program.
3. **WIKI** — JS/TS: bridge Rust compiled comment programs to the existing `hq.wiki-program/1` preview/diff/version/rollback and map wiki gestures and NL commands to one typed intent. No third language.

## Era 2 — Architectural law
The Rust comment compiler interprets **source comments as declarative wiki code**, with syntax, types, imports/references, source positions, deterministic diagnostics and a compiler output manifest. It does not run arbitrary JavaScript/Rust/shell or self-authorize an effect. The wiki evaluates approved program *data* through its existing registered components. NLCL stays JavaScript/TypeScript: do not port it to Rust merely for language uniformity. JS/TS bridges are the most practical way to consume these V0.1 contracts in the current Host/wiki.

Cross-repository convergence is a future integration concern: preserve project-neutral typed contracts and keep browser/Host effect ownership in the existing Host/Studio, not Repos HQ.

## Era 3 — Launch
Read [CONTRACT.md](CONTRACT.md), [SOURCE-MAP.md](SOURCE-MAP.md), [ACCEPTANCE.md](ACCEPTANCE.md). Open three independent High-effort chats and paste [Lane 1](prompts/LANE-1-COMMAND.md), [Lane 2](prompts/LANE-2-COMMENT-RUST.md), [Lane 3](prompts/LANE-3-WIKI.md).

Independent ownership; direct to main; no branches, PRs, GitHub Actions or force push. Fetch expected file SHA before updating. Run tests if tools permit; report live Windows/Android checks as unverified if they were not run.
