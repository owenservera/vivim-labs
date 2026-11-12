# Chrome DevTools Protocol / signed-in browser — choice intelligence

**Status:** SOURCE-OBSERVED / RESEARCH-OPEN, not an approved optimization or latest-version verdict.  
**ARBOR pin:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` (2026-11-12).

## Grounding
- **Declared / observed:** Browser version unpinned; in-repo Harness/Studio
- **Role in VIVIM:** Read-only capture and signed-in browser integration
- **Evidence:** https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/studio/package.json
- **Upstream releases/docs:** https://chromedevtools.github.io/devtools-protocol/

## Why it may be the right fit
Works through user-controlled browser web sessions rather than provider APIs.

## Why this, rather than an alternative?
Check browser update churn, consent boundaries and profile isolation.

## Features worth mapping
1. Product-specific capability and interaction boundary.
2. Security/privacy and ownership constraints.
3. Dependency size, reliability, operating cost and developer/CI friction.
4. Source pin, resolved version (once locked), transitive and reverse dependencies.

## Update radar
- Chrome stable changes
- CDP compatibility

## Evidence required before adoption or replacement
Record upstream release URL and published date, breaking changes, security advisories, alternatives considered, reproducible benchmark/fixture, source-owner decision, feature flags, rollback and user/device acceptance where applicable.

**Last verified upstream latest:** UNKNOWN — not checked.  
**Open research:** exact resolved version; actual usage path; alternative benchmarks; release/security impact.  
**Decision:** KEEP_CURRENT_DECLARATION_PENDING_EVIDENCE. Never auto-upgrade from this page.
