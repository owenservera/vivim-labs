# Node.js — choice intelligence

**Status:** SOURCE-OBSERVED / RESEARCH-OPEN, not an approved optimization or latest-version verdict.  
**ARBOR pin:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` (2026-11-12).

## Grounding
- **Declared / observed:** 24 minimum at root; >=22 in Host/Harness/Studio packages
- **Role in VIVIM:** Host, Studio, Harness and HQ processes
- **Evidence:** https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/package.json
- **Upstream releases/docs:** https://nodejs.org/en/about/previous-releases

## Why it may be the right fit
Node 24 unification simplifies use of built-in node:sqlite and avoids multiple toolchain assumptions.

## Why this, rather than an alternative?
Older Node may remain valid for certain local modules, but release should converge on tested runtime.

## Features worth mapping
1. Product-specific capability and interaction boundary.
2. Security/privacy and ownership constraints.
3. Dependency size, reliability, operating cost and developer/CI friction.
4. Source pin, resolved version (once locked), transitive and reverse dependencies.

## Update radar
- Node LTS lifecycle
- npm package and API changes

## Evidence required before adoption or replacement
Record upstream release URL and published date, breaking changes, security advisories, alternatives considered, reproducible benchmark/fixture, source-owner decision, feature flags, rollback and user/device acceptance where applicable.

**Last verified upstream latest:** UNKNOWN — not checked.  
**Open research:** exact resolved version; actual usage path; alternative benchmarks; release/security impact.  
**Decision:** KEEP_CURRENT_DECLARATION_PENDING_EVIDENCE. Never auto-upgrade from this page.
