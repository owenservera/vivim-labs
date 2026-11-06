# Performance Lab — Evidence and Decision Log

**2026-11-06 — Lab initialized.** No benchmark runs, real phone tests, repository inventory, source integration tests or validated performance comparisons have been completed in this lab. Do not mistake design recommendations for verified results.

## Research-only evidence
- Context7 documentation lookup: UniFFI `/mozilla/uniffi-rs/v0.32.2` verifies Rust→Kotlin bindings are supported; **does not** prove this VIVIM build works.
- Context7 `/websites/developer_android_develop_ui_compose` verifies Compose/ViewModel concepts; library version not surfaced.
- Context7 `/tauri-apps/tauri/tauri-v2.0.0` verifies Tauri mobile/Rust/WebView architecture at the referenced version; current release compatibility unverified.
- Architecture inference: use Rust shared core + Kotlin Android adapter as initial hypothesis; UI contender A/B/C remains open.
- No performance numbers, crash rates or TTM gains claimed.

## Append-only experiment entry template
```markdown
### YYYY-MM-DD — Pxx — <experiment name>
Status: planned | running | complete | inconclusive | invalidated
Owner / reviewer:
Repositories + exact SHAs:
Hypothesis and falsifier:
Contenders and equivalent functional contract:
Hardware / Android or Windows OS / build type / CPU governor / thermals:
Reference docs IDs and versions:
Fixture and data hashes / sample count / randomized order:
Commands + exit codes:
Raw results paths + reproducibility instructions:
p50 / p95 / p99 / memory / CPU / battery / binary size / build time:
Security / approvals / negative tests / recovery:
Engineering+AI effort / verification and rework:
Limitations / confounds / uncertainty:
Decision: adopt | keep | experiment | reject (advisory)
Owner approval and production promotion reference:
Rollback:
```

## Decision history
| Date | Component | Decision | Evidence | Status |
| --- | --- | --- | --- | --- |
| 2026-11-06 | Shared kernel | Investigate portable Rust | Context7 documentation + architecture reasoning only | Provisional |
| 2026-11-06 | Android adapter | Investigate Kotlin Android SDK | Native API compatibility reasoning only | Provisional |
| 2026-11-06 | Android UI | Compare Compose, hybrid WebView, Tauri 2 | No benchmarks | Open |

Record every later reversal, never erase the previous evidence. Link large raw outputs rather than embedding them in the one-pager.
