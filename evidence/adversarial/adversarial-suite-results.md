# Labs evidence — source-pinned adversarial + integration suite results (TierFlow)

**Record:** 2026-11-06 · **Factory:** 04 Adversarial Security Factory / 05 Integration Compatibility Lab
**Status:** MEASURED LOCALLY. Not device, Host, browser or production evidence.

## Source pin

| Item | Value |
|---|---|
| Repository | `owenservera/tierflow` |
| Source SHA | `ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7` (main) |
| Checkouts exercised | `C:\0-BlackBoxProject-0\_scratch_l3\clean-build-repro\tierflow-clean` (isolated, clean) **and** `C:\0-BlackBoxProject-0\vivim-ui` (working tree) — both at the same SHA, both green |
| Toolchain | bun 1.4.2 · TypeScript 7.0.2 · cargo 1.99.1 |

## Commands, exit codes, results

| Command | Exit | Result |
|---|---|---|
| `bun run verify` | 0 | typecheck → core:test → **449 pass / 0 fail**, 1120 expect(), 35 files |
| `bun test tests/` | 0 | 449 pass / 0 fail — **18 of those are `adversarial:` named negatives** |
| `bun test tests/visual-runtime/integration-2/real-a-h.test.ts` | 0 | 25 pass / 0 fail — **18 adversarial + 3 import-provenance + 4 chain/seam** |
| `bun test tests/visual-runtime/integration-2/compose.test.ts` | 0 | 15 pass / 0 fail |
| `bun test tests/ui-commands.test.ts tests/ui-store.test.ts` | 0 | **52 pass / 0 fail**, 250 expect() — Lane C `bun:sqlite` crash/replay safety |
| `bun test tests/visual-runtime/` | 0 | 204 pass / 0 fail, 18 files |
| `node --test tests/visual-runtime/integration-2/real-a-h.test.ts` | 0 | 25/25 |

## The 18 adversarial negatives — all refusing, none claiming success

Grouped by the guarantee each one pins. Every case asserts a **refusal or an uncertainty**, not a
success path.

**Lane A — document/action integrity (4)**
- a stale action version is refused
- an unknown action reference is refused
- malformed HTML and unsafe refs are refused by Lane A *and* Lane D

**Lane C — revision/CAS + SQLite durability (4)**
- a wrong principal cannot read or publish
- a revision CAS conflict and a digest mismatch are both refused
- reusing a command id with a different envelope is refused
- a crash before commit is uncertain and commits nothing
- a lost ACK after commit is uncertain but the outcome is queryable, never resent

**Lane F — knowledge provenance (3)**
- fake provenance is refused, not projected
- redacted records stay hidden unless trusted policy allows markers
- a tombstoned record is not resurrected and a revoked session denies

**Lane H — Host authority (5)**
- R2 on a phone surface is refused and R3 is always refused
- an uncertain receipt is read-only reconcile and never a duplicate claim
- a receipt with a forged signature state is denied as invalid evidence
- a Host command with forged fields is refused
- a wrong principal cannot prepare a Host effect

**Lane G / Lane E — semantics and recovery (3)**
- Lane G refuses an unknown semantic id instead of guessing
- Lane E never offers automatic retry for a denied Host effect

## What this does and does not prove

**Proves:** at source SHA `ec832f8`, on this machine, the real A–H modules refuse the 18 adversarial
inputs above; the SQLite revision store survives crash/ACK-loss/CAS-conflict without duplicate
claims or a second revision; and the whole suite reproduces identically from a clean isolated
checkout (see `repro/clean-build-reproducibility.md`).

**Does not prove:**
- **No live Host, no signed-in browser, no physical Android, no owner-scoped live KB, no native
  desktop.** Every case above is a fixture-level negative against real modules — the maturity is
  `integrated` (real neighbouring modules interoperate), **not** `environment verified` for any
  device and **not** `production verified`.
- **No provider browser or device was contacted.** No network calls were made by these tests.
- The 18 negatives are a **floor**, not a coverage claim. The first-campaign target is 2,000 adverse
  cases; **volume never proves correctness**, and 18 curated negatives are not equivalent to that
  target.
- Out-of-repo blockers named by the closure remain untested here and unowned by Labs: Harness
  Studio's 24 failing unit cases, the Android `PocketOwnerActions.java:69` `IOException`, and the
  `research/comment-program-rust/src/lib.rs:83` Rust literal (that `research/` tree is not present
  in this TierFlow checkout at all).

## Reproduce

```sh
cd C:/0-BlackBoxProject-0/vivim-ui
bun install && bun run core:build && bun run verify
bun test tests/visual-runtime/integration-2/real-a-h.test.ts
bun test tests/ui-commands.test.ts tests/ui-store.test.ts
```
