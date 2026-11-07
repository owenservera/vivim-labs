# 02 — deterministic executor

## Verdict
VIVIM has **one** real effect executor: `runAtom()` in `studio/src/studio/atom-dispatch.mjs:201`. It is a capability dispatcher, not a governed executor — no role check, no approval, no revocation, no risk level, no persisted receipt. Its only gate is one boolean (`execute:true`) that authorizes every non-read effect identically. The one CLI path to it can never supply the page it requires, so the shipped `studio atoms run` always fails. The HQ "action registry" is not a gate: it builds ids and stores *shell command strings*. The NL→action half and the atom→CDP half are never wired together.

## Verified facts

**Atom surface (all numbers executed, not read)**
- 2100 atoms, 21 families × 100, `complete: true`, 0 missing, 0 problems — `browser-atoms.mjs:11-17` (`TARGET_COUNT = 2100`, `PER_FAMILY = 100`); loader `:22-60`.
- 73 distinct primitives used; **all 73 have a handler; 0 handlers are unused** — `atom-dispatch.mjs:39-195` vs registry primitives.
- Effect distribution: `read 1280, page-write 540, local-write 160, browser-navigation 120, provider-send 0`.
- **12 of 73 primitives are throwing stubs**, binding **270 of 2100 atoms (12.9%)**. Families hit: `download 90, net 70, nav 30, safe 30, frame 30, trace 20`. Stubs at `atom-dispatch.mjs:98,100,101,154,155,156,158,171,172,176,181,182`.

**The entry point and its gate order**
- `runAtom(atom, args, { page, execute, now })` — `atom-dispatch.mjs:201`. Bound variant `createDispatcher(page)` — `:222`.
- Refusal order: unknown primitive → `ATOM_NO_HANDLER` `:203-206`; `atom.effect !== 'read' && execute !== true` → `ATOM_EXECUTE_REQUIRED` `:207-209`; no page → `ATOM_PAGE_REQUIRED` `:210-212`. Handler exceptions are caught into `{ok:false,error}` `:216-218`.
- `runAtom` contains **zero** references to `roleOf`, `ROLES`, `approval`, `revok`, `risk`, `validateRoles`, `planAtom` (grep: all ABSENT). Role/approval logic lives in sibling modules and is never called from the dispatch path.
- One `execute:true` ran `local-write net.block.all`, `page-write act.click.selector` and `browser-navigation nav.goto.url` identically (`ok=true` for all three). The effect ladder is declared but not enforced.
- `planAtom` argument validation (`browser-atoms.mjs:85-106`) is called **only** by the CLI (`atoms.mjs:130`), never by `runAtom`. A direct caller skips arg checking entirely.

**Only four importers exist** (whole repo, incl. Rust — `src-tauri` has no atom refs):
1. `studio/bin/commands/atoms.mjs:145` — `dispatch.runAtom(atom, args, { execute })`, **no `page`**. Proven dead: `bun studio/bin/studio.mjs atoms run act.click.selector --args '{"selector":"#x"}' --execute` → `error ATOM_PAGE_REQUIRED`.
2. `scripts/atoms-live-check.mjs:34-44` — the only caller that binds a real CDP page; hardcodes `execute:false` and 6 read atoms.
3. `studio/src/studio/system-wrap.mjs:103` — imports only `PRIMITIVE_HANDLERS` for a self-check.
4. `studio/test/unit/atom-dispatch.test.mjs:3`.

**Roles** — `atom-roles.mjs:57` `roleOf()` partitions 2100 into `browser 1450 / tab 180 / account 240 / provider 230`, `validateRoles` ok (`:77`). Used by `atoms.mjs` for display (`:63-64`) and validation only. Never consulted at dispatch.

**HQ side is a different system**
- `hq/lib/action-registry.mjs` executes nothing. `buildActions` `:124` expands domains×verbs×objects; `status: handler ? 'wired' : 'declared'` `:159-171`. `validate()` `:254` **returns** problems, enforces nothing.
- HQ catalog: **2000 actions, 410 wired / 1590 declared**, risk `high 600 / low 800 / medium 600`, 42 handler keys.
- A "handler" is a command string, not a function — `hq/lib/actions/handlers.mjs:16` (`'bus.create': { command: 'bun scripts/sync.mjs send …' }`).
- `hq/lib/action-router.mjs:143` `route()` scores by exact token weight; header `:8` states it "resolves no handler, calls no transport, and writes nothing".
- `hq do` never executes — `hq/bin/hq.mjs:1922`.
- The **only** real gate is `hq action <id> --run` at `hq/bin/hq.mjs:1893-1897`: requires `a.handler` **and** `risk === 'low'` **and** `!approval`, then `spawnSync(a.handler.command, { shell: true })`. **240 of 2000 actions (12.0%)** are eligible.

## Absent / broken
- **No audit log or receipt.** `runAtom` returns an envelope; nothing is written to disk, ledger, or graph. No journal-before-effect, so an effect cannot be reconciled after the fact.
- **Approval is not consulted.** `hq/lib/approvals.mjs` is imported only by `hq/lib/comms-roof.mjs:17` (pending count for a status line) and tests. Same for `revocations.mjs:18`. Neither can refuse an atom.
- **Role is decorative.** A `provider`-only capability would be dispatched by `browser`; nothing checks.
- **`provider-send` is vocabulary-only.** Legal in `atom-kit.mjs:18` and derivable via `send:true` (`:68`), but **0 atoms** carry it. The `SEND_ONLY_PROVIDER` check (`system-wrap.mjs:114-122`) passes **vacuously** — it proves the empty set is empty.
- **270 atoms advertise capability they cannot deliver** — they fail at runtime with `ATOM_NEEDS_*`.
- **`blockRequest` ignores its argument** and blocks `'*'` unconditionally — `atom-dispatch.mjs:157`. `net.block.<pattern>` blocks everything.
- **No bridge** from `route()` output to `runAtom`. HQ produces an action id + command string; Studio requires an atom id + `page`. Nothing translates, and no HQ action id is an atom id.

## Gaps by category

**LANGUAGE&INTENT** — `route()` maps a sentence to ranked ids but explicitly refuses to act (`action-router.mjs:8`); first match on `"send this to X"` reaches `send this to X`'s best token overlap, not an intent. No negation, no parameter extraction, no multi-step. `expand()` aliases are the only synonym layer (`aliases.mjs`), applied *before* scoring (`hq.mjs:1913`), so aliases cannot disambiguate.

**EXECUTION&EFFECT** — The ladder `read < local-write < page-write < browser-navigation < provider-send` exists in the type system and in `ROLES.may`, but dispatch collapses it to one boolean. `setStorage`, `clearStorage`, `setClipboard`, `click`, `submitForm` and `goto` all require exactly `execute:true`. No per-effect grant, no scope, no expiry, no one-shot nonce. `submitForm` (`:177`) is a real form post with no confirmation step.

**STATE/EVIDENCE/MEMORY** — Worst gap. `runAtom` persists nothing: no started/succeeded/failed record, no arguments, no resolved target, no page URL, no screenshot reference. `atom.effect` is echoed in the envelope but not stored. The dispatcher therefore cannot answer "what did VIVIM do to this page, when, on whose authority". HQ has `receipts.mjs`/`approvals.mjs`/`revocations.mjs` but the atom path touches none of them.

**SURFACE&REACH** — `studio atoms run` is the only operator surface and it is dead (`ATOM_PAGE_REQUIRED`, proven). The single working execution path is a dev script (`atoms-live-check.mjs`) limited to 6 read atoms with `execute:false`. No phone surface, no owner confirmation UI, no queue, no replay.

## Evidence commands
```sh
# surface counts, effect ladder, stub blast radius  (vivim-web-harness/studio/src/studio)
bun -e "const m=await import('./browser-atoms.mjs');const r=await m.loadRegistry();console.log(r.count,r.unique,r.complete)"
bun -e "const m=await import('./browser-atoms.mjs');const r=await m.loadRegistry();const e={};for(const a of r.atoms)e[a.effect]=(e[a.effect]||0)+1;console.log(e)"
bun -e "const s=await Bun.file('./atom-dispatch.mjs').text();console.log(s.split(/\r?\n/).filter(l=>/^\s{2}\w+:/.test(l)&&/throw new Error\('ATOM_/.test(l)).length)"   # 12
# gate order + absent checks
bun -e "const s=await Bun.file('./atom-dispatch.mjs').text();for(const t of ['roleOf','approval','revok','risk','planAtom'])console.log(t,s.includes(t))"
# CLI dead end (proven)
bun studio/bin/studio.mjs atoms run act.click.selector --args '{"selector":"#x"}' --execute   # ATOM_PAGE_REQUIRED
# HQ catalog numbers  (repos_hq)
bun -e "import('./hq/lib/action-registry.mjs').then(async R=>{const V=await import('./hq/lib/actions/verbs.mjs');const H=await import('./hq/lib/actions/handlers.mjs');const D=(await Promise.all([1,2,3,4].map(n=>import('./hq/lib/actions/domains-'+n+'.mjs')))).flatMap(m=>m.DOMAINS);const a=R.buildActions({domains:D,verbs:V.VERBS,handlers:H.HANDLERS});console.log(R.summarize(a).byStatus)})"
# approvals/revocations are display-only
grep -rn "approvals.mjs\|revocations.mjs" --include=*.mjs hq scripts
```

## Could not check
- **No live CDP run performed.** The mandate forbids browser/provider calls, so `atoms-live-check.mjs` was not executed; the 12 stubs are identified by source inspection plus the runtime-visible `ATOM_NEEDS_*` throw sites, not by observing a failure.
- **Tauri/Rust host path.** `src-tauri/src` contains no atom references (grep, empty), but I did not audit whether the Tauri host drives CDP through a separate Rust command surface. Any effects originating there are outside this map.
- **`apps/` and `runtime/` directories** were not exhaustively read; the importer grep covered all `.mjs/.js/.ts/.rs` outside `node_modules`, which found only four importers, so a hidden executor would have to re-implement `page.send` rather than reuse this one.
- **Whether 270 stub atoms are a deliberate backlog or an abandoned claim.** The code says `ATOM_NEEDS_*` (a boundary statement); no owner note states the intent.
