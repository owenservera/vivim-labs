# Takeover closure 003 — durable goal completion

Baseline: 16001852d7547d4d3b515c513cffbed28508b4d4 (local and origin/main reconciled before edit).

Finding: the outer service used successful controller process exit as completion, even when the controller exited normally due to idle, job limit or duration limit. A historical successful exit receipt could prevent resumed work.

Source change: derive disposition from reopened durable kernel state before launch and after controller exit; distinguish no goals, pending goals, terminal unsuccessful goals and complete goals; preserve bounded restart budgets and operator stop; persist goal disposition separately from exit outcome; validate historical receipt ordinals. Source-only unit fixture covers empty, pending, mixed, failed and completed populations.

Acceptance: a clean process exit must not cause a pending goal to be reported complete or prevent a budgeted restart. Terminal unsuccessful goals must not be reported successful. Existing restart budgets and stop controls remain in force.

Evidence: source diff checked; fixture authored, not executed. R004 remains uncompiled and unqualified. No gate status or score changes. Relevant gates E03, E26 and E47. No provider calls or experimental launches.

Remaining: qualify an actual clean controller exit with pending goals, then simultaneous process-death recovery. Continue persistence scale, authority, learning and accounting closures against original criteria. No simulated evidence credit.