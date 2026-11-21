# ARBOR Proposal 5 — Foundation PG Cohort: Shared Execution Contract

**Applies to:** PG-00 Source Truth, PG-01 Native, PG-02 Semantic, PG-03 PWA/Rock, PG-Z independent referee. Read this first in each lane. **These are real source-gathering and isolated pure-prototype work orders, NOT full product implementation, integration G or release authority.**

## Baseline and G preflight (must recheck at worker execution time)

1. Query `owenservera/ARBOR` for **current** `work/p5-foundational-architecture-prep-r001` HEAD, and read `.integration/proposal5/foundations/README.md`, `SOURCE-ATLAS.v1.json`, `GAP-REGISTER.v1.json`, `WAVE-MANIFEST.v1.json`, `FOUNDATIONAL-ARCHITECTURE.md`, `NATIVE-MACHINERY-AND-CONTRACTS.md`, `VERIFICATION-AND-ACCEPTANCE.md`, and original `.integration/proposal5/prompts/W1/G.md` on `work/p5-pre-g-convergence-r001`.
2. Check `refs/heads/integrate/proposal5-beta-r001`. **At cohort authoring, absent: original G has NOT run.** Do not infer state from historical assistant reports. If it appears, inspect `integration/reports/W01-GATE.md` and exact SHA; a branch's existence is not G acceptance. If G is active, do **not** interfere; continue read-only/isolation if compatible and report baseline divergence.
3. Re-fetch frozen worker branch HEADs, inspect each `integration/worker-reports/PREG-{A..F}.md` for FREEZE_READY and no later commits. At cohort authoring the full frozen HEADs are:
   - A `dda546a11294920d6521a0f64ab7ee62ec993c70` 
   - B `78a5105a9c1f1880d658f350e130b1e2632d3532`
   - C `f950b91685927f37b48d020433aa8d2d3c271419`
   - D `ad55fe28c2c331841675f308f9199ee4c2add86a`
   - E `1a5cf26ce14ebce4e31d9a4373060ace61852645`
   - F `be458f3c4cdb7eb38d47b1de2a00d441b7cdc704`.
   Any changed ref is a **blocking discrepancy to reconcile**, not an implicit new freeze.
4. Create **only** the one unique branch identified in your lane prompt from the **current, pinned** foundation program HEAD. If branch already exists, inspect its current code/report before changing it; never reset/recreate or overwrite another worker. Each worker writes strictly under its single assigned `.integration/proposal5/foundations/proofs/<lane>/` subtree.
5. An immutable Git branch commit + file blob pin is evidence of *source content*, not successful execution or permission. Source change classification must be `ALREADY_IMPORTED | SOURCE_ONLY | PREPARED_UNPROMOTED | ADAPTER_REQUIRED | TEST_ONLY | DESIGN_ONLY | BLOCKED | DEFERRED | DUPLICATE_REVIEW`, with owner and exact reference.

## Non-negotiable scope and technology

- **Source-grounded first:** read actual implementation, not only README, PRD or previous assistant recap. Record the repo branch HEAD and selected file blob IDs. Search Labs current `main` and specific research branches only for identified source gaps and avoid importing whole donor trees.
- **No writes outside lane prefix.** Never edit `main`, `integrate/proposal5-beta-r001`, G lane, A–F frozen workers, other Phase 2 S/V/W/N0/T/K/C lanes, product directories, original source trees, root Cargo/Bun lockfiles, release artifacts or native APK resources. Never create or merge a PR to G/main.
- **No duplicate authorities:** Host remains owner of action catalog, original-text revalidation, policy/approval/receipt; HQ dev semantic wiki/read; TierFlow UI World and publisher; Studio exact signed-in Chrome sender; shared Rust graph/engine is pure compute and scoped graph, not side-effect router; source index, user memory, DCBs and clocks retain their existing owners.
- **No provider APIs** for ChatGPT/Claude/Gemini, no Playwright/Puppeteer as product path, no alternate browser worker, extra scheduler/graph/wiki store, arbitrary native/JS plugin executor, dev model auto-grant or AI-authored executable source-comment action.
- **Mobile is PWA-first.** No APK work, APK signing/distribution, native Android bridge dependencies, Play Store, secondary phone backend or effectful offline replay. The `apk/` directory includes historical Host grant components; do not remove these because of naming.
- **Privacy:** no real user conversation exports, signed-in browser credentials, user-private DCB content, certificates, tokens or machine-local secrets in Git/fixtures. Use only synthetic/redacted inputs.
- **G has sole integration authority.** PRE-G preparation is **not** a prerequisite for G, must not wait for G or block it, and is not a shadow W1-G. A full product proof cannot be claimed from prototype code.

## Testing and negative evidence

- Run real tests when a proper runtime is available; state exact command, runner version, exit code, summary and source commit. GitHub-only code can be committed as test source but report `UNRUN`. In-memory V8 evaluation is **V8 only** and cannot be labeled Node/Bun/Cargo, Windows or device acceptance.
- Each lane must include executable negative fixtures or a concrete minimal test plan for its effect boundary, stale inputs, ambiguous version/owner, denied source, resource caps and replay where relevant.
- Never report a source test file or `cargo test` command as PASS if not physically executed. Claims like "device-ready", "native installed", "Host wired" and "90% interpreted" require actual evidence.
- Show unverified owner risks and not-run areas as first-class output, not hidden inside a summary.

## Required worker output contract

Every worker commits:
1. `README.md`: objective, exact frozen/current read refs, scope and tested-vs-UNRUN summary.
2. `SOURCE-DELTA.json`: source classification, owner/path/blob, reused vs missing vs duplicate analysis.
3. `PROOF-PLAN.md`: smallest pure proof/patch design and exact post-G code owner + commands. If safe, include actual isolated fixture/test code.
4. `NEGATIVE-ACCEPTANCE.md`: assertions, adversarial cases, expected refusals, real commands/exits or UNRUN.
5. `OWNER-HANDOFF.md`: **at most 5** high-value source-owner tasks, rebase/merge instructions only after accepted G, blockers and tests.
6. `RUN-LEDGER.json`: schema `arbor.proposal5.fa-worker-run/1`, branch/base/head (the exact HEAD in the final message), changed paths, provenance, test records with explicit status, risk findings, and `PREPARED_NOT_PROMOTED`.

**Atomicity:** commit complete artifacts under assigned lane prefix. Verify `git compare BASE..HEAD` has **0 out-of-scope paths**. If unable to commit, report `IN_PROGRESS/BLOCKED` with exact reason and no success assertion. **STOP WRITING** after final HEAD is verified; do not merge, promote or start another lane.

## Dependencies between lanes

- PG-00 is the definitive source/owner truth handoff. Other PG-01/02/03 can **run in parallel**, using their own source snapshots and flagging provisional/collision facts until PG-00 commits.
- PG-01 uses A/F/SN/P2-C source; PG-02 uses D/B/E plus S/V/W/N0 source; PG-03 uses A/B/E/D/F plus Rock and personal source. Each must read existing pre-G preparations and add **incremental difference**, not another prototype platform.
- PG-Z independent referee is started **after** the available four workers finish or are explicitly marked blocked, and can report incomplete without waiting. It does NOT merge or modify any worker branch.

**Final disposition:** `PREPARED_NOT_PROMOTED`. No new production feature is shipped by this cohort.
