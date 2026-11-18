# Native graph wiring — 2026-11-10 (draft)

## What actually connects now
- Product Rust `core/programmability/src/lib.rs` validates bounded typed programs without storage, OS effects, grants, provider sends or CLI.
- `core/programmability/src/lineage.rs` derives a **data-only, opaque** native transformation DAG graph from a successfully validated program. `wire::validate_json` includes it as `lineage`. IDs hide source identifiers; status explicitly records that policy is caller-supplied and source data is **not authenticated**.
- Existing Windows `src-tauri/src/main.rs::validate_native_program` returns the compiler and lineage read model as an inert Tauri command. It is NOT called by the Node Host or Android yet.
- Existing Host control journal `apps/vivim-host/control/journal.mjs` and personal semantic `personal-rule-store.mjs` remain canonical local writers. The existing Studio World service now reads **only bounded metadata** from those stores.
- `studio/src/studio/world-control-lineage.mjs` converts canonical observations into opaque control-plan, action and rule nodes/edges. The existing `studio/src/studio/world-relations.mjs::map` injects those into **the same existing `TemporalGraph` instance** as conversations, projects and notes. This is a derived projection, not a new database or scheduler.
- A bad/corrupt source sets `complete:false`; no effect is claimed verified. Private phrases, parameters, tokens, prompts and messages are never copied into this overlay.

## The two graph owners remain different
1. **Personal World graph:** Host/Studio owner-local, with source-backed observations and user-approved data. The product's read model now includes command-program lineage. No additional graph writer was introduced.
2. **HQKG development graph:** canonical cross-repo *engineering* source identity/index in `repos-hq`. It may index commit SHA + source contracts and redacted implementation evidence, never private source references, personal policy contents or live user graphs. This patch does not change its importer.

## End-to-end flow and current limitation
- Host journal and personal rule store -> trusted local bounded reader -> redacted metadata -> `TemporalGraph` read model -> existing `GET /world/map`. **Source-wired in this branch**.
- Tauri client -> native Rust typed JSON validator -> validated inert DAG -> native opaque lineage nodes/edges. **Source-wired in this branch; actual native compilation/IPC test UNRUN**.
- Node Host use of the Rust result and Android UniFFI/JNI integration are **not yet wired**. Do not claim the two source-wired paths are a production end-to-end pipeline.

## Security and regression gates
- `node --test studio/test/unit/world-control-lineage.test.mjs` — six pure tests passed in isolated Node v22.16.0 reproduction. Branch also adds two integration fixture cases to `world-relations.test.mjs`, **not run on a full source checkout**.
- `cargo test --manifest-path core/programmability/Cargo.toml` — UNRUN, no local rustc/cargo. The Rust native graph tests are authored, not executed.
- `cargo check --manifest-path src-tauri/Cargo.toml` — UNRUN. Update `src-tauri/Cargo.lock` from the target toolchain before any locked build.
- Run full Studio World tests, Host security regressions, source-corruption tests, native invoke, Windows MSI and real paired Android tests. Examine return authority, grants and accessibility. No device GO claimed.
- Keep browser provider send-once journal, Host approval and native device broker unchanged; native graph payload never authorizes execution.

## Next owner integration (do not skip)
1. Verify Rust/Windows toolchain and compile. Fix any diagnostics before main merge.
2. Crosswalk an actual Host `command-program/1` inert DAG into `native-program/1` *without losing* action IDs, approval requirements or risk. Differential-test existing Host and native results; fail closed on disagreement.
3. Define a trusted Host-native invocation boundary (not a new service): preferably an approved Node native binding/ABI or existing native IPC proven accessible from both required runtimes. Do not assume `Tauri.invoke` is callable from the Node child process.
4. Connect the validated result's **non-authoritative** lineage to the existing World map through authenticated current Host source data; bind exact source and policy revision. Preserve control journal and graph projection separation.
5. Android: prove portable native build/FFI and device-bound authority. Until then, Android receives the existing owner's graph projection and makes no native claims.
