# ARBOR Proposal 5 — Labs Readiness Owner Handoffs and Promotion Gates

**This is an implementation handoff**, not permission to merge any Labs code into W1 A–F branches or interrupt ongoing workers. Read [the 44-component matrix](PLACEMENT-MATRIX.md) and [the machine record](SOURCE-TO-ARBOR.json).

## A: Rust / Tauri / HQKG

**Ready to inspect while W1 runs:** C03 Node 0 Prime design, C04 Chronos Rust stage, C09 live graph provenance samples.

**After W1-G source-lock:** confirm whether HQKG Rust importer/engine and Harness programmability source already include equivalent functionality. Import Chronos only if it uses the one HQKG engine and passes Cargo tests; don't add an independent native clock daemon. For Node 0 preserve actor/session source scope, bounded pure fan-out, stale CAS and no effects. The R001 native spike is *not* proof of a compiled shipping runtime.

**Gate:** A-owned source/code/test SHA, no second graph engine, no per-keystroke thread storm, no native effect writer; Cargo compile and fuzz/IME tests are separately required.

## B: Host / World / Personal Knowledge

**Now:** C01 ZIP/stream/chat normalization and host-ledger adapter source exists with synthetic tests in Labs. It is not yet wired to real Host evidence/World writers. Review Host onboarding/service, bootstrap ledger and current World claims before importing any code.

**After W2-G:** land the narrowest internally imported intake adapter only if current Host implements the trusted evidence/receipt, idempotency, consent/revoke and graph epoch ports. Otherwise retain source/tests as a non-shipping candidate. Do not create a new personal DB, background scraper or unapproved data importer.

**Voice:** C08 workbench tests can graduate to F now; B can only mark an operational task write when an actual Host task admission and receipt are verified. A synthetic task proposal is not a task store.

**Gate:** consent, archive bounds, path traversal/ZIP bombs, checkpoint/recovery, Host native port identity, data erasure, owner-controlled source scope and real receipts.

## C: Studio and signed-in provider

**Now:** review Chronos pure read projection C06 tests and current Studio conversation owner. Pure projection is optional and cannot alter send attempts or receipts.

**After W2-G:** selective in-repo source transfer only if it matches exact Studio result types, tests and current authorized provider session; no outside Lab runtime.

**Gate:** no provider APIs, no Playwright/Puppeteer product automation, no change to exact-thread/send-once/uncertain receipt logic.

## D: Internal HQ / Development Knowledge

**Now:** E07 source-specific wiki-oracle tests and C05 Chronos supervisor read lens; both need exact match to the imported HQ JS source tree. Do not wholesale import the Architecture archive.

**After W1-G:** run internal HQ index/search/context/wiki on source-pinned fixture; promote only pure tests/adapter functions accepted by the current source owner. Developer wiki and personal World are separate source domains.

**Gate:** source SHA/index freshness, no personal data indexing by default, deterministic output, bounded context, no dependency on external HQ checkout or CLI.

## E: PWA / TierFlow / Visual

**Now:** E06 R005 pure semantic fixtures and C07 Chronos view tests are low-cost candidates. R005 has Labs prototype code and synthetic tests, but no real device/browser visual acceptance. R007 full hypergraph remains a later project.

**After W2-G:** render only source-backed evidence from current Native/Host; keep local editing/IME intact and preserve existing PWA. Visual affordances never author effect authority. If TierFlow already offers equivalent semantics, import **tests only**, not duplicate renderer modules.

**Gate:** one PWA, one publisher, revision-guard parity, no fake source/permission readiness, physical phone check later.

## F: Contracts / Observatory / Independent QA

**Now:** E01/E02 Observatory schema, fail-closed status classification and redacted snapshots; E03 verified extension hook map; E04 voice pure test pack; E05 Chronos schema; E06 R005 semantics and E07 selected HQ wiki fixtures.

**After W1-G:** compare with current W1-F contract inventory; promote tests/schemas only once, keep the action catalog Host-owned. Update capability-space derived metadata with actual status: DOCUMENTED → SOURCE_PRESENT → BUILD_VERIFIED → RUNTIME_WIRED → DEVICE_ACCEPTED → AVAILABLE_NOW.

**Gate:** source paths/commits and source status separate from execution/production. Unknown is not zero, recovered private material never copied into product.

## G: Integration Owner — the only promotion authority

For each proposed import, require:
- exactly pinned Labs branch SHA, file path and reason it is necessary for the beta;
- destination owner and explicit confirmation it wasn't already imported by W1;
- code/fixture dependency closure and source/license/privacy review;
- diff bounded to the owner package, one action catalog/effect writer/graph engine/UI publisher;
- actual executable tests with exit code, not recorded historical claims;
- confirmation no runtime/build wiring to a Labs checkout or process;
- Beta gate mapping and rollback/defer option.

### Sequencing matrix

| Item | May research while A–F run? | May edit active W1 owners? | Earliest product merge |
| --- | --- | --- | --- |
| Observatory tests/status | Yes | **No** | W1-G decision after F comparison |
| R005 semantic tests | Yes | No | W2-G after TierFlow/UI compatibility |
| Voice pure tests | Yes | No | W1-G / W2-G F acceptance |
| Chronos schema | Yes | No | W2-G after graph + Host contract |
| Chronos Rust/JS integration | Yes, code review | No | W3 or later, subject to beta priority |
| T=0 importer | Yes, source audit | No | W3 only if Host/Rust verified |
| Node 0 correctness | Yes, negative fixtures | No | Existing A/F W2/W3 owners |
| AutoWiki/advanced prediction | Yes, research | No | Post-beta, unless isolated low-cost read-only |
| General app builder / automation Queen | Yes, research | No | Post-beta with authority/security design |
| Archive/agentic corpora/PC model routers | Metadata review only | No | Excluded unless separately approved |

**Bottom line:** this readiness round is a parallel intake review, not a stealth second migration lane. Only serial G changes the integrated ARBOR product.
