# Programmable endurance — owner revision 001

Baseline: aefda82622a4d08c38038c44b40a3ae0267735c3. Owner explicitly removed the fixed 36/72-hour qualification thresholds. IDs E48/E49 and their weights remain unchanged. Historical receipts retain original criteria.

The runtime now accepts a versioned JSON policy specifying minimum verified tasks, recovered faults, optional elapsed time and recovery latency, maximum artifact loss, duplicate effects and interventions. Missing observed values remain unknown. A policy and supplied observation can be evaluated with `kernel <store> endurance-evaluate <policy.json> <observation.json>`. This evaluator is not an independent evidence collector or test runner and never awards acceptance credit. Example thresholds are illustrative, not mandatory. Execution still requires a finite operator-selected duration and bounded jobs; the former 72-hour maximum for service/controller durations is removed. Existing lease, polling and cancellation protections remain. Host operation wall duration no longer saturates at 72 hours.

The canonical tracker retains 39/50 source-implemented, 9 partial, 2 deferred; no verified score changes. E48/E49 remain partial until real evidence collection and comparative execution are wired and qualified. No compilation, tests, live providers or experimental launches. Existing isolated experiment folders remain untouched; new policy must be adopted explicitly when creating a new cohort.

Workflow: direct GitHub source coding on main, then fast-forward workstation; ASDF not used for this iteration.
