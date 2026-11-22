# Harness intake — candidate register (research, not product status)

As of 2026-11-22. `UNINSPECTED` means no source-level audit has yet been performed for this intake. Priority expresses expected product relevance, **not** implementation readiness.

| Priority | Candidate / source | Existing consumer or collision to inspect | Intended visible result | State |
|---|---|---|---|---|
| P0 | **Owner's newly created GitHub repo — URL/name pending** | ARBOR `harness/`, `apps/`, `studio/`, `native/` | To be determined from actual source | UNIDENTIFIED |
| P0 | [ARBOR current harness](https://github.com/owenservera/ARBOR/tree/main/harness) | Canonical baseline / contracts / test hooks | No regression to existing command surface | DIRECTORY CONFIRMED; NOT AUDITED |
| P0 | [ARBOR native](https://github.com/owenservera/ARBOR/tree/main/native) + [apps](https://github.com/owenservera/ARBOR/tree/main/apps) | GPUI implementation and desktop/mobile parity | Desktop native view and phone share visible state | DIRECTORY CONFIRMED; NOT AUDITED |
| P0 | [ARBOR wiki](https://github.com/owenservera/ARBOR/tree/main/wiki) | AutoWiki knowledge projections and host event contract | Typing visibly changes wiki | DIRECTORY CONFIRMED; NOT AUDITED |
| P1 | [ARBOR studio](https://github.com/owenservera/ARBOR/tree/main/studio) | Browser / provider surface integration; no unauthorized UI dependency | Internal command/feedback loop first | DIRECTORY CONFIRMED; NOT AUDITED |
| P1 | [Craft Anything adapter research](../../craft-anything-arbor-adapter/) | Existing research avoids duplicating an adapter | Later external web-app intelligence enrichment | FOLDER CONFIRMED; NOT AUDITED |
| P1 | [TierFlow repository](https://github.com/owenservera/tierflow) | Existing TierFlow visual runtime migrating toward GPUI | Single interaction/rendering authority, not a second UI | SOURCE LINK; NOT AUDITED |
| P2 | [Capability observatory](../../capability-observatory/) + [tech stack](../../tech-stack/) | Prior research, possible coverage overlap | Source-backed comparison only | FOLDER CONFIRMED; NOT AUDITED |

## Required decision record for each source

- **Source:** `owner/repo` + path + default branch + full SHA (pinned).
- **License & provenance:** SPDX or human review; compatibility and attribution requirements.
- **Actual implementation:** key files, APIs, dependencies, resources, tested behavior; avoid README-only claims.
- **ARBOR target:** precise module and owner contract; existing equivalent or collision.
- **Decision:** `ADOPT` / `ADAPT` / `REFERENCE` / `DEFER` / `REJECT`.
- **Why:** value, migration cost, footprint, performance, threat surface, extensibility, parity.
- **Acceptance:** a visible action on GPUI/mobile surface, explicit expected state and evidence.
- **Proof:** checks run and exit codes, desktop/mobile screenshot comparison as appropriate, rollback path.
- **Ownership:** ARBOR product coding agent; Labs holds evidence only.

## Working hypothesis, subject to inspection

Highest-value work is likely the **single-surface interaction path** (native GPUI ↔ phone state ↔ command feedback ↔ AutoWiki), not a broad clone/import of a third-party framework. Import selectively at module/contract boundaries. Keep previous Edge implementation as comparison/reference where useful, never as a required desktop UI.
