# Oracle reconciliation rules
1. Read ../OWNER-WORK/PROCESS.md, REQUESTS.jsonl, UPDATES.jsonl and NOW.md.
2. Resolve each OW ID from original request plus the latest valid update. NOW is a projection, not a separate authority.
3. Compare task owner, non-overlapping file surfaces, dependencies, prepared bundle and GitHub main SHA.
4. Distinguish PREPARED, CODED, TESTED, PUSHED, DEPLOYED, DEVICE_VERIFIED. Never infer the next stage.
5. A fresher direct receipt supersedes an older prose snapshot. Conflicting independent evidence is DISPUTED until checked.
6. Discrepancy report: OW ID | claim A | claim B | best evidence | required action | owner.
7. Do not edit another lane's files without coordination. An owner label is not a lock.
8. This is a manual process until an independently authorized reconciler is implemented and proven.