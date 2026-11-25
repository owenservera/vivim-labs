# VIVIM Universal Canvas — research dossier R001
2026-11-25 | Public-safe research, not product adoption or release acceptance.

## Mission
Natural language, pen, touch and agent-authored typed programs create editable visual objects, drawings and paintings inside the existing GPUI desktop surface. The same surface is relayed to the paired phone. Rendering is distinct from intelligence: GPUI can display instructions but cannot infer an artistic composition by itself.

- [Technical research and engine comparison](ENGINE-RESEARCH-R001.md)
- [Integration design and contracts](WIRING-DESIGN-R001.md)
- [Experiments, budgets and acceptance](EXPERIMENTS-R001.md)

## Recommendation
GPUI first for vector primitives and interaction; Kurbo/Peniko for portable geometry/style (adapters rather than exposing library types in persisted contracts), tiny-skia/resvg for CPU fallback/import, Vello CPU and vello_gpu for comparative 2D evaluation, wgpu for optional programmable brush/material simulation and later 3D. No new window, product renderer, browser, external API dependency, database or effect authority. Offscreen computation is not a second user-facing UI. First prove CPU pixel bridge; native texture sharing is an open experiment, not an existing API. Keep the existing Host in charge of persisted changes and effect receipts; existing VLANG/VEC contracts and remote viewport are extended, not replaced.

## Grounding
ARBOR main inspected 2026-11-25, exact Git blob IDs: root Cargo.toml 753dbfdb25354a8298e5c401b69f3e262ab078dd; native/gpui-desktop/Cargo.toml 2eb43353b68d9234b9f7e84a1d5221bd6e910e24; native/gpui-desktop/src/visual.rs d5ccd055fad05d7973e6f34a79ed29fcdefec181; core/programmability/README.md a52d72e68b644d16328c6ce64343b95d1d59d482. Blob IDs establish file content, not a common commit pin. Local read-only directory inspection through VIVIM confirmed native desktop source filenames; no private workstation contents published. No build/test/benchmark performed. Upstream links point at rolling documentation unless otherwise noted; pin revisions and licenses before any implementation.

Respect [Labs rules](../AGENTS.md), [ARBOR source audit](https://github.com/owenservera/ARBOR/blob/main/docs/product/ARBOR-NOW-FUTURE-WIRING-20261123.md), [current execution pack](../ZCODE-DESKTOP-COMMANDER-OPERATIONS/DEVOPS-PLANNING/PACKS/EXECUTION-20261123-R001/WIRING-DELTA-R002.md), existing task authority [#33](https://github.com/owenservera/ARBOR/issues/33) and existing claims. This is an experiment dossier, not a competing backlog or implementation claim.
