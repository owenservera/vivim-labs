# Migration sequencing, dependencies, and authority

**W0 — Wholesale external Craft.** Keep website capture, walkthrough and MCP runtime in Craft. Use the existing Labs `craft-anything-arbor-adapter` exporter/interpreter, then measure real authorized MVP states. Do not port the CDP actor, session or sign-in profile.

**W1 — External evidence bridge and integrity.** Close genuine Craft Rust `Evidence::verify` parity with the Labs adapter. Existing Labs R002.1 reported 23 passing local tests but actual Craft native parity, first-party capture and Studio readback UNRUN. Recheck fresh evidence; don't reuse synthetic fixture outcomes as product verification.

**W2 — Focused pure Rust harvest.** Candidate group A: canonical evidence/unknowns. Group B: pure comparison and expectation ledger. Group C: semantic feature tracing and JS operations. Compare with ARBOR `core/programmability/src/semantic_observation.rs`, Studio factory, HQKG and VLANG before moving source. Every proposed move needs full transitive closure and real parity tests.

**W3 — Conditional advanced analysis.** JS AST/OXC analysis, HAR redaction and API outlines, ASAR/sourcemap/SQLite parsing, optional stand-ins and mobile-native app studies. Start only when direct Studio/Harness evidence calls for them; review `craft-libs` git pins and security concerns.

**W9 — Do not port.** Craft browser/CDP, signed-in profile handling, independent approvals, process/window effect owner, project journal, LLM orchestrator, egui UI and large unrelated game/firmware parsers. VIVIM PWA + existing Host/GPUI boundaries remain authoritative.

### Exact duplicated-owner references verified in ARBOR
- `studio/src/studio/adapter-evidence-intake.mjs`: Core-verified chat-centric evidence admission. Craft cannot mint its accepted capture classes.
- `studio/src/studio/adapter-factory-service.mjs`: read-only learned adapters, server-side held-out re-verification; not send path.
- `studio/src/profiles/sites/claude.mjs` versus `harness/src/providers/claude.mjs`: currently divergent generic textarea fallback, pending runtime measurement.
- `native/hqkg/src/web_memory.rs`: metadata-only web visit graph, not UI evidence blob store.
- `core/programmability/src/semantic_observation.rs`: passive aggregation only, not authorization.
- `hq/lib/kg-export.mjs`: authoritative HQ source index export, not an external Craft record importer.

### Dependency caution
Workspace `Cargo.toml` pins numerous external `storytold/craft-libs` revisions. `crates/formats/src/har.rs` and `sourcemap.rs` wrap external crates. `crates/analysis/Cargo.toml` depends on formats/disasm/JS; `crates/js/Cargo.toml` pulls OXC syntax/semantic tooling. Treat file-level reuse as a transitive closure question, not simply copy the .rs.

### Release meaning
Prioritize real source verification, dependency closure, accepted owner, test vectors and license audit before changing any row to `MIGRATED`. This is a **disposition tracker, not an automatic migration ledger**.
