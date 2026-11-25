# Rust keeper workplan — engineering pending

Planning/bootstrap completed 2026-11-11. This is the keeper's execution sequence, not a product roadmap or authority grant. Evidence/reasons and all file:line anchors are in STRATEGY.md. First obtain a bounded Rust assignment through the parent and reconcile foreign `src/observability.rs` work. Do not repeat baseline gates for this documentation-only turn.

## 1. HQKGX1 ordering regression — first concrete action

**Proposed scope:** `C:/0-BlackBoxProject-0/repos_hq/research/rust-knowledge-graph/tests/import.rs`, then `src/import.rs` only after a reproduced red. Not authorized/applied here.

Add `rejects_tombstone_before_redeclared_node` with valid manifest digest/counts and tombstone→node for the same canonical ID; require rejection. Existing test at `tests/import.rs:184-198` covers the reverse order only. Add edge-after-tombstone coverage and preserve legacy manifests. Expected fix: monotonic record phase plus symmetric node/tombstone disjointness, without changing schema or optional defaults. This is a predicted failure based on source, not an executed red.

**Acceptance after authority:** run directly from the crate, capturing each command's own exit:

```text
cargo test --features importer --test import rejects_tombstone_before_redeclared_node -- --exact
cargo test --features importer --test import
cargo fmt --check
cargo check
cargo test --features importer
cargo build --features importer
```

Record actual totals and failing→passing evidence. Parent must isolate accepted source from concurrent observability changes; a green whole-tree run does not assign ownership of foreign edits.

## 2. Mutation trust and failure atomicity

**Proposed scope:** targeted tests in `tests/import.rs`, `tests/graph_change_set.rs`, `tests/scoped_graph.rs`; possible `src/import.rs`, `src/graph_change_set.rs`, `src/scoped_graph.rs`, `src/lib.rs` only after contract decision.

Test a late collision after an earlier valid mutation, extra reserved exchange keys in an otherwise valid scoped record, public overwrite/removal of an imported record, and reserved-property preservation on journal replay. Explicitly distinguish caller staging from atomic library apply and trusted raw change sets from untrusted ingress. Do not break deliberate raw APIs under a blanket security claim.

**Acceptance:** relevant test executables via `cargo test --features importer --test <name>`, then the four core gates above. Assert graph serialization/epoch/indexes and ledger unchanged where atomic refusal is promised. Decision needed: internal-grade API visibility versus universal owner-fenced semantics.

## 3. Existing-capability product slice (W04)

**Proposed scope:** no new module by default. Agree existing HQ/context output and Harness consumer insertion point with hq-data-keeper and Host owners before authorizing any source write. Use synthetic, nonprivate fixture inputs.

Trace approved source identity/revision→existing graph/context computation→existing Host memory/World consumer→phone projection, including revoke/delete/stale negatives. Keep personal and development graph ownership distinct. Native `command-program/1`→`native-program/1` crosswalk is a coordinated separate requirement; preserve action IDs, risk, approval and policy/source revision. ABI/invocation is unresolved, not assumed to be Tauri from Node.

**Acceptance definition pending selected consumer:** differential results against existing JS behavior; exact entrypoint test command chosen from that consumer's existing scripts, run with `bun run <script>` where applicable. Do not invent a passing end-to-end command. Harness native gates when admitted: `cargo test --manifest-path C:/0-BlackBoxProject-0/vivim-web-harness/core/programmability/Cargo.toml` and `cargo check --manifest-path C:/0-BlackBoxProject-0/vivim-web-harness/src-tauri/Cargo.toml`, then actual authorized IPC/device evidence. All UNRUN here. JS exporter regressions route to hq-data-keeper (`hq/test/kg-export.test.mjs`), not keeper edits.

## 4. Recovery / deletion / epoch boundary matrix

**Proposed scope:** `tests/import.rs`, `tests/core.rs`, `tests/durable_dag.rs`, then minimal implementation changes if justified. Cover tombstone cascade of foreign-owned incident edges; mixed batch failure; HQJ1 legacy recovery versus HQD1 refusal; restart, stale writer, byte/epoch CAS. Preserve currently intentional HQJ1 recovery unless explicitly changed. Add process/crash injection only in an isolated synthetic fixture, no live journal.

**Acceptance:** `cargo test --features importer --test import`, `--test core`, `--test durable_dag`, full core gates, exact bytes/epoch/index/projection assertions. Windows filesystem/power-loss guarantees need separate evidence; unit restart tests do not establish them.

## 5. Accept foreign observability + measurement

No parallel edits to `src/observability.rs`. Parent obtains that lane's checkpoint, diff and exact tests; keeper checks revision/evidence retention, ambiguous-vs-confirmed causal claims and unresolved results. Re-pin heads/dirty paths after integration.

Measure the chosen consumer with identical synthetic corpus, build/hardware, n, warm/cold split, p50/p95, memory/bytes, freshness/quality/omissions and journal/cache costs. No performance claim or speed-led architecture migration until this evidence exists.

## Current evidence and stop rules

Inherited clean baseline at HQ a1d6ada4: fmt/check/test-importer/build-importer = 0; 225 passed / 0 failed / 11 ignored, exact log retained in STATUS. Current foreign dirty observability change is not covered by that claim. All tasks/tests above are pending and UNRUN in this turn. Stop on foreign overlap, unsupported authority, consumer disagreement, stale revision or missing authentication; report the exact blocker instead of expanding infrastructure.
