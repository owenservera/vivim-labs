# HVI Version Intelligence — Canonical Test Gate

HVI (HQ Version Intelligence) is tested as **synthetic, fail-closed, receipted**
testing, governed by the Proposal 5 scope correction:

- `.integration/proposal5/hq-version-intelligence/10-NATIVE-TOOLING-ONLY-SCOPE.md`
  — native tooling only; no real multi-repository / HQ migration audit in this wave.
- `.integration/proposal5/hq-version-intelligence/05-ACCEPTANCE-TEST-MATRIX.md`
  — HVI executable and negative acceptance matrix (T01–T24).

All HVI tests run on **synthetic invented Git fixtures and injected Git-object
adapters**. Synthetic same-repo SHA1/SHA2/SHA3 and distinct synthetic repositories
cover 2..N comparison, exact relocation, lineage, digest, incomplete scans,
invalid source/secret denial, redaction, authorization/revocation and offline
no-donor replay. Tests are not a real migration analysis.

The gate lives in `scripts/` and is the repository's canonical test runner:
`scripts/test-gate.mjs` over the committed manifest `scripts/test-manifest.json`.

---

## Commands

All commands are invoked from the repo root:

| Command | Behavior |
|---|---|
| `bun run test:gate` | Run the tests selected by impact analysis and write receipts. |
| `bun run test:gate --all` | Run the **full** manifest suite (271 tests), ignoring impact selection. |
| `bun run test:gate --plan` | Print the selection only; **no execution**. |
| `bun run test:gate --no-cache` | Bypass `.hq/test-cache/` entirely; force re-execution. |
| `bun run test:gate --explain` | Print why each test was selected. |
| `bun run test:gate --gate NAME` | Run only the named gate (repeatable). Each named gate forces a full run of that gate, so a receipt is never produced from a partial selection. |

**The canonical HVI gate** — the exact command CI runs and the exact command the
committed receipts were produced by:

```sh
bun scripts/test-gate.mjs --no-cache \
  --gate hvi-g1-g2 --gate hvi-host-read-port --gate hvi-discovery \
  --gate hvi-hq-boundary --gate hvi-tierflow-inspector --gate hvi-modules
```

### Gates

The `hvi-*` gates mirror the canonical CI contract **file-for-file**, so a
per-gate receipt is exactly the evidence the CI step claims.

| Gate | Files | What it covers |
|---|---|---|
| `hvi-g1-g2` | 6 | The G1/G2 synthetic schema, contract-QA, N-way and negative suites. |
| `hvi-host-read-port` | 2 | The Host read port and the development-source grant. |
| `hvi-discovery` | 1 | Native HQ capability discovery and invocation (the MCP tool). |
| `hvi-hq-boundary` | 3 | The existing HQ boundary suite. |
| `hvi-tierflow-inspector` | 1 | The TierFlow inert inspector. |
| `hvi-modules` | 6 | The HVI module suites (compare, identity, parity, report, sources, catalogue-identity). |

The wider repository is covered by `host-suite`, `hq-suite`, `interop`,
`compatibility` and `uncategorized`. The wider suites still carry pre-existing
failures that reproduce identically on `origin/main`, so they are an
operator-level gate rather than a CI gate until those are repaired.

---

## Runtime matrix

The runtime is declared per test in `scripts/test-manifest.json` and derived
from the test's **transitive import closure**, not from the test file alone:

| Runtime | Rule | Count |
|---|---|---|
| `node24-ajv` | the default; nothing in the closure needs bun or TypeScript | 193 |
| `bun` | anything in the closure imports a `bun:` module | 32 |
| `node-strip-types` | anything in the closure is or imports TypeScript | 46 |

The closure is what makes this correct: `packages/tierflow/tests/hvi-inspector.test.mjs`
imports `.mjs` files that in turn import `.ts`, so it needs type stripping even
though its own imports are all `.mjs`.

Node version in use: `v24.22.1` (per `engines.node: >=24`). AJV 8.17.1 is the
schema validator (`tests/hq-version-intelligence/package.json` devDependency);
install it CI-exactly with:

```sh
npm install --prefix tests/hq-version-intelligence --ignore-scripts --no-audit --no-fund --no-save --package-lock=false ajv@8.17.1
```

---

## Fail-closed guarantees

1. **Unknown or uncovered changes → full suite.** If impact selection cannot
   prove a change is covered by an existing test's dependency closure, nothing is
   skipped and the full suite runs. Verified: adding one unimported file
   selected all 271 tests with the reason `fail-closed: uncovered change …`.
2. **No proof for failed or interrupted runs.** A **cache proof** is only written
   when the test exits 0. A non-zero exit, timeout or interruption grants no
   proof and the test must be treated as UNRUN. Verified: a deliberately broken
   test produced exit 1 and a cache file with **zero** entries.
3. **A receipt records what happened, including a failure.** A receipt is always
   written for every gate that ran, and it carries the real totals and exit
   code. A failing gate produces a receipt saying so — it does not vanish.
   The receipt never grants authority.
4. **Cache is never a substitute for execution.** The proof cache lives under the
   gitignored `.hq/test-cache/` directory. A missing, corrupt, unreadable, or
   bypassed (`--no-cache`) cache forces full re-execution; cache hits are an
   optimization only and never replace a recorded exit code.
5. **A named test without an executed command + exit code is UNRUN.** GitHub
   source inspection is never test execution (per `05-ACCEPTANCE-TEST-MATRIX.md`
   truth rule).
6. **Global inputs force the full suite.** `package.json`, the lockfile, the
   runner, the workflow files and `scripts/lib/` can change any test's outcome,
   so a change to them selects everything.

### Impact selection actually narrows

A change to a module a test imports selects that test — not everything. Verified:
touching `hq/version-intelligence/identity/index.mjs` selected 6 of 271 tests,
each naming the changed file in its reason.

---

## Receipts

- **Where they live:** `.integration/proposal5/hq-version-intelligence/proofs/G2/`.
- **Gate receipts:** `<gate>-receipt-<commitSha>.json`, schema `arbor.hvi.gate-receipt/1`.
- **Run receipt:** `run-receipt-<runId>.json`, schema `arbor.hvi.gate-run-receipt/1` —
  carries the full per-test list with each test's exact command, runtime, gate and totals.
- **What they contain:** command, OS (platform, release, architecture), the runtime
  that actually executed the tests (`testRuntimes`), the gate's own runtime
  (`gateRuntime`), Node and Bun versions, AJV version, commit SHA, totals
  (`tests`, `pass`, `fail`), exit code, `startedAt`/`finishedAt` and `durationMs`.

The receipt records the **dispatched binary's** version, not the gate process's
`process.version`. The gate runs under Bun, and Bun reports `process.version` as
the Node version it emulates; recording that would claim a runtime that never
executed a test.

The CLI offline replay receipt uses schema `arbor.proposal5.hvi.g2.receipt/1`
(see `cli-offline-replay-receipt.json`).

---

## Offline no-donor replay

The HVI CLI supports replay without a donor checkout or any network:

```sh
node hq/version-intelligence/integration/cli.mjs \
  --synthetic-stdin < fixture.json
```

- The CLI accepts only `--synthetic-stdin` and requires every source manifest
  pin's `repoIdentity` to start with `synthetic/`.
- It routes through the same HQ code paths (`runHviTool`) as the registered
  capability, with `authorize: async () => true` for the synthetic operator path.
- Output is stamped `provenance: "SYNTHETIC_ONLY"`.

**What it proves:** offline replay of already-pinned manifests works with no
GitHub/API network (matrix T08); synthetic-only capability routing
(`arbor.hvi.compare.pinned/1`); and deterministic pin resolution across the
invented repo `synthetic/offline-replay` at pins v1/v2/v3 (3 source pins).

Receipt: `.integration/proposal5/hq-version-intelligence/proofs/G2/cli-offline-replay-receipt.json` (single successful replay:
tests 1, pass 1, fail 0, exit code 0).

---

## Restrictions (synthetic-only boundary)

- **Synthetic invented Git fixtures only.** No real ARBOR / VIVIM / Labs
  repository comparisons.
- No provider APIs (no GitHub, no donor source acquisition).
- No browser automation.
- No personal corpus.
- No remote privileged access.
- No real source snapshots acquired for "dogfood"; no automatic comparisons
  when idle; no background watches.

The HVI tool never executes repo code, providers, browser automation, or
untrusted package hooks, and attacker-controlled Git content is treated as
untrusted data, not agent instructions (matrix T17–T24).
