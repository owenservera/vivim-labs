# Source closure 005 — native qualification and comparative planning

Baseline: 110b08448c6486d850b60c9034c815931f6693e3. Direct GitHub coding only. No runtime execution or experiments.

- Native qualification: explicit fresh-root invocation, actual executable SHA256, declared source pin, bounded arithmetic workload, independently computed expected outputs, consistent state/anchor observations, immutable controller reservations and fault intent/exit/recovery receipts, real process-kill path restricted to owned controller, generation-fenced recovered work, loss and duplicate-task tracking, optional time targets. Unknown and failed observations are not accepted. Report is not gate credit.
- Comparative planning: versioned manifest, reproducible shuffled order across four arms, matching workload/seed/budgets/fault milestones, explicit allowed persistence authorities and no cross-arm learning. No live arm implementation or OS-enforced persistence isolation yet.
- Demand generation checkpoint: exact goal/family scope and profile allowlist, bounded budgets and native-first scheduling. No live adapter wiring.
- CLI: qualify-native <fresh-root> <plan.json>; comparative-plan <spec.json>. Example plan is illustrative; replace declared source pin with actual reviewed source when running.
- Limitations: native runner only tests controller interruption, not independent simultaneous OS worker death; missed milestone is a failed workload condition, not invented evidence. Polling only measures sampled lease concurrency. Native pure workload does not exercise external effects. Runner interruption can leave partial immutable evidence and no final report. No automatic restart of qualification runner itself. Real four-arm adapter isolation remains open.

Tracker: 39/50 provisional source-complete, 11 partial, no deferred; original weighted criteria preserved except owner-authorized E48/E49 revision. R004 verified score remains 0. No provider calls, compile, build or tests. Next: complete real adapter isolation, scalable persistence and generalized learning/accounting.
