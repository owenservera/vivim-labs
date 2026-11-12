# Selection research and adoption decision method

This is an **evidence notebook**, not a verdict about whether a named library is good/bad. Every technology in [catalog](../catalog/current-stack.json) owns a folder at `technologies/<id>/`.

## Required inquiry for each dependency
1. **Identity:** package/framework/platform, owner, license, distribution, major/minor/pin, manifest file + blob SHA, transitive lock resolution, direct and reverse consumers.
2. **VIVIM fit:** concrete features used, which exact ARBOR capability they enable, why they outperform the simplest plausible alternative *for this workload*, and constraints that make alternatives costly.
3. **Operational economics:** cold-start, runtime memory, storage, disk build cost, shipping size, power/network overhead on Windows and Android, licensing and deployment price.
4. **Correctness and trust:** CVEs/advisories, patch windows, permission surfaces, Tauri IPC, signer/replay, browser session scope, data residency and offline behavior.
5. **Change:** release URL, version/date, notes, deprecations, migration guidance, proposed feature use, reproducible benchmark with before/after source pins, negative tests, rollback.
6. **Decision:** `KEEP`, `PATCH`, `TRIAL`, `MIGRATE`, `RETIRE`, or `DEFER` with owner confirmation and explicit risk/benefit evidence.

## Decision thresholds
- A security fix can override performance ranking **only** with safe migration/mitigation and tests.
- New major versions require compiler/build validation, source closure, contract conformance and OS/device checks.
- Replace only when a documented capability, risk or measured cost materially justifies complexity and one source-owned runtime owner is preserved.
- A package not listed in `package.json` might still be imported in source or as a runtime API; mark as source observed, not automatically resolved.
- An advertised upstream "latest" is valid only when recorded with official URL, published timestamp, checked timestamp, release channel and compatibility scope.
- Record `UNKNOWN` rather than inferring adoption, runtime performance or CVE absence.

## Scoring rubric (research only)
Use a reproducible 0–5 score and raw evidence for **capability fit (30%)**, **reliability/security (25%)**, **maintenance/ecosystem (15%)**, **performance/resource footprint (15%)**, **migration/ownership fit (15%)**. No auto-adoption from an aggregate score. Include an alternatives table and migration rollback plan. See [decision template](../templates/decision-record.md).
