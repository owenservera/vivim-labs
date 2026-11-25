# STATUS — auto-cmd-wiki-keeper standing workspace

**Phase:** baseline  
**Progress:** 0 of 0 deliverables complete  
**Bootstrap date:** 2026-11-11  
**Self-bootstrap status:** step 3 of 6 complete; seed + STATUS + LOG read

## Baselines (read-only, no regeneration)

| Check | Command | Exit | Output |
|-------|---------|------|--------|
| Sync check | `bun scripts/sync.mjs check --as auto-cmd-wiki-keeper` | 3 | out of sync: 133 unread for auto-cmd-wiki-keeper |
| Wiki check | `bun hq/bin/hq.mjs wiki --check` | 1 | Wiki is stale: 5100 changed page(s) |
| Status | `bun hq/bin/hq.mjs status --json` | 0 | see below |

### wiki --check details (verbatim head)
Exit 1 = stale. 5100 changed page(s). Listed pages include: capabilities.md, changes.md, context.md, deprecated.md, docs.md, plus file-surface pages under `files/openmausbot/...` (e.g. composio-broker src files, control-plane src files). These appear to be general capability/context/file pages — NOT per-command reference pages derived from the action catalog. **Gap confirmed: no generated per-command reference pages yet.**

### status --json (verbatim summary)
- Schema: hq.status/1
- Root: C:\0-BlackBoxProject-0\repos_hq
- DB: C:\0-BlackBoxProject-0\repos_hq\.hq\index.db
- lastIndexed: 2026-11-11T00:58:09.048Z
- Projects: 4 (openmausbot 6530 files/185962 symbols; repos-hq 1783 files/24914 symbols; vivim-ui 583 files/7556 symbols; vivim-web-harness 1686 files/41539 symbols)
- Totals: files 10582, symbols 259971, imports 43664, annotations 1722, receipts 41, notes 0, commits 501
- Note: status does NOT expose a wiki page count; page coverage must be derived from the wiki generator output directly.

## Next steps (per seed)
- Step 4: write self-reflection to oracle/bootstrap/auto-cmd-wiki-keeper.reflection.md
- Step 5: claim session via claim.mjs (session id unknown → journal intent in LOG.md)
- Step 6: report identity, baseline exits + counts, reflection path, claim outcome, top three contracts, first proposed generator scope
