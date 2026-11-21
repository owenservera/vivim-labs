# W-AUTOWIKI — STORAGE-GUARDS
Status: **design guardrails only; no disk admission or deletion code in W.** Source: HQ Rock donor `storage-policy.mjs` blob `4606b90c2635564e74c712e19771c5d244576e3e` and `rock-membership.mjs` blob `9023bd026a311cadf9f9d6d024562166a3176a3c`.

## ERA 1 — Byte classes and unbreakable protection
- **Protected/unique:** owner identity and mandatory Rock seeds, source vault originals, accepted decisions/unique personal memories, Host receipts/approval journals, current TierFlow world/editor/draft, recovery references and active writer leases. Never auto-evict or imply source replay without proof. Rock target is soft: overflow is disclosed, not silently truncated.
- **Derived/rebuildable (conditional):** disposable screen projection cache, generated wiki pages, cold index partitions, regenerable graph projections. Eligible only with ownership manifest, unchanged source revision, successful read/rehydration proof, non-pinned state and no lease. A boolean `rebuildable:true` alone is NOT proof.
- **W prototype:** always memory-only, max 20 source cards and 8 allowed section slots (currently 1); no disk cache/exports, no raw transcript/draft bodies, and `close()` discards the active projection. W cannot cause real storage pressure beyond bounded ephemeral page assembly.

## ERA 2 — Advisory policy and low-disk ladder
Existing research `planRockStorage` (not copied into W) uses profiles survival/lean/balanced/full with managed caps 96/256/768/4096 MiB and soft Rock targets 32/64/128/384 MiB respectively. Wiki targets 12/48/128/1024 MiB. Proposed free reserve is `min(2 GiB, max(512 MiB, 2% of volume))`, or stricter owner reserve. Real source-vault/authoritative bytes are accounted separately, NEVER treated as reclaimable managed bytes.

- **Healthy:** permit only bounded optional derived work after real owner inventory.
- **Reclaim required:** stop speculative prefetch and suspend newly generated pages, propose only provably recoverable expired derived cache; remeasure actual bytes after owner reclaims; an estimate is not admission.
- **Read-only protected:** if reserve/quota cannot be satisfied, refuse optional index/wiki/Rock growth but keep current authorized Rock and existing knowledge reads. Expose “source unavailable/partial” instead of fabrication.
- **Overflow:** if mandatory Rock seeds exceed budget, report overflow and refuse silent truncation; modifying profile never modifies authoritative data.
- **No destructive compaction:** no delete/truncate/vacuum/journal.compact in W. Any future owner-side compaction requires exact source recovery proof, writer lease, sufficient physical scratch, atomic commit and verified rollback/replay. Never delete the last unique copy.

## ERA 3 — Proof and rejection gates for F
- Inventory actual physical bytes including SQLite `-wal`/`-shm`, indexes, snapshots, temp scratch; do not use row counts as disk capacity.
- Proof object per prospective eviction: ownerRef/worldRef, category, artifactId, source owner/path/manifest revision/digest, current authorization, online source availability, independently successful rebuild check, lease/pin state, measured reclaim bytes, and operator/owner approval scope where required.
- Negative admission cases: false rebuildability, missing source, revoked permission, active lease, pinned Rock, full quota, scratch under reserve, stale revision, out-of-order inventory, interrupted reclaim; all fail closed.
- Non-destructive recovery gate: rebuild on synthetic fixture after dropping only a derived projection; verify identical canonical source IDs, content digests and read statuses. **NOT RUN in W**. Actual Rust/HQ/Host storage integration and byte-inventory tests are UNRUN.
- If the source is revoked, do not retain page cache pretending it is recoverable. Do not delete protected originals as a side-effect of permission changes.
