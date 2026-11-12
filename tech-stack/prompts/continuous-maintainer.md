# Daily/weekly stack observatory maintainer (future automation)

**Prerequisite:** owner has explicitly enabled a scheduled/event runner and least-privilege GitHub read access.

Daily source: compare ARBOR default branch HEAD/tree and manifests against the last accepted pin; surface changed direct/resolved deps, reverse-impact paths, missing locks and readiness-gate changes. Daily external: verify official upstream releases and advisories; if inaccessible, report unknown, never safe. Weekly: prioritize choices where patch/security or material optimization evidence exists. Monthly: revisit next-gen candidates and deprecations.

Never mutate ARBOR, deploy, fetch private user data, auto-accept a major upgrade, or treat latest upstream as installed. Open a Labs review PR only on verified metadata changes. Store source SHAs, run-time timestamps, linked evidence, and command exits. Use HQ comparator when operational. Error states fail closed; report `SYNC_FAILED` and retain previous pin.
