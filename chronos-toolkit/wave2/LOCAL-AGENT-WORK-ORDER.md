# Local integration writer work order — Chronos Wave 2

Read `README.md`, `EVIDENCE.md`, `changes/replacements.json`. User asked to integrate across VIVIM; this is an implementation handoff, **not** a green release claim.

1. Check exact five repo `main` SHAs and all dirty paths/active lane leases. Preserve existing unrelated work. Do not fetch-reset a dirty worktree.
2. Verify no local owner is writing Rust `Cargo.toml`/`lib.rs`, Web Harness `conversation-runtime.mjs`/`conversation-api.mjs`, or newly introduced Chronos files. If they are leased or modified, stage separate safe worktrees or stop and report conflicts.
3. Read contract and source, run offline `INTEGRATE-ALL.mjs --root ... --check`, review all 20 target paths. If any context drift exists, reconcile rather than overwrite.
4. Run `--apply` explicitly **only in lease-safe working trees**. No automatic commit. Review diff, especially source identity, auth entrypoint, route method and status codes.
5. Run `node SOFTWARE_HQ/scripts/chronos-verify.mjs --root ... --run`; capture exact stdout, exit statuses and Rust importer/feature test failures. Independently re-run source-only security negative fixtures.
6. In normal signed-in Host only, owner-approved read-only GET `/api/v1/conversations/<known-id>/timeline`; confirm POST 405, unknown ID refusal, no provider send or browser effect, no prompt/reply content leaked.
7. Gate activation: no reminder registration/scheduled execution until durable CAS, effect-time grant, unique cross-process claim, revocation and crash-uncertain reconciliation are proven. Calendar scheduling across DST and native Android alarm behavior are separate physical acceptance gates.
8. Only the canonical integration writer may commit/push with fast-forward ancestry and evidence. Publish final SHAs, test receipts, affected paths, blocked gates and rollback steps. Never claim main or physical device GO based on this source package.
