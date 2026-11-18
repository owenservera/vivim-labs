# ARBOR Acceleration R002 — Parallel W1 Lane C

**Assignment:** Realtime Wiki second-screen source-backed projection and visual acceptance
**Branch:** `work/accel-r002-w1-c-wiki-visual`
**Maps to:** A02 UI integration, bounded A04 prework only; A04 activation still needs accepted A02
**Priority:** Visible product payoff lane
**Parent contract:** [parallel/README.md](README.md) — read it first. Do not touch `main`, do not merge, and do not change the canonical roadmap.

## Exclusive write scope

- `apps/vivim-host/public/pages/wiki.js`
- `apps/vivim-host/public/pages/wiki.css`
- `apps/vivim-host/public/pages/wiki.html`
- `apps/vivim-host/test/wiki-visual-contract.test.mjs`
- `docs/acceleration/w1/C.md`

Only these paths are writable. All other repository content is read-only even if a nearby fix seems convenient. Produce your final report at `docs/acceleration/w1/C.md` within the branch; source path ownership is exclusive.

## Mandatory additional source inspection

- `apps/vivim-host/public/pages/chat.js (read-only; not owned)`
- `apps/vivim-host/chat/product-journal.mjs (read-only: Lane A owns)`
- `apps/vivim-host/server.mjs (read-only: Lane A owns)`
- `apps/mobile-relay/vec/compiler.mjs (read-only)`
- `docs/product/roadmap/roadmap.v1.json (read-only)`

Read actual current contents and verify branch base; do not substitute the work-order prose for source inspection.

## Execution steps

1. Make screen 2 a useful intelligent source-backed projection over the existing Wiki, roadmap, file-surface and journal endpoints. Surface source references, revisions, error/retry states and a clear indicator when data is seed/demo vs verified live state.
2. Implement a bounded human-readable edit preview/diff with a fresh expected Wiki revision and explicit Host acknowledgement. On 400/403/409/network failure, present refusal and keep edits for revision/retry; never optimistically say saved.
3. Use the existing typed VEC contract where available for display composition; do not implement a new NL parser, imperative visual publisher or privileged command interpreter. If A04 compiler integration is blocked, leave a typed read-only adapter seam and a demonstrated source-based projection rather than fake full realtime compilation.
4. Keep strict self-only CSP, external CSS and JS, no inline scripts/styles, accessible mobile/wide-screen layout and efficient refresh without unnecessary full repaint on unchanged revisions.
5. Add source/DOM contract tests scoped to the Wiki frontend; use real browser only if available and report actual check exits. Do not modify Host server routes or add a second API.

## Run/prove (actual exits or UNRUN)

- `node --test apps/vivim-host/test/wiki-visual-contract.test.mjs`
- `node --test apps/vivim-host/test/product-routes.test.mjs (read-only observation)`
- `Manual or real-browser /wiki rewalk with data source/revision evidence when available`

## Acceptance criteria

- ${s}
- ${s}
- ${s}
- ${s}

## Required final handoff

One exact branch head SHA, base/main pin, per-file change list and why, actual test commands with literal exit codes versus UNRUN, observed runtime/browser/CI environment, security/ownership checks, unresolved blockers, dependency acceptance state, cross-lane integration request(s), and the next ready frontier as read-only observation. Report a measured subsequent-work acceleration only with real evidence; otherwise `NOT_YET_MEASURED`. Do not claim production/owner acceptance based on source-only checks. Preserve all original historical receipts.
