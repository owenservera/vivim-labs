# ARBOR Acceleration R002 — Parallel W1 Lane E

**Assignment:** Deterministic demo feedback-to-work-order compiler
**Branch:** `work/accel-r002-w1-e-feedback-compiler`
**Maps to:** A03 preparatory implementation; cannot be activated until A00 independently accepted
**Priority:** Highest next compounding accelerator
**Parent contract:** [parallel/README.md](README.md) — read it first. Do not touch `main`, do not merge, and do not change the canonical roadmap.

## Exclusive write scope

- `scripts/feedback-loop/**`
- `docs/acceleration/w1/E.md`

Only these paths are writable. All other repository content is read-only even if a nearby fix seems convenient. Produce your final report at `docs/acceleration/w1/E.md` within the branch; source path ownership is exclusive.

## Mandatory additional source inspection

- `apps/vivim-host/public/demo/controller.js (read-only)`
- `apps/vivim-host/public/demo/journal.jsonl (read-only illustrative source, do not mutate)`
- `docs/product/roadmap/roadmap.v1.json (read-only authority)`
- `scripts/roadmap/acceleration-queue.mjs (read-only)`
- `docs/product/roadmap/ACCELERATION-R002.md`

Read actual current contents and verify branch base; do not substitute the work-order prose for source inspection.

## Execution steps

1. Build an actual deterministic source-only normalizer that consumes existing demo verdict/journal entries and failed verifier receipts. It must compile one actionable candidate with source pin, owner lane, narrow paths, reproduction command, acceptance condition, dependency IDs and provenance—not a second roadmap.
2. Deduplicate repeated failures by canonical source/condition identity; preserve original distinct evidence references. Reject malformed, oversized, untrusted or instruction-injected feedback that tries to expand scope or request privileged writes.
3. Produce a dry-run machine-readable suggestion and human summary; no direct writes to roadmap.v1.json, no agent launches, no Host/provider effects and no invented work order ownership. Unknown source ownership must yield REVIEW_REQUIRED instead of an auto-assigned lane.
4. Add tests for one real deliberately failing fixture, duplicate reports, stale SHA/revision, unknown owner, no new source effects and replays that produce identical digest. Source pins are required on every generated order.
5. Document exactly how the canonical keeper can consume the reviewed output after A00 is independently accepted. Demonstrate one feedback→candidate transformation, but report A03 activation/acceptance as BLOCKED on A00 until its executed receipt exists.

## Run/prove (actual exits or UNRUN)

- `node --test scripts/feedback-loop/*.test.mjs`
- `node scripts/feedback-loop/compile.mjs --help (if CLI added; do not fake working commands)`

## Acceptance criteria

- ${s}
- ${s}
- ${s}
- ${s}

## Required final handoff

One exact branch head SHA, base/main pin, per-file change list and why, actual test commands with literal exit codes versus UNRUN, observed runtime/browser/CI environment, security/ownership checks, unresolved blockers, dependency acceptance state, cross-lane integration request(s), and the next ready frontier as read-only observation. Report a measured subsequent-work acceleration only with real evidence; otherwise `NOT_YET_MEASURED`. Do not claim production/owner acceptance based on source-only checks. Preserve all original historical receipts.
