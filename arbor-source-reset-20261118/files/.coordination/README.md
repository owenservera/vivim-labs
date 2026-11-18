# .coordination/ — ARBOR joint working folder (oracle-managed)

Owner directive 2026-11-14: a working folder the coordinator (oracle) manages that BOTH workstream lanes access for joint build artifacts and docs. Coordinator infra — not product; roadmap-gate canon does not gate it, but the reuse-first + one-Host canons bind its contents.

## Contract

| Path | Owner | Who writes | Purpose |
|---|---|---|---|
| `README.md` | oracle | oracle only | this contract |
| `pdr/` | oracle | oracle synthesizes; lanes drop INPUT DRAFTS into `pdr/input-roadmap.md` / `pdr/input-demo.md` | PDR sections: lane inputs + oracle synthesis + owner-gated skeleton |
| `artifacts/roadmap/` | arbor-roadmap lane | keeper lane | roadmap-side joint build artifacts (L2 item texts, gate-evidence rules, def hole-prompt revisions) |
| `artifacts/demo/` | arbor-demo lane | demo lane | demo-side joint build artifacts (route/asset inventory, draft-evolution spec, JSONL migration plan) |
| `artifacts/joint/` | shared | both lanes, non-overlapping filenames | artifacts both lanes produce together (schemas, contracts) |
| `status/roadmap.md` | arbor-roadmap lane | keeper lane | one-screen lane status, updated each turn |
| `status/demo.md` | arbor-demo lane | demo lane | one-screen lane status, updated each turn |

## Rules

1. Oracle manages the root and `pdr/`; lanes own their `artifacts/<lane>/` + `status/<lane>.md` subtrees. No lane writes into another lane's subtree.
2. Filenames carry a round/version suffix (`-r2`, `-v2`) — append-only within a round, never overwrite another seat's file.
3. Long artifacts live HERE, not in bus message bodies — the bus carries pointers (path + one-line gist) + decisions + acks.
4. The refinement ledger stays at `.zcode/workflows/arbor-3page-refinement.v1.md` (oracle-owned, facilitator-automated) — this folder hosts the JOINT ARTIFACTS, not the round log.
5. Consensus-first still binds: everything here is consensus input or coordinator infra until the owner gates the decisions. Nothing in this folder is product code; product surfaces come only through oracle ack windows.
6. Anything sensitive (credentials, tokens, personal identifiers) never enters this folder.

## Current round

Round 2 in flight (see ledger). Expected first artifacts: `pdr/input-roadmap.md` (keeper), `pdr/input-demo.md` (demo), both referenced on the bus with their round-2 critiques.
