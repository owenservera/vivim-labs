# Research decision checkpoint — do not delegate yet

Status: evidence gathering, no implementation selection.

## Preliminary finding
Mobile extension support is a candidate **browser observation and interaction adapter**, not a substitute for (1) Android native API permission handling, (2) long-running Host/effect journaling and single-owner authorization, (3) arbitrary OS access, or (4) the current canonical World/KB writer. A controlled native browser engine and extension-capable external browser must be compared on identical requirements.

## Candidate paths
1. Extension-capable external browser + VIVIM APK native bridge: smallest experiment, but browser vendor/extension API control and lifecycle uncertainties.
2. VIVIM-owned embedded browser (WebView/GeckoView) + explicit instrumented pages: more direct app control but different login/browser feature guarantees, WebView is not extension-equivalent.
3. Hybrid phone + PC Host: existing authority and browser parity on PC, richer local Android captures/native features; dependable near-term fallback.
4. Developer-only ADB/CDP: useful experimental upper bound, **not** mainstream product path.

## Mandatory go/no-go metrics
Observed DOM fidelity on provider mobile apps; permitted interaction coverage; multi-provider sessions/tab support; safe sends/uncertain receipt behavior; resilience after screen-off/background/restart; app trust/update provenance; APK integration work; installability; graceful degraded states. Require reproducible device results.

No implication that existing VIVIM effect ownership has changed. Android-only autonomy may be evaluated as a future proposal, not quietly introduced.
