# Founder interview and decision register — R001
Owner interview required before ratification. Do not reopen settled architecture rules (Rust-first, Host effect authority, single GPUI surface, user-owned data, no duplicate send). These questions concern product boundaries, not implementation architecture.

| ID | Unresolved decision | Working recommendation (NOT owner approved) |
| --- | --- | --- |
| D01 | Who is the first release for: founder/private alpha, AI power users, developers or general users? | Founder/private alpha first, then AI power-user beta |
| D02 | What is the one indispensable result in first 5 minutes? Chat, phone control, or personal intelligence? | Connect an existing AI and continue the same conversation across devices, with local context |
| D03 | Is ChatGPT alone sufficient for launch, or must Claude/Gemini and other accounts also work? | ChatGPT reference path; preserve on-demand providers; no blanket provider restriction |
| D04 | Is full real-time historical sync a V1 must, or is live selected-thread capture plus optional backfill enough? | Selected-thread live sync first; full sync scope explicitly decide |
| D05 | Which local commands are non-negotiable? File search/open, app launch, file edit, terminal, agent execution, social apps? | File search/open, app launch, explicit file attachment; clarify destructive/terminal exposure |
| D06 | What must the user actually see and be able to do in Personal World/AutoWiki: project knowledge, conversation capture, profile, all three? | Source-backed live Wiki and correction; fuller personal profiling optional |
| D07 | Where does release stop: working on founder PC+phone, installable private beta, or public self-serve installer? | Separate private-alpha acceptance from public distribution; owner selects V1 milestone |
| D08 | Must offline capabilities, backup/export, live updates/rollback, and remote operation outside home network be ready for this release? | Local offline reads and safe restart/rollback; external network access and backup breadth explicit |
| D09 | Should Facebook/WhatsApp, autonomous agents, visual app creation and native external-app understanding be first-release journeys? | Preserve strategic programs; choose only named mandatory journeys |
| D10 | What exact limitations are unacceptable on day one? | No stale/mismatched UI, silent data loss, wrong-account effects, unverified coverage or duplicate sends |

Interview order: D01, D02, D07 first; then scope/depth questions D03–D06, D08–D10. Record each explicit answer as dated owner decision with source, impacted journey and changed boundary. No implied acceptance from silence.
