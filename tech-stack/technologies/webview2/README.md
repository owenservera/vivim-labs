# Windows WebView2 and windows-rs — choice intelligence

**Status:** SOURCE-OBSERVED / RESEARCH-OPEN, not an approved optimization or latest-version verdict.  
**ARBOR pin:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` (2026-11-12).

## Grounding
- **Declared / observed:** webview2-com-sys =0.33.0; windows/windows-core =0.58.0
- **Role in VIVIM:** Windows system webview and native interop
- **Evidence:** https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/src-tauri/Cargo.toml
- **Upstream releases/docs:** https://learn.microsoft.com/en-us/microsoft-edge/webview2/release-notes/

## Why it may be the right fit
Avoid shipping a separate embedded Chromium engine where OS webview suffices.

## Why this, rather than an alternative?
Evaluate WebView2 runtime compatibility and OS packaging matrix.

## Features worth mapping
1. Product-specific capability and interaction boundary.
2. Security/privacy and ownership constraints.
3. Dependency size, reliability, operating cost and developer/CI friction.
4. Source pin, resolved version (once locked), transitive and reverse dependencies.

## Update radar
- WebView2 runtime releases
- windows-rs releases

## Evidence required before adoption or replacement
Record upstream release URL and published date, breaking changes, security advisories, alternatives considered, reproducible benchmark/fixture, source-owner decision, feature flags, rollback and user/device acceptance where applicable.

**Last verified upstream latest:** UNKNOWN — not checked.  
**Open research:** exact resolved version; actual usage path; alternative benchmarks; release/security impact.  
**Decision:** KEEP_CURRENT_DECLARATION_PENDING_EVIDENCE. Never auto-upgrade from this page.
