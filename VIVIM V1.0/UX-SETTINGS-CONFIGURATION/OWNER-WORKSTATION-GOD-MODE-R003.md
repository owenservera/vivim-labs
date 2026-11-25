# Owner workstation God Mode — settings integration R003
2026-11-25 | Owner decision | P0 development enablement

The current production VIVIM ARBOR connector is God Mode, not consumer mode. User Mode is deferred. Every authorized ChatGPT development session should use the same installed VIVIM connector for general PowerShell 7 terminal work, workspace access, stock development CLI programs and session output/input/cancel; no per-session application registration or second runtime. This is a standing locally owner-approved development profile, not permission to remotely alter its own grants, extract credentials, bypass OS privileges or authorize unrelated third-party effects.

## Existing configuration authority
Native workstation owner owns implementation and live deployment. Settings workstream owns the shared descriptors, discoverability and effective-state projection. Use the existing Host authority, runtime/terminal/root/grant bindings, local selection proposal and receipt lifecycle. No duplicate settings store, new browser controller, or new per-app MCP tools. The GPUI settings entrypoint and phone viewport remain the shared UX. Studio/Harness convergence remains on hold in this lane.

| Proposed stable setting ID (not registered yet) | Effective source / scope | Intended experience |
|---|---|---|
| workstation.profile.kind | current installed owner profile; manifest | God Mode on current connector, not a second instance; User Mode later |
| workstation.profile.workspace | owner-approved root selection and actual grant | Batch discovery of local development repos, no per-session configuration |
| workstation.terminal.access | Host terminal policy / actual admitted programs | General pwsh 7+ terminal, ordinary developer commands and existing session controls |
| workstation.profile.lifecycle | live grant expiry, local approval and release receipts | One standing profile review and supported renewal/revoke; actual state, not aspirational setting |
| workstation.profile.fileWrites | current grant | File mutation availability, separate from OS privileges of terminal |
| workstation.profile.interactive | current grant / terminal policy | Actual interactive capability |
| workstation.profile.release | installed runtime digest, health and promotion receipt | Stage, activate, reconnect, rollback; no fake ready state |
| workstation.integrations.craft | discovered stock CLI, admitted root/program and live version/doctor receipt | Native Craft 0.6.1 as delivered; no browser controller or fork |

## Implementation and evidence
[ARBOR batch setup](https://github.com/owenservera/ARBOR/commit/488ab7365a9c902e06e6346ed878b2bb0a7dcf60), New-CurrentOwnerProfile.ps1 discovers immediate repo siblings under selected workspace parent, includes Labs and stock Craft executable, creates one private reviewed proposal. Actual Windows scratch fixture via VIVIM ctl-3762ec66c7f33f36 exit 0: script parse, batch roots/program, missing-repo refusal, no grant mutation. Existing live grant remains arbor/scratch, 7 terminal programs. No actual expanded profile activation, Craft doctor, or per-user UI migration has been performed. Current P0 is owner-authorized activation of the existing connector, preserve working tunnel, verify live commands in new roots and Craft stock version/doctor/setup dry-run. Do not use old remote scope to grant itself new authority. No Desktop Commander calls.

[ARBOR operating contract](https://github.com/owenservera/ARBOR/blob/main/tools/mcp/vivim-arbor/windows/EXPANDED-OWNER-PROFILE.md). [Stock-first Craft contract](../../craft-anything-arbor-adapter/docs/STOCK-FIRST-OPERATING-R006.md).

## R004 — Owner directive: live workspace membership, no per-folder deployment

The current God Mode connector must let authorized ChatGPT sessions add and remove development folders on demand. A folder registration is ordinary runtime configuration, not a new binary release, plugin, tool schema or per-session installation. Host remains the single authoritative writer; integrate the existing Settings configuration domain and existing native file/terminal ports rather than introduce a separate store or renderer.

P0 implementation contract (proposed, not live): locally owner-approved standing development workspace scope; stable root IDs and revisioned membership; list/add/remove through the existing authenticated Host/MCP control path; validate canonical paths and exclude protected authority/credential locations and reparse escapes. Adding a folder already within the approved scope needs no separate local approval or build. Outside the standing scope requires the supported owner authorization path. Removal stops new managed operations on that root, reports running jobs and invalidates stale bindings; it must not delete files or be represented as OS sandbox confinement. Persist and read back the new revision; all authorized sessions see the same effective membership; no metadata refresh for ordinary membership changes. Existing terminal scope and Host-owned receipts remain intact. Tool list changes require discovery only once when the new operations are introduced. No Studio/Harness changes (R033 hold).

Acceptance: two ChatGPT sessions share the same revision; add a real approved development repo, list/read and execute a harmless command there, remove its registration, verify subsequent managed operations are denied, preserve files and unrelated sessions, restart/reconnect and verify persisted membership. Test stale revision, duplicate ID, protected path, unauthorized boundary, and concurrent modifications. No rebuild/restart/refresh per folder. Priority above further static-profile expansion. The current staged batch still uses the old fixed-root contract and does not satisfy this requirement.
