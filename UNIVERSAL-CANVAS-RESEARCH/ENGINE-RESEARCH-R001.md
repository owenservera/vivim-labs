# Engine research and decision matrix R001
Observed 2026-11-25. Context7 research performed with three library resolutions and three documentation queries (tool limit). Upstream source corroboration follows. No source was vendored.

## Context7 evidence
| ID | Query scope | Observed findings | Primary source |
|---|---|---|---|
| /websites/rs_gpui | custom paint, paths, images, SVG, external GPU handles | Window::paint_path(Path<Pixels>, impl Into<Background>), paint_image(Bounds, Corners, Arc<RenderImage>, frame_index, grayscale), paint_svg monochrome; paint-phase only. PathBuilder uses Lyon; fill/stroke, dash, quadratic/cubic and arc paths. No public cross-backend texture import established by result. | https://docs.rs/gpui/latest/gpui/struct.Window.html ; https://docs.rs/gpui/latest/src/gpui/path_builder.rs.html |
| /linebender/vello | scene, clipping, layers, render-to-texture and GPU requirements | Research compute renderer Scene fill/stroke/push_layer/render_to_texture; newer vello_gpu has different scene/render APIs, CPU preprocessing, clip/opacity layers, linear/radial/sweep paints, RenderTargetConfig, texture/depth views. Do not mix example generations. | https://github.com/linebender/vello/blob/main/vello_gpu/examples/render_to_file.rs ; https://github.com/linebender/vello/blob/main/ARCHITECTURE.md |
| /gfx-rs/wgpu | offscreen render, storage textures, readback, HAL | render attachment/storage usage as supported by format/features; COPY_SRC to COPY_DST/MAP_READ buffer; row pitch 256 alignment; async mapping after submit; unsafe as_hal handle guard has device lifetime/locking constraints, not automatic interoperability. | https://github.com/gfx-rs/wgpu/blob/trunk/examples/features/src/render_to_texture/mod.rs ; https://github.com/gfx-rs/wgpu/blob/trunk/wgpu/src/api/texture.rs ; https://github.com/gfx-rs/wgpu/blob/trunk/examples/README.md |

## Candidate disposition
| Candidate | What it adds | Cost/risk | Recommendation |
|---|---|---|---|
| GPUI canvas + Lyon | Native interactive path drawing, fills, strokes, UI text, images; no extra GPU device | Paint-phase, tessellation/caching, advanced blends & pigment missing | Baseline |
| Kurbo | Bezier geometry, affine transforms, bounds, intersections/flattening algorithms | Conversion and numeric policy | Core candidate |
| Peniko | Shared paint, gradient, blend/color vocabulary | Keep stable own serialization | Core candidate |
| tiny-skia | Rust CPU raster paths, masks, compositing, pixmap | CPU work and uploads; not full Skia | First image-buffer proof/fallback |
| usvg/resvg | static SVG normalize and render | Does not implement scripts, dynamic SVG, interaction or preserve all semantics | Import/export/reference bridge |
| vello_cpu | CPU vector alternative | Profile on target, features vary by revision | Benchmark |
| vello_gpu | CPU path preprocessing, GPU raster/composite | wgpu/device/version mismatch; renderer is evolving | Preferred accelerated vector experiment |
| Vello research compute | GPU compute-centric large scene renderer | experimental; compute support, driver and shader costs | Benchmark only |
| wgpu/WGSL | general GPU render/compute, texture processing, filters, 3D | async dispatch, device loss, readback and interop | Optional compute foundation |
| rust-skia | mature graphics capabilities, broad GPU backends | C++ FFI/build weight, binaries, interop and compliance | Contingency if advanced feature gap justified |
| libmypaint | pressure/dynamics, established brush engine | C FFI, licensing/packaging audit; Rust alternative first | reference or bounded exception |
| Umber | Rust GPU painting, smudge and OpenRaster workflows | GPL review; study interfaces only, no copied code | prior art |
| Bevy | ECS, 3D lighting/cameras, render targets | substantial build and runtime cost; avoid its window/UI | later offscreen 3D experiment |
| WGPUI | GPUI-derived wgpu-first candidate; advertised offscreen surface handle | separate framework/fork, Rust 1.100 requirement reported, migration cost; not chosen | watch/compare only |

## Windows interoperability: explicit unresolved seam
Upstream GPUI main currently uses a DirectXRenderer on Windows; pinned crate may differ. The November upstream discussions about external GPU surfaces and custom GPU passes are proposals, not proof of merged support. Inspect pinned 0.2.2 backend and exact Windows adapter/LUID, D3D11/D3D12/Vulkan capabilities. Prefer a CPU buffer bridge into RenderImage first. Test offscreen Vello/wgpu plus bounded staging ring, then compare same-device compositor integration vs native shared resources (D3D11/D3D12 shared handle, fences, resource state, lifetime, format). HAL access is unsafe and requires exact ownership/synchronization proof; do not create a second swapchain or overlay HWND. A small upstream-compatible GPUI extension is preferable to a long-lived fork if warranted. WGPUI is a watch candidate only: do not replace GPUI or adopt incompatible Rust requirements without explicit decision.

Sources:
- https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/painting.rs
- https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/canvas.rs
- https://github.com/zed-industries/zed/blob/main/crates/gpui_windows/src/window.rs
- https://github.com/zed-industries/zed/discussions/64849
- https://github.com/zed-industries/zed/discussions/66371
- https://github.com/linebender/vello ; https://github.com/linebender/vello/releases
- https://github.com/linebender/kurbo ; https://github.com/linebender/peniko ; https://github.com/nical/lyon
- https://github.com/linebender/tiny-skia ; https://github.com/linebender/resvg
- https://github.com/gfx-rs/wgpu ; https://github.com/rust-skia/rust-skia
- https://github.com/mypaint/libmypaint ; https://github.com/mypaint/mypaint-brushes
- https://github.com/Spillebulle/umber ; https://github.com/bevyengine/bevy/blob/main/examples/ui/render_ui_to_texture.rs
- https://docs.rs/wgpui/latest/wgpui/

## Artistic intelligence research
1. Learning to Paint (ICCV 2019): learns reference-image decomposition into stroke positions/colors. Use as algorithmic inspiration for an optional image-to-strokes optimizer, not a guarantee of text-to-paint. https://github.com/hzwer/ICCV2019-LearningToPaint
2. SVGDreamer (CVPR 2024): text-guided vector synthesis via diffusion-guided optimization; complex, compute-intensive; compare path quality/editability. https://openaccess.thecvf.com/content/CVPR2024/html/Xing_SVGDreamer_Text_Guided_SVG_Generation_with_Diffusion_Model_CVPR_2024_paper.html
3. OmniSVG (NeurIPS 2025): multimodal SVG generation; upstream config includes 4B/8B variants, not a small guaranteed-local model. Treat as optional research; parse/sanitize outputs, no arbitrary code. https://github.com/OmniSVG/OmniSVG
4. MyPaint dynamics and Umber smudge/OpenRaster suggest pressure/velocity mappings, stamp textures, pickup and blending, tiled compositing. Independent Rust design first.
5. Natural media: dry brush noise and stamped texture first; wet pigment uses water, deposited pigment, paper height, diffusion/advection, evaporation; oil uses pigment load and impasto height. Spectral Kubelka–Munk mixing is a separate physically motivated experiment, not ordinary RGB alpha compositing. Reference implementation to study: https://github.com/lambentLogic/wet-edge (browser implementation, not a target runtime).

## License/build discipline
GPUI crate manifest declares Apache-2.0; tiny-skia upstream describes BSD license. Confirm exact versions/SPDX and all transitive dependencies, shader assets, sample images, brush presets, fonts and model weights separately. No GPL incorporation without explicit license analysis. Pin crate graph and minimal features; no large model downloads or duplicated GPU stacks by default. Source links are rolling observations; before experiments record commit hashes, Cargo.lock, adapter and driver. No performance or interoperability measured yet.
