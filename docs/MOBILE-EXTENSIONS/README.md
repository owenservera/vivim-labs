# Canonical home: VIVIM Labs / Mobile Extensions

**All Mobile Extensions research, engineering fixtures and test evidence live in `owenservera/vivim-labs`.** Repos HQ and Harness are read-only input authorities, not a research publication destination. No Wave 4 implementation approved.

# VIVIM LAB — Mobile Extensions

Status: research scaffold (2026-11-06). Owner: VIVIM architecture / mobile runtime research.

## Question
How much of VIVIM desktop Host's *browser-facing* functionality can be performed on Android using an extension-capable browser, and where are native Android or remote Host capabilities indispensable?

## Guardrails
- Research only: no changes to live Host, paired phone runtime, auth, or effect ownership.
- DEC-MOBILE-001 remains authoritative: `vivim-web-harness/apps/mobile-relay` is the single phone UI; PC Host remains the only canonical World writer and provider effect path. A standalone mobile experiment must not bypass that invariant.
- Local mobile experiments may generate synthetic/read-only evidence. Actual provider sends and authoritative writes still require existing Host-mediated consent, approval and receipt contracts. A mobile-only autonomous send would require a separately approved architecture change.
- Do not capture tokens, cookies, private conversations or raw authentication secrets. Do not publish private browsing evidence.
- No Playwright/Puppeteer. Avoid unsupported browser automation.
- AI implementation cost is not a selection variable.

## Research tracks
1. Android browser support: Firefox, Quetta, Edge stable/Canary, Yandex, Vivaldi, Lemur, Mises, Helium; Chrome as negative control; Kiwi archived.
2. WebExtension APIs: content scripts, isolated execution, background lifecycle, tabs/navigation, messaging, storage, permissions, debugger/CDP exposure, installation/distribution.
3. Host-to-phone parity: site observation, scripted page interactions, multi-tab orchestration, signed-in webapp mediation, capture, approvals/receipts, local files and OS integrations.
4. Android bridges: APK/foreground service, Android share intents, Accessibility (explicit user enablement), notification listener, document picker, VPN service, companion browser protocol, WebView constraints.
5. Security/reliability: browser provenance, updates, sandbox, website isolation, extension permissions, process death, battery optimization, Android versions.

## Deliverables
- `BROWSER-MATRIX.md`: shortlist and source confidence.
- `PARITY-MATRIX.md`: feature classification and proposed test.
- `EXPERIMENTS.md`: first controlled test plan.
- `evidence/`: real device/build version, result, repro command or manual steps, date, artifacts and negative cases (add after execution).

## Decision outputs
- Primary supported Android browser for experiments
- Secondary independent browser implementation
- Minimal cross-browser VIVIM extension adapter
- Native APK bridge boundary and PC Host fallback boundary
- Explicit list of impossible or unsupported features

## Relationship to canonical work
Read `docs/architecture/MOBILE-ONE-AUTHORITATIVE-PATH.md` first. This lab is additive research; it does not create a second mobile client or second authorized provider send path.

## Research index (2026-11-06)
- [Host capability dependency graph](HOST-CAPABILITY-DEPENDENCIES.md) — source-anchored critical path and OS/architecture block classification.
- [Recovered Android native research](ANDROID-NATIVE-RESEARCH.md) — APK07 history, privilege tiers, platform limits, superseded sections.
- [Decision checkpoint](RESEARCH-DECISION.md) — alternatives and device acceptance criteria; no delegation yet.

Research order: desktop Host required primitives -> Android native/browser alternatives -> test evidence -> architecture selection. Do not ask MiniMax to execute until the source inventory and platform comparison are concluded.

## Wave 4 program build scaffold (2026-11-06)
- [Six-wave alignment and four lab-only lanes](WAVE-PROGRAM-ALIGNMENT.md)
- [Source-inspected browser/Android/Host primitive census](SOURCE-PRIMITIVES.md)
- [Browser engine candidates and GeckoView tradeoffs](BROWSER-ENGINE-DECISION.md)
- [Lane work, build gates and test plan](LANES-GATES-TESTPLAN.md)
- [Machine-readable **research-only** dependency fixture](fixtures/primitive-requirements.v0.json)
- [Pure Node structural tests](tests/primitive-graph.test.mjs), execute from repository root with `node --test docs/MOBILE-EXTENSIONS/tests/primitive-graph.test.mjs`

**HOLD:** No TierFlow W4 or Harness/native implementation before published I3 GO and explicit path ownership. No MiniMax delegation. Current evidence is source inspection plus isolated fixture validation, not provider, APK or physical device parity. The dedicated vivim-labs engineering factory may link to this research without cloning its authority.
