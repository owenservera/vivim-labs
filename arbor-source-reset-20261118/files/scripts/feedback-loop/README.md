# ARBOR A03 feedback compiler — read-only preparation

Canonical authority: `docs/product/roadmap/roadmap.v1.json`; owner: keeper. This is a **source-only proposal compiler**, not a roadmap, queue, publisher, worker, or Host effect runtime. A03 cannot activate until A00 has independently executed verification and owner acceptance.

Run on an **isolated checkout** whose current HEAD has been pinned independently by the keeper:

```sh
node scripts/feedback-loop/compile.mjs --receipt .coordination/pdr/evidence/WIKI-BASELINE.verify.oracle-executed.json --source-sha 2a42f1af025db8aacb965326b11c73da671ffb3f --current-sha 2a42f1af025db8aacb965326b11c73da671ffb3f --revision R002 --current-revision R002 --condition wiki-page-renders
node scripts/feedback-loop/compile.mjs --receipt scripts/feedback-loop/fixtures/wiki-baseline.failed.json --source-sha 2a42f1af025db8aacb965326b11c73da671ffb3f --current-sha 2a42f1af025db8aacb965326b11c73da671ffb3f --revision R002 --current-revision R002 --condition wiki-page-renders --summary
node --test scripts/feedback-loop/*.test.mjs
```

The sample SHA is the **initial baseline**, not the current SHA of this new lane branch. On the branch, obtain and pass a new independently observed current HEAD in both SHA fields (or run against the pinned baseline checkout), rather than asserting a stale HEAD. The fixture copies the *actual historical* `WIKI-BASELINE.verify.oracle-executed.json` failure checks/digest (8/10), not a synthetic passing receipt. `--current-sha` is a caller assertion, **not independently verified** by this CLI. For a new failure, supply independently recorded evidence with actual outcomes and do not treat fixture replay as live execution.

The compiler deterministically maps two known failed Wiki verifier check names to the existing A00 item, **only when** the canonical roadmap `sourceEvidence`, `writeScope`, lane and acceptance clauses agree. Each failure has a stable `sourcePath#condition` identity; repeated reports dedupe but distinct evidence references survive. Each run chooses at most one candidate; other distinct failure conditions are reported for later review. Demo `revise` verdicts have no trustworthy code owner or source path in the posted schema; they produce `REVIEW_REQUIRED`, not a fabricated lane assignment. Unrecognized verifier checks do likewise. Malformed, oversized, instruction-like, out-of-scope, stale-pin and stale-revision inputs fail closed. Successful/approved verdicts and Host journal events never become work orders.

The output includes a machine-readable suggestion plus `--summary` text view; it does **not** write any file. Reproduction commands are output-only and marked `UNRUN`; the Wiki `--live` command is intentionally limited to a separate isolated Host. No subprocesses, Git writes, host endpoints, agent launch, network calls, roadmap writes, or owner acceptance transitions exist in this code.

## Keeper handoff (after A00 accepted)

1. The independent keeper verifies A00's executed receipts, owner acceptance, source HEAD and roadmap revision **outside this compiler**. Until then A03 is BLOCKED even if unit tests pass.
2. Keeper reads the suggested `candidate` and `reviewRequired` entries, independently rechecks `sourcePin`, `narrowPaths`, `ownerLane`, roadmap ownership/acceptance, and every original `evidenceRef`; asks bounded questions for unknown ownership.
3. Keeper chooses whether to convert the suggestion into an approved existing-roadmap work order through the **existing** sole-writer CAS protocol. No direct inclusion/queue mutation is performed here. Real fixes need Host authority where applicable, independent executed tests, and fresh rewalk receipts.
4. Measure feedback-to-verified-repair time on a subsequent work order. Until measured, use `NOT_YET_MEASURED`.
