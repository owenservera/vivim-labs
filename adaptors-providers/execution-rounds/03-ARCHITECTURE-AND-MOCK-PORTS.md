# Proposed ARBOR internal architecture: one backend, injected mocks

**Not existing API claims.** W0 must reconcile this shape against source-owned ARBOR before code. Prefer existing ARBOR code when behavior and ownership match.

~~~text
Existing Studio/PWA/natural-language UI
    ↓ typed, inert read or action intent
ARBOR apps/vivim-host/provider-hub (one shared service)
    ├─ registry + feature/capability descriptors (10 packs)
    ├─ SourcePort → fake in mock; existing signed-in Studio Chrome owner in future live mode
    ├─ WorldPort → fake contract in mock; migrated ARBOR Host World later
    ├─ ActionPort → fake Host authorization/receipt in mock; existing Host dispatcher in live mode
    ├─ GraphPort → fake proposal/revision checks in mock; internal ARBOR native/hqkg behind Host
    └─ UiPort → read-only current PWA/TierFlow renderer/publisher; never a second writer
~~~

**Default proposed source homes:** contracts/provider-hub/v1; apps/vivim-host/provider-hub/{core,identity,streams,projections,ports,mock,packs,experiences}; studio/src/studio/provider-hub-ui; apps/mobile-relay/provider-hub; tests/provider-hub. G adjusts based on actual W0 mapping, never by inventing another repository.

## Core proposed operations
- ProviderHub.listProviders(context) -> capabilities with mock/unsupported/live verification state.
- ProviderHub.listStreams(context, kind, cursor, limit) -> bounded source-linked StreamPage.
- ProviderHub.readThread(context, threadRef, cursor) -> account-scoped message history and gaps.
- ProviderHub.readUnit(context, unitRef) -> host-controlled content pointer and evidence.
- ProviderHub.search(context, query) -> permitted read projection, never a raw global scrape.
- ProviderHub.proposeAction(context, proposal) -> inert Host typed plan bound to exact provider/account/thread/draft and revision.
- ProviderHub.actionStatus(context, proposalRef) -> only the existing Host receipt projection; no infer-success-from-UI.

Hub **must not** expose direct send(), providerFetch(), world.commit(), graph.write(), or grantPermission(). Model generated content never grants authority. Studio existing read-only learned adapter cannot enter a send path. Real send remains an existing Host dispatcher action + manually verified source-owned Studio browser adapter, not automatic promotion of learned selectors.

## Canonical concepts
- ProviderDescriptor: providerId, productId, version, modes, streamKinds, capabilities, maturity, evidenceRef.
- AccountBinding: principal/world scope, opaque accountRef, provider origin, verified identity evidence, observedAt, accountEpoch and freshness.
- StreamEnvelope: sourceRef, qualified unitId, parent/thread, provenance digest, occurredAt vs observedAt, cursor, sensitivity, coverage and source revision.
- ActionIntent: exact target/recipient/thread, private draftRef/hash, attachment digests, approval policy, expected source revision, idempotency.
- ReceiptProjection: proposed/planned/approved/claimed/uncertain/applied/failed; provider delivery/read markers only if independently supported by evidence.
- Mock markers: synthetic:// identifiers, MOCK_ONLY mode, MOCK_RECEIPT, seeded clock and controlled error scripts; impossible to confuse mock with live.

## World migration assumption
ARBOR Host owns personal World, grants, journal, evidence and eventual durable state. Source in ARBOR exists, but the precise World migration completeness remains to be verified by W0. Implement a fake WorldPort with these *interfaces* and behavior, not a persistent competing World. No runtime connection to vivim-web-harness/repo-hq/tierflow donor checkouts. No new Rust graph store; native/hqkg scoped_graph structurally checks records while Host grants/identity remain authoritative.

## Mock fidelity
All fake observations must travel via the same public ProviderHub service and ARBOR source-owned read/projection handlers consumed by Studio and PWA. Do not let UI import fixture JSON directly. For every user journey, swap only injected fake Source/World/Action/Graph ports. Disable any implicit fallback to mock when live binding fails. Cross-owner isolation, wrong recipient, account swap, expired grant, E2EE partial, offline, corrupt export and uncertain-send must be deterministic test cases.
