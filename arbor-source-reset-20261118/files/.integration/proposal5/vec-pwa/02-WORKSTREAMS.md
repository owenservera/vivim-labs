# Six workstreams — clear owner boundaries and staged migration

## A · Unified compiler contract and SDK registry
**Objective:** One VEC utility, one typed output envelope, three pluggable compiler frontends. Do NOT create another registry authority. Today: pure browser-compatible kernel added; UI adapter installed, HQ/command owners not migrated. Next: define installed SDK manifest and canonical cross-runtime source packaging after package/asset/Node ABI analysis.
**Gate:** independent synthetic contract tests, no caller-controlled adapters/policy, compile outputs always inert.

## B · HQ → VEC Wiki compiler transition
**B0 BASELINE:** pin `protocol.mjs`, `annotation-v2.mjs`, `semantic-registry.mjs`, `wiki-program.mjs`, `wiki.mjs`, Rust-comment bridge. Record actual fixtures, hashes, wrong-source and rollback behavior.
**B1 SHADOW:** write VEC Wiki adapter that *reads/delegates* incumbent HQ parsers and `validateProgramConfig`; compare canonical AST/diagnostics, deterministic hashes and preview/diff output against golden synthetic fixtures. HQ still sole writer.
**B2 COMPILER TAKEOVER:** move source-comment and Wiki program compilation logic one grammar family at a time into VEC, with VEC output the same versioned Wiki config/AST; HQ keeps development index and authoritative activation/writer; dual-run until exact parity plus explicit revised semantics are independently accepted.
**B3 WIKI VIEW TAKEOVER:** allow VEC to own Wiki projection assembly/compilation into Markdown/UI only after generated Wiki byte/diff and link provenance parity. HQ remains index/database/source evidence owner; no second Wiki source-of-truth. Remove retired duplicate compilers only after cutover and rollback plan.
**Gate:** bit-identical or explained and approved differences; owner/source grants/lineage not weakened. This lane is NOT IMPLEMENTED YET.

## C · Command/NLCL → VEC typed command compilation
**C0 BASELINE:** pin Omega NCLL, `control-language.mjs`, `command-intent.mjs`, `intent-planner.mjs`, `command-program.mjs`, Host action catalog and personal semantics. Create synthetic grammar/negation/ambiguous/risk fixtures.
**C1 SHADOW:** use VEC as read-only adapter around existing NLCL/control planner, return inert typed candidates, diff exact outcomes/slots/risk/diagnostics/lexicon against incumbent Host. NO dispatch.
**C2 COMPILER TAKEOVER:** replace duplicate intent+slot compilation in steps, preserving current Host-approved catalog owner, risk/policy revision and personal rules. The same compiled meaning from phone text, desktop chat and tested synthetic inputs.
**C3 COMPOSITION TAKEOVER:** compile bounded multi-command plans using existing `command-program/1` invariants and current catalog versions, with independent equivalence/counterexample proofs. Host dispatcher, actual grants, one-writer journal and action receipts remain untouched.
**Gate:** unknown/negation/ambiguous syntax fail closed, no duplicate sends, no new effect authority. This lane is NOT IMPLEMENTED YET.

## D · First real reprogrammability: paired PWA UI
**D0 SOURCE_IMPLEMENTED:** narrow deterministic UI editing on the existing Build screen; four operations, same `ui-proposal/1` validator, same Host Activate and undo, new scripts in Direct allowlist and shell cache.
**D1 EXECUTE/ACCEPT:** Node24 tests, static allowlist, shell service worker, paired Host integration and revision-conflict/revocation negative matrix.
**D2 EXPAND:** make selection and gestures/typed target IDs first class, semantic disambiguation, theme/layout/filter templates and explicit source-ref grants; keep data-only VEC IR.
**D3 TIERFLOW:** exact versioned crosswalk between phone Surface/1, TierFlow World/1 and Visual/1. Activate only one canonical publisher; dynamic data bindings, a11y and phone/desktop parity separate gates.
**Gate:** no auto-publish, no untrusted script, no stale replay, no PC-private leak; physical PWA acceptance before GO.

## E · Rust native pure compute and SDK capsules
**E0 BASELINE:** product `core/programmability` and HQKG typed DAG/engine already exist, with different ownership.
**E1:** JS VEC ↔ Product Rust typed IR parity and Host-authenticated ABI; no new scheduler/data store. Benchmark pure transforms on invented fixture corpus before moving any hot path.
**E2:** optional reviewed Rust comment capsule SDK (isolated build, independent source/oracle/verifier, constrained runtime). No per-utterance rustc and no automatic source edits.
**Gate:** actually executed Cargo/FFI tests and trusted source/policy; untrusted capsule runtime MUST be isolated.

## F · Independent verification, security and release
**Objective:** prove A–E across real source runtime, not with labels. Test Node/Bun/Rust separately, Host pairing/auth/revocation, visual revision conflicts, offline PWA/Service Worker no API cache, actual phone Chrome, Windows native/Host process and deterministic fallback.
**Gate:** actual exit codes, install artifact, physical-device checks and owner acceptance. Never invent beta GO from authored test files.

## Workstream independence
A establishes common output contract. B and C can run **in parallel as read-only shadow adapters** against their current owners. D can independently validate in isolated PWA scope while B/C build proof. E starts native only after A + concrete parity. F independently checks each before any promotion.

**Cross-lane freeze:** no one edits the Host action dispatcher, HQ Wiki writer, TierFlow publisher, root lockfiles or main unless a new explicit owner-scoped integration work order and acceptance gate is issued.