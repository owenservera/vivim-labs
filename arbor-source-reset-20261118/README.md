# ARBOR source-first reset — one-time staging

**State: remote archival complete. All 671 initial ARCHIVE candidates were copied into this VIVIM Labs folder and verified against their original Git blob SHAs. No ARBOR files were removed, renamed or modified.**

Snapshot: [ARBOR main @ 516a429599418f629b1bb343e47828a6bf22a532](https://github.com/owenservera/ARBOR/commit/516a429599418f629b1bb343e47828a6bf22a532), 2510 Git-tracked blobs. This inventory does **not** cover untracked, ignored, unstaged, or Windows-local-only files.

## Intent

Keep ARBOR focused on executable VIVIM product source, app assets, tests, runtime contracts and build-essential files. Move historical plans, agents' written instructions, architecture, README files, coordination, research and receipts into this VIVIM Labs folder, with original relative paths preserved under `files/`.

This is a one-time **copy-before-removal** reset. The eventual clean ARBOR tree should retain only source and the minimal metadata the product actually needs. Reintroduce vision later as short, source-linked notes instead of reviving old process machinery.

## Complete inventory

`tracked-files.tsv` has **every one** of the 2510 tracked files, its original Git blob SHA, size, initial classification and reason.

- **ARCHIVE (671)** — high-confidence non-code copy candidates, including Markdown anywhere, `.coordination/`, historical `evidence/` and documentation-oriented sections of `.integration/`.
- **REVIEW (285)** — ambiguous files, especially `.integration/` code/schemas, `integration/` scripts, `docs/` JSON machine state (including the live roadmap), `scripts/` and CI configuration. Assess consumers before later removing from ARBOR.
- **KEEP (1554)** — presumptive source, tests, contracts, build configurations and runtime assets. This is a **default**, not proof that every file belongs in the slimmed product.

**Remote copy verification:** 671 source files matched 671 destination files by path and Git blob SHA; zero missing, mismatched or extra. The 285 REVIEW files have not yet been archived. The classifications are a *copy triage*, **not deletion authorization**. Some `.md` or `docs/` artifacts may have live readers; test those before eventual removal. Likewise, some program files may be obsolete.

## Optional local reconciliation (not executed)

The remote archive has already been pushed to VIVIM Labs. On your Windows PowerShell 7 machine, these optional commands can reconcile the remote archive against your local ARBOR working tree after pulling **vivim-labs**:

```powershell
Set-Location 'C:\0-BlackBoxProject-0\vivim-labs'
pwsh -File '.\arbor-source-reset-20261118\copy-archive.ps1'
# Dry run only. To actually copy the ARCHIVE candidates:
pwsh -File '.\arbor-source-reset-20261118\copy-archive.ps1' -Apply
```

The script compares already archived files, and if any are missing locally, copies selected tracked files from `C:\0-BlackBoxProject-0\ARBOR` to this folder's `files/<original-path>`, checks the pinned Git blob and compares copied bytes; it never removes files, stages files or commits. It refuses changed or conflicting files.

Before running, inspect local-only material with:

```powershell
git -C 'C:\0-BlackBoxProject-0\ARBOR' status --short --untracked-files=all
git -C 'C:\0-BlackBoxProject-0\ARBOR' ls-files --others --ignored --exclude-standard
```

## Removal gate for a later pass

1. Review every **REVIEW** file and any ARCHIVE file with actual imports or runtime consumers; move indispensable executable pieces to normal source locations.
2. Copy all approved non-source files and verify matching content, including local-only items.
3. Confirm product PWA → Host → core → Wiki journey works before **and after** removing anything.
4. Remove from ARBOR **only after** successful backup and real product verification. Do not confuse the existence of Git history with preservation of uncommitted local data.

No new agent framework, process system or roadmap is introduced here.
