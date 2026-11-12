# ARBOR Tech Stack Observatory — Labs research overlay

**Purpose:** a source-pinned, human-readable and machine-readable map of the technology currently **declared or observable** in [ARBOR](https://github.com/owenservera/ARBOR), its dependencies, choice rationale, update radar, optimization opportunities and long-horizon adoption options.

**Current observed pin:** `f9823a5131e73e7562db5a4ec14e1ec481557d69` on ARBOR `main`, inspected 2026-11-12.  
**Labs base pin:** `022608f5797e7b4a14ccc4c393bd3c5a4704a4be`.  
**Truth boundary:** ARBOR source/manifests and [integration release gates](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/integration/release-manifest.json) are authoritative for what exists. This folder is **not** a second runtime, product roadmap, development index, dependency graph database or Host authority. HQ should eventually project the centralized live view from ARBOR and these evidence records.  
**Freshness:** **PINNED SNAPSHOT, NOT LIVE SYNC.** Upstream latest releases/advisories are **NOT YET VERIFIED**. This distinction is deliberate.

## Where to start
- [Architecture and runtime topology](docs/architecture.md)
- [Versioned stack catalog](catalog/current-stack.json) · [Workspaces](catalog/workspaces.json) · [Dependency edges](catalog/dependency-edges.json)
- [Pinned source evidence and lockfile gaps](catalog/source-pins.json)
- [Per-selection technology research](technologies/) — dedicated page for every initial selection
- [Release radar](tracking/release-watch.json) · [Optimization backlog](tracking/optimization-backlog.json) · [Adoption horizon](tracking/adoption-horizon.json)
- [Refresh protocol](docs/refresh-and-operations.md) · [Decision method](docs/research-and-decision-method.md) · [Milestones](docs/forward-roadmap.md)
- [Future HQ interface](docs/hq-integration-contract.md)
- [Read-only drift detector](tools/reconcile.mjs) · [Offline consistency verifier](tools/validate.mjs)

## Rules of interpretation
1. **Present in manifest** is not **installed/resolved**. Without a root lockfile, exact transitive versions and reproducibility are unknown.
2. **Source-observed** is not **runtime accepted**. ARBOR release-manifest is `BLOCKED_NOT_BUILT`; Windows MSI, physical PWA, security and no-donor builds are not accepted.
3. **Latest release** is a separate, timestamped upstream claim, never inferred from the version specified by ARBOR.
4. **Optional adapter** is not a production dependency. Feature-gated Rust DB drivers and cloud relay stores are separately marked.
5. A migration requires an owner, measurements, negative tests, rollback and a pinned before/after comparison.

## How to operate (with Node >=24)
```sh
node tech-stack/tools/validate.mjs
node tech-stack/tools/reconcile.mjs --branch main
# optional: node tech-stack/tools/reconcile.mjs --ref <specific-sha>
```
The reconcile command reads only public-safe GitHub metadata/manifests; use a read-only `GITHUB_TOKEN` for private ARBOR access. It prints a drift report to stdout and never changes ARBOR, Labs or package versions. See [runbook](docs/refresh-and-operations.md) for promotion of observed diffs into reviewed snapshots.

## First honest finding
ARBOR currently declares **Bun 1.4.4**, **Node >=24**, **Rust 1.99.0**, **Tauri =2.0.0**, **TypeScript ^7.0.2**, **@vercel/blob 2.8.2**, plus native and optional DB crates. Root `Cargo.lock` and `bun.lock` are absent at the pinned commit. The first technical priority is lock/source closure and security/packaging validation, not swapping frameworks.

**Privacy:** do not place owner corpora, browser sessions, credentials, private build logs or raw private code inside this observatory, even if Labs is currently private. Keep derived metadata publication-safe.
