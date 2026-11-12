# ARBOR Proposal 5 — Labs First-Pass Readiness

**Purpose:** read-only evidence-based graduation readiness, inside Labs. This package neither owns a Host action catalog nor creates product runtime or outside wiring. ARBOR remains the single product destination.

## Current first pass

- [Human report](reports/FIRST-PASS-2026-11-12.md)
- [Machine-readable GitHub tree snapshot](reports/FIRST-PASS-2026-11-12.json)
- [Source-to-destination component manifest](components.json)
- [Deterministic local scanner](run-first-pass.mjs)
- [Scanner unit tests](tests/run-first-pass.test.mjs)
- GitHub workflow: .github/workflows/p5-labs-readiness.yml (feature branch only)

The committed snapshot was produced by fetching **actual GitHub tracked trees** at pinned SHAs. It confirms source paths and file types, **not** runtime wiring or test execution. Earlier test counts in donor README files are reported history only.

## Run locally from the Labs checkout

Requires Node 24+ and Git. The baseline Git commit objects must already exist locally; no external VIVIM repositories or network services are used by the scanner.

~~~bash
node --test readiness/tests/*.test.mjs
node readiness/run-first-pass.mjs --out .readiness-output/first-pass.json --md .readiness-output/first-pass.md --strict

node --test personal-knowledge-lifecycle/runtime/tests/*.test.mjs
node --test capability-observatory/lane-d/tests/observatory.test.mjs
node --test vivim/voice-to-tasks/workbench/workbench.test.mjs
node --test chronos-toolkit/wave2/vivim-web-harness/studio/test/unit/chronos-conversation-projection.test.mjs
node capability-observatory/scripts/validate.mjs
~~~

On a machine where the UI Labs experimental commit is fetched, optionally run its Node tests *inside that branch's own isolated worktree*. Do not copy its server implementation into the production app and call it integrated.

## Status semantics

- SOURCE_TREE_PRESENT: the pinned Git blob exists. It does not prove compiled source, an active subscriber, actual data source, permissions, runtime availability, or device acceptance.
- SOURCE_PATH_MISSING: an exact source prefix was not found; correct the manifest rather than assume removed.
- REF_UNAVAILABLE: the pinned commit wasn't fetched; mark unknown and retry after fetching that exact historical commit.
- REPORTED_SYNTHETIC_TESTS: a historical source-authored test claim; only newly executed runs with exit codes can become a fresh test receipt.
- HOST_WIRING_UNVERIFIED and DEVICE_UNRUN remain release blockers if needed by the declared beta scope.

Do not promote a Lab experiment to product runtime merely because tests pass. Effect admission, personal evidence and World writes belong to existing Host; HQKG graph source and TierFlow renderer have separate owners.

## Security and source privacy

The scan reads **Git trees and paths only**, not raw conversation data, browser profiles, archive payloads, local credentials, provider sessions or .private-local content. Reports contain tracked path metadata. Source subjects under archived corpora, private project descriptions or historical operational logs are not automatically safe to move into ARBOR. Source review/promotion requires owner authorization and license/privacy checks.

## Promotion process

1. E-category (easy) contract/test candidates can be prepared now on separate branches and reviewed by main migration F/E/D; no direct edits to their active code branches.
2. C-category (core) requires implementation owners A–F after their Wave 1+ gates and exact source compatibility.
3. H-category (high-value development) is design/test backlog until product sources and beta gates permit a narrowly scoped prototype.
4. X-category stays Labs/research by default; never bulk-import historical corpora or PC tooling into VIVIM.

**First-pass results belong to the work/p5-phase2-readiness-r001 research branch.** Integrating any selected code into ARBOR is a later explicit serial-G decision.
