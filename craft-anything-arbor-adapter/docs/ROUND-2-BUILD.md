# R002 — executable surface-intelligence bridge, native parity path and profile reconciliation

**Date:** 2026-11-21. **Status:** implementation tested locally; actual Craft/Rust capture + ARBOR runtime integration UNRUN.

## Scope and result

This round continues R001 **without modifying ARBOR's product runtime or the Craft clone**. It upgrades Labs' research-only importer and adds a source-profile comparison lane. Its goal is to make real Craft website captures useful to Studio/Harness developers before any broad Rust port, while keeping all discovery records inert.

### Actual new components

- `src/adapter.mjs` now recognizes Craft blueprint HTML tags `a`, `input`, `textarea`, `select`, headings and navigation as standardized roles. Password/hidden/file input nodes are excluded. Noninteractive heading/body text is never imported as a control label.
- Optional `capture.json` import cross-checks actual outline/headings/links/forms/image counts against the content-hashed Craft evidence summary when supplied. Design summaries are checked if recorded in evidence. PNG files referenced by Craft evidence are checked against its SHA-256, with path constraints and strict 32-MiB per-image bound. The **capture.json itself still lacks a Craft-signed digest**, so that limitation remains explicit.
- Optional `states.json` import matches the exact `states` array embedded in the verified Craft record, the URL and capture width. Referenced state screenshots have their attested SHA-256 checked. No UI actions are performed.
- Accessible structure is summarized by bounded counts (unnamed controls, missing alt tags, landmarks) without exporting page text.
- Unknown IDs now follow Craft's actual `unk_` + 16 hex format, with validated revisions/severity/status. Raw questions remain excluded.
- Duplicate evidence records may vary by display-only `summary`, `subject.name`, and `carries_target_content`; identical content-derived IDs deduplicate. Hashed-content changes still refuse.
- `src/reconcile.mjs` compares a normalized atlas with an **explicit, operator-supplied JSON source baseline**. It never imports/evaluates `.mjs` provider code. It produces `reconciliation.json` and `reconciliation.md`, including source selector drift, unverified label correspondences and unmapped *observed* labels.
- `src/diff.mjs` compares two atlases with the same target identity, including evidence-ID and role-count changes; it **does not infer a capability removal or feature regression from an absent observation**.
- `src/native-verifier.mjs` adds an opt-in subprocess boundary to a pinned Rust executable. `tools/vivim_verify_bundle.rs` is a small Rust example using Craft's actual `craft_anything_evidence::Evidence::verify`. It prints a digest/ID/count receipt without target content. `--native-bin` requires actual process execution and validates the returned full bundle digest and IDs before import. The Rust helper was **not executed** in this environment; Cargo is unavailable here.
- Four versioned JSON schemas include new profile baseline, reconciliation and diff contracts; source-only `baselines/claude-source-baseline.json` captures actual inspected Studio/Harness configuration differences, pinned to Git blob SHAs.

## Current source inventory

| Owner | Path | Source status |
|---|---|---|
| Craft evidence | `crates/evidence/src/lib.rs`, `canonical.rs`, `unknown.rs` | Inspected; native ID + `unk_` format confirmed from code |
| Craft export | `crates/engine/src/tools/session_tools.rs` | Native `format=craft-anything-evidence-bundle`, `version=1` |
| Craft web | `crates/engine/src/tools/website_tools.rs`, `browser_tools.rs` | Source of `capture.json`, `states.json`, PNG digest attestation |
| ARBOR Studio | `studio/src/studio/adapter-evidence-intake.mjs`, `adapter-factory-service.mjs` | Core-qualified original capture requirement, read-only learned-adapter lifecycle |
| ARBOR Claude | `studio/src/profiles/sites/claude.mjs` | Profile source only; hydration-specific composer implementation |
| ARBOR Harness | `harness/src/providers/claude.mjs` | Profile source only; retains generic textarea fallback |

## CLI reference

```powershell
# From vivim-labs/craft-anything-arbor-adapter
node --test test/*.test.mjs

# 1. Development-only research import; no Rust claim unless --native-bin is supplied.
node src/cli.mjs import --bundle 'C:\captures\craft-evidence.json' `
  --site vivim-mvp --origin 'http://127.0.0.1:4173' `
  --capture-dir 'C:\captures\site' --states-dir 'C:\captures\states' `
  --out 'C:\captures\atlas-r002'

# 2. Reconcile with a JSON profile baseline for the same target (never import .mjs directly).
node src/cli.mjs reconcile --atlas 'C:\captures\atlas-r002\atlas.json' `
  --baseline 'C:\captures\vivim-source-baseline.json' --out 'C:\captures\reconcile-r002'

# 3. Compare two genuine authorized capture atlases, same siteId and origin.
node src/cli.mjs diff --before 'C:\captures\atlas-r002\atlas.json' `
  --after 'C:\captures\atlas-r003\atlas.json' --out 'C:\captures\diff-r002-r003'
```

Each output directory must be **new**. No input files are modified, and nothing is sent to a browser, provider, HQKG or Host.

### Running Craft's own native identity verifier

**Do not install a second Rust dependency tree or rewrite Craft's hash code.** Use the cloned Craft workspace and copy the Labs example into the existing evidence crate:

```powershell
$Labs = 'C:\path\to\vivim-labs\craft-anything-arbor-adapter'
$Craft = 'C:\path\to\craft-anything-VIVIM'
New-Item -ItemType Directory -Force "$Craft\crates\evidence\examples" | Out-Null
Copy-Item "$Labs\tools\vivim_verify_bundle.rs" "$Craft\crates\evidence\examples\vivim_verify_bundle.rs"
Push-Location $Craft
cargo build -p craft-anything-evidence --example vivim_verify_bundle
$NativeExe = "$Craft\target\debug\examples\vivim_verify_bundle.exe"
& $NativeExe 'C:\captures\craft-evidence.json'
Pop-Location
```

The compiled `vivim_verify_bundle.exe` must be the real example built from the Craft clone. To require native hash verification during the Labs import:

```powershell
node src/cli.mjs import --bundle 'C:\captures\craft-evidence.json' --site vivim-mvp `
  --origin 'http://127.0.0.1:4173' --native-bin $NativeExe `
  --out 'C:\captures\atlas-native-verified-r002'
```

The native receipt checks only consistency and IDs, **not a security signature or an authorization grant**. It does not write the original bundle or create a Craft session. Keep the Rust example uncommitted in Craft unless separately approved; canonical implementation stays in Labs.

## Tests and verified status

Command executed: `node --test test/*.test.mjs` under Node v22.16.0. **19 PASS / 0 FAIL.** Includes CLI import/reconcile/diff processes, capture provenance, screenshot byte-digest tampering, state artifact inconsistency, source selector divergence, label-only hint/non-execution, mixed-origin refusal, deduplication, invalid unknown IDs and native-receipt protocol validation. These are isolated contract tests, NOT live product data.

Schema validation: four Draft 2020-12 schemas passed Python `jsonschema` schema validation; the Claude source baseline passed its schema and runtime validator. No actual Craft-generated bundle was supplied; Rust helper compilation and execution, native hash roundtrip, ARBOR Studio/Harness readback and real MVP capture remain **UNRUN**.

## Risks and next work

1. **Native Rust check in local workspace first.** The Labs JS digest uses JavaScript number serialization; Craft uses `serde_json`. For floating-point and lexical-integer differences, JS hash mismatch can occur. Pass `--native-bin` to require actual Craft-native ID verification of every record; otherwise output `source.nativeVerified=false`.
2. **Real first-party capture.** Use Craft's own isolated Chromium on a VIVIM-owned MVP and export `core_evidence_export` from the same MCP session. Choose new capture directories. Do not assume Claude.ai authenticated automation is authorized by a paid subscription.
3. **Actual source baseline generation.** Inspect refreshed ARBOR Studio and Harness source, pin file blob digests, export a site-specific JSON baseline following `schemas/application-profile-baseline.schema.json`. The Claude baseline is source-only research and never executable.
4. **Read-only Studio port next.** Import a verified atlas as untrusted external research evidence through a new development-only read projection, with Core-qualified source authorization separate from Studio's first-party `aion.adapter-evidence-intake/1` capture acceptance. Do not give Craft a fake capture ID, receipt or send permission. Retain one existing owner for graph/journal and provider effects.
5. **No automatic selector generation/promotion.** A label or role correspondence never proves unique target identity. Promote only after genuinely held-out Studio observations plus Host's existing authority.

**Next-thread entrypoint:** run the Rust helper against a genuine Craft export, then feed a verified first-party atlas and a real site baseline into this R002 CLI. Attach the exact artifacts privately; do not upload personal conversations/screenshots or captured session content to public repos.
