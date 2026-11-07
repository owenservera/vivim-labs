# 01 — intent front end

## Verdict
Yes — one real NL front end exists, but it is a **deterministic catalog matcher**, not an intent router. It lives only in
`vivim-web-harness`: vendored Omega NLCL (11 files, 2205 lines) → `control-language.mjs` → `intent-planner.mjs`. It parses a free
sentence into a typed `{actionId,args}` and stops there. `vivim.intent-route/1` exists in **zero** code. No tier decision exists.
Negation is silently dropped and reaches executable R1/R2 plans. No miss log. The LLM "entry point" is an unfed intake slot.

## Verified facts
1. **NLCL is vendored and byte-pinned.** 11 files / 2205 lines; `node tools/control/vendor-nlcl.mjs --check` → `vendored NCLL ok: 11 files @ 79ee558d6c7ea77c20f220db2341adaa57b11d44`. Dir: `vivim-web-harness/apps/mobile-relay/nlcl/`.
2. **Sentence → typed candidate is real and pure.** `interpretControl(text,{catalog,world})` at `apps/mobile-relay/control-language.mjs:141`; determinism law stated at `nlcl/index.mjs:7`; test `determinism (N1)` passes.
3. **Typed plan schema is real.** `planText` → `vivim.control-plan/1` at `apps/vivim-host/control/intent-planner.mjs:150-171`; outcomes `empty|candidate|incomplete|clarification_required|interpretation_required` (`control-language.mjs:117`).
4. **Catalog is small and unproven.** `describe()` output: `total 19, sourceWired 17, verified 0, unavailable 2`; 15 of 19 carry a `language` block (`intent-planner.mjs:113`).
5. **Confidence floor 0.6** (`control-language.mjs:14`), applied at `:217` → under-floor parses degrade to `interpretation_required`.
6. **Reads execute immediately; writes need approval.** `dispatcher.mjs:186` `if (p.outcome==='ready') return runRead(p)` — R0 runs with no approval; `approval_required` otherwise; mobile R2 is forced `unavailable` (`dispatcher.mjs:191-195`).
7. **Journal-before-effect is real.** Plan/approve/claim/receipt with one exclusive `claimed` fence (`control/journal.mjs:1-7,66-72`).
8. **LLM entry is intake-only, no caller.** `planCandidate` (`intent-planner.mjs:174`) is reached only via `dispatcher.plan({candidate})` (`dispatcher.mjs:184`). No model call exists in the control path (grep for openai/anthropic/llm → only unrelated executor paths). The phone says so verbatim: *"AI interpretation through your signed-in apps is not wired yet"* (`apps/mobile-relay/control-phone.mjs:94`).
9. **`vivim.intent-route/1` is spec-only.** Zero code hits in all three repos; named as *missing* at `repos_hq/docs/OWNER-PROMPTS.md:394` and `repos_hq/docs/integration/SOTA-TARGETS.md:120`.
10. **Second, unrelated NL path exists in vivim-ui — and it is not NL.** `detectMode` (`vivim-ui/src/tier1/agent.ts:29-44`) is a regex over `!run|!use|!design|!maintain|!inspect|!sim`; everything else → `"chat"` → `#chatBuiltin` returns a static help string (`tier2/router.ts:259-270`). Live over HTTP: `POST /api/chat` (`vivim-ui/src/server.ts:297-303`).
11. **Negation defect reproduced (current bytes).** `planText("never disconnect my pc")` → `machine.network.disable_all`, R3, `unavailable`; `planText("don't remember my salary")` → `personalization.preference.add`, R1, **`approval_required`**; `planText("never exclude my medical history")` → `privacy.hard_no.add`, R2, **`approval_required`**.
12. **Mechanism: the negator is discarded with no gap.** `interpretText("never disconnect my pc")` tokens = `never` (no role, no family), `disconnect` (verb), `my` (filler), `pc`; `canonical: "/revoke 'pc'"`; `gaps: []`. `never` is also a *catalog verb* for `privacy.hard_no.add` (`action-catalog.v1.json`, verbs `never,exclude,forbid`), so the two roles collide.
13. **Conjunction handling is partial.** `and`/`then`/`&`/`;` followed by a known verb → `clarification_required` "one command at a time" (`control-language.mjs:165-169`). But comma/period are absent from the splitter: `"pause egress, show status"` → `ready`, `vivim.status.show` — the first clause is **silently dropped**. `"pause egress and the theme to dark"` → `ui.theme.set_preset` (first clause dropped).
14. **No miss log.** `journal.record` is called once, only on the `approval_required` path (`dispatcher.mjs:201`). `interpretation_required` returns to the client and is printed (`control/cli.mjs:48`, `control-phone.mjs:78`), never persisted.
15. **Test truth today: 66 pass / 2 fail.** `bun test apps/vivim-host/test/control-{language,intent-planner,dispatcher}.test.mjs`. Fail A = stale assertion `19 !== 12` (`control-intent-planner.test.mjs:8`). Fail B = vendor gate crashes under Bun (`SyntaxError: Export named 'stripTypeScriptTypes' not found in module 'node:module'`); under `node v24.22.1` it passes — so bytes are fine, the **gate is unrunnable on the mandated runtime**.
16. **vivim-core declares the missing contract but implements none of it.** `REFUSAL_CODES` incl. `NO_MATCH`, `AMBIGUOUS`, `MISSING_PARAM` and `candidates?:{id,title,score}[]` at `repos_hq/vivim-core/src/contract.ts:71-97`; the package contains only `contract.ts` + `revision/apply.ts` — no matcher.

## Absent / broken
- **`vivim.intent-route/1`: absent.** No schema id, no producer, no consumer. Closest shapes are `vivim.control-plan/1` and `TurnResult` (`vivim-ui/src/tier2/router.ts:41`).
- **No tier decision.** Grep for instant/medium/high in intent context across code: 0 hits. Stated outright: *"No tier decision exists"* (`OWNER-PROMPTS.md:393`).
- **No negation semantics.** Section 11/12. `OWNER-PROMPTS.md:393` records 0% negation and the same example; my run confirms it and shows it reaches R1/R2 `approval_required`, not just blocked R3.
- **No scoring/ranking.** Frame match + scope words only; no ranked alternatives with scores outside the refusal-`candidates` type. `SOTA-TARGETS.md:61` grades router matching "no scoring, no fallback scoring model".
- **No LLM tie-break wired.** `interpretation_required` becomes text, not a model call.
- **No miss log, no paraphrase memory, no cross-turn intent memory.** Nothing consumes a miss.
- **Free text in vivim-ui is not parsed.** It degrades to a canned help reply.

## Gaps by category
**LANGUAGE&INTENT**
- Negation/complement vocabulary absent (0%); a negator is either dropped or hijacked as a verb (fact 12).
- No ranked/scores intent resolution, no confidence margin published to the caller (only one boolean `0.6` gate).
- No tier decision: nothing classifies a sentence as instant (deterministic hit) vs medium (deterministic miss → catalog-constrained proposal) vs high (needs clarification/owner).
- Coverage is 15 spoken actions of 19; no open-vocabulary mapping. Spanish 0% per `OWNER-PROMPTS.md:393` (UNVERIFIED by me).
- No contraction handling, no comma/period clause splitting (fact 13).

**EXECUTION&EFFECT**
- `verified: 0` — no action is live-accepted; "executable" means `source_wired` only (`EXECUTABLE` set, `intent-planner.mjs:20`).
- The negation defect is an effect-layer hazard: a dropped "don't" lands on R1/R2 `approval_required`, where owner approval completes the wrong action.
- R3 action `machine.network.disable_all` sits in the speaker's vocabulary while its executor is null; only `readiness` prevents it.
- Conjunction clause loss can drop a mutation silently while executing a read (fact 13).

**STATE/EVIDENCE/MEMORY**
- No miss log and no negative-intent record: failures are unlearnable (fact 14).
- R0 reads bypass the journal entirely (`dispatcher.mjs:186`), so read evidence is absent from the ledger.
- No record of which reading was rejected when `clarification_required` fired; choices are returned but not journaled.
- Taught-word history is kept (`control/lexicon.mjs:1-5`) — the one durable language-memory that does exist.

**SURFACE&REACH**
- Two live NL surfaces: Host control (HTTP/CLI) and the paired phone; `vivim-ui` has a third endpoint that does not parse language.
- No cross-machine or relay-level intent: parsing is local to each Host.
- No surface publishes a parse's alternative readings to the human before approval beyond a canonical echo (`control-phone.mjs:73`).

## Evidence commands
```sh
# inventory
grep -ril "intent-route\|nlcl\|intentRoute\|miss-log\|missLog" repos_hq vivim-web-harness vivim-ui --include=*.mjs --include=*.ts --include=*.md | head -60
grep -rn "intent-route" vivim-web-harness --include=*.mjs --include=*.ts   # → 0 hits
# negation + conjunction, pure, no execution
cd vivim-web-harness && bun -e 'import{createControlPlanner}from"./apps/vivim-host/control/intent-planner.mjs";const p=createControlPlanner();for(const s of ["never disconnect my pc","don\u0027t remember my salary","never exclude my medical history","pause egress, show status","pause egress and show status"]){const r=p.planText(s);console.log(JSON.stringify({s,o:r.outcome,id:r.actionId??null,risk:r.risk??null,ap:r.requiresApproval??null}))}'
# token-level proof the negator is dropped
cd vivim-web-harness && bun -e 'import{createControlPlanner}from"./apps/vivim-host/control/intent-planner.mjs";const r=createControlPlanner().interpretText("never disconnect my pc");console.log(JSON.stringify({o:r.outcome,c:r.candidate,t:r.feedback.tokens,g:r.feedback.gaps}))'
# coverage
cd vivim-web-harness && bun -e 'import{createControlPlanner}from"./apps/vivim-host/control/intent-planner.mjs";console.log(JSON.stringify(createControlPlanner().describe().coverage))'
# tests + provenance gate
cd vivim-web-harness && bun test apps/vivim-host/test/control-language.test.mjs apps/vivim-host/test/control-intent-planner.test.mjs apps/vivim-host/test/control-dispatcher.test.mjs
cd vivim-web-harness && bun tools/control/vendor-nlcl.mjs --check; node tools/control/vendor-nlcl.mjs --check
# negator-as-verb collision
grep -n "never\|exclude\|forbid" vivim-web-harness/apps/vivim-host/control/action-catalog.v1.json | head
```

## Could not check
- Held-out paraphrase 47.4% / golden 37/37 / Spanish 0% — quoted from `docs/OWNER-PROMPTS.md:392-396`; I did not re-run that eval harness (**UNVERIFIED**).
- No live phone or Host HTTP session was exercised (read-only mandate); the dispatcher's approval/claim path is asserted from source + passing tests, not from a real receipt.
- Full `bun test` suite not run; only the three control test files.
- Upstream `vivim-omega` source not in these repos, so I cannot say whether the vendored copy has drifted from its origin — only that the pin matches the local manifest.
- Whether any *provider-side* (Studio/browser) path parses sentences into intents outside the control catalog — not swept beyond the mandated grep.
