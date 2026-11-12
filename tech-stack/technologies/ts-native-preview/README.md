# TypeScript native preview — choice intelligence

**Status:** SOURCE-OBSERVED / RESEARCH-OPEN, not an approved optimization or latest-version verdict.  
**ARBOR pin:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` (2026-11-12).

## Grounding
- **Declared / observed:** ^7.0.0-dev.20260707.2
- **Role in VIVIM:** Experimental typecheck path
- **Evidence:** https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/packages/tierflow/package.json
- **Upstream releases/docs:** https://github.com/microsoft/typescript-go

## Why it may be the right fit
Possible faster large-project checks; source already exposes typecheck:tsgo.

## Why this, rather than an alternative?
Never replace regular tsc gate until both outputs and diagnostic parity pass.

## Features worth mapping
1. Product-specific capability and interaction boundary.
2. Security/privacy and ownership constraints.
3. Dependency size, reliability, operating cost and developer/CI friction.
4. Source pin, resolved version (once locked), transitive and reverse dependencies.

## Update radar
- Native preview release cadence
- parity with tsc

## Evidence required before adoption or replacement
Record upstream release URL and published date, breaking changes, security advisories, alternatives considered, reproducible benchmark/fixture, source-owner decision, feature flags, rollback and user/device acceptance where applicable.

**Last verified upstream latest:** UNKNOWN — not checked.  
**Open research:** exact resolved version; actual usage path; alternative benchmarks; release/security impact.  
**Decision:** KEEP_CURRENT_DECLARATION_PENDING_EVIDENCE. Never auto-upgrade from this page.
