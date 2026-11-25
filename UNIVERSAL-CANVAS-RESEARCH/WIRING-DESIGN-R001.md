# Universal Canvas integration design R001
Proposal, not implemented. Rust-first. One GPUI surface, one Host authority, one phone viewport.

## Existing source touchpoints
- ARBOR root workspace excludes native/gpui-desktop; desktop has independent workspace, optional desktop feature pins gpui =0.2.2 and gpui-component =0.5.0. Avoid introducing wgpu into root workspace. Blob 2eb43353b68d9234b9f7e84a1d5221bd6e910e24.
- core/programmability is the inert deterministic transformation core. Existing visual/VEC/LiveSurface contracts must be reused. README blob a52d72e68b644d16328c6ce64343b95d1d59d482.
- native/gpui-desktop/src/visual.rs imports Visual/Node/RevisionGuard, validates Host source revision, compiles VEC and diffs view. Blob d5ccd055fad05d7973e6f34a79ed29fcdefec181. Extend via versioned canvas region projection, not a replacement visual state.
- Local read-only VIVIM directory inspection confirmed main.rs, surface_registry.rs, remote_viewport_contract.rs, remote_host_transport.rs, desktop_capture.rs, live_delivery.rs, control.rs. Their detailed current contracts must be reread at implementation time; these are candidate seams, not tested integrations.
- Existing Host apps/vivim-host and Studio own effects/provider authorization. No new daemon, browser controller or parallel persistence.

## Architecture
Natural language / human pointer / approved provider proposal -> existing VLANG/NLCL -> typed CanvasOp batch -> validator -> inert preview -> Host admission with actor/document/base revision and existing approval policy -> durable scene revision/receipt -> GPUI region with hit tests and render adapters -> existing paired GPUI viewport -> mapped inputs returned to native controls. A renderer is never an authority. Local transient strokes may be displayed immediately; completed edits become committed through existing Host transaction path. No provider calls just for panning, recolor, repeat or undo.

### Proposed modules (names illustrative)
core/programmability/src/canvas/ : schema, validation, patch, deterministic recipes, semantic handles, capability descriptions (no GPU dependency).
native/gpui-desktop/src/canvas/ : GPUI element, input routing, selection, transforms, frame scheduling, cache, presentation adapter; optional native graphics dependency separate from core.
Host: extend existing dispatcher/journal/persistence for versioned canvas actions and source revisions; inspect exact existing ports before implementing.
PWA: existing authenticated viewport transport; no Canvas2D/WebGL replica and no separate artistic state.
HQ/AutoWiki: derived semantic object descriptions, provenance and accepted reusable techniques through existing source model; do not index private canvases without grants.

### Domain model
CanvasDocument { schema_version, document_id, revision, units, artboards, objects, layers, assets, timeline, metadata }; stable typed IDs; canonical serialization and bounded sizes. Scene nodes: Group, Path, Shape, Text, Image, StrokeBatch, RasterLayer, Mask, Effect, Camera/SpatialScene extension. Object references and roles (cat.eye.left) are semantic handles, not guaranteed visual recognition. Each node carries transform, bounds, paint, opacity, blend mode, clip, parent and provenance. Retain original editable strokes and authoring source; renderer caches disposable. Asset references hash local content, no automatic network loading. Geometry f64 in document space, validated finite bounded values; convert to f32/pixels at backend. Define fill rules, cap/join, antialias, premultiplied alpha and linear-light compositing; explicit sRGB input/output conversion. Text shaping should reuse existing GPUI or dedicated consistent shaping adapter; record fonts and fallback.

### Illustrative Rust contract, intentionally not compiled
```rust
struct CanvasPatch { document: DocumentId, base: u64, op_id: OpId, ops: Vec<CanvasOp> }
enum CanvasOp {
    Insert { parent: NodeId, node: Node },
    SetPaint { target: NodeId, paint: Paint },
    Transform { target: NodeId, affine: [f64; 6] },
    AppendStrokes { target: NodeId, strokes: Vec<Stroke> },
    Remove { target: NodeId },
}
trait CanvasBackend {
    fn capabilities(&self) -> BackendCaps;
    fn prepare(&mut self, snapshot: &Scene, dirty: &DirtyRegion) -> Result<(), RenderError>;
    fn render(&mut self, viewport: &Viewport) -> Result<Frame, RenderError>;
}
enum Frame { NativeDrawList(DrawList), PixelTiles(TileSet), /* GPU token only after proof */ }
```
Host validates exact base revision, idempotency key, actor, asset access, complexity budget, finite geometry, DAG cycles, balanced layer stack, valid object references, command allowlist. Atomic apply or no change, no stale overwrite. Undo/redo uses inverse patches or checkpoints and does not delete journal evidence. Crash recovery reloads snapshot + verified ops. Render job carries scene revision and viewport generation; discard stale frames. No arbitrary generated Rust, shader source, shell, script, HTML or executable SVG from AI; registered shaders only.

## Rendering
A GPUI canvas paint closure draws cached paths and display list, uses Window::paint_path/image in paint phase; capture events via enclosing interactive element. Build geometry/tessellation asynchronously or cache by scene revision, scale and object. Request refresh only on dirty state/active animation. Avoid blocking UI on disk, inference or map readback.

CPU bridge: rasterize dirty regions with tiny-skia/resvg/vello_cpu -> normalize channel order/alpha/stride required by pinned RenderImage -> Arc image resource -> GPUI paint_image. Confirm image constructor and texture caching behavior in pinned source; retire old frames without leaking atlas resources. For correctness start with full image, then tiles.

GPU bridge: request compatible adapter/device/queue on render worker; choose supported texture formats/usages and renderer version; Vello render into offscreen view; compute brush stamps into ping-pong storage textures or render targets; copy dirty regions into bounded staging ring. Row pitch = ceil(width*bytes_per_pixel/256)*256; strip padding, normalize format and alpha; map async with correct device polling and unmap before reuse. Do not read and write aliased storage resources without supported safe semantics. GPU loss -> CPU fallback with committed scene unchanged. Zero-copy only after proving compatible adapter, shared-resource handle/fence, resource state and lifetime, synchronization, format and composition with GPUI; no undocumented API assumed. Bench GPU memory, transfer and full window capture overhead.

### Input / phone
Map physical pointer to canonical viewport via actual DPI, crop, zoom/pan inverse and remote frame ID. Preserve pointer ID, pressure/tilt when available, monotonic sequence, button, coalesced samples; distinguish finger pan/zoom from brush; cancel on lost focus, stale revision or disconnect. Physical pen capabilities optional and need test. Hit testing via bounds spatial index plus path/mask geometry, stable object IDs and accessibility labels. Phone consumes existing captured GPUI frames with revision and coordinate mapping; cannot claim parity until real device evidence. Network throttles, backpressure, authenticated session, reconnect resync, no replay of already committed stroke IDs.

## Artistic compiler and techniques
Resolve intent into composition, palette, lighting, pose, shapes, brush recipe, order. Example prompt 'paint a ginger cat beside a window' -> insert background/window/cat groups; paths for silhouette/head/ears/eyes; seeded fur curves, soft shadows and whiskers; separate named eye nodes. Render preview then commit via existing policy. Follow-up 'make its eyes green' resolves target IDs, deterministic SetPaint patches. Recipes store algorithm version, seed, parameters, dependencies, bounds, attribution and optional user approval; reusable without LLM. Unsupported concepts can use existing authorized AI provider to propose validated typed data, no new API dependency. Raster reference-to-stroke optimization optional; semantic reconstruction not automatic. Evaluate both aesthetic quality and semantic editability; avoid claiming a stylized vector cat equals photorealistic oil art.

Brush design: sample interpolation by arc length; pressure curves; velocity/direction; seeded scatter; stamp texture/rotation; alpha accumulation; eraser mask; tile-local damage. Smudge reads previous pixels with bounded pickup state; wet paint adds water/pigment/height fields and fixed time stepping; separate display color transform from pigment mixing. Deterministic replay within algorithm/version; CPU/GPU pixel tolerance across drivers rather than promise bit equality. Render video/timeline by fixed clock, not wall-time. Store checkpoints for simulations; cap memory/steps.

## Future domains
Use same scene/operation vocabulary for graphs, charts, diagrams, mathematical plots, interface mockups, document layouts and animation. CAD needs separate exact constraints/topology and units solver, not just pixels. 3D requires mesh/material/camera/lighting/depth and picking; optional offscreen wgpu/Bevy integration, no game window. SVG/PNG/OpenRaster first, PDF/glTF later as separately validated export adapters; SVG semantic metadata may not roundtrip, PNG flattened, OpenRaster preserves raster layers not entire typed document. Keep own native versioned document authoritative.

## Security / lifecycle
Limits for image decompression, SVG depth/paths, canvas dimensions, layers, strokes, tile count, timeline, GPU allocation and work per tick; no external resource links/scripts/foreignObject, path traversal, shell effects or arbitrary plugin execution. Treat provider data and imports as untrusted. Explicit export destination via Host; no silent overwrite. Content-addressed local assets, cleanup only after reference accounting. Central config UI exposes optional quality/backend/memory/brush and remote bitrate settings, no duplicate config registry. Capability-selected fallback, per-session errors and diagnostics. Additive feature flags and migration readers, staged install and rollback; pinned versions and lockfiles, license review, preserve existing GPUI startup, pairing URL and Host receipt flow.
