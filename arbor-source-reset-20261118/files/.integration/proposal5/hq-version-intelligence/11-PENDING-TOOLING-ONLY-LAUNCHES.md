# HVI launch set — NATIVE TOOLING ONLY (2026-11-12)
**Do not run a real comparison/audit of user repositories in this wave.** Read [governing scope](10-NATIVE-TOOLING-ONLY-SCOPE.md). Historical S0/A–F already committed and their source baseline stays fixed. Z1X is **already launched** in another conversation: don't start a second Z1X or overwrite its proof branch.

## Exact pending sequence
1. **G1 — sequential:** [30-G1-SERIAL-INTEGRATION.md](prompts/30-G1-SERIAL-INTEGRATION.md), AFTER Z1X `GO_FOR_G1_REVIEW`. Serially integrate A–F, explicit schema adapters and executable synthetic tests. Native API readiness, not real repo comparison.
2. **H — parallel with U:** [40-H-HOST-BINDING.md](prompts/40-H-HOST-BINDING.md), AFTER G1 accepted main `GO_FOR_W2_TOOLING`. Install and register the discoverable capability within existing NL/agent/Host machinery, live authorization, typed errors and instructions. No real source comparison.
3. **U — parallel with H:** [40-U-TIERFLOW-INSPECTOR.md](prompts/40-U-TIERFLOW-INSPECTOR.md), AFTER G1 accepted main. Build optional reusable empty/result interface with synthetic fixtures; do not build a fixed actual-result dashboard.
4. **X — ON HOLD, DO NOT LAUNCH:** [40-X-REAL-REPO-AUDIT.md](prompts/40-X-REAL-REPO-AUDIT.md) only if the user later explicitly requests the actual comparison. It is **not** a prerequisite to Z2/G2/R.
5. **Z2 — sequential:** [50-Z2-CONVERGENCE.md](prompts/50-Z2-CONVERGENCE.md) AFTER H/U; independent authority and native discovery verification using synthetic fixtures.
6. **G2 — sequential:** [60-G2-SERIAL-INTEGRATION.md](prompts/60-G2-SERIAL-INTEGRATION.md) AFTER Z2 `GO_FOR_G2_TOOLING_REVIEW`; integrate H/U and real (synthetic-data) tool invocation tests into main.
7. **R — sequential:** [70-R-FINAL-SELF-AUDIT.md](prompts/70-R-FINAL-SELF-AUDIT.md) AFTER G2 `GO_FOR_READINESS_REVIEW`; certify readiness; do NOT compare actual repo pins.

**This is installation work, not usage.** Future natural language command is the sole trigger for a real comparison; it needs no special prompt because agent capability/tool discovery is installed now.

## Copy launcher formula
`ARBOR PROPOSAL 5 — HVI <LANE>. Execute the complete work order at <ABSOLUTE_URL> via connected GitHub, first read 10-NATIVE-TOOLING-ONLY-SCOPE.md. Build and verify native tool capability ONLY using synthetic source data, not a real VIVIM repo comparison. Inspect actual prerequisite SHAs, commit scoped work, report exact SHA, tests with actual exits vs UNRUN, gate and blockers; never overwrite main, merge outside G authorization, or claim beta acceptance.`
