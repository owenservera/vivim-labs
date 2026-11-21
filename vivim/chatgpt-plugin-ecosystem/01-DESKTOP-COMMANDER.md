# Desktop Commander — architecture, capabilities, permissions and economics

**Research date:** 2026-11-21. **Verification:** official documentation and open-source local source references; hosted relay implementation not independently audited; live end-to-end tests UNRUN.

## 1. Two products, do not conflate

| Dimension | Desktop Commander (local MCP) | Remote Desktop Commander |
|---|---|---|
| Client | AI applications with local MCP support | ChatGPT web, Claude web and remote MCP clients |
| Transport | Local stdio MCP | Public `https://mcp.desktopcommander.app/mcp` Streamable HTTP + OAuth |
| Code | Open-source local server at `wonderwhy-er/DesktopCommanderMCP` | Public manifests/docs at `desktop-commander/remote-desktop-commander`; hosted service proprietary |
| Where effects run | User's machine | User's machine, via paired running agent |
| Middle service sees tool arguments/results | No Desktop Commander hosted relay | Yes, messages transit their service |
| Offline | Local server/client may work offline | Requires hosted relay connectivity |
| Scope | User account privileges unless OS isolation | Same privilege boundary, plus remote authentication/device pairing |
| Multiple devices | Local process per machine | One account may pair multiple devices |

The remote path is: ChatGPT model picks tool → ChatGPT MCP client sends authenticated call → hosted Desktop Commander relay → authenticated running device agent → local filesystem/shell/process → result via relay to ChatGPT. OAuth 2.0 + PKCE authorizes the ChatGPT client; device pairing uses a device authorization flow; server TLS encrypts in transit. Running an agent is a prerequisite: stop with Ctrl+C to disconnect. These are vendor architectural descriptions, not a cryptographic audit.

## 2. Surface map

- Files: read one/many files, recursive list, metadata, write/append, move, create folders, targeted block edits, PDF generation/edit and some Excel-aware operations.
- Search: file-name or content searches with paged results, session lifecycle and cancellation.
- Shell: start interactive persistent process, write to its stdin, read output, list sessions/processes, terminate processes. Persistent state is **per process/session**, not shared across independently started shells.
- Configuration: inspect and set allowed directories, blocked command names, file pagination, default shell, telemetry.
- Remote-specific: list paired devices, device ping/revocation and some account/device controls.
- Audit/help: recent tool call log and usage statistics.

Context7's inspected tool registry (`src/tools/schemas.ts` in local upstream) enumerated get_config, set_config_value, read_file, read_multiple_files, write_file, write_pdf, create_directory, list_directory, move_file, start_search, get_more_search_results, stop_search, list_searches, get_file_info, edit_block, start_process, read_process_output, interact_with_process, force_terminate, list_sessions, list_processes, kill_process, get_usage_stats, get_recent_tool_calls, give_feedback_to_desktop_commander and track_ui_event. Remote may add device-specific tools, and actual installed server versions can expose different tool lists.

## 3. ChatGPT on Windows — conservative install

1. Connect **Remote Desktop Commander** from ChatGPT's plugin directory / Settings → Plugins or Apps.
2. Sign into Desktop Commander and open its device-pairing page; independently inspect the install script if using one.
3. Since Node is available, a less opaque advanced launch on Windows is:
   ~~~powershell
   npx.cmd @wonderwhy-er/desktop-commander@latest remote
   ~~~
4. Verify the terminal pairing code matches the browser code before approving. Keep terminal running.
5. In a new ChatGPT chat test read-only enumeration of a temporary scratch directory. Confirm device name, file listing and command audit; do **not** begin with the repository root, home directory or secrets.
6. Review at https://mcp.desktopcommander.app and revoke unused devices. Ctrl+C stops availability.

The remote open-source device client may auto-pull latest versions when invoked with @latest: pin a reviewed version for sensitive environments. A vendor setup script may instead be used but should be read before piping into a shell.

## 4. Privacy and safety: decisive caveats

- By default it can read and modify files and run commands with **all permissions of the logged-in operating-system user**. It is not read-only by default.
- `allowedDirectories` applies to **filesystem MCP operations**, not shell commands. A shell can traverse outside those folders; blockedCommands is also no security sandbox. Do not advertise allowed-directories as full containment.
- If you need a real boundary, isolate using a dedicated low-privilege OS user, VM, container with mounts/network constraints or separate PC. For Windows, a dedicated standard-user account and restricted ACLs are straightforward, but actual confinement must be verified.
- Vendor says remote requests and responses pass through its servers and are transiently stored; account/device metadata and usage stats persist. Main servers described as EU-based with possible subprocessors elsewhere. The model provider also receives data returned into the chat.
- For local MCP, vendor says no tool contents go to Desktop Commander servers but anonymous usage telemetry can be enabled; your AI app may still upload the tool results.
- Desktop Commander logs tool actions locally under the user profile's `.claude-server-commander` directory. Log access/retention need inspection; don't commit these logs to Labs.
- Prompt injection can enter via README, issue, downloaded text or browser content. Treat the contents of tools as data, never delegated authority to broaden scope.
- Maintain explicit approvals for commands, file changes, terminal process manipulation and network/exfiltration operations. MCP tool descriptions are not an OS safety boundary.
- Treat plugin account authentication and device pairing as remote administrative access to the connected user's account; MFA and revocation are necessary.

## 5. Pricing snapshot (not a guaranteed future offer)

Official pricing page on 2026-11-21: **Remote Free $0/month with 10,000 tool calls/month**; **Remote Pro $20/month plus applicable VAT with advertised unlimited tool calls**; **local MCP free and open source**. Vendor service quotas and ChatGPT plan/model restrictions are distinct. "Unlimited" is vendor-plan wording, not a measured throughput guarantee.

## 6. Source/truth matrix

| Claim | Evidence | Status |
|---|---|---|
| OAuth remote MCP URL, running agent, pairing | GitHub remote README | Official documented |
| Local vs remote transport | Vendor getting-started docs | Official documented |
| Filesystem + terminal tool schema | Context7 local upstream README and `src/tools/schemas.ts` | Code/doc extracted; may version drift |
| Default privilege and sandbox caveat | Vendor safety help, local upstream README | Explicit vendor warning |
| Remote requests transit hosted relay | Vendor privacy help | Self-reported service policy |
| Price and calls | Vendor pricing | Time-sensitive snapshot |
| Exact current install behavior on owner's Windows | Not executed | UNRUN |
| Cryptographic end-to-end privacy, independent security audit | Not established | UNKNOWN |

**Sources:** https://github.com/desktop-commander/remote-desktop-commander ; https://github.com/wonderwhy-er/DesktopCommanderMCP ; https://desktopcommander.app/help/getting-started/what-is-desktop-commander ; https://desktopcommander.app/help/security-privacy/use-safely-limit-access ; https://desktopcommander.app/help/security-privacy/data-privacy ; https://desktopcommander.app/pricing/
