# Algorithm tracks

Five independent optimization problems. Each directory owns only its own experiments, fixtures and results. No heavy scoring, new tracker or status database.

| Track | What to improve | Existing contract / owner |
|---|---|---|
| [01 priority ranking](01-priority-ranking/) | Evidence-backed “what needs attention next?” | Repos HQ KB/graph; TierFlow F read projection |
| [02 semantic resolution](02-semantic-resolution/) | Understand intent, synonyms and ambiguity; abstain safely | HQ semantics; TierFlow G typed IR; Host remains executor |
| [03 context selection](03-context-selection/) | Retrieve useful, nonredundant, provenance-backed context | Repos HQ canonical KB/search/graph |
| [04 incremental reconciliation](04-incremental-reconciliation/) | Detect meaningful changes with less work and no stale state | TierFlow F/C and existing Host/mobile transport |
| [05 outcome certainty](05-outcome-certainty/) | Explain proven/uncertain/denied outcomes and safe recovery | Host journal/grants; TierFlow H/C read-only status |

For each run: pin source SHAs, state a baseline, create executable fixtures/benchmarks, save actual results **in that track**, and recommend KEEP/REVISE/REJECT. Nothing integrates automatically. Wave 4–6 files remain untouched.
