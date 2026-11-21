# ZCode Native Observer (Rust) — R003 G1

Read-only, independently executable ZCode app-server protocol check. It starts and stops **its own** installed ZCode child, makes only `session/list`, `plugins/list`, `workflows/list`, `v4/commands/query`, and `v4/conversation/subscribe` queries, and prints sanitized status/counts (no session content or provider secrets).

Run from this crate: `cargo run --offline` (or `cargo test --offline`). Requires the existing Windows Node 24 and installed ZCode CLI at their standard paths, plus ARBOR checkout. No session mutation, workflow execution, provider changes, seat wakes, Git operations, or bus writes.

G1 PASS requires: legacy reads + unknown command read-only query + V4 subscribe ACK and one snapshot frame, actual exit 0. Delta replay, resync, cancellation and cross-thread delivery remain separate tests. This is a Labs validation harness, not a second runtime or Host effect authority.
