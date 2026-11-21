# VEC / paired PWA — first implementation program

**Branch:** `work/p5-vec-pwa-ui-kernel-r001` from ARBOR main `9b79d732da8b92713d399040db4a02d505bc7cf1`.
**State:** `SOURCE_IMPLEMENTED_TESTS_AUTHORED_EXECUTION_UNRUN`; not merged, not physically tested, not release GO.

VEC is ONE eventual compiler utility with three **distinct products**:
1. **Wiki compiler**: source comments/semantic data → deterministic Wiki program/projection, eventually absorbing existing HQ compiler code.
2. **Command/Natural Language compiler**: user intent → validated *candidate* command or multi-command plan; Host is still sole effect/grant/journal owner.
3. **UI programming compiler**: user intent → *revision-bound UI change proposal*; user previews, then existing paired Host publishes if authorized.

Do not blur #2 and #3. Sending a prompt to run a capability is not the same as rewriting the PWA interface. A VEC compiler result is never proof of approval/execution.

## Directories
- [Verified source inventory](00-SOURCE-TRUTH.md)
- [One compiler contract](01-COMPILER-CONTRACT.md)
- [Six workstreams with staged takeover](02-WORKSTREAMS.md)
- [Paired PWA test and release gates](03-PAIRED-PWA-ACCEPTANCE.md)
- [Ready-to-execute next work orders](04-IMPLEMENTATION-WORK-ORDERS.md)
- [Machine workstreams](machine/workstreams.json) and [source pins](machine/source-pins.json)
- [D0 source proof and CI truth](D0-STATIC-PROOF-AND-CI-STATUS.md)
- [UI capability taxonomy backbone](ui-taxonomy/README.md): 9 domains, 34 families, 417 registered component kinds, traits/contracts and cross-schema adapters.

## Initial source slice already present on branch
- `apps/mobile-relay/vec/compiler.mjs`: **one pure front-end/domain contract**, with `wiki`, `command`, `ui` product kinds. Only UI is installed as an adapter today; HQ and Host command compilers are NOT displaced.
- `apps/mobile-relay/vec/ui-adapter.mjs`: deterministic, bounded grammar for add/rename/remove/move UI components → existing `vivim.ui-proposal/1` compiler.
- `apps/mobile-relay/build-composer.mjs`: `resolveBuildRequest` pure integration.
- `apps/mobile-relay/build-canvas.mjs`: deterministic first, unsupported to existing model proposal path, Activate unchanged.
- `apps/mobile-relay/sw.js`, `apps/vivim-host/direct-phone.mjs`: new scripts added to approved **static read-only shell** only; no API or write path.
- `apps/mobile-relay/test/vec-ui-compiler.test.mjs`: synthetic negative/security/parity tests authored, UNRUN.

## Architectural transition rule
This browser-safe VEC compiler contract is the first *deployment seam*, not a decision that the final shared compiler permanently lives in the phone app. Promote to one internal cross-runtime source package only after the shared code distribution/Host/Node24/PWA static-asset contract is proved. HQ and command migration must be dual-read/shadow-tested before switching their canonical compilers. No second runtime action catalog, Wiki writer, clock, graph, UI publisher or program store.

The separate VLang research is in [draft PR #22](https://github.com/owenservera/ARBOR/pull/22); it is not an accepted implementation gate. This branch is independently based on current `main` and does not merge unaccepted SCF/SEVP/HVI source.