# SCF acceptance tests and denial corpus — normative intended tests, UNRUN
## Agent/source metadata
T01 first-party touched file carries v1 context+owner and 1+ registered capability refs or explicit audited classification waiver.
T02 same file 3 capabilities compiles 3 declared membership edges; each independently verified and not automatically executable.
T03 feature with 40 files creates one canonical passport with 40 source contributions, no duplicated owner.
T04 file vs symbol scope precise across JS/TS/Rust/Python comments; annotation string/regex/template false positives rejected.
T05 malformed/unknown/deprecated/duplicate semantic ID diagnoses line and blocks effective promotion.
T06 source header/tag alteration without code change yields source declaration delta, not runtime delta.
T07 moved file same blob and stable ID preserved; moved to new repo without accepted origin gets candidate lineage, not certified replacement.
T08 removed annotation while implementation survives detected via symbol/consumer inference and labeled unannotated candidate.

## Compiler/semantic/versions
T09 source bytes and sha pin independently verified, with canonical stable digest across ordering and repeated runs.
T10 2,3,5 immutable pins include same repo at three commits; no ref re-resolve, branch drift cannot mutate historical snapshots.
T11 pre-v3 v1/v2 parser existing behavior identical; unknown v3 syntax in opt-out source never silently accepted.
T12 contract change invalidates only dependency fanout; unrelated wiki pages/IR unchanged.
T13 compiler emits typed source-to-capability links with explicit declared/derived/evidence statuses; no scalar "confidence=verified" shortcut.
T14 HVI atlas still reproduces 676/315/314/1/361 fixed HQ source measurements, with one record per missing HQ path.
T15 unannotated Labs code remains unknown/research, docs-only feature remains design; missing files in partial scan not marked retired.
T16 similar names or matching comments in different repos do not create canonical ID merge/owner promotion.
T17 checked-in test ref exists but test UNRUN; no fictitious passed test receipt or Host handler.
T18 exact valid source matches but live Host catalog changes: stale owner action crosswalk rejected; show missing active action.

## Security/runtime/data
T19 source path secret, symlink escape, ../ traversal, oversized file, generated/binary/non-owned source handled via exclusions, not leaked.
T20 untrusted comment containing "run shell", grant instructions, Markdown prompt-injection, suspicious HTML/SVG never treated as tool instruction or executable code.
T21 source permissions revoked mid-query cause denial; no cached confidential content from unapproved root or personal World.
T22 no source comment can execute/enable an action, write Host World, create a permission, send a provider message, or publish UI outside existing owner.
T23 low disk/ENOSPC or broken cache preserves last verified immutable snapshot and Host originals; incomplete outcome explicit.
T24 combined source-only Node tests vs real Rust, Host, Windows and phone acceptance separated in receipts.
T25 agent modifies exported function without updating annotation, compiler flags mismatch/unknown proof; policy not blindly trusting metadata.
T26 semantic identity collisions and source-linked v3 migration equivalence lacking reviewer receipt fail closed.
T27 differential tests for source-to-semantic-to-UI/Host adapter: no double writer, old parser and current catalog remain authoritative.
T28 benchmark incremental reparse cost (changed units+impact) and snapshot read latency with observed OS/CPU/repo size; **no claimed speedup until measured**.

## Required proof
Every executed case: `testId, command, cwd, runner/version, exact source SHA, stdout/stderr digest, exit code, duration, artifacts, failure, environment`. Unrun case: `UNRUN` with reason. A parser-only fixture is not integrated product verification. Device/Windows PWA or signed-in Chrome tests only PASS with actual physical or verified runner receipts.
