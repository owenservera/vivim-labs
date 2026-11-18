# Permanent Windows CLI

Run from any PowerShell, using the checkout's absolute installer path if needed:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\studio\scripts\windows\install-cli.ps1 -DataDir C:\VIVIM
vivim --help
vivim status
vivim doctor --json
vivim start --dry-run
vivim start
vivim stop
```

The installer requires an existing workspace `chrome-profile`; it never creates or substitutes a profile. Node.js 22 or newer is required by the existing Studio runtime. The installer captures its absolute executable path. Bun remains suitable for repository tooling, but this adapter preserves the existing Node runtime contract.

The stable command is `%LOCALAPPDATA%\VIVIM\cli\vivim.cmd`. The installer appends that directory to the **user** PATH only if absent, preserves all other entries, and broadcasts the environment change. It needs no administrator access, shell-profile changes, scheduled task, or service. Existing terminals may retain their inherited PATH; reopen their parent terminal application, or sign out and back in. Windows loads the persisted user PATH after reboot.

`installation.json` in the same directory stores the selected workspace, Studio port, Node executable and checkout entrypoint. Reinstalling without parameters preserves workspace selection. Keep the checkout at its installed path; after moving it, rerun the installer from the new location. `user-path-before-install.json` preserves the original user PATH for reference. To uninstall, remove only this CLI directory's entry from the user PATH and delete the CLI directory; do not restore the whole snapshot over newer environment changes.

As of 2026-10-31, `start`, including `--dry-run` and `-LanPair`, resolves the workspace manifest and passes its executable, profile and CDP port to `VIVIM.ps1`; invalid configuration fails before the launcher runs. `stop` delegates to `VIVIM.ps1` and supports `--dry-run`/`-DryRun` and `-Force`. Stop retains existing recording and process-ownership protections; it never closes Chrome. `status [--json]` reports Studio identity and health without launching anything; stopped Studio is a successful status query. `doctor` forwards its options to the existing Studio diagnostics. No Host/relay supervisor, provider send, sign-in verification, or automatic login-time startup is added. Full integrated startup and cold-reboot acceptance remain separate START01–START03 work.

## Installation evidence, 2026-10-31

The reported installation for commit `65e4bbb8ec3aa8545bde5fee641d8d61efaee6da` placed the launcher at `C:\Users\VIVIM.inc\AppData\Local\VIVIM\cli\vivim.cmd` and its directory on the persistent user PATH. `installation.json` recorded workspace `C:\VIVIM`, the existing Node runtime, and the checkout entrypoint; the profile is derived as `C:\VIVIM\chrome-profile`. Reinstallation was idempotent: the original PATH was preserved, the CLI directory appeared once, machine PATH was unchanged, the user PATH was persisted in HKCU, and an environment broadcast was sent.

In fresh Windows PowerShell 5.1 from `C:\` and PowerShell 7 from a temporary directory, `Get-Command` and `where` resolved `vivim`; `--help` and `status` exited 0. Status reported stopped with a stale owner record. `doctor --json` exited 0 while warning that CDP port 38702 was unavailable, three recordings were interrupted, and two profile warnings were present. `start --dry-run` exited 0 and selected the existing launcher profile and Studio port 38730; an actual `stop` exited 0 with no process to stop. The reported `bun run -- node --test studio/test/unit/windows-cli.test.mjs` result was 7 passed, 0 failed.

These are historical command observations, not current health. Actual browser startup, sign-in, cold reboot, and Host/relay supervisor acceptance were not tested. Check live ownership before any browser effect.
