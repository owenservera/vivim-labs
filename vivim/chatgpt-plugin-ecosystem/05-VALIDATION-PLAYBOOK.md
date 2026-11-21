# Verification playbook: Desktop Commander installation → ARBOR plugin prototype → directory submission

**No tests have been executed in this study.** This file is a repeatable owner-local test plan, not proof. All first probes should run on a scratch directory/account; never ask the tool to dump user secrets or unrestricted profile data.

## Era 1 — Desktop Commander live smoke test (owner's Windows)

### Prerequisites

- ChatGPT installation/connection of **Remote Desktop Commander** completed by owner.
- Node installed. Prefer reviewing and pinning the npm package version; use vendor command only after inspecting its code/provenance.
- A dedicated disposable scratch folder, e.g. `C:\dc-scratch`, containing `hello.txt` with non-sensitive content; no private repo checkout, secrets or user home backup.
- A running device agent and verified pairing code. Record package version, OS version, device alias and tool catalog **without revealing tokens**.

### Sequence and evidence

1. Confirm ChatGPT lists the plugin's real available tools (not just successful directory installation). **PASS:** correct tool metadata and paired machine visible. **FAIL:** tool missing/OAuth/agent offline. **UNRUN** until observed.
2. Read `hello.txt` through the file tool and list only scratch folder. **PASS:** exact content + source path. No write.
3. Perform a harmless shell version check as a separately approved action (e.g. `rustc --version`), only if user wishes to exercise terminal. Confirm logging.
4. Start a short harmless interactive process, inspect its session and terminate it deliberately. Verify process isolation, output and exit.
5. Explicitly refuse a request to broaden directory permissions, read home secrets, or run a destructive shell command. **Policy result is not an OS guarantee**.
6. Stop the device agent; verify subsequent tool calls fail as offline and do not silently target another device.
7. Revoke device in dashboard and verify reconnection requires explicit re-pairing.

### Useful first prompt (read-only)

> Use Remote Desktop Commander to list files only in C:\dc-scratch, inspect hello.txt, and report the exact tool names and permissions you used. Do not change any file or run any shell commands.

### Evidence record template

~~~yaml
observed_at_utc:
plugin_installed: unknown
server_tool_catalog_version: unknown
package_version: unknown
device_alias: redacted
read_only_probe: UNRUN
shell_probe: UNRUN
process_probe: UNRUN
offline_probe: UNRUN
revocation_probe: UNRUN
commands_and_exit_codes: []
security_notes: []
~~~

## Era 2 — First-party Rust MCP gateway (ARBOR-owned, Labs designs only)

1. Pin current `owenservera/ARBOR@main` SHA, inspect existing Rust/Host interfaces and identify the smallest typed read-only port.
2. Define `vivim_capabilities` and `vivim_wiki_search` schemas + defensive input limits; no public shell.
3. Implement read-only queries and structural errors through existing Rust kernel/HQKG; ensure sensitive data is redacted and the Host scope is never skipped.
4. Implement remote Streamable HTTP and OAuth token validation against resource audience; require complete user/world mapping.
5. Exercise unauthorized scope, expired token, invalid audience, disconnected device, stale revision, concurrent requests, oversized input and prompt injection in source text.
6. Add a proposal-only action; fail closed until Host approval exists; run duplicate retry and unknown-result recovery.
7. Test with actual ChatGPT private plugin connection and preserve real commands/exits/receipts. No fake green tests.

**First acceptance gate:** a read-only search result with precise source/revision provenance plus demonstrated rejection of an unauthorized scope, all from an actual connected client.

## Era 3 — Public plugin release readiness

- [ ] Domain, business verification, publishing permissions and reliable production HTTPS endpoint available.
- [ ] User-facing privacy policy, terms, website, support, account disconnect and deletion processes documented.
- [ ] One public product purpose and legal basis; no unsupported third-party unofficial connector features.
- [ ] MCP tool schemas, consistent naming, structured errors, request size limits and timeouts proven.
- [ ] Every tool has accurate `readOnlyHint`, `openWorldHint`, `destructiveHint` and written justifications.
- [ ] OAuth scopes and reviewer demo credentials work without inaccessible second-factor onboarding.
- [ ] Domain challenge served exactly at `/.well-known/openai-apps-challenge` and verified in portal.
- [ ] Exactly five positive cases and three negative cases, product demo video, release notes and screenshots where UI exists.
- [ ] ZIP package assembled, automated checks passed, current MCP scan passed, reviewer submission accepted.
- [ ] Approved release published explicitly; operational monitoring, rescans and incident/revocation runbook owned.

## Implementation handoff for local agent (copy into ARBOR work queue only when authorized)

> **Goal:** Evaluate a narrow Rust-native read-only MCP facade in ARBOR, but DO NOT begin by creating a second Host/effect engine. Inspect latest main and its existing Rust kernel, Host authorization/receipt/journal, HQKG wiki retrieval and device pairing. Map `vivim_capabilities` and `vivim_wiki_search` to exact callsites. Deliver source/contract gaps and, only if safe, a small working read-only prototype with real tests and source SHA. No provider APIs, no raw arbitrary terminal tool, no separate KB, no new approvals or permission paths, no JavaScript core. Use TypeScript 7+ only for necessary UI glue. Preserve main and other changes. Use the Labs research path `vivim/chatgpt-plugin-ecosystem/` as non-authoritative design input.

## Sources

- https://desktopcommander.app/help/getting-started/what-is-desktop-commander
- https://desktopcommander.app/help/security-privacy/use-safely-limit-access
- https://desktopcommander.app/help/security-privacy/data-privacy
- https://github.com/desktop-commander/remote-desktop-commander
- https://developers.openai.com/plugins/build/plugins
- https://developers.openai.com/plugins/deploy/submission
- https://developers.openai.com/plugins/deploy/submission-errors
- https://developers.openai.com/plugins/deploy/desktop-only
