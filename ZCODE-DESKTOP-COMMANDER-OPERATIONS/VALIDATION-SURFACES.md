# Native-first validation — GPUI / PWA (owner directive, 2026-11-21)

- **Android is NOT required** to build, run, test or accept the Rust Core, live composer, or AutoWiki loop. Do not block on Android SDK, emulator, ADB or physical-phone access.
- **GPUI is the preferred native desktop validation surface** where a GPUI view/bridge is implemented. Verify the actual GPUI integration before claiming it works; do not assume GPUI can render PWA HTML.
- **One authoritative Rust Core:** commands, events, revisions, live state and AutoWiki logic live in Rust. GPUI and PWA are separate surfaces over the same verified state, not duplicate implementations.
- **Acceptance A:** human types in a connected composer -> core receives the input/state -> AutoWiki reflects it live in a visible surface.
- **Acceptance B:** authorized agent types into that composer -> core receives the input/state -> AutoWiki reflects it live; show verified activity feedback, not simulated taps or fabricated acknowledgments.
- **Verification order:** Rust unit/integration tests -> GPUI native observation if wired -> PWA browser/Host round-trip. Real Android testing is optional, not a core gate.
- **Evidence discipline:** report GPUI, Rust, browser, and phone checks separately as PASS / FAIL / UNRUN; never label browser simulation as native GPUI or physical device proof.
- **Engineering authority:** ChatGPT codes cloud-first and pushes to GitHub main; ZCode/Desktop Commander are limited to explicitly scoped local execution and verification, per CLOUD-FIRST-CODING-POLICY.md.
