# Wave 1 / Lane D — generated HQ wiki quality oracle

**Status: harness implemented and locally tested; live A+B+C+Rust acceptance BLOCKED.**

This lane **does not create wiki pages, modify the HQ SQLite DB, write Rust journals, grant actions or register semantics**. `hq/lib/wiki.mjs:buildWiki()` remains the *only* Markdown/HTML wiki generator. This read-only oracle checks its generated `.md` output against Lane E's exact `repos-hq.hqkg-exchange/1` records (from source-owned A+B+C) and produces an optional diagnostics JSON report. Synthetic tests cannot certify the integrated runtime.

## Run the test harness

From this directory:

```sh
node --test wiki-oracle.test.mjs
```

The tests are labeled **synthetic**. They exercise exchange integrity/digest/counts, denial of code/permission input, cross-domain chains, missing source/meaning/commands, stale/unknown state, broken links, determinism, symlink safety and non-destructive JSON reporting.

## Execute the real integration (after A+B+C+E and scoped reset)

From `repos-hq`, **only with the owner's approved local DEV index**, using a fresh dedicated output directory (not production wiki):

```sh
bun hq/bin/hq.mjs wiki --out /absolute/dev-only/wave1-hq-wiki --force --json
```

Lane E must first build and actually import an HQKGX1 `.jsonl` with the **real** source/KB/semantic/action records into its dedicated `.hqj`, and produce repeat/no-op/read-query/restart evidence. Lane D must not substitute a hand-authored JSONL for Lane E's output. Then invoke:

```sh
node wiki-oracle.mjs \
  --exchange /absolute/evidence/lane-e/hqkg-exchange.jsonl \
  --wiki-dir /absolute/dev-only/wave1-hq-wiki \
  --report /absolute/new/unwritten/path/wiki-quality.json
```

The tool exits **2 (BLOCKED)** if the real graph/wiki crosslink is missing, no journal proof is supplied, or integrity fails. The optional `--rust-evidence /absolute/observed-rust-receipt.json` must identify the exact exchange digest, observed runtime and journal SHA; even then the result is only `READY_FOR_INDEPENDENT_VERIFICATION` and **Lane F must separately rerun and verify**. A report path must not already exist; this tool never overwrites it.

## Required E→D data interface

Use the **existing** Rust importer format, not a second graph store: first line manifest `schema: repos-hq.hqkg-exchange/1`, `source`, `authority`, SHA-256 of all following raw record lines, `node_count`, `edge_count`, followed by canonical `hqg1:` node records then edge records. Within node scalar `metadata`, preserve **explicit** `domain: code|knowledge|semantic|command`, `source_repo/source_path/source_sha` (code), `semantic_id/description/status` (semantic), `syntax/handler/status` (command), and stable record IDs. Edges need `link_status: verified|observed|source_pinned` and `evidence_ref`; do not infer verification from mere static imports, comments or names. Unknown or absent fields become defects rather than guessed truths.

This is a **D-side expectation and diagnostic**; Lane E must reconcile the concrete metadata names to its validated source-owned contract, and may provide a documented bounded read adapter. An E projection with other metadata names will not silently be reinterpreted.

## Quality and safety

The oracle reports an auditable five-dimension 0–100 observed-sample score (with per-dimension denominators), domain coverage, verified candidate chains, missing semantic descriptions, missing source provenance/links, unknown statuses, declared/unwired commands, orphaned KB/commands, relationships without evidence, missing cross-domain wiki pages and dead local Markdown links. A candidate chain is counted only when code→semantic→command is **explicitly linked** in the graph and its canonical source SHA/path, semantic ID, operation ID and syntax coexist in an HQ-generated page. It does not certify knowledge/KB trust solely from a description. The final integration owner must independently verify full KB presence and source authority.

It reads wiki Markdown only (no HTTP links), refuses symlinks, guards secret-looking paths, limits input and page sizes, never accepts code-bearing graph metadata, never invokes a Host effect and never uses Playwright/Puppeteer or provider APIs. The graph exchange is **not** evidence that Rust imported it; native graph/journal acceptance remains with E and F.

## Native acceptance blockers

1. No confirmed completed Lane A reset, multi-root indexing intent and live source extraction in this session.
2. No confirmed integrated Lane E Rust journal/import/query/restart transcript or real `A+B+C` exchange.
3. No local checkout of `repos-hq`, no Bun 1.4+/Node 24+ native HQ execution. Current isolated runner is Node 22.16.0 and cannot access the owner's HQ SQLite index.
4. Existing HQ wiki generator is index-derived; any missing cross-domain B/C meaning is a **real integration defect** to resolve by its original owner, not permission to introduce a second wiki renderer.

**Do not call Wave 1 GO or design Wave 2 from this harness alone.**