# Paste into ChatGPT High: Lane 2 — Rust Inline Comment Programming Language V0.1

You own **Lane 2** of `owenservera/repos-hq/CHATGPT-SPRINT/SPRINT-02`. Work now with High reasoning effort; implement a tested Rust compiler and push source to main. Read README, CONTRACT, SOURCE-MAP, ACCEPTANCE and existing HQ annotation/wiki program code and tests first.

**Owner requirement:** This language, and only this language in the sprint, is **Rust-based**. It is a genuine typed declarative **programming language embedded in source code comments**. Its FIRST executable consumer is the **intelligent interactive wiki**: compile comment programs to versioned wiki program IR/config, not prose metadata. It must remain safe: the compiler itself never executes source comment text, grants permissions or calls shell/provider actions.

**Write ONLY** NEW `owenservera/repos-hq/research/comment-program-rust/**`. Don't change Rust HQKG, existing HQ JS annotation/wiki modules, Host or other lanes.

**Must ship:** Rust crate + CLI compile/check; real comment-only lexer (strings ignored) with explicit supported languages; tags for wiki/source/filter/layout/select/bind/requires/evidence; typed parser, source-map (path/line/hash), version and deterministic JSON diagnostics; registration resolution, strict bounds/unsafe/secret rejection; V1/V2 @hq compatibility; canonical `hq.comment-program/0.1` JSON carrying actual `hq.wiki-program/1` compatible config plus inert bindings. Cross-check source and actual validator enums instead of trusting examples.

**Stretch:** multi-file dependency references, incremental hash caching, 100+ syntax/refusal fixtures, compilation benchmark, source update changes the resulting interactive wiki view deterministically. Tests must show comment program is not authority to run effects.

Direct commits to main; SHA-safe updates, no PRs/branches/GitHub Actions, never overwrite other lane files. Show actual Cargo tests run or honest unverified state, compiler example, generated JSON, commits and integration handoff.
