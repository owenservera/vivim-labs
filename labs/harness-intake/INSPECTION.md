# Harness candidate — source inspection / acceptance checklist

## A. Source identification
- [ ] New repo owner/name URL confirmed (do not guess).
- [ ] Default branch, full SHA, licensing and attribution inspected.
- [ ] Source tree and binaries inspected, and top-level dependencies mapped.
- [ ] Upstream update strategy considered without auto-ingestion into ARBOR.

## B. Architectural fit
- [ ] Compare candidate to existing ARBOR `harness/`, `apps/`, `native/`, `studio/`, `wiki/` and `contracts/`.
- [ ] Show exact files and symbols to **reuse**, modify, or leave untouched.
- [ ] No additional UI authority: native GPUI is the intended master surface; phone receives matching semantic view/state.
- [ ] No Edge/WebView as required PC application UI; avoid inventing unsupported GPUI parity.
- [ ] Preserve Host effect authority, explicit approval, receipt, provenance and recovery.
- [ ] Rust first, TS 7+ only where necessary; keep a small, testable interop seam.

## C. Behavior-first acceptance (must demonstrate a visible change)
1. Type a short instruction in the PC command composer: state and visible text update without unexplained stall.
2. AutoWiki opens/responds incrementally to the same command; each projection is revision-aware.
3. Mobile PWA shows semantically equivalent command and wiki state, with documented transport and latency measurement.
4. Long press / hard press shows immediate feedback; cancel or release has an observable transition (later phase, if source supports it).
5. Reload or reconnect restores approved state without duplicate side effects.
6. Unauthorized/unavailable provider effect shows a truthful blocked/pending result, never a fake success.

## D. Verification
- [ ] Source baseline pinned with GitHub URLs and immutable SHA.
- [ ] Actual build/test commands, version and exit codes recorded; `UNRUN` where unavailable.
- [ ] Desktop GPUI screenshot after typed input.
- [ ] Desktop GPUI screenshot with AutoWiki projection.
- [ ] Mobile PWA screenshot of corresponding revision.
- [ ] Record mismatches, logs safely redacted, and measured parity failures.
- [ ] Regression and rollback documented; ARBOR main implementation commit linked.

## E. Decision format

```yaml
source_repo: owner/repo
source_commit: UNVERIFIED
source_path: UNVERIFIED
license: UNVERIFIED
candidate: UNVERIFIED
existing_arbor_path: UNVERIFIED
decision: UNDECIDED
duplicate_or_conflict: UNINSPECTED
user_visible_acceptance: UNDEFINED
checks_run: []
device_checks: UNRUN
owner_approval: PENDING
```

**Publication rules:** no personal logs, private browser sessions, passwords, Git bundles, quarantined PC source or private archive contents in public Labs. Research approval is not authorization to import or deploy.
