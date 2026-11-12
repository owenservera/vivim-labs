# R001-05 — Source Truth Delta: Canonical Full-Update Capability Is Not Yet Registered

Date: 2026-11-12. Read-only audit based on connected GitHub source. Not a runtime scan.

## Verified Host catalog facts
- File: \`vivim-web-harness/apps/vivim-host/control/action-catalog.v1.json\`
- Branch: \`main\` at \`cae97afe848d79957c9d6c698a90996a758bfd56\`
- Schema: \`vivim.control-action-catalog/1\`
- Catalog version: \`6\`
- Entries found: \`19\`
- Existing related read actions: \`vivim.status.show\`, \`devices.summary\`, \`personalization.context.summary\`
- No action title/ID/example found for an explicit canonical \`full project update\` command. The current Host catalog therefore cannot authorize or even declare a full-update action based on this source alone.

## Engineering consequence
- The synthetic \`full\` test in \`core/programmability/examples/user_io_r001.rs\` registers \`fixture.full.update\` and \`fixture.readonly.project_update\` inside a Rust example **only**. They are not real Host action IDs, not a current runtime capability, not permissions, and not user-facing production commands.
- The new Tauri \`user_io_event\` initially has \`TrustedSources::default()\`, therefore no project identity, template or user habit can be falsely claimed from the live product.
- UI R006's two-Tab full-update is a product goal that still requires a source-owner integration path.

## Required product integration for "full"
1. Review what existing project status, wiki, graph, mission, task/issue/decision and context-read capabilities are actually executable and currently verified.
2. Build an **approved read-only full-update capability or canonical composite action** against these source-owned read ports, not a new fake registry or "LLM guess = action".
3. Register its stable ID, scope/slots/risk/readiness/capability in the existing authoritative Host catalog or reviewed source-owned crosswalk, then expose through a trusted native Rust read adapter.
4. Provide current active project from an authenticated, revisioned Host/World or HQ development workspace source as appropriate; do not assume the text "XYZ" is a real identity.
5. Keep effect-free "read summary" path distinct from communications, publishing, device changes and other Host effects.
6. Wire Rust dependency graph + pure aggregation for current issues, decisions, next steps and sources under privacy/revision budgets.
7. Build differential tests against current JS planning and a real project source corpus, then connect UI read-only and test actual Tab interactions.
8. Claim "real-time verified" only when the relevant source owner supplies an observed freshness/availability proof. Graph prediction, static code registration and AI completion are not verification.

## Unverified
- Local checkout/provider sessions, source index currentness, actual active project, complete operational action surface outside the single Host catalog, Windows installer, phone transport and live acceptance were not verified by this source inspection.
- Do not infer there are no *other* ways to read project state anywhere in the platform; the concrete claim is only that this canonical Host action catalog has no explicit full-update action.
