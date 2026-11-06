# Performance — Research Notes

**2026-11-06:** High-level scaffold created. No performance benchmarks or production changes have been made.

Initial documentation consulted through Context7:
- UniFFI `/mozilla/uniffi-rs/v0.32.2`: Rust-to-Kotlin bindings are supported.
- Android Compose `/websites/developer_android_develop_ui_compose`: native Kotlin UI direction; exact library version not surfaced.
- Tauri `/tauri-apps/tauri/tauri-v2.0.0`: Android/Rust/WebView architecture exists; latest-version compatibility not yet checked.

**Working hypothesis, not a decision:** portable Rust core + Kotlin Android integration looks promising; native Compose vs TypeScript/WebView vs Tauri needs further comparison.

When there is real evidence, append concise notes with date, machine/build, what was compared, result and link to raw measurements. No need for a heavyweight evidence template now.
