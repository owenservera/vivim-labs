# Performance — Opportunity Backlog

**Now: identify promising stack improvements, not launch a big test program.**

| Priority | Question | Why it might matter | Next lightweight action |
| --- | --- | --- | --- |
| High | Can one Rust kernel serve Android and Windows? | Less duplication, shared local intelligence | Identify which existing modules could be portable |
| High | Compose vs WebView hybrid vs Tauri on Android? | Native capabilities vs visual-runtime reuse | Map requirements and obvious limitations |
| High | Are graph and knowledge queries using the right engines? | Retrieval and agent response speed | Identify potential compute/storage bottlenecks |
| Medium | Are TypeScript↔Rust↔Kotlin boundaries too chatty? | Bridge/serialization and responsiveness | Map important crossings |
| Medium | Is offline sync overcomplicated or too costly? | Battery, correctness and responsiveness | Compare protocol options |
| Medium | Where might Go or custom Rust services outperform current choices? | Better runtime fit for specific workload | Shortlist only concrete candidates |

**Later, when physical machines are available:** validate selected opportunities with equivalent functional slices, real Android and Windows testing, release builds, latency, memory, battery, reliability and migration risk. Only then make evidence-based migration decisions.

**Rule:** A promising technology is a research candidate, not an automatic rewrite. Do not expand this into detailed protocols, scoring matrices or dozens of experiments until needed. AI implementation cost is excluded entirely.
