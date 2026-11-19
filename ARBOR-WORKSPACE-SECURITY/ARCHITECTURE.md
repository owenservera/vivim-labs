# Workspace architecture

## Trust boundaries
Owner identity and devices; Host authority and Rust native primitives; private data; generated execution; interaction surfaces; external services; software integrity. Treat a compromised AI or generated program as an expected threat, not a trusted principal. Fully compromised OS/kernel/admin is outside the confidentiality guarantee.

## Logical zones
system: verified installed runtime; vault: encrypted secrets; knowledge: HQKG/Wiki/context; personal: documents; programs: installed versioned programs; execution: isolated workers; browser: dedicated authenticated profiles; inbox: untrusted imports; journal: existing receipts; recovery: encrypted backups; shared: explicit exports. Map incumbent stores rather than duplicate them. Separate installed runtime, owner data and ARBOR development. Folder names alone do not enforce isolation.

## Responsibilities
Host authorizes and owns effects; Rust implements enforceable OS handles, crypto and isolation; HQKG owns knowledge with authenticated provenance; Studio uses brokered browser authority; VLANG supplies typed meaning; VEC validates executable IR; TierFlow renders constrained UI; PWA supplies authenticated interaction. Keep direct parallel IO/event paths, but require enforcement at the actual resource boundary.

## Execution classes
T0 validated declarative UI; T1 deterministic typed transformations; T2 restricted Wasm or OS-contained workers; T3 fixed privileged Host capabilities. Evaluate AppContainer on Windows; prove isolation with real tests. Generated code must never execute in privileged Host by default.

## Identity, cryptography and data
Separate per-device keys and workspace data keys. Use OS-backed key wrapping (evaluate DPAPI/TPM), reviewed AEAD and recoverable offline ownership. Encrypt sensitive stores and sidecars, logs, backups and temp artifacts; protect plaintext at runtime. Enrollment uses authenticated TLS origin, single-use challenge, proof of possession, owner approval, bounded session, revocation. PWA storage guarantees vary; do not assume hardware-backed keys. Cached copies cannot reliably be erased remotely.

## Semantic information flow
PUBLIC, WORKSPACE, PRIVATE, SENSITIVE, SECRET are proposed labels; maintain ACLs and destination constraints in addition to labels. Derived outputs inherit restrictive inputs; downgrade requires authorized declassification. Track trust independently: public external content is not owner instruction. Context, summaries, graph nodes, indexes, prompts and UI preserve provenance and restrictions. Never send secrets to a model. Provider web-app operations remain brokered, with exact destination and data authorization.

## Owner UX
Security panel for identity, devices, permissions, vault, programs, connections, activity, recovery and emergency isolation. Trusted security UI cannot be overridden by generated programs. Observation, assist, operate and autonomous modes express bounded delegation; not a replacement for resource authorization.
