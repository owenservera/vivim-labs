# ARBOR Acceleration R002 — Parallel W1 Lane B

**Assignment:** One-command reproducible Host + evidence acceptance
**Branch:** `work/accel-r002-w1-b-runtime-proof`
**Maps to:** A01 execution proof; confirms A00/A02 observable behavior without owning their source
**Priority:** Immediate verification accelerator
**Parent contract:** [parallel/README.md](README.md) — read it first. Do not touch `main`, do not merge, and do not change the canonical roadmap.

## Exclusive write scope

- `scripts/acceleration/live-host-check.mjs`
- `scripts/acceleration/live-host-check.test.mjs`
- `.github/workflows/acceleration-r002-w1-b.yml`
- `docs/acceleration/w1/B.md`

Only these paths are writable. All other repository content is read-only even if a nearby fix seems convenient. Produce your final report at `docs/acceleration/w1/B.md` within the branch; source path ownership is exclusive.

## Mandatory additional source inspection

- `apps/vivim-host/server.mjs (read-only: Lane A owns)`
- `apps/vivim-host/public/pages/wiki.js (read-only: Lane C owns)`
- `scripts/batch-evidence/WIKI-BASELINE.mjs (read-only: Lane D owns)`
- `scripts/test-manifest.json (read-only: keeper-owned)`

Read actual current contents and verify branch base; do not substitute the work-order prose for source inspection.

## Execution steps

1. Run the existing one-command isolated Host loop under available Node 24 (and Bun only when appropriate). Identify actual import/dependency failures and test exit codes. Fix defects in this lane's runner only; request changes to other sources from their owner rather than editing out of scope.
2. Record per-run Git SHA, OS/runtime/dependency versions, command line, subprocess exit code, timeout, stdout/stderr excerpts and temporary data-root proof. Running isolated tests must never touch committed demo journal or owner data.
3. Exercise GET /chat, /wiki, /demo, script+CSS assets, typed /api endpoints, chat append, Wiki edit CAS, cross-origin/unauthorized refusal, restart replay and read-through; compare API state to journal and capture negative checks.
4. Add a scoped read-only CI entry or test command that executes the same Node test path, without making PRs/merges or claiming CI passed when GitHub Actions starts zero jobs. If workflow infrastructure fails, retain true startup_failure evidence and a local fallback command.
5. Make cross-lane acceptance conservative: validate what the source branch actually contains; missing Lane A/D changes are integration handoffs, not an excuse to copy other lane code.

## Run/prove (actual exits or UNRUN)

- `node scripts/acceleration/live-host-check.mjs`
- `node --test scripts/acceleration/live-host-check.test.mjs`
- `node --test apps/vivim-host/test/product-journal.test.mjs apps/vivim-host/test/product-routes.test.mjs (read-only observation)`

## Acceptance criteria

- ${s}
- ${s}
- ${s}
- ${s}

## Required final handoff

One exact branch head SHA, base/main pin, per-file change list and why, actual test commands with literal exit codes versus UNRUN, observed runtime/browser/CI environment, security/ownership checks, unresolved blockers, dependency acceptance state, cross-lane integration request(s), and the next ready frontier as read-only observation. Report a measured subsequent-work acceleration only with real evidence; otherwise `NOT_YET_MEASURED`. Do not claim production/owner acceptance based on source-only checks. Preserve all original historical receipts.
