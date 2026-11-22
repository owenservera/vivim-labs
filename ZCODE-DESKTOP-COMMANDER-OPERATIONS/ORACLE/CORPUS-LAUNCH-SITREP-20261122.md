# Oracle personal corpus — R001 source + ZCode verification SITREP

Date: 2026-11-22 Europe/Madrid. This is a factual bounded receipt, not a claim that live ChatGPT extraction is running.

## Source owner and current tested pin

- Canonical product: owenservera/ARBOR main, commit `194ea96e643607f9bda2b17d036afc9e5465e22b`.
- New capability: F02/F01 plus five D03–D07 owner-reviewed ChatGPT discovery prompts; background Host scheduler reuses the existing Studio send-once journal and trusted SQLite corpus intake. Native Oracle/AutoWiki source-backed retrieval of these new facts remains an unverified downstream connection.
- New ZCode workflow: ARBOR `.zcode/workflows/arbor-oracle-corpus-verify.dwf.ts`. It is a read-only deterministic verification DWF, requiring a clean checkout at `origin/main`; older write-capable ARBOR workflows must not run unchanged.
- Cloud/GitHub source edits on main only. Local owner's working ARBOR checkout had unrelated dirty files, untouched.

## Actual post-push Windows test evidence

- Desktop Commander device `DESKTOP-9KB03LU` online and responsive.
- `git fetch --no-tags origin main` observed `194ea96e643607f9bda2b17d036afc9e5465e22b` in FETCH_HEAD; local dirty checkout was NOT reset, patched, built, committed or pushed.
- Generated isolated temporary source archive `%TEMP%\arbor-oracle-194ea96` using `git archive`; no user content or credentials copied.
- Command executed from extracted archive:

```text
node --test apps/vivim-host/test/oracle-onboarding-prompts.test.mjs apps/vivim-host/test/onboarding.test.mjs apps/vivim-host/test/bootstrap.test.mjs apps/vivim-host/test/bootstrap-grant.test.mjs
```
- Outcome: **43 tests passed, 0 failed, 0 skipped** at exact source SHA; includes seven approved/once-only mocked prompts, expired/revoked grants, typed D03–D07 validator, mismatched generation rejected without write. Historical earlier snapshot `b8c9114` passed 42/42 before adding mismatch regression.

## Limitations / next real acceptance

- **No real signed-in ChatGPT Studio send or provider content extraction has been performed.**
- **No personal Oracle corpus was populated from a live ChatGPT session**, and no real auto-generated AutoWiki result was demonstrated.
- ZCode DWF created/pushed, but **DWF itself not executed through the ZCode workflow runtime** in this round. Only its underlying exact Node test command was executed independently via Desktop Commander. Onboarding model scout from prior handoff hit free OpenRouter HTTP 429; do not silently use paid routing or claim a healthy parallel agent fleet.
- The temp archive has no Git metadata, so new DWF's HEAD==origin/main guard will refuse until an isolated clean Git checkout is prepared. Do not run against dirty owner tree.
- Phone GPUI/PWA actual onboarding visual progress and cross-device corpus retrieval not verified; no deployment/promote operation performed.

## Tight next loop

1. Cloud Dev verifies clean `main` and prepares any source defect fix there, not via Desktop Commander.
2. Operator creates clean exact-SHA isolated Git checkout and launches one bounded ZCode read-only verifier. Record DWF run ID and actual exit; no shell writable source permissions.
3. Deploy only through existing release/Host authority. Present seven exact prompts in onboarding for owner approval, not covert background sends.
4. Studio owner checks signed-in readiness, sends approved prompts once, records verified reply/thread IDs, Host validates and writes receipt/claims; inspect bounded ledger/readback and privacy guard.
5. Wire retrieval of actual accepted facts to existing Oracle/HQKG/AutoWiki read port rather than introducing a second store. Verify same Host revision on GPUI + phone PWA. Use each independent finding to drive one cloud source change and a new exact-SHA ZCode replay.

Use ARBOR [Oracle corpus ZCode instructions](https://github.com/owenservera/ARBOR/blob/main/docs/product/ORACLE-CORPUS-ZCODE-ITERATION-R001.md) as execution contract.
