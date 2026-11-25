# Experiment sequence and evidence R001
All experiments proposed / UNRUN. No new master tracker; coordinate with ARBOR #33 and TASK-CLAIMS before source changes. Do not create production effect paths merely to demonstrate graphics.

| ID | Experiment | Acceptance evidence |
|---|---|---|
| UC01 | Pinned GPUI 0.2.2 paint example inside existing GPUI region | real Windows screen shows vector cat/paths, freehand, pan/zoom; same GPUI window |
| UC02 | Native document / op contract | validation, stale patch rejection, idempotent op, save/reopen, undo/redo, semantic eye recolor |
| UC03 | CPU image bridge | tiny-skia raster test, non-square widths/alpha/color/dpi correctness, frame lifetime; actual GPUI presentation |
| UC04 | Compare direct GPUI, tiny-skia, vello_cpu, vello_gpu | same scenes and timings; adapter/driver/source/locks logged; unsupported backend explicit |
| UC05 | wgpu offscreen renderer and staging ring | GPU render/readback, no UI block, padded rows, stale discard, resize/device loss fallback |
| UC06 | Optional zero-copy proof | pinned backend source and verified resource ownership/fences; real Windows compositor output and stress; if fail retain CPU bridge |
| UC07 | Procedural paint | seeded brush strokes, brush texture, layer compositing and incremental damage; reproducible within version |
| UC08 | VLANG creative path | prompt -> typed editable cat -> eyes green -> undo -> replay without fresh LLM; ambiguous target asks clarification; no arbitrary code |
| UC09 | Phone | actual paired phone receives same GPUI frame revision, input mapped to canvas, disconnect/reconnect without duplicated patch |
| UC10 | Art extensibility | same runtime produces chart, geometry, diagram, editable painting; export/import with explicit loss |
| UC11 | Advanced brushes | smudge, wet media/impasto, fixed-step simulation benchmark, optional local GPU |
| UC12 | Optional 3D | offscreen scene with picking and native GPUI presentation; no alternate UI surface |

## Benchmark design
Use target Windows hardware, integrated/discrete status and driver measured rather than inferred. Log source SHA, rustc, lockfile, adapter/backend, texture format, feature limits, build time/disk delta, process peak RSS, GPU memory where observable, warm/cold timings. Compare 512/1024/2048 pixel canvases, 100/1k/10k paths, 1k/10k strokes, masks/layers/gradient mix, resize/pan/zoom, stylus/touch and remote capture. Measure p50/p95 frame and input-to-visible latency, frame drops, copy bytes/s, readback/upload stalls, cache hit rate and color differences. Candidate target: 60 fps for light local edits (16.7ms frame), p95 local pointer-to-visible under 50ms, phone under 150ms on controlled LAN; targets not claims. Degrade to 30fps/dirty tiles/reduced preview while preserving document. Measure actual GPU/time before choosing backend.

RGBA8 buffer sizes: 1024²=4 MiB, 2048²=16 MiB, 4096²=64 MiB. Full 2048² readback at 60 fps is 960 MiB/s before upload/overhead; two copies roughly double traffic. 256² RGBA8 tile = 256 KiB. 3 full 2048² staging buffers = 48 MiB before textures/layers. Use caps and dirty region scheduling, bounded worker queue, coalescing and cancellation. Begin at 1024 preview, allow higher resolution export outside UI frame budget. Actual budgets configurable and benchmarked. Do not compile every heavy candidate simultaneously.

## Verification
Deterministic core: finite bounds, DAG cycles, validation, transaction atomicity, stale/op replay, undo, serialization migration, seed recipes, coordinate transforms. Renderer: alpha/blend/gradient and clip reference images, tolerance metrics, asymmetric row stride, DPI, zoom and color spaces, memory/resize/long-run stress, worker cancellation. Security: oversized/decompression/import fuzzing, external resource rejection, no script/exec, authorization and export denial, private asset isolation. Device acceptance: real native GPUI output with source hash, actual paired Android frame and touch receipt, same document revision; no synthetic result represented as physical success. Tests can use controlled fixtures as test evidence only. No regression to existing native desktop/control/provider flows.

## Source/implementation handoff
Re-read current ARBOR HEAD and all exact interfaces, active path claims, upstream pinned crate/API/license and lock compatibility. Baseline existing desktop build first; use independent feature-gated experimental module, never widen Host authority. Proposed commands once implemented: cargo test --manifest-path core/programmability/Cargo.toml; cargo test --manifest-path native/gpui-desktop/Cargo.toml; cargo check --manifest-path native/gpui-desktop/Cargo.toml --features desktop. Confirm existing build scripts and feature names before run; these are proposed, not executed. Use VIVIM MCP for authorized local verification, GitHub main for source commits, no Desktop Commander dependency. Evidence format: experiment, exact source and lock, command/exit, artifacts hash, backend/driver, local/physical proof, limitations, decision. If zero-copy or advanced media fails, preserve stable CPU/GPUI baseline. No production adoption or scope claim until owner and source authority review.
