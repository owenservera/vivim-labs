# HVI pending launch board — updated 2026-11-12

> **2026-11-12 SCOPE CORRECTION — NATIVE TOOLING ONLY.** Read [10-NATIVE-TOOLING-ONLY-SCOPE.md](../10-NATIVE-TOOLING-ONLY-SCOPE.md) when this is a prompts/ file; for root docs see [10-NATIVE-TOOLING-ONLY-SCOPE.md](10-NATIVE-TOOLING-ONLY-SCOPE.md). This correction governs any conflicting original instructions below.

## Final user-scope correction
**Current launch board:** [11-PENDING-TOOLING-ONLY-LAUNCHES.md](11-PENDING-TOOLING-ONLY-LAUNCHES.md). Z1X is in progress; do not relaunch or change its running work order. G1 then H/U parallel then Z2/G2/R. **X and real migrations/audits deferred**. If any older instructions conflict, the [tooling-only scope](10-NATIVE-TOOLING-ONLY-SCOPE.md) and updated direct worker prompts govern.


---

Authoritative observed ARBOR main on check-in: `f9823a5131e73e7562db5a4ec14e1ec481557d69`. This is **not a mutable progress automation**; reconfirm refs on every launch.

## Already delivered on isolated branches — DO NOT relaunch absent an owner repair request
- S0 GO_TO_PARALLEL `2ef7a9f66419d681e040b459ba4762a63aa9564f`.
- A immutable source pins `53d4bddf4c2a8fd22f12c8d7f3949aac5fd2fe38`.
- B catalogue identity `1206a343ab98a6e7abe0c19a9549c6b8c32ea9e0`.
- C N-way comparator `7f72c1415d75a7904eb5c267c2a55a9172d33cd8`.
- D parity atlas `622569e9b20f1d6dfae474594eecf544d4fe3d9d` (361 enumerated, 23 inspected; 338 unknown).
- E deterministic report exports `d8080a4056361266702f38cf47f2a0885fe892eb`.
- F schemas and QA `34f5c1568007977e0e3bf0286d4c1ec730d6af26`.
- None merged into ARBOR main at this check-in. Node24 integrated tests/Host grants remain UNRUN. Status PREPARED_NOT_PROMOTED.

## One high-intensity update prompt (launch FIRST)
**Z1X — High-Intensity Convergence Referee:** [complete work order](prompts/20-Z1-HIGH-INTENSITY-CONVERGENCE.md)
This replaces the original Z1 *launch* (do not run both). It **does not merge or patch worker source**; it generates exact G1 repair instructions and independently checks actual A–F inputs. Branch `work/p5-hvi-w1-z-referee-r001`; proof-only.

## Remaining pending work in dependency order
| Launch | Mode | Complete existing work order | Required prior gate |
|---|---|---|---|
| **Z1X** | sequential, now | [20-Z1-HIGH-INTENSITY-CONVERGENCE.md](prompts/20-Z1-HIGH-INTENSITY-CONVERGENCE.md) | A–F final commits observed |
| **G1** | sequential | [30-G1-SERIAL-INTEGRATION.md](prompts/30-G1-SERIAL-INTEGRATION.md) | Z1X reviewed `GO_FOR_G1_REVIEW` and repair orders |
| **H** | parallel with U/X | [40-H-HOST-BINDING.md](prompts/40-H-HOST-BINDING.md) | G1 `GO_FOR_W2` + accepted main SHA |
| **U** | parallel with H/X | [40-U-TIERFLOW-INSPECTOR.md](prompts/40-U-TIERFLOW-INSPECTOR.md) | G1 `GO_FOR_W2` + accepted main SHA |
| **X** | parallel with H/U | [40-X-REAL-REPO-AUDIT.md](prompts/40-X-REAL-REPO-AUDIT.md) | G1 `GO_FOR_W2` + accepted main SHA |
| **Z2** | sequential | [50-Z2-CONVERGENCE.md](prompts/50-Z2-CONVERGENCE.md) | H/U/X final handoffs or explicit blockers |
| **G2** | sequential | [60-G2-SERIAL-INTEGRATION.md](prompts/60-G2-SERIAL-INTEGRATION.md) | Z2 `GO_FOR_G2_REVIEW` |
| **R** | sequential | [70-R-FINAL-SELF-AUDIT.md](prompts/70-R-FINAL-SELF-AUDIT.md) | G2 accepted main + `GO_FOR_SELF_AUDIT` |

## Launch instructions template
For each pending order open a separate ChatGPT conversation using **connected GitHub**:
`ARBOR P5 HVI — Execute the COMPLETE work order <URL> plus every shared HVI contract. Verify the latest real prerequisite gate and exact source/head/base SHA. Commit only to the prescribed isolated branch and exclusive paths. Report exact commit SHA, observed tests with exits versus UNRUN, blockers and gate status. Never force push or overwrite main, never claim runtime/beta GO without real proof; stop if dependencies remain blocked.`

**Critical dependencies:** Z1X → G1 → (H || U || X) → Z2 → G2 → R. This replaces the older launch index for **current pending status**, while preserving the original complete wave and original evidence.
