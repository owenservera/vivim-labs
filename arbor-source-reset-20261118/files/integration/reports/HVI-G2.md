> **MAIN DRIFT RECONCILED AFTER INITIAL STAGING (2026-11-12):** Concurrent ARBOR main PR #23 advanced main from `3d45fbf1c3963e4834a5e51ee7f6e63f3a0da298` to `9b79d732da8b92713d399040db4a02d505bc7cf1`. Compare identified 11 paths all under `.integration/proposal5/rescore-expansion/**`, none touching HVI. A two-parent, no-force integration-branch merge `08abdd1f71471fa9d87ab89c3c46c449ff9d9b52` preserved new main's exact blobs and staged G2 source. No main write by this worker. See `.integration/proposal5/hq-version-intelligence/proofs/G2/MAIN-DRIFT-RECONCILIATION.md`. This does **not** change the `REPAIR_REQUIRED` gate or make G1 accepted.

# ARBOR Proposal 5 — HVI G2 serial integration (NATIVE TOOLING ONLY)

**2026-11-13 decision:** `REPAIR_REQUIRED`. **Status:** `PREPARED_NOT_PROMOTED` / `HVI_TOOLING_INSTALLED_NOT_VERIFIED` on the G integration branch only. **NOT** `GO_FOR_READINESS_REVIEW`, `HVI_TOOLING_READY`, real-repo audit, runtime security acceptance or beta GO.

The executable-evidence blocker that made every earlier G2 receipt read `UNRUN` is now closed with real receipts from a checked-out Node 24 runner. The gate itself remains `REPAIR_REQUIRED` for the reasons in the blocker table — not because the tests never ran.

## Authority and exact bases
- Repository: `owenservera/ARBOR`; work branch: `integrate/p5-hq-version-intelligence-r001`.
- G1 candidate source parent: `69447c09f420310aa84c24a1908dede91c963ec8`.
- `main` observed: `9b79d732da8b92713d399040db4a02d505bc7cf1` (unchanged since the G2 staging merge; `main` is the merge base of this branch, so the branch is 0 behind and no main drift needed reconciling).
- H final source head `d24daa9966abb222cd25210f41dcbb4758aa1209`: seven files staged in commit `546132d92639af78f70d398c8a5a86d5d6a64738`.
- U final source head `5836e9c9f5ba3bee36dbe6896ebfd22364b4c450`: eight files staged in commit `4bb70bfbadc9b35f13b36232b44044c16900976c`.
- Z2 independent proof head `725132c640a1f21d3af5c4b9123b314e4df3133d`: five proofs staged in commit `9aac22f151e89ea7554ef7ee5cc65b30e5b34e21`. Z2's actual gate is `REPAIR_REQUIRED`, NOT `GO_FOR_G2_TOOLING_REVIEW`.
- H/U forked common W2 owner-waiver candidate `bbe7e4896eea02bcdec6f8fedb66eab885250161`; final G1 changed one documentation/report commit beyond this point. No old pinned source or worker branch reset, no X audit import. X remains DEFERRED.

## G2 reconciliations actually committed
1. Corrected U synthetic test's incorrect expectation of more than four *direct* canonical root children; TierFlow groups sections. `packages/tierflow/tests/hvi-inspector.test.mjs`, commit `535293ae94626a8b78617803a51ff7b8a2a8f950`.
2. Added a new **pure derived projection, not a source store or authority**, `hq/version-intelligence/integration/host-inspector-projection.mjs`. It verifies G1 comparison envelope and digest, bounds rows, redacts secret-looking paths, retains original digest provenance while recomputing a distinct derived digest, rechecks each pinned source grant before and after projection. Initial commit `9713499f86bfb4ce8f7069ccd876deb06ed22d4d`; regex fix `2acc4fb69dcdc21e4403ffc4b909d83aaecaf88f`.
3. Existing H local read port now returns one sanitized, grant-rechecked H→U typed inspector DTO alongside its read-only summary; `apps/vivim-host/control/hvi-capability.mjs` commit `cd6764ab291157dbeca5ac0923b662341ac932a4`. Canonical Host catalog action `versions.compare.pinned` remains **enforcement_missing / null executor**; see the blocker table for why it is not promoted.
4. TierFlow inspector requires an explicit trusted fixture marker in addition to the existing synthetic preview flags; prevents treating arbitrary output as a synthetic fixture. `packages/tierflow/src/visual-runtime/render/hvi-inspector.mjs` commit `c9092d8bab26de270113cc7a2895c71cea2229b9`, corresponding U synthetic test fixture change `0539d23404ef5b1de12ad646d807a2fda57c6ab6`. **Live rendering still disabled.**
5. Added synthetic Host→U tests in `tests/hq-version-intelligence/g2-host-ui.test.mjs`: three immutable SHA pins in one invented repo; five independent invented repos; current-grant/refusal/revoke, metadata redaction, typed DTO, non-publisher and no-effects. Commits `8dcbc05695c7f2262157de684189c7db4bfabc46` and `bc644e2e3314ed80a3f9891695d8ad4798cee6b9`.

## Second G2 pass: defects only an actual run could find (2026-11-13)

The HVI suites had never executed on a real Node 24 runner, so several authored tests carried assumptions no run had ever falsified. Commit `4a26e641445c89256a900768300b88f62c3c6578` repairs them:

1. **`hq/version-intelligence/identity/index.mjs`** — `seen.values()` returns a Map *iterator*, so `.sort()` threw a `TypeError` and every identity crosswalk carrying decisions crashed. Spread it, matching the `[...uniqueEdges.values()]` pattern already used three lines below. This was a real engine defect, not a test defect.
2. **`tests/hq-version-intelligence/g2-host-ui.test.mjs`** — asserted `originalComparison.sourcePins[0]` was the first *requested* revision. The C comparison envelope is canonically ordered by `pinId` (`compare/index.mjs` `cleanPins = ordered(...)`), and that ordering is what keeps `reproducibilityDigest` stable across differently-ordered requests. The assertion is now order-independent and additionally pins the canonical ordering, so it is **stronger** than before.
3. **`apps/vivim-host/test/hvi-capability.test.mjs`** — each negative case replaced `sources` with fewer than three entries while `req()` kept `targetIndex: 2`, so the targetIndex range check fired first and masked the code under test; the first case was passing for the wrong reason. `targetIndex` is now explicit per case, and the secret-subpath, unapproved-repo and moved-SHA cases genuinely assert `SCOPE_ESCAPE`, `SOURCE_UNAUTHORIZED` and `PIN_MOVED`.
4. **`packages/tierflow/tests/hvi-inspector.test.mjs`** — counted every node naming the repo, but the target-relative coverage rows legitimately repeat the repo label. It now counts the pin **selectors** (`SOURCE`/`TARGET` prefixed), which is what the test name asserts, and separately asserts the coverage-row count.
5. **`hq/lib/schema.mjs`** — a failed `openDb` leaked the sqlite handle: the caller never receives the `db` object, so nothing could close it, and the file stayed `EBUSY`-locked on Windows, breaking disposable-index cleanup. The handle is now closed on the failure path. **Pre-existing on `origin/main`** (proved by a baseline run of the same suite at `9b79d73`), and required for the HQ boundary suite to pass.
6. **`hq/test/arbor-service.test.mjs`** — omitted the now-mandatory `mode`, so the case failed at construction instead of exercising the source-root denial. Also pre-existing on `origin/main`.
7. **`.gitignore`** added — the repository had none, and `npm install --prefix tests/hq-version-intelligence` puts `node_modules/` inside a tracked package directory, which `git add -A` would otherwise have committed.

## Second G2 pass: native registration, grant and revision (2026-11-13)

Commit `a256611b9375bbed1c657393f2ff034e278b8bed`:

1. **Native HQ capability discovery and invocation** — `hq/lib/mcp.mjs` registers `hvi_compare_pinned` in the existing MCP tool list, so the capability is discoverable through `tools/list` and invocable through `tools/call`. `tools/call` is awaited (`mcp.mjs:322`), which is what makes the async HVI engine reachable at all: the Host control plane is synchronous by construction (`dispatcher.mjs` contains no `async`/`await`/`Promise`), so an async executor there would store a Promise as the result, serialise to `{}`, escape the receipt `try/catch` as an unhandled rejection and turn the `uncertain` outcome into a false `applied` after the effect fence was already claimed. The tool is synthetic-only in this wave: every source repository identity must start with `synthetic/`, exactly like the operator CLI gate, and the result carries `provenance: 'SYNTHETIC_ONLY'` and `runtimeAccepted: false`.
2. **Revocable, expiring development-source authorization** — `apps/vivim-host/control/hvi-source-grant.mjs`. The grant names **which** repositories the owner authorised and **for how long**, and can be revoked. It reuses the existing bootstrap ledger `meta` store and the existing bounded-grant vocabulary (`readGrant`/`writeGrant`/`grantExpiryMs`/`GRANT_MAX_EXPIRY_MS` from `bootstrap/service.mjs`): no new store, no new authority, no new endpoint, no scheduler. Evaluation order is fixed and matches the house order — revoked → not granted → expired → repository scope — and absence, expiry and revocation all fail closed. An absent or unreadable expiry is treated as expired, because this grant type has no legacy unbounded form. Revocation writes a tombstone under its own key, so `HVI_SOURCE_GRANT_REVOKED` stays distinguishable from `HVI_SOURCE_GRANT_REQUIRED`. The grant authorises **repositories, not identities**: who the caller is stays the Host owner's business, and a valid owner session alone never grants repository access.
3. **UI revision verification** — `apps/vivim-host/control/hvi-capability.mjs` now **observes** the current UI revision from the canonical owner (`runtime.uiRevisionStatus()`) instead of taking it from the caller. The request schema is closed and carries no revision field, so no caller can claim one; the observed revision travels with the result as `uiRevision`. A failing observer fails closed. The registration guard now reads readiness from the catalog instead of hard-coding `enforcement_missing`, and still refuses a `verified` row (which asserts live Windows Host and paired-phone receipts this branch does not hold).
4. **Docs** — `hq/version-intelligence/integration/AGENT-USAGE.md` corrects eight stale or missing claims, including that `runHviTool` is async and every failure is a promise rejection. `apps/vivim-host/control/hvi-native-guide.json` now states the three registrations and what remains pending.

### What is deliberately NOT wired, and why
The canonical Host action `versions.compare.pinned` stays `readiness: enforcement_missing` with a **null executor**. The Host control plane is synchronously fenced by design and the HVI engine is async; promoting the row to `source_wired` would make the catalog claim an executor that cannot run the analysis, which the catalog's own glossary defines as "Host dispatcher executor exists **and offline tests pass**". The exact remediation options, and the files each touches, are in the blocker table. The native HQ registration above is what makes the capability discoverable and invocable by an agent today, without inventing a second executor.

## Canonical test gate (2026-11-13)

ARBOR previously had **no canonical testing system** at all. The intelligent impact-aware runner in the original workspace (`repos_hq scripts/test-smart.mjs` + `scripts/lib/test-selection.mjs`) is Bun-only and discovers only `hq/test`, so it cannot run ARBOR's Node 24 + AJV HVI gate and cannot see most of this repository's tests. Commits `d12aa2c`, `77fc1bc`, `962ba96`, `997eff0` add the ARBOR-native equivalent under `scripts/`, to the same invariants:

- **Unknown or uncovered changes fail closed to the full suite.**
- **A cached proof is never granted for a failed or interrupted run.**
- **Local only** — the proof cache lives under the gitignored `.hq/test-cache/`.
- **No network, no provider API, no browser, no personal corpus.**

What it adds over the original runner:
- **Multi-root discovery: 271 tests** across `hq/test`, `apps/vivim-host/test`, `apps/mobile-relay/test`, `tests/hq-version-intelligence`, `packages/tierflow/tests`, `tests/compatibility`, `tests/privacy`, `harness/test` and the nested suites. The original runner saw 51.
- **A committed manifest** (`scripts/test-manifest.json`) declares each test's runtime and gate, so selection and receipts are driven by data rather than by filename convention.
- **A real per-test runtime matrix**, derived from the transitive import closure: `bun` when anything in the closure imports a `bun:` module, `node-strip-types` when anything in the closure is or imports TypeScript, `node24-ajv` otherwise — 193 / 32 / 46 respectively. The TierFlow inspector is `node-strip-types` because the `.mjs` it imports pulls in `.ts`; a first-pass detector that looked only at the test file's own imports got that wrong.
- **A real transitive static import-graph dependency closure**, so impact selection actually narrows instead of always failing closed to everything.
- **Per-gate and per-run machine-readable receipts** carrying the command, OS, tool versions, commit SHA, test totals, exit code and duration.
- **Gates that mirror the canonical CI contract file-for-file**: `hvi-g1-g2` (6), `hvi-host-read-port` (2), `hvi-discovery` (1), `hvi-hq-boundary` (3), `hvi-tierflow-inspector` (1), `hvi-modules` (6) — 19 files that are exactly the HVI surface. Broader suites are `host-suite`, `hq-suite`, `interop`, `compatibility`, `uncategorized`.

### Observed gate behaviour (all three invariants verified by execution)

| Property | How it was proved | Observed |
|---|---|---|
| Impact selection narrows | touched `hq/version-intelligence/identity/index.mjs`, ran `--plan` | 6 of 271 selected, each naming the changed file |
| Unknown change fails closed | added `hq/lib/__probe_uncovered__.mjs`, ran `--plan` | all 271 selected, reason `fail-closed: uncovered change …` |
| No proof for a failure | broke `g1-v2-schema.test.mjs`, ran the gate | exit 1, `70 pass, 1 fail`, cache written with **0 entries** |

### Canonical gate command

```
bun scripts/test-gate.mjs --no-cache \
  --gate hvi-g1-g2 --gate hvi-host-read-port --gate hvi-discovery \
  --gate hvi-hq-boundary --gate hvi-tierflow-inspector --gate hvi-modules
```

## Observed test truth — executed receipts

All suites below ran on the integration branch in a checked-out worktree on Windows 10.0.29704 x64 (MINGW64_NT-10.0-29704 3.6.5-22c95533.x86_64), Node **v24.22.1** (the dispatched binary), bun 1.4.2 (the gate runtime), AJV 8.17.1 (installed CI-exact). All are synthetic-only: invented Git-object fixtures, no real repository, no network, no provider, no browser, no personal data, no donor comparison.

The canonical gate run against commit `997eff0844b2106b2c91efb10c651f2c2567e6a5`: **19 test files, 201 tests, 201 pass, 0 fail, exit 0.**

| Gate | Tests | Passed | Failed | Exit | Receipt (schema `arbor.hvi.gate-receipt/1`) |
|---|---|---|---|---|---|
| `hvi-g1-g2` | 73 | 73 | 0 | 0 | `hvi-g1-g2-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` |
| `hvi-host-read-port` | 17 | 17 | 0 | 0 | `hvi-host-read-port-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` |
| `hvi-hq-boundary` | 9 | 9 | 0 | 0 | `hvi-hq-boundary-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` |
| `hvi-discovery` | 4 | 4 | 0 | 0 | `hvi-discovery-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` |
| `hvi-tierflow-inspector` | 17 | 17 | 0 | 0 | `hvi-tierflow-inspector-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` |
| `hvi-modules` | 81 | 81 | 0 | 0 | `hvi-modules-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` |
| CLI offline no-donor replay | 1 execution | 1 | 0 | 0 | `cli-offline-replay-receipt.json` (schema `arbor.proposal5.hvi.g2.receipt/1`) |

All receipt files are under `.integration/proposal5/hq-version-intelligence/proofs/G2/`. The per-run receipt `run-receipt-20261112T234955Z-997eff0844b2.json` carries the full per-test list with each test's exact command, runtime, gate and totals.

The CLI offline no-donor replay was `node hq/version-intelligence/integration/cli.mjs --synthetic-stdin < synthetic-request.json` over the invented repo `synthetic/offline-replay` at pins v1/v2/v3: exit 0, `provenance: SYNTHETIC_ONLY`, 3 source pins, 2 coverage entries, 11 source deltas, `runtimeAccepted: false`, `hostBound: false`, `grantProvenance: CALLER_SUPPLIED_NOT_HOST_ATTESTED`.

**Honest framing:** every passing result above proves ONLY synthetic-source conformance. None of it is G1/G2 runtime acceptance, a live donor comparison, Host security acceptance, canonical dispatch acceptance, or executable evidence of main promotion. G1/G2 still require independent review and live canonical Host grant/publisher gates. No real donor comparison or runtime acceptance is claimed.

### Wider-suite regression check (not a gate)
- Full Host suite: `node --test apps/vivim-host/test/*.test.mjs` → 454 tests, 447 pass, **7 fail**. The same 7 failures reproduce identically on `origin/main` (`9b79d73`) and are in files this branch never touched: vendored Omega NCLL provenance, versioned ingress registry, asset/relative-utility JSON Schemas, bounded inventory, private inventory route, phone QR/Direct transport, and `kpi-utc.test.mjs` (which imports a `tools/kpi/` module that does not exist in this repository). **No new failure was introduced by this branch.**
- Full HQ suite under Bun: 426 pass, 19 fail, 7 errors — against a `origin/main` baseline of 380 pass, 19 fail, 7 errors. Identical failure counts; the difference is the 46 additional tests this branch adds under `hq/`, all passing. The Node-only `bun:`-scheme failures disappear under Bun by design; the remaining failures are pre-existing.

## P0/P1 blockers — exact owners

| Blocker | Status | Kind | Owner | Notes |
|---|---|---|---|---|
| `G1_NOT_ACCEPTED` | unchanged | process | G1/CI | No accepted G1 `main` commit (accepted G1 main SHA: null). Outside this branch's scope. The Node24+AJV receipts this gate needed now exist under `proofs/G2/`; merging via the normal gate remains a G1/CI action. Owner development waiver grants remote source repair only, not main promotion. |
| `REMOTE_CI_STARTUP_FAILURE` | confirmed | infrastructure | CI | CONFIRMED account-level, not repo-config: every workflow on this repo (including unrelated branches such as `docs/p5-vlang-program-sitrep-r001`) returns `startup_failure` with 0 jobs at ~0s. Repo Actions permissions are `enabled: true, allowed_actions: all`. **NOT fixable from this branch.** |
| `LIVE_SOURCE_GRANT_MISSING` | implemented_pending_acceptance | process | Host security | The grant IS implemented (`apps/vivim-host/control/hvi-source-grant.mjs`) and verified on synthetic fixtures (issue → authorize; wrong-repo scope refusal; revocation with tombstone; expiry at the exact boundary; absent/unreadable expiry failing closed; and the Host read port denying an ungranted or revoked repository before any Git object read). It awaits Host security acceptance; there is no live grant service and no production invocation. |
| `HOST_NATIVE_DISPATCH_DISABLED` | documented | architecture | H/G2 | The Host control plane is synchronous by construction (`apps/vivim-host/control/dispatcher.mjs` contains no `async`/`await`/`Promise`; the two executor call sites at `:182` and `:279` are bare calls) and the HVI engine is async, so a direct executor would store a Promise as the result, serialise to `{}`, escape the receipt `try/catch` as an unhandled rejection, and turn `uncertain` into a false `applied` after the effect fence was claimed. Two remediations, both Host-owner changes: (a) make the control plane async-aware — `dispatcher.mjs` `runRead`/`plan`/`execute`, `server.mjs:660-662`, `apps/vivim-host/direct-phone.mjs:218-260`, and `apps/vivim-host/test/control-dispatcher.test.mjs` (which contains no `await` today) — or (b) extend the frozen `BOOTSTRAP_ACTIONS` allowlist (`apps/vivim-host/control/bootstrap-actions.mjs`), which is shared with `apps/mobile-relay/lib/relay-core.mjs`, `apps/mobile-relay/consumer-commands/registry.mjs` and two allowlist tests asserting `size === 6`. Until one is accepted the catalog row stays `enforcement_missing` with a null executor, and the capability is discoverable and invocable through the HQ MCP tool instead. |
| `EXECUTABLE_EVIDENCE_UNRUN` | resolved | process | G1/H/U/G2 QA | RESOLVED with real receipts (table above): 19 files, 201 tests, 201 pass, 0 fail, exit 0, plus the CLI offline replay at exit 0. All synthetic-only. |
| `PUBLISHER_REVISION_UNATTESTED` | partially_addressed | process | U/Host publisher | The Host side now observes the current revision from the canonical owner instead of accepting a caller-controlled one, and the TierFlow renderer already refuses a real envelope (`syntheticPreviewAllowed` is deliberately `false`, locked by `tests/hq-version-intelligence/g2-host-ui.test.mjs:39`, "live result cannot masquerade as a synthetic preview"). The TierFlow publisher's own live revision observation and CAS guard are out of scope; the live feature stays OFF. |
| `BETA_GATES_BLOCKED` | blocked | process | beta release owners | `integration/beta-gates.json` remains `BLOCKED_NOT_BETA`; out of scope; zero inference from HVI staging. |

## Security, privacy and rollback
Read-only source analysis only. No real ARBOR/VIVIM/Labs/donor comparison; no personal World data; no provider API, Playwright/Puppeteer, Chrome sender, effect writer, source grant database, scheduler, graph store or competing TierFlow publisher was created. The development-source grant reuses the existing bootstrap ledger `meta` table and the existing bounded-grant vocabulary; it adds no second authority. No privileged remote source reads enabled. All changes are on the existing integration branch. No modification or merge to `main`, no rewriting immutable S0/D counts or source pins, no changed beta gate. Rollback is a reviewed candidate-branch revert; **rollback drill UNRUN**.

## Required next owner gate
Accept G1 `main` with the Node24+AJV receipts now present under `proofs/G2/`; re-review G2 drift; have the Host security owner accept the development-source grant and choose remediation (a) or (b) for the synchronous control plane; attest the current UiCore revision on the TierFlow publisher side; obtain a new independent Z2 consent/review. **Only after the verified combined gate** may the integrator submit/merge a reviewed PR to main and issue `GO_FOR_READINESS_REVIEW`. Current final gate is `REPAIR_REQUIRED`; do not bypass gates based on owner source-development approval.

Machine-readable receipts: `.integration/proposal5/hq-version-intelligence/proofs/G2/hvi-*-receipt-997eff0844b2106b2c91efb10c651f2c2567e6a5.json` (schema `arbor.hvi.gate-receipt/1`), `run-receipt-20261112T234955Z-997eff0844b2.json` (schema `arbor.hvi.gate-run-receipt/1`) and `cli-offline-replay-receipt.json` (schema `arbor.proposal5.hvi.g2.receipt/1`). Historical ledger: `.integration/proposal5/hq-version-intelligence/proofs/G2/G2-RECEIPT.json` (immutable, not edited).

## G2 follow-through: native HQ source-read security repairs (2026-11-12)

**Actual code commits** on the existing integration branch after original G2 staging:

1. `77bec293a7f0144d1c0bae01a1102a26bf1a7e9f` — align the low-level immutable Git source helper's sensitive-path exclusions with C bridge policy, including `.ssh`, `.aws`, private paths and key/certificate file extensions; `readPinnedBlob` now reuses the broadened helper.
2. `eec8a16f00e1e26459c6ad6a7e4c4c9851300d23` — directly supplied N-way comparator source manifests use the same sensitive-path guard, not only the A→C bridge.
3. `06858bf389338695969cff12e55ea1f1138bb20e` and `33b6059615aab0ec5571a3acf5674725b67fcd9f` — authored new synthetic regression cases for direct credential reads, projection redaction, disallowed repos, cancellation, and a malicious C manifest with a recomputed digest. **These now execute** as part of the `hvi-g1-g2` and `hvi-host-read-port` gates.

**Observed verification**: the 14/14 V8 assertions of the fetched native sensitive-path source slice and V8 syntax parse of the extended test file, **plus** the integrated Node 24 + AJV 8.17.1 run of the whole gate: 19 files, 201 tests, 201 pass, 0 fail, exit 0. GitHub Actions latest [43319733234](https://github.com/owenservera/ARBOR/actions/runs/43319733234) on source SHA `33b6059615aab0ec5571a3acf5674725b67fcd9f`: `startup_failure`, jobs=0. A failed-job retry returned 403 `This workflow run cannot be retried`. Opened [tracking issue #24](https://github.com/owenservera/ARBOR/issues/24); [PR #21](https://github.com/owenservera/ARBOR/pull/21) remains DRAFT, no reviews, not merged.

This advances source hardening but **does not change** `G1_NOT_ACCEPTED`, `Z2_REPAIR_REQUIRED`, `REPAIR_REQUIRED`, `HVI_TOOLING_INSTALLED_NOT_VERIFIED`, the disabled production Host action, the pending UiCore authoritative-revision requirement, or `BLOCKED_NOT_BETA`. A final R certificate is impermissible before real main and security gates. No real user-repository/donor comparisons were performed.

**Dedicated test automation:** `.github/workflows/hvi-native-synthetic.yml` now runs the explicit Node24 commands **and** a `test-gate` job that runs the six HVI gates with `--no-cache` and uploads the receipts as an artifact. The workflow remains infrastructure preparation: GitHub Actions still returns `startup_failure` with zero jobs for every workflow on this repository, so **the local run is the executed evidence**, not the CI. Acceptance stays `REPAIR_REQUIRED`.
