# VIVIM Android Self-Development Wave R001

Mission: one integrated native Android development node capable of real self-testing, authorized install/update, evidence readback, and SDK Factory-assisted improvement. Baseline Labs main 467f9b8e03c3a084bd19d369a7cd0ceb8ed3ee70. Current actual evidence: v0.1 owned display rendered changing frames; v0.8 APK built and signed; later startup code compiled Java/Dex and Rust 5/5, not installed; exact latest installed version must be re-read. Concurrent download work exists; preserve it. Android MCP runs as Termux UID, not APK UID; no assumption of privileged control. No production migration or new architecture in Labs without Product integration ownership.

## Lanes
- **L00-CORE** — Core, convergence and release ownership. CONTRACT.md, integration manifest, actual integrated source, pinned build/release evidence, final consolidated APK and honest native acceptance report.
- **L01-BRIDGE** — Native APK–Termux–MCP bridge. Bridge implementation and protocol contract, actual native transport and denial tests, session/receipt evidence, integration patch.
- **L02-TEST** — Real live Android self-testing. Owned test catalogue and executable runner, live receipts, native tests and results, integration handoff.
- **L03-UPGRADE** — Native installation, self-upgrade and recovery. Installer implementation, release journal, negative candidate tests and physical update evidence or exact OS blocker, recovery design and handoff.
- **L04-FACTORY** — SDK Factory, release automation and optimization. Executable Factory adapter, measured baseline and repeat run, release and diagnostic evidence mapped to existing SDK, integration handoff.

## Coordination
L00 owns shared MainActivity, manifest/build/release changes and common contract; other lanes own dedicated modules and submit integration handoffs on main. Verify fresh source before modifications. L01 owns scoped diagnostics transport; L02 test runner; L03 updater; L04 Factory adapter. Use separate native staging directories and never race a signer or installed package. L00 coordinates physical acceptance and a single consolidated owner installation, where required. No background autonomous agent promise. Three rounds within available sessions: implement, integrate, harden. Record blockers instead of claiming unobserved effects.

## Governing sources
- ../SELF-DEVELOPMENT-LOOP.md
- ../DEVELOPER-INSTRUMENT.md
- ../R010-SELF-TEST-STARTUP.md
- ../R009-DEVELOPER-INSTRUMENT.md
- ../R008-THREE-ROUND-BUILD.md
- ../prototype/
- ../../PARITY-100/PLAN.md
- https://github.com/owenservera/vivim-product
- https://github.com/owenservera/ARBOR

## Acceptance
Actual installed package/source/signer/digest identity, automatic owned-app tests, supported authorized update, post-update tests and receipts linked to existing Factory. Record any unsupported installer/IPC operation, no synthetic pass. Preserve owner control, stop/revoke, private data, and existing product features. [Launch prompts](LAUNCH-PROMPTS.md).
