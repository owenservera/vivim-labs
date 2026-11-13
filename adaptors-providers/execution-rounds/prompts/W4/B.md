# ARBOR PROVIDER HUB R002 — W4 LANE B: YouTube pack

**Execute this worker now using connected GitHub.** Target: owenservera/ARBOR. Branch: work/provider-hub-W4-B-r001. Base: accepted W3 gate head.

## Objective
Mock YouTube watch history/subscriptions/playlists/channels/comments, unavailable media and pagination.

## Owner paths
Only write to apps/vivim-host/provider-hub/packs/youtube/** and docs/provider-hub/worker-reports/W4-B.md. If source requires another shared file, document the exact patch for G to apply serially. Never silently modify another lane or a root manifest.

## Required steps
1. Read current ARBOR AGENTS.md (if present), Proposal 5 authority/ownership, this program README, 03-ARCHITECTURE-AND-MOCK-PORTS.md, WAVE-MANIFEST.json and the accepted gate head.
2. Inspect actual existing ARBOR source functions; reuse what is already real and classify proposal vs implementation. Pin current SHA; if the branch base changed unexpectedly, reconcile safely.
3. Deliver implementation and focused synthetic unit tests whenever this lane is code-owning. W0 is evidence only. Make mocked data exercise the intended production-facing typed service/ports rather than a standalone toy UI.
4. Inject errors, denial, account swap, partial/uncertain outcomes and privacy/refusal tests relevant to your lane. No secrets, personal social data, live provider traffic or real sends.
5. Commit only authorized paths and a worker report. Show exact SHA, test commands/exits, UNRUN, duplicate-authority checks, blocker list and proposed G decision.

## Hard architecture
ARBOR is self-contained and Host controls World/effects; no second Host, World, browser controller, TierFlow publisher, graph store, or phone application. The existing AI-chat provider registry is not this social backend. Learned Studio adapters are read-only; no provider APIs, Playwright/Puppeteer or implicit mock fallback in live mode. Where World wiring is not present, use an injected contract-faithful mock and label it truthfully.

## Acceptance evidence
At minimum: source/diff and test summary with deterministic fixtures, negative refusal case, stable versioned boundary, no unauthorized external effects, and exact handoff for G. Never mark LIVE_VERIFIED from mock tests.
