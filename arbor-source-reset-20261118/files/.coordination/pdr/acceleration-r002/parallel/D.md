# ARBOR Acceleration R002 — Parallel W1 Lane D

**Assignment:** Independent A00 verifier, evidence integrity and release-truth refusal
**Branch:** `work/accel-r002-w1-d-evidence-truth`
**Maps to:** A00 independent proof / false-green elimination; no authority to flip canonical queue
**Priority:** Immediate independent acceptance lane
**Parent contract:** [parallel/README.md](README.md) — read it first. Do not touch `main`, do not merge, and do not change the canonical roadmap.

## Exclusive write scope

- `scripts/batch-evidence/WIKI-BASELINE.mjs`
- `scripts/batch-evidence/wiki-evidence-regression.test.mjs`
- `scripts/roadmap/evidence-truth.mjs`
- `scripts/roadmap/evidence-truth.test.mjs`
- `docs/acceleration/w1/D.md`

Only these paths are writable. All other repository content is read-only even if a nearby fix seems convenient. Produce your final report at `docs/acceleration/w1/D.md` within the branch; source path ownership is exclusive.

## Mandatory additional source inspection

- `docs/product/roadmap/roadmap.v1.json (read-only: keeper sole writer)`
- `scripts/acceleration/live-host-check.mjs (read-only: Lane B owns)`
- `apps/vivim-host/server.mjs (read-only: Lane A owns)`
- `integration/beta-gates.json and .integration/proposal5/08-BETA-GATES.json and integration/release-manifest.json (read-only)`

Read actual current contents and verify branch base; do not substitute the work-order prose for source inspection.

## Execution steps

1. Inspect existing Wiki verification code against actual external JS/CSS, JSON-array /api/journal, append/replay rules and explicit isolated editing contract. Correct false-positive/false-negative assertions; never test browser DOM by looking for asynchronous content in response HTML.
2. Keep source/API checks distinct from independent browser rendering. Add deterministic negative regressions: forged origin/session, missing revision, stale edit, corrupt journal, missing assets, empty journal, source seed unchanged and route returning an error.
3. Implement evidence-truth reports that enumerate conflicting beta/release claims and actual Node/CI/device evidence without promoting release when claims are internally consistent; never overwrite original claims or receipts.
4. Run available scoped Node tests and capture literal process exit codes. If no Node/CI access, report UNRUN and construct reproducible commands. Do not mark owner acceptance or edit roadmap.v1.json.
5. Publish a bounded A00 GO/NO-GO recommendation to the keeper, separating observed PASS, FAIL and UNRUN and specifically calling out whether Lane A/B deltas are required.

## Run/prove (actual exits or UNRUN)

- `node --test scripts/batch-evidence/wiki-evidence-regression.test.mjs scripts/roadmap/evidence-truth.test.mjs`
- `node scripts/roadmap/evidence-truth.mjs --check`
- `node scripts/batch-evidence/WIKI-BASELINE.mjs (static only; --live requires isolated Host evidence)`

## Acceptance criteria

- ${s}
- ${s}
- ${s}
- ${s}

## Required final handoff

One exact branch head SHA, base/main pin, per-file change list and why, actual test commands with literal exit codes versus UNRUN, observed runtime/browser/CI environment, security/ownership checks, unresolved blockers, dependency acceptance state, cross-lane integration request(s), and the next ready frontier as read-only observation. Report a measured subsequent-work acceleration only with real evidence; otherwise `NOT_YET_MEASURED`. Do not claim production/owner acceptance based on source-only checks. Preserve all original historical receipts.
