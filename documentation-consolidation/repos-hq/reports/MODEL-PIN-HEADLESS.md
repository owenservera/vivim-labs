VERDICT: PARTIAL — every new conversation can be pinned to one chosen model with a single config key, but a headless `spawn.mjs new` conversation cannot be pinned *per conversation* from the CLI, and no flag, env var or per-cwd file exists that does it. Measured, not inferred.

## 1. The mechanism (exact file, key, command)

`~/.zcode/v2/provider_config.json` accepts a **`config.defaultModelSelection`** key. It is part of the
strict zod schema, so it is a legal key and not an "unknown key kills the file" hazard:

```
config: m.object({
  providerOrder, providerConfigRules, modelConfigRules,
  defaultModelSelection: Nl.optional()        // <- the key
}).strict()

Nl = m.object({
  providerId: string.trim().min(1),
  modelId:    string.trim().min(1),
  options:    { reasoningLevel?: string }     // strict
}).strict()
```
(`zcode.cjs` @14323506 region; schema at @767238 and `Nl` definition at the `GDe="$"` site.)

Current state, read directly:

```
TOP KEYS: ['schemaVersion','config']
CONFIG KEYS: ['providerOrder','providerConfigRules','modelConfigRules']
defaultModelSelection PRESENT: false
providerOrder: ["deepseek","opencode-zen-chat-2","new-provider","minimax-m-plan","openrouter", ...]
```

### Why headless runs deepseek today

The resolver decides the model for a new session, in this precedence:

```js
function Qhc(e){                       // resolveInitialModelSelection caller
  if(e.runtimeConfig?.modelSelection){ ... return ...; }   // 1. explicit per-input/programmatic
  if(e.resume) return;                                     // 2. resume: don't override the session
  let n = int({configuredDefault:e.configuredDefaultModelSelection, registry:t.getView()});
  ...
}

function int(e){
  if(e.configuredDefault && pss(e.registry,e.configuredDefault))
    return {source:"configured-default", selection:A0r(e.configuredDefault)};
  for(let t of e.registry.providers)
    if(t.config.visibility!=="hidden")
      for(let n of t.models){
        let o = dss(e.registry,{providerId:t.providerId,modelId:n.modelId});
        if(o) return {source:"registry-fallback", selection:A0r(o)};   // 3. FIRST provider wins
      }
  return {source:"none"};
}

function pss(e,t){                     // isSelectable
  let n=e.providers.find(i=>i.providerId===t.providerId);
  if(n?.config.visibility==="hidden") return false;
  let o=n?.models.find(i=>i.modelId===t.modelId);
  return o ? Yhe(o,t).ok : false;
}
```

With `defaultModelSelection` absent, every new conversation lands in **branch 3**: the first
non-hidden provider in `providerOrder`, which is **`deepseek`**, first model **`deepseek-flash`**. That
is the whole explanation of the observation that started this task — no setting needs to be *wrong*
for headless to run deepseek; nothing needs to be set at all.

`pss` is also the silent trap: **if the requested provider/model is not in the live registry, it does
not error — it falls through to branch 3 and runs deepseek.** A wrong model id looks like success.

### Two supporting facts measured in the store

The model is chosen **per input**, not stored on the session row (`session` has no model column):

```json
// ~/.zcode/cli/db/db.sqlite, table session_input, payload
{"text":"...","intent":{"inputOrigin":"desktop",...,"kind":"sendText",
  "modelSelection":{"providerId":"deepseek","modelId":"deepseek-flash","options":{"reasoningLevel":"max"}}}}
```

The desktop client supplies it. `-p` builds its input from argv, and argv has no such field, so the
input arrives with no selection — there is a literal `null` among the distinct selections in the DB.

The resolved result is then frozen per session as one `session_entry`:

```json
type: "runtime/model_selection"
data: {"modelSelection":{"providerId":"deepseek","modelId":"deepseek-flash","options":{"reasoningLevel":"max"}}}
```
238 entries for 238 sessions — one pin per session, written on first resolution.

## 2. The probe — one turn, and what it proved

One headless turn, temp cwd `C:\Users\VIVIM.inc\AppData\Local\Temp\zcode-model-probe`:

```sh
node "C:/Program Files/ZCode/resources/glm/zcode.cjs" \
  -p $'/model minimax-m-plan/MiniMax-M3.1-Flash\nReply with exactly PROBE-OK' \
  --json --mode yolo --cwd "C:/Users/VIVIM.inc/AppData/Local/Temp/zcode-model-probe"
```

| field | value |
| --- | --- |
| session id | `sess_0ad49bc2-7fd1-4dba-8f27-ae64de886517` |
| turn id | `turn_b29411a8-82e2-4db3-8bac-6536cadd5415` |
| reply | `PROBE-OK` |
| **`model_usage.provider_id`** | **`deepseek`** |
| **`model_usage.model_id`** | **`deepseek-flash`** |
| `variant` / `agent` | `max` / `zcode-agent` |
| persisted pin | `runtime/model_selection = deepseek/deepseek-flash` |

**`/model` did not execute.** The slash line was consumed as ordinary prompt text — the model answered
only the second line, and the turn ran on deepseek. Headless `-p` has no slash-command dispatch; the
`sendInput` handler that recognises `model`/`effort` commands belongs to the app-server/TUI path
(`ut.sendInput` → `Lle(...)` → `if(pt?.type==="known" && (pt.name==="model"||pt.name==="effort"))`),
which `-p` does not use.

So the answer to the question as asked — "can a headless conversation be pinned to a chosen model?" —
is: **not through any argument `-p` accepts today**, and the one trick that looked cheapest is dead.

Corroborating negative: this is not a per-agent problem. Recent **subagent** sessions
(`sess_subagent_agent_*`) are also recorded on `deepseek/deepseek-flash` even though all 53 agent
definitions carry `model: openrouter/nvidia/nemotron-3.5-super-vl:free`. The definitions are not read
by these launch paths either.

## 3. Why the fleet's intended id would fail silently (the important operational finding)

The fleet is pinned to `minimax-m-plan/MiniMax-M3` per the brief. **That model id is no longer in the
live registry.** `providerModelRules` for `minimax-m-plan` contains exactly one enabled model:

```
{"providerId":"minimax-m-plan","modelId":"MiniMax-M3.1-Flash","config":{"enabled":true}}
provider_config: providerRules[minimax-m-plan].config.personalModelIds = ["MiniMax-M3.1-Flash"]
```

`MiniMax-M3` still appears in history (1140 turns, 1111 ok, last at 2026-11-04T18:44Z) — it was removed
by an 18:52/20:29 prune. Consequence: `defaultModelSelection: {providerId:"minimax-m-plan",
modelId:"MiniMax-M3"}` **passes the zod schema, fails `pss`, and silently runs deepseek.** The owner
rule ("only the CEO may spend deepseek") would be broken by the very edit intended to enforce it, with
no error anywhere.

The exact edit that *would* work, for the record — **not applied**:

```jsonc
// ~/.zcode/v2/provider_config.json  →  .config.defaultModelSelection
"defaultModelSelection": {
  "providerId": "minimax-m-plan",
  "modelId": "MiniMax-M3.1-Flash",
  "options": { "reasoningLevel": "max" }
}
```
Omit `options` only if you are willing to `ule()` throw on `reasoning-level-missing`; include it if the
model declares that value. Either way the acceptance test below must pass, because the failure mode is
silence.

## 4. Smallest change that makes it possible, and the test

**The CLI surface change that actually enables it** (upstream, not ours): accept
`--model <providerId>/<modelId>` on `-p`. The parser already exists — `uW(e)` splits exactly that
string form on the first `/`, and `setModel` already consumes a string via
`ldi(registry, u, configuredDefault, {allowMissingReasoning:true})`. It is a flag-wiring job, not new
logic. The app-server input schema already carries `modelSelection`, so `zcode app-server` is a working
second route today for anything willing to speak the protocol.

**The smallest change in `hq/bin/spawn.mjs`** — and the one that pays for itself immediately — is not
to add the flag but to **make the fallback loud**:

1. `cmdNew` accepts `--model <providerId>/<modelId>` and records it in the ledger row.
2. After a successful spawn, `runSpawn` reads the session's real pin:
   `select data from session_entry where session_id=? and type='runtime/model_selection'`.
3. If the recorded `providerId/modelId` differs from what was asked, the spawn **exits non-zero**
   naming both, instead of returning a session id that quietly burns the wrong provider.

That converts a silent rule violation into a refusal, and it catches the unregistered-model trap in §3
for free.

**The test that would prove it** — `hq/test/model-pin.test.mjs`, in the existing
`HQ_SPAWN_RUNNER` seam (no provider reach):

- Fake runner returns `sess_test_...`; a temp SQLite (never the live `.hq/index.db`, and never the live
  ZCode DB) is seeded with `session_entry(type='runtime/model_selection')` set to `deepseek/deepseek-flash`.
- `spawn new --model minimax-m-plan/MiniMax-M3.1-Flash` → **exit 9**, stderr names `deepseek-flash`
  as the model that would actually run. This is the failing test that documents today's bug.
- Re-seed the entry as the requested model → **exit 0**, ledger row contains the model.
- A registry case: `--model minimax-m-plan/MiniMax-M3` (unregistered) is refused before launch, not
  after.

Live acceptance after the change: `select provider_id, model_id from model_usage where session_id=<new>`
must equal the requested pair.

## 5. What to do with the fleet until then

1. **Do not pin `minimax-m-plan/MiniMax-M3`.** It is unregistered; the pin will fall back to deepseek
   silently. If MiniMax is required, the only selectable id is `MiniMax-M3.1-Flash`, and it is
   thin — 7 turns in history (5 ok, 2 err).
2. **Use `openrouter/nvidia/nemotron-3.5-super-vl:free` as the fleet model.** It is free, it is what all
   53 definitions already name, and it has the strongest evidence in the DB: **1432 turns, 1351 ok,
   25 err**, 986 of them in the last 24h. It is not the "never used, unproven" model the brief claims.
3. **The one lever that works today is the global default.** Setting `config.defaultModelSelection` to
   that pair fixes every *new* conversation — headless and desktop alike — in one edit. It cannot
   differentiate CEO from fleet; the CEO override is a `/model` switch in the TUI for that session.
4. **Do not rely on `--resume` as a pin.** It is not one. Sessions whose persisted pin is nemotron ran
   1500 deepseek turns against 1336 nemotron turns — the per-input selection wins in practice.
5. Treat the 53 agent definitions as **inert for spawn-driven work** until §4 lands. They are not read
   by `-p`, and recent subagent sessions confirm it.

## 6. Corrections to the "measured today" block

| claim | measured |
| --- | --- |
| all 53 definitions: CEO=deepseek-flash, rest=minimax-m-plan/MiniMax-M3 | **stale.** 53/53 are `openrouter/nvidia/nemotron-3.5-super-vl:free`, repinned at 22:55Z |
| `openrouter/nvidia/nemotron-3.5-super-vl:free` never used (ok=0 err=0) | **wrong.** 1432 turns, 1351 ok, 25 err, last at 2026-11-04T18:50Z |
| `minimax-m-plan/MiniMax-M3` healthy (ok=370 err=0) | DB says 1140 turns / 1111 ok / 8 err, **and the id is no longer registered** |

## 7. Not proven here, and the exact next tests

- **`config.defaultModelSelection` was not exercised end-to-end.** Applying it was out of scope (the
  file was off-limits), and there is no alternate config root to test against: the bundle reads only
  `HOME`, `LOCALAPPDATA`, `XDG_DATA_HOME`, `ZCODE_DEBUG`, `ZCODE_GIT_BINARY`,
  `ZCODE_MAX_TOOL_CONCURRENCY`, `ZCODE_E2E_COVERAGE` — there is no `ZCODE_CONFIG_DIR`. The claim is
  code-read plus schema-read, not runtime-measured. Test: apply the §3 edit, run the §2 probe command
  unchanged, and require `model_usage.model_id` to change.
- **Whether `zcode app-server` accepts `modelSelection` on a create/prompt call from stdio.** The
  schema that carries it (`{sessionId, modelSelection, content, ...}`) is present; the method name was
  not resolved. Test if the app-server route is wanted: one turn through `app-server` with a free
  model, asserting the recorded `model_id`.
- **Whether a per-cwd `.zcode/config.json` could ever carry a model.** No: the repo's own
  `.zcode/config.json` is parsed against a schema with `mcp` only, and the bundle has no project-level
  model key.

## 8. Hygiene note

`provider_config.json` stores provider credentials in plaintext at `config.access.apiKey`. While
inspecting provider rules for this report, those values were printed into this session's transcript by
my own command — no value is reproduced here, and none was sent anywhere. Treat that file as a secret
and prefer fingerprints in any future dump; a `JSON.stringify` replacer that matches on the key name
does not catch a secret nested inside an object named `access`.
