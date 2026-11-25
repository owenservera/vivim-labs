# Owner workstation God Mode — R001
2026-11-25 | Owner requested whole-PC access for private VIVIM ARBOR plugin, not ordinary product default.

## Requirement
Provide a distinct locally owner-enabled workstation profile for broad development access to owner-selected local drives, repositories, terminal programs, and installed applications, within Windows account privileges. Never expose this mode as default onboarding, grant itself from a remote session, bypass OS ACLs, or expose credential material. Retain private authenticated transport, local revocation, command/session receipts, bounded grant lifetime, recovery and explicit ownership. Broad workspace authority does not authorize third-party effects or credential extraction.

## Source-grounded current blocker
Existing ARBOR `tools/mcp/vivim-arbor/windows/Install-WorkstationProfile.ps1` hardcodes `arbor` and `scratch` in roots.json, tunnel grant, and terminal policy. Runtime manifest and local Host policy pin config digests; changing one JSON file is not a valid upgrade. Current remote tool catalogue does not expose an owner grant-management operation. Read-only native filesystem access works for arbor/scratch; earlier terminal probe was denied/uncertain. Do not circumvent this boundary or repeat uncertain effects.

## Handoff to native workstation owner
Extend the supported local profile installer and staging workflow with an explicit local owner-selected expanded-root profile and installed program registry. Preserve ordinary defaults and verify actual Rust path handling/Windows ACLs, protection of authority/credential material, local revocation and rollback. Do not let remote tools edit their own authority. Keep root/terminal/tunnel/manifest digests coherent; stage, verify and locally activate through the existing owner-controlled path. Include Craft and Labs repositories and the compiled Craft CLI first; validate all other selected drives before claiming whole-PC coverage. Live acceptance: grant summary, harmless directory listing and file read in each selected root, harmless native terminal command, Craft CLI version, persistent MCP tools/list, revoked access denied. No raw credentials or private data in Git.

Status: owner requirement documented; no permissions changed or deployed in this session. This is a handoff to the existing workstation independence owner, not a second runtime or bypass.
