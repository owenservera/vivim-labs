# HVI launch index — paste one prompt per ChatGPT conversation

> **2026-11-12 SCOPE CORRECTION — NATIVE TOOLING ONLY.** Read [10-NATIVE-TOOLING-ONLY-SCOPE.md](../10-NATIVE-TOOLING-ONLY-SCOPE.md) when this is a prompts/ file; for root docs see [10-NATIVE-TOOLING-ONLY-SCOPE.md](10-NATIVE-TOOLING-ONLY-SCOPE.md). This correction governs any conflicting original instructions below.

## Updated execution scope — supersedes original sequence
Read [10-NATIVE-TOOLING-ONLY-SCOPE.md](10-NATIVE-TOOLING-ONLY-SCOPE.md) and [11-PENDING-TOOLING-ONLY-LAUNCHES.md](11-PENDING-TOOLING-ONLY-LAUNCHES.md). The **current** sequence is already-running Z1X → G1 → (H || U) → Z2 → G2 → R. **X must NOT launch**. Old historical sequence shown below is retained only for traceability.


---

**All prompts are committed work-order files;** each asks the worker to read this folder and its own exact prompt from GitHub, create an isolated branch, perform work and report an exact commit. Do not copy a partial summary instead of the full order.

## Launch order
1. **Sequential S0** (must finish): [00-S0-PIN-AUDIT.md](prompts/00-S0-PIN-AUDIT.md). Continue only on S0 `GO_TO_PARALLEL`.
2. **Parallel A–F** (six *different* ChatGPT conversations): [10-A-SNAPSHOT-SOURCES.md](prompts/10-A-SNAPSHOT-SOURCES.md) · [10-B-IDENTITY-CROSSWALK.md](prompts/10-B-IDENTITY-CROSSWALK.md) · [10-C-NWAY-COMPARATOR.md](prompts/10-C-NWAY-COMPARATOR.md) · [10-D-PARITY-ATLAS.md](prompts/10-D-PARITY-ATLAS.md) · [10-E-REPORT-EXPORT.md](prompts/10-E-REPORT-EXPORT.md) · [10-F-CONTRACT-QA.md](prompts/10-F-CONTRACT-QA.md).
3. **Sequential Z1 then G1**: [20-Z1-INDEPENDENT-REFEREE.md](prompts/20-Z1-INDEPENDENT-REFEREE.md) → [30-G1-SERIAL-INTEGRATION.md](prompts/30-G1-SERIAL-INTEGRATION.md). G1 must obtain a real accepted `GO_FOR_W2` and resulting main SHA before the next product wave.
4. **Parallel H/U/X** (three conversations from newly accepted main): [40-H-HOST-BINDING.md](prompts/40-H-HOST-BINDING.md) · [40-U-TIERFLOW-INSPECTOR.md](prompts/40-U-TIERFLOW-INSPECTOR.md) · [40-X-REAL-REPO-AUDIT.md](prompts/40-X-REAL-REPO-AUDIT.md).
5. **Sequential Z2 → G2 → R**: [50-Z2-CONVERGENCE.md](prompts/50-Z2-CONVERGENCE.md) → [60-G2-SERIAL-INTEGRATION.md](prompts/60-G2-SERIAL-INTEGRATION.md) → [70-R-FINAL-SELF-AUDIT.md](prompts/70-R-FINAL-SELF-AUDIT.md).

## Minimal ChatGPT launcher
`ARBOR P5 HVI — execute the complete connected-GitHub work order at https://github.com/owenservera/ARBOR/blob/main/.integration/proposal5/hq-version-intelligence/prompts/<FILENAME>.md. Respect the shared contract/branch/write scopes; verify actual refs and evidence, commit all scoped work, report exact SHA and executed tests vs UNRUN. Do not invent runtime GO or touch main unless the G integrator instruction explicitly authorizes an evidence-backed PR merge.`

Use one conversation per lane. **S0's actual post-plan main workerBaseSha** is the authority, not the older `370d18d92dfa463f58bd145b307b488a64746ee5` pre-preparation main. If source tips have moved, S0 annotates fresh tips without altering the original source-pins baseline.

## Prompt matrix
| Step | Role | Branch | Complete order |
|---|---|---|---|
| S0 — Source-pin preflight and measured census | sequential | `work/p5-hvi-s0-source-lock-r001` | [00-S0-PIN-AUDIT.md](prompts/00-S0-PIN-AUDIT.md) |
| A — Immutable Git snapshots and source adapters | parallel | `work/p5-hvi-w1-a-snapshots-r001` | [10-A-SNAPSHOT-SOURCES.md](prompts/10-A-SNAPSHOT-SOURCES.md) |
| B — Identity crosswalk and orphan evidence | parallel | `work/p5-hvi-w1-b-identity-r001` | [10-B-IDENTITY-CROSSWALK.md](prompts/10-B-IDENTITY-CROSSWALK.md) |
| C — Deterministic N-way version comparator | parallel | `work/p5-hvi-w1-c-compare-r001` | [10-C-NWAY-COMPARATOR.md](prompts/10-C-NWAY-COMPARATOR.md) |
| D — HQ functional parity and Labs migration atlas | parallel | `work/p5-hvi-w1-d-parity-r001` | [10-D-PARITY-ATLAS.md](prompts/10-D-PARITY-ATLAS.md) |
| E — Read-only migration report and export | parallel | `work/p5-hvi-w1-e-reports-r001` | [10-E-REPORT-EXPORT.md](prompts/10-E-REPORT-EXPORT.md) |
| F — Independent contract and adversarial tests | parallel | `work/p5-hvi-w1-f-verify-r001` | [10-F-CONTRACT-QA.md](prompts/10-F-CONTRACT-QA.md) |
| Z1 — Independent Wave 1 convergence referee | sequential | `work/p5-hvi-w1-z-referee-r001` | [20-Z1-INDEPENDENT-REFEREE.md](prompts/20-Z1-INDEPENDENT-REFEREE.md) |
| G1 — Serial Wave 1 integrator to main | sequential | `integrate/p5-hq-version-intelligence-r001` | [30-G1-SERIAL-INTEGRATION.md](prompts/30-G1-SERIAL-INTEGRATION.md) |
| H — Host-authorized HQ comparison read port | parallel | `work/p5-hvi-w2-h-host-r001` | [40-H-HOST-BINDING.md](prompts/40-H-HOST-BINDING.md) |
| U — TierFlow read-only version inspector | parallel | `work/p5-hvi-w2-u-inspector-r001` | [40-U-TIERFLOW-INSPECTOR.md](prompts/40-U-TIERFLOW-INSPECTOR.md) |
| X — Real five-repository migration audit | parallel | `work/p5-hvi-w2-x-source-audit-r001` | [40-X-REAL-REPO-AUDIT.md](prompts/40-X-REAL-REPO-AUDIT.md) |
| Z2 — Independent post-wiring referee | sequential | `work/p5-hvi-w2-z-referee-r001` | [50-Z2-CONVERGENCE.md](prompts/50-Z2-CONVERGENCE.md) |
| G2 — Serial Host/UI integration | sequential | `integrate/p5-hq-version-intelligence-r001` | [60-G2-SERIAL-INTEGRATION.md](prompts/60-G2-SERIAL-INTEGRATION.md) |
| R — Final self-audit and release-neutral decision | sequential | `work/p5-hvi-w3-r-self-audit-r001` | [70-R-FINAL-SELF-AUDIT.md](prompts/70-R-FINAL-SELF-AUDIT.md) |

**Do not launch H/U/X merely because G1 opened a PR.** W2 requires accepted G1 main and explicit GO_FOR_W2.


## CURRENT CHECK-IN UPDATE (2026-11-12)
The original S0 and A–F lanes have committed isolated results. **Do not relaunch them by default**. For the updated pending-only order, use [09-PENDING-LAUNCH-BOARD.md](09-PENDING-LAUNCH-BOARD.md). The next gate is the **single** [high-intensity Z1X convergence work order](prompts/20-Z1-HIGH-INTENSITY-CONVERGENCE.md), **instead of**, not in addition to, the original Z1 prompt. Its follow-on G1/H/U/X/Z2/G2/R dependencies are unchanged. Always verify actual branch heads before launching.
