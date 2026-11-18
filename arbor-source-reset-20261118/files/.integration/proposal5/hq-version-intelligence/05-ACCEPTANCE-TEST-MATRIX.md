# HVI executable and negative acceptance matrix

> **2026-11-12 SCOPE CORRECTION — NATIVE TOOLING ONLY.** Read [10-NATIVE-TOOLING-ONLY-SCOPE.md](../10-NATIVE-TOOLING-ONLY-SCOPE.md) when this is a prompts/ file; for root docs see [10-NATIVE-TOOLING-ONLY-SCOPE.md](10-NATIVE-TOOLING-ONLY-SCOPE.md). This correction governs any conflicting original instructions below.

## Revised present-wave acceptance scope
T01–T08, T12–T24 shall be tested with synthetic/injected Git object trees, including 3 SHA pins of one invented repo and 5 invented repos. T09–T11 **fixed S0/D source counts are historical fixture preservation checks only**, not an instruction to acquire or run a current corpus comparison. H/G2 must demonstrate live authorized agent tool discovery, refusal/revoke, typed NL routing and UI empty state, without real source acquisition. No exact real 361-path semantic classifications in this wave. X real audit tests are deferred until explicit user request.


---

**Truth rule:** a named test without an executed command+exit code is UNRUN. GitHub source inspection is never test execution. Windows/MSVC and physical Android acceptance remain separate Proposal 5 beta gates.

## Source and pin
- T01 resolve branch/tag/sha to 40-hex commit, freeze; same ref drift after acquisition cannot mutate old pin.
- T02 compare same repo SHA1 vs SHA2 vs SHA3 in one call: distinct pins, chronological and non-chronological permutations yield deterministic results.
- T03 compare five different repos + research branch pinned to SHA; no implicit HEAD.
- T04 dirty worktree excluded from pinned commit manifest or separately marked; never pretend dirty bytes equal commit.
- T05 canonical digest stable across input ordering, replay, repeated runs and provider independence.
- T06 rename/exact relocation captured through blob identity; deletion vs missing-path and incomplete scan distinct.
- T07 symlink, junction, submodule, Git LFS pointer, binary, oversized blob, unreadable repository, sparse clone and detached HEAD fail/mark honestly.
- T08 no GitHub/API network required to re-compare already pinned offline manifests.

## Capability and lineage
- T09 676 original HQ paths => 315 mapped, 314 exact, 1 patched, 361 unmapped at **fixed** source pins. Missing at original path stays `MISSING_AT_TARGET` pending semantic analysis; all 361 have individually emitted rows.
- T10 donor Harness/TierFlow prefix census as in `02-SOURCE-CROSSWALK.md`, discrepancies flagged with exact source SHA (no silent update).
- T11 Labs `main` plus research branch candidate classes: executable research vs design-only, never automatic product promotion.
- T12 cloned files with different names, split/combine candidate, semantic alias, identity collision, same canonical ID different implementation => REVIEW_REQUIRED.
- T13 partial/missing donor cannot create tombstones or false dropped capabilities; distinguish unknown from absence.
- T14 test *source presence* is separate from observed executed-test receipt; release claims do not establish runtime.
- T15 predecessor/successor lineage for one repo at multiple SHAs and migration into new repository; no cross-repo identity collision.
- T16 authoritatively different Host action and HQ development action registries are not merged.

## Safety, authority and performance
- T17 unapproved source roots, ../ escapes, symlink escapes, malicious .git refs, path injection, secret-looking file refs => denied.
- T18 source scanner never executes repo code, providers, browser automation or untrusted package hooks.
- T19 scan limits, cancellation, binary size, large repos, pagination and low disk => bounded and explicit incompleteness, no old snapshot destruction.
- T20 corrupted digest/replay fork/duplicate delivery/conflicting pinned revision/invalid reviewer approval => fail closed.
- T21 revoke grant between operations: Host read denied immediately; no stale cached authority, personal World source never included.
- T22 no Host effect writer, scheduler, graph store, duplicate catalogue, provider sender or second TierFlow publisher.
- T23 CLI and report outputs cannot leak credentials/personal corpora and cannot disclose unapproved file text.
- T24 attacker-controlled Git path, markdown, comments, source content are treated as untrusted **data**, not agent instructions.

## Combined executable gate commands
F specifies exact conformance scripts in `tests/hq-version-intelligence/`. G1 runs them in an installed Node 24 environment plus existing HQ smoke; G2 runs Host authorization, read/revoke and UI projection tests. Record OS, Node/Bun, stdout/exit, SHA, fixtures. Explicit 10k-record bounded-query benchmark with observed memory/time and truncation counts; no invented threshold pass. No donor checkout required for offline replay test.

## Product acceptance
R returns: `PINNED_REPRODUCIBLE` vs `INCOMPLETE` for each source; `SOURCE_PARITY` vs `SEMANTIC_PARITY` vs `RUNTIME_PARITY`; `GO_FOR_OWNER_REVIEW` or `NO_GO`. No beta acceptance. No silent omission of original 361 HQ paths.
