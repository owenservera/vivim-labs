# WNI03 — native-terminal-agents

Priority: P0. Integration owner: workstation + Host. Source readiness: src/session.rs (receipt-fenced lifecycle reducer); ConPTY and ACP transport pending. All public action candidates remain proposed until wired and independently verified.

## Extraction
Donor WTA master pools agent CLI connections and routes ACP sessions to owners. Reuse protocol discovery and lifecycle ideas; do not copy its renderer, COM terminal controller, named-pipe server or install another terminal. Extend existing Windows Job Object worker with optional CreatePseudoConsole, separate pipe IO, PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE and deterministic shutdown. ACP profile selection uses existing provider registry and cost policy. Keep approval/receipt authority in Host; treat delegate proposals as proposals, not completed effects.

## Shared capability actions
- `windows.terminal.capabilities.read` / `vivim_windows_terminal_capabilities_read` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.agent.sessions.list` / `vivim_windows_agent_sessions_list` — R0; fields: none. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.agent.session.status` / `vivim_windows_agent_session_status` — R0; fields: session_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.agent.session.start` / `vivim_windows_agent_session_start` — R2; fields: profile_id, scope_id, request_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.
- `windows.agent.session.cancel` / `vivim_windows_agent_session_cancel` — R2; fields: session_id. Same Host action from GPUI, mobile, VLANG and MCP. Exact user/session grant; results carry receipt or observation evidence.

## Acceptance
Real ConPTY echo/resize/interrupt, no stuck handles, child cancellation, two owned ACP sessions with no output mix-up, settled receipt and no unexpected provider charges.

## Source references
- https://github.com/microsoft/intelligent-terminal/blob/7ef2e637b810d58f20ef53f61275a0ec849cf26f/tools/wta/OVERVIEW.md
- https://github.com/microsoft/intelligent-terminal/blob/7ef2e637b810d58f20ef53f61275a0ec849cf26f/doc/agent-profiles.md
- https://github.com/microsoft/intelligent-terminal/blob/7ef2e637b810d58f20ef53f61275a0ec849cf26f/doc/security-model.md
- https://learn.microsoft.com/en-us/windows/console/createpseudoconsole

## Integration handoff
Review existing code and claims before edits. Stage through existing source-pinned build, no new daemon, no automatic effects or grants. Register candidate IDs in existing Host catalog; only mark source_wired after actual route/adapter exists. MCP and GPUI must use same dispatcher and live authorization; readback, cancellation and unsupported outcomes are explicit. Store approved observations in existing semantic fabric, never a competing database. Native collector and app acceptance are UNRUN unless documented separately.
