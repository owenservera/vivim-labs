# Paste into ChatGPT High: Lane 1 — Command Language V0.1

You own **Lane 1** of `owenservera/repos-hq/CHATGPT-SPRINT/SPRINT-02`. Work now with High reasoning effort; produce executable source, tests and direct-to-main commits. First read README, CONTRACT, SOURCE-MAP and ACCEPTANCE from that folder, then inspect the latest actual files.

**Target:** settle the natural-language command **V0.1**, using the most practical/current **JavaScript/TypeScript** implementation: existing vendored NLCL via `apps/mobile-relay/control-language.mjs`, `apps/vivim-host/control/intent-planner.mjs` and the Host action catalog. **Do not port NLCL to Rust.** Rust belongs to Lane 2's separate comment compiler.

**Ownership:** Only `owenservera/vivim-web-harness/apps/mobile-relay/control-language.mjs`, `apps/vivim-host/control/intent-planner.mjs`, NEW adapters under `apps/vivim-host/control/**`, and focused tests within `apps/vivim-host/test/**`, `apps/mobile-relay/test/**`. Do NOT modify vendored `apps/mobile-relay/nlcl/**`, Host dispatcher, HQ/wiki or another lane's files unless coordination is explicit.

**Must ship:** typed `vivim.command-intent/0.1` with deterministic parse/normalize, explicit negation and conjunction refusal, ambiguity and missing argument outcomes, catalog revision/scope/risk/provenance, canonical symbolic form, bounded AI candidate validation (AI never authorizes), preserved current dispatcher backward compatibility. Prove no negated or compound sentence silently becomes a runnable single action. Include concrete normal English, Spanish if supported, and hostile/ambiguous fixtures.

**Stretch:** 100+ conformance sentences, editable aliases/lexicon without parser rewrite, and a stable typed-intent fixture consumable by Lane 3's wiki gesture adapter. Focus on user-visible command correctness, not new CLIs.

Fetch exact file SHA before updates; small commits direct to main, no PRs/branches/Actions/force push. Do not claim Windows/Android or model end-to-end tests without actual evidence. Final report: commits, files, tests, syntax and outcomes, remaining gaps.
