# Next executable work orders — start small, prove and promote

## Order D1 — Test and harden the first PWA VEC slice
**Base:** `work/p5-vec-pwa-ui-kernel-r001`. **Exclusive scope:** `apps/mobile-relay/vec/**`, `apps/mobile-relay/{build-composer,build-canvas,sw.js}`, `apps/mobile-relay/test/*vec*`, `apps/mobile-relay/test/service-worker.test.mjs`, `apps/vivim-host/direct-phone.mjs` ONLY for static read-only allowlist. Refresh real branch HEAD first, preserve concurrent commits. Run the exact matrix in `03-PAIRED-PWA-ACCEPTANCE.md`. Fix all failures, commit tests/evidence. **Never modify Host dispatcher, bootstrap publisher, action catalog or private-data store.**

## Order A1 — Versioned installed SDK descriptor and cross-runtime packaging proposal
**Base:** current main after source-pin review. **Scope:** design + pure typed adapter only, agreed with Host owner. Prove one stable capability manifest that can feed VEC without a second runtime action registry. Show source-to-artifact digest, code version, args/output types, effect class, availability and live Host grant dependency. Respect that `apps/mobile-relay/vec/compiler.mjs` is the provisional PWA source and that final shareable packaging may move to a canonical internal module. Never require direct HQ developer-index access for ordinary phone commands.

## Order B1 — HQ Wiki shadow parity (not takeover)
**Base:** re-pin main and relevant HQ/SV source; separate branch. **Exclusive scope:** new VEC Wiki adapter and HQ synthetic tests; no changes to `hq/lib/wiki-program.mjs`, `hq/lib/wiki.mjs` writer/render behavior or `hq/lib/semantic-registry.mjs` authority until gate. Feed identical invented configs/source-pin failures into incumbent HQ and VEC shadow; compare exact diagnostics, program config, previews, rollback histories and rendered Wiki hashes. Prove same inputs give same output. Report measured deviations with root cause; only afterward propose B2 takeover.

## Order C1 — NLCL/Host command shadow parity (not dispatch)
**Base:** re-pin current Host action catalog and command-intent version; separate branch. **Exclusive scope:** VEC read-only NLCL adapter and synthetic fixture tests. Compare `control-language`, `command-intent`, `intent-planner`, `command-program` on commands vs UI reprogramming requests. Preserve negation/ambiguous/risk/slot refusal and actor/policy/version gates. No Host action dispatch, no provider sends, no grant escalation or changes to source-of-truth catalog. Produce C2 cutover proposal only after actual tests.

## Order E1 — Rust ABI differential proof
**Base:** after A1 contract agreement, current `core/programmability`, `native/hqkg` source. Map existing bounded inert Host command plan to Product Rust transform validation with explicit versioned schema and serializable refusal codes. Cargo and Node parity tests with synthetic invented inputs; no new Rust scheduler or personal graph writer. HQKG only pure scoped compute reuse. Benchmark before promotion.

## Order F1 — Independent security/release acceptance
**Base:** exact integrated heads after D1/A/B/C/E, no guessed merged base. Execute actual Node24/Bun/Cargo, Host Device/Direct PWA, offline cache noninterference, PC-private, revocation, uncertain sends, stale revisions, physical Android Chrome and native Windows. Commit independent proof with GO/NO_GO and actual exit codes.

Stop at every required prerequisite gate. Do not issue GO or merge from a static source inspection. Each worker must return exact branch SHA, modified file list, actual tests versus UNRUN, permission impact, independent blockers and next gate.