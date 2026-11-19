# Implementation lanes

All lanes are proposed, not completed. Recheck live source; each lane owns a concrete change, negative and positive tests, and migration/recovery where applicable. Waves are dependency guidance, not organizational gates.

| ID | Scope | Owner | Work order |
|---|---|---|---|
| S01 | Workspace provisioning and isolation | Rust / Host | Map existing roots into system, private data, development and execution zones; provision OS identities and ACLs; migrate without resetting data. |
| S02 | Owner identity and root trust | Rust / Host | Define owner recovery and local authentication; isolate root credentials from worker identities; use platform-backed key protection. |
| S03 | Device authentication and revocation | Host / PWA | Unify enrollment, proof of possession, TLS origin trust, scoped sessions, atomic redemption and revocation. |
| S04 | Cryptographic key management | Rust | Use reviewed AEAD and key wrapping; version keys; protect with OS facilities; separate device and data keys; test recovery and rotation. |
| S05 | Encrypted workspace storage | Rust / HQKG | Map existing stores, WAL, indexes, temporary data and backups; migrate atomically and verify recovery. |
| S06 | Canonical capability contracts | Host / Rust | Extend existing grants to bind principal, workspace, resource, action, program version, destination, expiry and delegation. |
| S07 | Native authorization enforcement | Rust / Host | Authenticate caller at native boundary; evaluate authoritative grants and revocation; bind exact effect and revision; no alternate effect writer. |
| S08 | Filesystem security | Rust | Broker handles and resource identities; verify containment against traversal, reparse points, hard links and races; isolate worker access. |
| S09 | Process sandboxing | Rust | Separate privileged and worker identities; assess Windows AppContainer and restricted Wasm; deny inherited credentials and uncontrolled subprocesses; bound resources. |
| S10 | Network and egress control | Rust / Host | Broker destinations and data classes; enforce redirects, address resolution and upload scope; deny unapproved direct network access. |
| S11 | Browser profile and session security | Studio / Host | Preserve explicit signed-in profile; isolate browser control transport; constrain navigation, clipboard, upload, submit and download capabilities. |
| S12 | VLANG semantic authority | VLANG / Host | Keep author, source, trust and authorization separate; typed meaning is not permission; validate commands through existing planner. |
| S13 | VEC executable program security | VEC / Host | Version and digest executable IR; validate effects, resource budgets and capabilities; never load arbitrary generated code in privileged process. |
| S14 | Generative UI containment | TierFlow / PWA | Separate trusted approval surfaces from programmable UI; escape untrusted content; constrain renderer capabilities and authenticated transport. |
| S15 | Knowledge security and provenance | HQKG / AutoWiki | Bind authenticated ingestion to workspace, source revision and labels; enforce scoped retrieval, invalidation and derived-data restrictions. |
| S16 | Prompt injection and memory poisoning | Semantic Fabric | Mark untrusted content independently of confidentiality; prevent instruction promotion; preserve provenance and restrict provider context exports. |
| S17 | Secure program distribution | Host / VEC | Verify publisher and content digest, review permission deltas, isolate installs, reject rollback and preserve revocation. |
| S18 | Transactions, audit and recovery | Host / Rust | Reuse effect fences and journal; record redacted receipts; reconcile uncertain effects; verify encrypted backup restoration. |
| S19 | Software supply-chain integrity | ARBOR | Pin dependencies and artifact identities; verify releases and updates; preserve direct-main work with non-destructive integrity controls. |
| S20 | Adversarial security verification | Cross-system | Inventory all real effect paths; run negative and concurrency tests against actual runtime; record PASS/FAIL/UNRUN, not inferred results. |

Wave 1: S01–S08. Wave 2: S09–S11. Wave 3: S12–S16. Wave 4: S17–S20. S20 begins with topology inventory immediately and continues throughout.
