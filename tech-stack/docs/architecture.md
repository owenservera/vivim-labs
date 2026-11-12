# Grounded technology topology and responsibility boundary

**Evidence pin:** [ARBOR `f9823a5131e73e7562db5a4ec14e1ec481557d69`](https://github.com/owenservera/ARBOR/tree/f9823a5131e73e7562db5a4ec14e1ec481557d69); view edges in [machine catalog](../catalog/dependency-edges.json). Edge types distinguish source, declared optional and designed-only.

```text
                  Windows (MSI unverified)        Android (PWA/device unverified)
                  Rust / Tauri v2                 Web standards + Service Worker
                  WebView2 / windows-rs           Pairing/direct mobile + optional cloud relay
                         |                                      |
                  launches Node >=24 Host  <------ local Direct -+
                         |                 (security gate OPEN) |
                  authority for actions/receipts               |
                /         |            \                optional @vercel/blob 2.8.2
           Studio      native pure      development HQ        / Redis REST adapter
        Chrome CDP     Rust core        Node 24 + node:sqlite
           |          HQKG / DAG        source index/wiki
      Harness signed-in  |
      browser sessions   +-- optional rusqlite / postgres
                         |
        TierFlow internal typed UI/projection (Bun / TypeScript)
        [convergence defined, exact live publisher parity not accepted]
```

## Placement map
| Area | ARBOR path | Technology and status |
|---|---|---|
| Workspace | root `package.json`, `Cargo.toml` | Bun 1.4.4 and Rust 1.99.0 declarations; root locks missing |
| Native shell | `src-tauri/` | Tauri =2.0.0 / WebView2; native launches Node Host; Windows MSI UNRUN |
| Host and permissions | `apps/vivim-host/` | Node HTTP, local World and approval/receipt authority; no duplicate effect owner |
| Chrome integration | `studio/`, `harness/` | CDP and browser sessions; no provider APIs |
| HQ development intelligence | `hq/` | Node built-in SQLite rebuildable development data; not private personal World |
| Native algorithms | `native/hqkg/`, `core/programmability/` | Rust typed graph/compute; optional DB drivers |
| Visual system | `packages/tierflow/` | TypeScript/Bun; in-repo renderer, not separate product service |
| Mobile | `apps/mobile-relay/` | PWA/Service Worker, signer and optional Vercel Blob or Redis REST relay |

## Key distinctions
- The graph engine and Host effect authority are **not interchangeable**; deterministic native computation may propose data/operations but must not become a second effect writer.
- Rust HQKG's `rusqlite` and `postgres` are **optional feature flags**; no deployed DB server follows from listing them.
- `apps/mobile-relay` ships web standards and a cloud-compatible relay; direct and cloud routes have different privacy/security implications.
- Source presence does not establish cross-device sync or production behavior. Reconciliation and security gates remain open.
- Root `package.json` declares six JS workspaces; root `Cargo.toml` declares three Rust members. These are **not** proof that all imports resolve or compile.
- Labs is research. ARBOR is source. HQ produces derived development views when implemented and authorized.

## Source links
[Workspace root](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/package.json) · [Rust root](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/Cargo.toml) · [Tauri launcher](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/src-tauri/src/main.rs) · [Host entry](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/apps/vivim-host/server.mjs) · [Studio config](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/studio/src/config.mjs) · [HQ SQLite](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/hq/lib/schema.mjs) · [Mobile PWA](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/apps/mobile-relay/sw.js) · [Release gate](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/integration/release-manifest.json)
