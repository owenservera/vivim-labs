# Takeover closure 004 — native recovery and measured-attempt accounting

Input: 551facfb9aac6bf0e7747dd1819aa2e453c3aabe. Owner direction: complete the original source scope, then build and test, then allocate Astra for independent improvements.

- E47: Windows recovery fixture enumerates the owned controller's children and retains real process handles. The durable lease worker must be observed alive before controller death; observed workers must exit, at least one via parent-death watchdog, and no live child may remain. Subsequent goal and generation assertions remain. Timing races can cause a legitimate fixture failure. Non-Windows execution retains weaker observation.
- E42: host-only RecordAttemptAccounting commits elapsed cost, measured attempt and OS/artifact counters atomically. Legacy actions remain for replay. Supervisor uses the combined action for completion, timeout and stop. Negative fixture checks unauthorized actor, process mismatch, rollback and duplicate accounting. Crash before receipt can still leave missing measurements; these remain unknown.

Source edits only. No build, tests, provider calls or endurance experiments. E42/E47 remain PARTIAL_SOURCE. E45/E46 still need policy-isolated comparative execution; E48/E49 need actual elapsed time. No acceptance-score changes. Preserve experiment isolation.