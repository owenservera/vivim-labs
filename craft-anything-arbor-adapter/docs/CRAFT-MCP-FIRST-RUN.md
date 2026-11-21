# Craft MCP first run — exact tool names and observed Rust input schemas

Use this document **only for a VIVIM-owned MVP or other specifically authorized application**. These tool payloads are documentation-derived examples; no live session was executed in this chat. Ask the MCP server for `tools/list` on refreshed Craft HEAD before sending anything. Browser and profile ownership stays separate from Studio/Harness.

## 1. Open one persistent Craft MCP session

PowerShell from the Craft clone:

```powershell
cargo build --release -p craft-anything-cli
.\target\release\craft-anything-cli.exe --clean-room mcp --restricted --allow-web --allow-capture
```

Connect an MCP client to this process's stdio. Never run one-shot `web-site-open` followed by separate one-shot commands: each CLI invocation creates and closes its own browser. Don't use `--allow-run`; respect consent. The site must be owned/authorized, and the app may execute JS merely by opening the page.

## 2. Open the owned app

MCP tool: `web_site_open` (Rust `OpenParams`):

```json
{
  "url": "http://127.0.0.1:4173/demo",
  "width": 1440,
  "height": 900,
  "settle_ms": 2000,
  "headless": true,
  "signed_in": false
}
```

The URL is a placeholder example until the agent confirms the actually running ARBOR MVP address and path. Source result returns the final URL; record its **origin** separately. For a local backend serving a different port or path, change the exact URL instead of copying the example blindly.

## 3. Capture responsive surface

MCP tool: `web_site_capture` (Rust `CaptureParams`):

```json
{
  "output_directory": "C:\\captures\\vivim-site-01",
  "widths": [1440, 768, 390],
  "max_height": 12000,
  "include_html": false,
  "noise_pass": false
}
```

Directory must not already exist. Clean room will omit HTML regardless of this flag; we explicitly set it false. Capture generates `capture.json` plus screenshots/README. Preserve the original folder location and the evidence ID returned by the tool.

## 4. Expand safe visible states

MCP tool: `web_page_states_capture` (Rust `StatesParams`):

```json
{
  "output_directory": "C:\\captures\\vivim-states-01",
  "max_states": 8,
  "width": 1440
}
```

The Rust source limits `max_states` to 1–8, and may click menus, tabs, disclosures, or navigation. Treat this as active UI interaction, even though Craft allows it in Clean room. Do not run it on third-party logged-in services without authorization. Preserve `states.json` and screenshots in their original folder.

## 5. Export evidence as-is

MCP tool: `core_evidence_export` (Rust `ExportParams`):

```json
{
  "path": "C:\\captures\\vivim-craft-evidence-01.json",
  "overwrite": false
}
```

Exporter creates `{ "format": "craft-anything-evidence-bundle", "version": 1, "records": [...], "unknowns": [...] }`. Do **not** edit exported JSON before hashing: IDs are content-derived, and a data-field edit invalidates the record. Keep the entire bundle private; it may contain target content. Use `core_evidence_browse` to inspect a specific record instead of dumping large results into logs.

## 6. Import privately to Labs adapter

```powershell
node src/cli.mjs import --bundle 'C:\captures\vivim-craft-evidence-01.json' --site vivim-mvp --origin 'http://127.0.0.1:4173' --capture-dir 'C:\captures\vivim-site-01' --states-dir 'C:\captures\vivim-states-01' --out 'C:\captures\vivim-atlas-01'
```

Read `coverage.md` and `atlas.json`, and review the source/label risk. With `--owned-target --include-labels` short non-sensitive control labels are added; this option is still not a Host authorization grant. Never commit raw capture data or private report output to Labs or ARBOR.

## 7. Known failure conditions

- `EVIDENCE_HASH_MISMATCH`: numeric serialization edge cases or damaged/mutated Craft evidence. Verify with native Craft Rust crate; **do not turn off hashing**.
- `MIXED_TARGET_ORIGINS`: a bundle contains evidence from another site or conflicting URL anchors; capture in a fresh targeted Craft session.
- `EVIDENCE_ORIGIN_UNVERIFIED`: missing origin anchors; do not guess the target from `subject.name` (display-only, excluded from evidence ID).
- `CAPTURE_PROVENANCE_MISMATCH`: `--capture-dir` / `--states-dir` does not resolve to the actual folder recorded by the evidence record.
- `CAPTURE_NOT_ANCHORED` / `STATES_NOT_ANCHORED`: appropriate evidence record was not present.
- `EEXIST`: report directory already exists; select a new report directory.
- Craft sign-in/auth wall: do not interpret a login page as the complete app, and never paste cookies or session storage values into the CLI.