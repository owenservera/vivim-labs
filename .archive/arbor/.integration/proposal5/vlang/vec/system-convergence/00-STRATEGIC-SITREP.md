# Strategic synthesis — one programmable VIVIM, several bounded engines

## ERA 1 — Source reality
ARBOR now contains substantial HQ, native Rust, TierFlow and Host/phone source, but this is not the same as a working VEC product. HQ's development index is real source; HQKG has graph, typed DAG, durable journaling and hot-swappable **trusted** compute; the smaller Product Rust crate validates inert typed program graphs; TierFlow has Worlds, revisioned UI, safe renderer and a static-only World→visual bridge; Host already has NLCL and effect authority. The separate phone Build path remains model-design-prompt-first.

## ERA 2 — The architecture we should converge on
**At SDK development time:** a first-party Rust comment capsule is parsed as untrusted source, tied to a Git blob, validated against a reviewed SDK ABI, isolated-compiled, independently checked and accepted as a product capability manifest. HQ owns *development* evidence and Wiki, not the user's private programs.
**At user runtime:** natural language -> existing NLCL + installed capability manifests -> typed VEC Program IR -> Product Rust pure validation and optional scoped HQKG pure algorithm -> Host preview, current grants and personal program revision -> TierFlow bound World/visual projection. An installed routine should execute via precompiled local SDK, not repeatedly generate code with LLM calls.
**At effect time:** Host alone authenticates actor/device/session, current policy, data disclosure, one-use approvals, idempotency and receipts. TierFlow is UI; HQKG is compute; neither grants access or executes browser provider sends.

## ERA 3 — The dream
A normal user says, 'Build me a dashboard for today's work', 'Make this card show only my current project', 'Every Friday archive completed tasks', or drags a data source onto a card. VIVIM compiles the intent to a durable, explainable, reversible **personal program** and renders its interface. Same semantics across speech, text, touch, Studio and phone. The VEC SDK becomes increasingly capable, causing eligible future requests to take the cheap deterministic path. Novel, creative or unrecognized requests can use optional model-assisted **proposals**, never implicit grants.

## Non-negotiable distinctions
- **HQ development semantic IDs** versus **Host installed product capability IDs** versus **user-owned program instances**: linked, not interchangeable.
- **HQKG native graph** versus **Product Rust IR validator**: reuse compatible pure algorithms, do not fuse stores or policy.
- **TierFlow persisted World/1**, **Canonical Visual/1**, **phone UI Surface/1**: different schemas, no unsafe implicit migration.
- **UI/layout undo** versus **compensation for an external action**: not equivalent.
- **Source present** versus **runtime/device/security acceptance**: this research executed no tests.

**Highest-value choice:** build one end-to-end deterministic user program before migrating dozens of JS algorithms to Rust.