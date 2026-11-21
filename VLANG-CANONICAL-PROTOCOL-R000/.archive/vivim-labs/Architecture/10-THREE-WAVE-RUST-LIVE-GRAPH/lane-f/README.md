# Wave 1 / Lane F — Independent verification (Labs-owned)

## Purpose and boundary

This is a **read-only independent verifier** for the A+B+C→Lane E Rust HQKG integration, with D's generated wiki graded as a derived correctness oracle. It writes only to the selected report directory. It does not own a semantic registry, graph import, KB/SQLite database, Host action, or wiki builder. Configured native commands may call the existing Lane E DEV importer, which alone owns its journal. No HQ reset is attempted here; the owner-authorized reset is Lane A's separate local duty.

## Run

Requires Node.js 22+ and a locally approved run manifest. The report directory MUST be under the manifest's workspace.

```sh
node --test test/acceptance.test.mjs
node acceptance.mjs --manifest /absolute/path/to/RUN.json --out /absolute/path/to/reports --execute
```

Exit code `0` means every defined gate check passed; `2` means **BLOCKED** or invalid input. Omitting `--execute` is a non-executing intake check and **cannot** yield PASS. `--execute` runs only explicit `cargo`, `node`, or `bun` argv arrays without a shell, within the selected workspace. Treat RUN.json as an owner-trusted verification recipe, never content from providers or untrusted repositories. Never configure effects, resets, privileged Host calls or a production journal.

## Manifest contract (version 1)

```json
{
  "schema": "vivim.wave1.lane-f/1",
  "project": {"id":"vivim","roots":[{"id":"repos-hq","head":"<full 40-hex current main SHA>","indexed":true}]},
  "files": {
    "reset":"path/to/local-scoped-reset-preservation-receipt.json",
    "graph":"path/to/REAL-LANE-E-readonly-graph-query.json",
    "wiki":"path/to/REAL-LANE-D-generated-wiki.md",
    "intent":"path/to/source-pinned-actual-intent-cases.json",
    "compiler":"path/to/source-pinned-real-compiler-cases.json",
    "placement":"path/to/native-measurements.json",
    "journal":"path/to/isolated-DEV-graph.hqj"
  },
  "commands": [
    {"id":"cargo-test","argv":["cargo","test","--manifest-path","research/rust-knowledge-graph/Cargo.toml","--features","importer"]},
    {"id":"rust-invalid","argv":["cargo","run","--manifest-path","research/rust-knowledge-graph/Cargo.toml","--features","importer","--bin","hqkg-import","--",".hq/lane-f-dev.hqj","evidence/invalid.jsonl"],"expectedExit":2,"expectedText":"hqkg-import:","journalMustNotChange":true}
  ]
}
```

Expected command IDs are exported as `REQUIRED` in the implementation and include native HQ/Cargo tests, import/no-op/restart, invalid/collision/stale/secret refusals, all four queries and wiki regeneration. Exactly reproduce actual E CLI flags after E publishes its handoff; the example is illustrative and not an authorized instruction to run an unspecified path. Do **not** substitute historical research exports or generated fixtures for real locally indexed source records.

The Lane E query JSON must supply `nodes:[{id,kind,sourceSha,...}]` and `edges:[{from,to,evidenceClass,...}]`; a wiki containing the same source→semantic→command IDs is necessary but not sufficient. Compiler and intent evidence JSON must provide `cases[]` with real observed outcomes and source SHAs. The native placement file must provide `components:[{id,owner,recommendation,evidence:[...]}]` with evidence for source-index, comment-compiler, KB-semantic, command-routing, rust-graph, sqlite-search, wiki, Host effects and UI-document ownership. Unknown is not absence; status is **BLOCKED** when required proof is unavailable.

## Output and acceptance

The tool generates graph-vs-target JSON/Markdown, blind-spot register, command receipts with exit codes/durations, compiler conformance, intent-safety baseline, Rust/SQLite placement JSON/Markdown and a **Wave 2 INPUT dossier only**. Its current published baseline is deliberately BLOCKED because these raw local proofs are not accessible in this session. No design or execution of Wave 2 is authorized. Preserve Wave 3's source-span/typed-intent/alternatives/revision/visual-projection interface constraints.

## Source contract caveat

Lane F does not convert Lane E schema into its own replacement IR. `graph` is a **read-only query artifact** produced by E or by the integration owner. The owner must align real E query output to the documented inspection shape and supply raw command receipts; evidence fields and query behavior cannot be guessed. Run locally from the real repos-hq workspace and adjust only F-owned acceptance adapters after seeing E's actual HANDOFF.