# VIVIM Chronos — five-repository integration handoff, wave 2

**Status: staged source changes, tests partially executed, no main push, no live VIVIM activation.**
**Baseline checked on 2026-11-08:** repos-hq `d8ba1ef4`, vivim-web-harness `b32402dd`, tierflow `dbf2c829`, vivim-labs `80290807`, SOFTWARE_HQ `84c86ce7`.

## Era 1 — What the code does

This bundle extends the canonical **Rust HQKG** compute registry with a std-only Chronos time toolkit, a JSON read-only CLI, source-grounded conversation timeline API, pure HQ supervisor timing projection, inert TierFlow read models, a VIVIM Labs coverage inventory, and a cross-repo verification runner in SOFTWARE_HQ. The existing scheduler, Host effect writer and approval/receipt paths remain untouched. No new provider APIs, no Playwright/Puppeteer, no external clock authority, no second graph or automation engine.

### Files/changes

- `repos-hq`: Rust `chronos.rs`, `hqkg-time.rs`, Rust tests, `contracts/chronos/contract.v1.json`, HQ supervisor read lens, HQ `chronos-inspect` CLI, Node tests, architecture doc. **Two** exact in-place edits to existing Rust `Cargo.toml` and `src/lib.rs`.
- `vivim-web-harness`: pure `chronos-conversation-projection.mjs`, Node unit/route tests. **Six** exact edits to existing `conversation-runtime.mjs` and `conversation-api.mjs` via `changes/replacements.json`.
- `tierflow`: pure TS `chronos-read-projection.ts` and unit tests; no mutation of existing Host authority.
- `vivim-labs`: cross-repo coverage/blocked-gate inventory.
- `SOFTWARE_HQ`: cross-repository read-only verification and schema parity tests.

## Era 2 — How the local integration writer applies it safely

Unzip beside the **existing** five repository checkouts, and first ensure the local agent has fetched current remote `main`, recorded dirty paths and established an exclusive integration lease for affected files. The script will not fetch, commit, push, rewrite your working tree or launch background agents.

From PowerShell/Node (adjust the paths to where the ZIP was unpacked):

```powershell
node .\chronos_integration_v2\INTEGRATE-ALL.mjs --root C:\0-BlackBoxProject-0 --check
node .\chronos_integration_v2\INTEGRATE-ALL.mjs --root C:\0-BlackBoxProject-0 --apply
node C:\0-BlackBoxProject-0\SOFTWARE_HQ\scripts\chronos-verify.mjs --root C:\0-BlackBoxProject-0 --run
```

`--check` preflights all five checkouts and every exact file/context. It refuses to overwrite an edited target or a different existing file. `--apply` writes only bounded staged changes. It does not modify Git history. A second `--check` recognizes the exact staged files as already present. If a target source has moved, stop and reconcile it manually; never force/replace.

### Direct smoke commands

```powershell
cargo test --manifest-path C:\0-BlackBoxProject-0\repos-hq\research\rust-knowledge-graph\Cargo.toml --test chronos
cargo test --manifest-path C:\0-BlackBoxProject-0\repos-hq\research\rust-knowledge-graph\Cargo.toml --features importer
cargo run --manifest-path C:\0-BlackBoxProject-0\repos-hq\research\rust-knowledge-graph\Cargo.toml --bin hqkg-time -- capabilities
cargo run --manifest-path C:\0-BlackBoxProject-0\repos-hq\research\rust-knowledge-graph\Cargo.toml --bin hqkg-time -- due 0 1000 0 10000 2
node C:\0-BlackBoxProject-0\repos-hq\hq\bin\chronos-inspect.mjs --file C:\0-BlackBoxProject-0\repos-hq\.hq\supervisor\events.jsonl
```

The inspector reads the existing supervisor ledger only; do not paste raw local private logs into a public issue. The new conversation API route requires normal authenticated Host use and must not be exposed as an unauthenticated port.

## Era 3 — Gates and responsibilities

**Executed in this ChatGPT environment:** standalone conversation projection Node tests; isolated TierFlow transpiled Node tests + source-only TypeScript strict check; SOFTWARE_HQ static cross-repo contract tests; offline multi-repository installer fixture check/apply/recheck.

**Unrun here:** Cargo tests (no `cargo` binary), whole Bun/TierFlow workspace, live Host integration, Windows installed binaries, Android physical device. The supervisor replay adapter needs the real existing `hq/supervisor/recovery.mjs` in the local repo for its test (not present in this staging-only copy). The conversation route test similarly needs the complete existing Web Harness source tree, so it is staged but not asserted as passing.

The local single integration writer should review results and either fast-forward main with tests/evidence or report precise blocked failures. A GitHub source commit is not a release GO. For real scheduled reminders, use the existing Host/supervisor to add durable registration, fresh effect-time grant checks, ledger claims, revocation, crash-uncertain reconciliation, IANA calendar logic, native phone alarms, and rollback **in later bounded gates**. Do not let this read-only toolkit execute provider messages or grant permissions.
