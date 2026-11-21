# Craft Anything → ARBOR — Migration-Disposition R001

Snapshot: [Craft tree `e64d7cb73d7f39323b7040a984e0f9b3f7981146`](https://github.com/owenservera/craft-anything-VIVIM/tree/e64d7cb73d7f39323b7040a984e0f9b3f7981146); [ARBOR tree `171c483b9a449d0e396144712b099540509fb617`](https://github.com/owenservera/ARBOR/tree/171c483b9a449d0e396144712b099540509fb617).

**First pass:** 1190 tracked Craft files; **75** explicitly reasoned candidate/dependency rows; **40** source headers/declarations actually inspected; all 20 feature targets mapped. No files migrated to ARBOR.

Files in this folder:
- `FILE-DISPOSITION.csv`: one row for **every tracked Craft file**, with source Git SHA, decision, wave, case IDs, target, reason, blocker and evidence depth.
- `PRIORITY-FILE-REVIEW.md`: file-by-file justified candidates and source links.
- `USE-CASES-20.md`: all twenty conceptual feature goals mapped to source areas and ARBOR owner.
- `DEPENDENCIES-AND-WAVES.md`: W0/W1/W2/W3/W9 migration sequencing and ownership boundaries.
- `MANIFEST.json`: source snapshot and row counts.
- `LOCAL-AGENT-NEXT.md`: exact next research-and-implementation pass.

Decision: **Use Craft wholesale as a separate investigator now. Integrate verified evidence into Studio/Harness through the existing Labs bridge. Harvest only the pure Rust algorithms that solve proven gaps; never duplicate Host/graph/browser ownership.**

**Important:** `EXTRACT_PURE` marks a migration *candidate*, not a ready-to-copy module. Many files have only been classified from paths; inspect their bodies before code changes. Third-party application inspection requires proper authorization.
