# sha2 — choice intelligence

**Status:** SOURCE-OBSERVED / RESEARCH-OPEN, not an approved optimization or latest-version verdict.  
**ARBOR pin:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` (2026-11-12).

## Grounding
- **Declared / observed:** 0.10
- **Role in VIVIM:** Hashing source facts and pure contract digests
- **Evidence:** https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/core/programmability/Cargo.toml
- **Upstream releases/docs:** https://github.com/RustCrypto/hashes/releases

## Why it may be the right fit
Content-addressed evidence without introducing additional database engine.

## Why this, rather than an alternative?
Hashes establish integrity, not origin authentication or authorization.

## Features worth mapping
1. Product-specific capability and interaction boundary.
2. Security/privacy and ownership constraints.
3. Dependency size, reliability, operating cost and developer/CI friction.
4. Source pin, resolved version (once locked), transitive and reverse dependencies.

## Update radar
- RustCrypto releases
- hash interoperability

## Evidence required before adoption or replacement
Record upstream release URL and published date, breaking changes, security advisories, alternatives considered, reproducible benchmark/fixture, source-owner decision, feature flags, rollback and user/device acceptance where applicable.

**Last verified upstream latest:** UNKNOWN — not checked.  
**Open research:** exact resolved version; actual usage path; alternative benchmarks; release/security impact.  
**Decision:** KEEP_CURRENT_DECLARATION_PENDING_EVIDENCE. Never auto-upgrade from this page.
