# Refresh, version-tracking and maintenance operating model

## Present state — deliberately not real time
The `2026-11-12` snapshot is pinned to ARBOR `f9823a5131e73e7562db5a4ec14e1ec481557d69`. It has no live GitHub subscription, no background job, no verified upstream latest release and no security attestation. `reconcile.mjs` provides **on-demand read-only drift detection**; it is not a background monitor. This separation protects against silently stale "real-time" labels.

## Proposed update architecture (not installed)
1. **Source event:** ARBOR push to `main` dispatches a read-only metadata signal (commit SHA, no source payload) to Labs. Requires a scoped GitHub App or least-privilege token and owner-approved workflow. Alternatively run after HQ's version comparator emits an accepted source snapshot.
2. **Fast refresh:** read exact ARBOR commit, tree and relevant manifests; create a proposed snapshot with source SHA, changed paths, dependencies, affected workspaces and uncertainty state.
3. **Scheduled reconcile:** daily full tree diff to recover dropped events. Avoid infinite bot-commit loops and never mutate ARBOR.
4. **Upstream watch:** daily official release/advisory feed inspection and weekly human triage; store link, release date, check date, channel, current declared/resolved and impact. Unknown on fetch failure. Do not claim current CVE-safe status from empty feed.
5. **Impact engine:** pass normalized records as **inputs** to ARBOR HQ Version Intelligence's source/version graph; HQ remains canonical read-model/index owner, Labs remains research/evidence owner.
6. **Owner gate:** PR in Labs for research/evidence updates; product upgrade PR only in ARBOR after an approved decision. No auto-upgrades, deploys, schema changes or personal-data indexing.

## Local read-only commands
```sh
node tech-stack/tools/validate.mjs
GITHUB_TOKEN=<read-only-token> node tech-stack/tools/reconcile.mjs --branch main
GITHUB_TOKEN=<read-only-token> node tech-stack/tools/reconcile.mjs --ref f9823a5131e73e7562db5a4ec14e1ec481557d69
```
`reconcile.mjs` prints a JSON report; status values:
- `PIN_MATCH`: queried ref equals this observation pin; does **not** mean release accepted.
- `SOURCE_DRIFT`: commit or observed manifest blob changed; inventory requires review.
- `UNREADABLE`: API/network/rate limit/auth failure; not "no changes".
- `INVENTORY_GAP`: observed paths or root locks differ from pin inventory.

## Freshness SLO targets (future, once implemented)
- ARBOR source delta detected within **one hour** of main push via event, with daily backfill; report SLA misses, not fake freshness.
- Security advisories triaged within **one business day** of a verified alert; critical alerts immediately to owner.
- Weekly upstream maintenance digest; monthly adoption horizon review; quarterly alternative re-evaluation.
- Exact versions from lockfiles where present; `UNKNOWN` whenever resolution unavailable.
- Every snapshot has source commit, manifest blob SHAs, generated time, query status and gate evidence.

## Required runtime verification after any upgrade
`cargo metadata --locked`, `cargo tree`, `cargo test --workspace`, `bun install --frozen-lockfile`, Node/TierFlow tests, Windows MSI installation, signed-in Chrome, paired HTTPS Android PWA and signer/revoke/replay negatives. Record every command exit/host environment or mark UNRUN. This research repo **cannot** claim those checks.
