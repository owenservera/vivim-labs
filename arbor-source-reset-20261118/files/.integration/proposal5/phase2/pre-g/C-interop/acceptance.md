# P2-C acceptance — post-G execution matrix and falsification

Status: **PREPARED_NOT_PROMOTED**. No beta or product runtime acceptance. Scope is *one minimal actor/source/revision/event/read projection*, not a second native I/O fabric, graph, command processor, publisher or scheduler.

## Evidence separated by execution environment

| Check | Observed in this lane | Meaning |
| --- | --- | --- |
| GitHub exact ref/source/blob inspection | OBSERVED | Source paths and pinned Git objects exist on the specific refs; A/D/E/F are separate branches |
| Synthetic JS function assertions (same function bodies committed in `interop-read.mjs`) | **22/22 PASSED in connector V8** | Pure guard invariants only, no Node or runtime proof |
| `node --test .integration/proposal5/phase2/pre-g/C-interop/interop-read.test.mjs` | **UNRUN** | Eight authored synthetic Node suites; no CLI exit code |
| `node --check .integration/proposal5/phase2/pre-g/C-interop/interop-read.mjs` | **UNRUN** | Parse/syntax check pending |
| `tsc --noEmit` (exact G/F project config) | **UNRUN** | TypeScript structural interfaces not compiled |
| `rustc --test contracts/interop/v1/types.rs` | **UNRUN** | F Rust mirror still test-only |
| `cargo test --manifest-path native/hqkg/Cargo.toml` | **UNRUN** | Existing A native regression, Chronos not yet linked |
| Labs Chronos Rust/JS/TS test modules | **UNRUN this lane** | Existing test declarations/reports are not this lane's observations |
| Combined ARBOR A–F build, Windows/Host owner launch, WebView2, PWA and physical Android | **UNRUN** | G/W2 owners must run and report separately |

## Smallest useful real executable tests after G

1. **F owner — pure differential guard:** from the integrated ARBOR root, run `node --test .integration/proposal5/phase2/pre-g/C-interop/interop-read.test.mjs tests/compatibility/wave1f.test.mjs`. Compare the staged read gate with F's `contracts/interop/v1/validate.mjs`. If F's existing validator already covers all relevant cases, promote only negative fixtures, not another validator. Expected: 8 authored suites plus existing F tests pass. Actual: **UNRUN**.
2. **A/F — native structural mirror:** `rustc --test contracts/interop/v1/types.rs -o /tmp/arbor-f-types-test && /tmp/arbor-f-types-test` on a compatible POSIX machine, or a Windows MSVC equivalent; `cargo test --manifest-path native/hqkg/Cargo.toml` on G's combined root. Inspect `cargo metadata` and A's selected features first. Do not add Chronos runtime until the pure `chronos.rs` test module is mapped to A's existing `EngineRegistry` and demonstrated to compile. Actual: **UNRUN**.
3. **D/C/E — adapter read models:** after owner-approved selective import, run `node --test hq/test/chronos-supervisor.test.mjs` **only if** D has supplied a compatible `hq/supervisor/recovery.mjs` or an accepted existing replay port; run `node --test studio/test/unit/chronos-conversation-projection.test.mjs` only if C has physically placed the named Labs test with compatible import paths; run E's TierFlow test command under its package scripts for `chronos-read-projection.test.ts`. Exact package runner/TS paths must be resolved by those owners, not guessed as already installed. Actual: **UNRUN**.
4. **B/D/F — trusted owner integration:** mount the real D `createDevIntelligence({mode:'host',actorId,sessionId,approvedRoots,authorize,dataDir})` against B's actual action/grant/journal owner; check `sourceEvidence` and `indexWatermarks` under grant, denial, revoke and stale/dirty Git. Never send Personal World into D. Missing owner binding must reject. Actual: **UNRUN**.
5. **B/A/E/F — integration adversarial tests:** synthesize fake Host loopback listener, wrong child process/nonce/owner, Android active session replay, stale TierFlow World revision, corrupt source digest, lost browser key, uncertain effect with no receipt and partial disk/full. Execute only in nonproduction test fixtures and on supported Windows/phone staging. No private user material required. Actual: **UNRUN**.

## Mandatory negative matrix

| ID | Synthetic input / failure | Required observable outcome | Owner |
| --- | --- | --- | --- |
| NEG-C01 | Client-forged actor/session or unbound owner | Refuse, no projection to wrong audience, no Host credentials | B/F |
| NEG-C02 | Personal source without owner, or development index used as Personal World | Refuse cross-namespace join and raw evidence display | A/D/F |
| NEG-C03 | Same event seq, replay, out-of-order or source revision mismatch | Refuse/demand authorized snapshot; no effect or successful UI state | A/F/E |
| NEG-C04 | Input requests `approve`, `execute`, token data, new catalog ID via an inert projection | Refuse; UI cannot create grants, receipts or action definitions | B/E/F |
| NEG-C05 | Chronos due/retry calculation and synthetic `applied` label without Host receipt | Remain estimated read; unknown/uncertain effects are never auto-resubmitted | A/B/F |
| NEG-C06 | Future wall clock/boot change, elapsed regression or DST boundary | Degraded/unknown or tested canonical time answer; never a verified run claim | A/F |
| NEG-C07 | Supervisor projection import with missing replay dependency | Import/build fails closed; D supplies existing canonical replay contract, no new journal | D/G |
| NEG-C08 | R005 quoted negative text or lexical project name interpreted as active project or permission | Read-only lexical hint; no command send/Host execution | E/F |
| NEG-C09 | UI World digest/source revision stale, unsupported object, personal source revoked | E bridge rejects/no publisher side effects | E/F |
| NEG-C10 | R001 per-keystroke thread fanout or Rust mutex stalls or exceeds bounded budgets | Native A design must use bounded scheduling/coalescing, no uncontrolled thread growth | A/F |
| NEG-C11 | Reactivated device with lost key or stolen active grant+session headers | Host B policy rejects without fresh request binding; physical E compatibility is tested | B/E/F |
| NEG-C12 | Disk-full during journal/graph/World persistence | No false success receipt, no destructive retries, recover by owner-approved reconciliation | A/B/F |

## Exit criteria and dependency order

- **First G:** merge A–F only and pin their final tips, dependency graphs and root workspace; verify no donor checkout/runtime dependencies. Do not gate G on P2-C.
- **Then F:** choose reusable *tests/types only* and reconcile F's source vocabulary. Duplicate implementation is a failing acceptance outcome.
- **Then A/B/D:** verified actor/source/revision provenance and S1 secure Host process identity are prerequisites for any real native interaction. No native runtime mutation in P2-C.
- **Then C/E:** read projections may render only after SourceRef and current revision match; Chronos wall estimates cannot advertise execution, permission or receipts.
- **Finally W2/F:** report actual executed command, environment, exit code, failures, hashes and owner signoff. Only then change `UNRUN` or `PREPARED_NOT_PROMOTED`.

Tests are designed to fail safe. A successful static source audit or V8 assertion does **not** upgrade SOURCE_PRESENT to HOST_WIRED or DEVICE_ACCEPTED.
