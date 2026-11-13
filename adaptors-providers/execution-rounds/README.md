# Unified Provider Hub — ChatGPT execution rounds R002
**2026-11-13 · DESIGN / EXECUTION WORK ORDERS · NOT IMPLEMENTED BY THESE DOCUMENTS**

**Product target:** one ARBOR-native provider backend for ten social/messaging/media/email services. Ten fully mock-exercisable provider packs, bound to the same ARBOR Host, Studio, Rust-scoped graph and PWA ports intended for future live integration. This is not a second runtime in Labs.

**First read:** [Master launch prompt](00-MASTER-EXECUTE.md), [architecture](03-ARCHITECTURE-AND-MOCK-PORTS.md), [top ten](02-TOP10-MATRIX.md), [lane index](prompts/INDEX.md), [machine manifest](WAVE-MANIFEST.json).

## Required outcome
- 7 waves W0–W6, each with 6 parallel ChatGPT lane workers (A–F) and one serial integrator (G). Total 42 worker prompts + 7 gates + 7 wave launchers.
- Mock mode must pass through the actual ARBOR source-owned Hub API and Host/PWA read/action routes; only the provider SourcePort, future ARBOR WorldPort, action receipts and graph execution are faked via dependency injection.
- Build against ARBOR-internal source, not separate donor servers or Labs imports. World is assumed to converge into ARBOR but must not be claimed migrated if source/testing does not prove it.
- All ten have a provider descriptor, synthetic source records, normalized read stream, UI preview and explicit unsupported/live-unverified states. Messaging/action-capable products must also demonstrate proposal, trusted review, fake Host approval and unambiguous mock receipt.
- No provider APIs, Playwright/Puppeteer, credential collection, second World, second Host dispatcher, second phone app, second graph store, silent send, or automatic retry on uncertain action.
- Every real account/session/provider-browser interaction is reserved for a later approval and live acceptance program.

## Files
- [Waves and dependencies](01-WAVE-MATRIX.md)
- [Top-ten matrix](02-TOP10-MATRIX.md) / [machine catalog](top10.providers.json)
- [Backend contract and injected ports](03-ARCHITECTURE-AND-MOCK-PORTS.md)
- [Parallel ownership and merge policy](04-EXECUTION-PROTOCOL.md)
- [Acceptance and negative tests](05-ACCEPTANCE.md)
- [Source pins](SOURCE-PINS.json)
- [Prompt index](prompts/INDEX.md)

**Evidence context:** ARBOR Proposal 5 explicitly requires internal consolidation, Host as sole World/effect authority, existing Studio Chrome actor, and one paired PWA. Existing AI-provider registry in harness/src/providers is NOT a generic social channel. Studio learned adapters are data-only read adapters, never trusted send adapters.

**Platform caution:** Messenger desktop web path is facebook.com/messages after standalone Messenger website retirement in April 2026: https://techcrunch.com/2026/02/19/meta-is-shutting-down-messengers-standalone-website/ . WhatsApp linked devices: https://www.whatsapp.com/download/desktop . Meta export management: https://about.fb.com/news/2023/10/manage-your-information-across-apps/ . None of those references verifies an ARBOR connection or authorizes automation.
