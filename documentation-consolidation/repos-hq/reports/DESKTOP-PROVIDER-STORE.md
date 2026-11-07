# ZCode desktop — where the Model Settings provider list lives

Investigation date: 2026-11-04. Read-only. App under test: `C:/Program Files/ZCode/ZCode.exe`,
ProductVersion **3.17.2.9661**, running (10 processes, main pid 3640).

## Verdict

**Store: `C:/Users/VIVIM.inc/.zcode/v2/setting.json`** — the desktop settings JSON.

The desktop Model Settings provider surface is not a registry of API providers. It is a **provider
family** model with a hard-closed domain, persisted in that one file under these keys:

| Key | Live value (2026-11-04 20:01) | Schema in bundle |
| --- | --- | --- |
| `providerFamilyConnectionSelections` | `{"zai":{"kind":"individual-coding-plan"}}` | `mge=z.object({zai:ytn.optional(),bigmodel:ytn.optional()}).partial()` |
| `providerFamilyDomain` | `"zai"` | `cvr=z.enum(["zai","bigmodel"])` |
| `providerFamilyDomainUpdatedAt` | `1791919119689` | number |
| `providerFamilyDomainMigrated` | `true` | boolean |
| `enabledBuiltinAgentCliProviders` | `["glm"]` | `avr=z.array(z.enum(["claude","opencode","gemini","codex","glm"])).transform(()=>[...dge])` |
| `startPlanRecommendationDismissed` | `false` | boolean |

The provider-id map in the bundle closes the list to two families and six account ids:

```
tre={zai:"zai-api",bigmodel:"bigmodel-api"}
Zp={zaiIndividualCodingPlan:"account:zai-individual-coding-plan",
    zaiTeamCodingPlan:"account:zai-team-coding-plan",
    zaiStartPlan:"account:zai-start-plan",
    bigmodelIndividualCodingPlan:"account:bigmodel-individual-coding-plan",
    bigmodelTeamCodingPlan:"account:bigmodel-team-coding-plan",
    bigmodelStartPlan:"account:bigmodel-start-plan"}
```

That is why the page shows exactly "Z.ai" and "Start Plan": `providerFamilyDomain` admits only `zai`
or `bigmodel`, and "Start Plan" is the `*-start-plan` connection kind (`Start Plan is busy and
automatic model stream recovery reached the maximum retry count.` is a bundle string).
`provider_config.json`'s 8 providers can never be rendered by this page — it is a different surface
(the CLI/node provider registry, see below), not a second store for the same list.

## Step 1 — exact grep counts (`C:/Program Files/ZCode/resources/glm/zcode.cjs`)

```
      3 enabledBuiltinAgentCliProviders
      5 personalProviders
```

Zero occurrences of `modelProviders`, `provider-store`, `providerStore`, `modelProviderStore`.

`personalProviders` is not a persisted key name. It is a property on the resolved registry object:

```
zcodeBuiltinRevision:t.revision, personalRevision:n.revision,
zcodeBuiltinProviders:t.providers, zcodeBuiltinProviderTemplates:t.providerTemplates??dre.empty(),
personalProviders:n.providers, zcodeBuiltinModelRules:t.models, personalModels:n.models,
personalProviderOrder:n.providerOrder??[]
```

The desktop writes the same key it reverts: `enabledBuiltinAgentCliProviders`. The revert is not a
wrong-store artefact — it is in the code, twice:

```
avr=z.array(z.enum(["claude","opencode","gemini","codex","glm"])).transform(()=>[...dge])
function Ics(e){return!e||typeof e!="object"||Array.isArray(e)?e:{...e,enabledBuiltinAgentCliProviders:[...dge]}}
IG="glm", dge=[IG]        // dge === ["glm"]
mvr=z.preprocess(e=>vcs(ycs(kcs(wcs(xcs(_cs(Ccs(Ics(bcs(Scs(e)))))))))),Ecs)
```

`avr`'s `.transform()` discards whatever was parsed and returns `["glm"]`; `Ics` does the same again
inside the preprocess chain. Any value other than `["glm"]` is destroyed on the next parse.

## Step 2 — AppData enumeration

`C:/Users/VIVIM.inc/AppData/Roaming/ZCode/` contains only `.updaterId`, `lockfile`,
`rum-electron-store/ZGVmYXVsdA.json` (117 bytes, RUM arms/uid only), `zcode-data-size-telemetry.json`,
and `session/` (Chromium profile: Cache, Code Cache, IndexedDB, Local Storage, Session Storage,
Partitions for `zcode-coding-plan` and `zcode-embedded-browser`, WebStorage, GPUCache, ...).
`C:/Users/VIVIM.inc/AppData/Local/ZCode/` does not exist.

## Step 3 — Electron web stores: not the source

`Local Storage/leveldb` does match `deepseek` and `openrouter`, but only inside per-session
transcript blobs, never as a provider list:

```
..."modelSelection":{"providerId":"deepseek","modelId":"deepseek-flash","options":{"reasoningLevel":"max"}}...
..."modelSelection":{"providerId":"openrouter","modelId":"openrouter/auto",...}...
```

Those are per-conversation model choices written by sessions (ids also seen: `new-provider-2`,
`kilocode-free`, `minimax-m-plan`), not a roster.

A recursive grep for `personalProviders|zcodeBuiltinProviders|enabledBuiltinAgentCliProviders|
personalProviderOrder` over the entire `AppData/Roaming/ZCode` tree matched **only** `Code Cache`
files (compiled renderer JS that reads the settings object). Zero matches in Local Storage,
Session Storage, IndexedDB or WebStorage. `Session Storage`, `IndexedDB/file__0.indexeddb.leveldb` and
`WebStorage` returned nothing for those keys and nothing for `templateId`.

Conclusion: the desktop persists no provider list in Chromium web storage. `setting.json` is the only
store it holds.

## Step 4 — app-owned SQLite

No SQLite database with provider data exists. The only `.db` files under AppData/Roaming/ZCode are
Chromium internals: `session/declarative_performance_observer.db`,
`session/GPUPersistentCache/DawnGraphiteCache/*/cache.db`,
`session/Partitions/zcode-embedded-browser/declarative_performance_observer.db`.

`C:/Users/VIVIM.inc/.zcode/v2/tasks-index.sqlite` (read-only, bun:sqlite) tables:

```
automation_runs, automations, hosts, off_peak_tasks, project_roots, projects,
remote_project_entries, schema_migration, session_creation_commands,
task_group_admissions, task_group_members, task_group_view_node_orders,
task_group_workspace_bootstraps, task_groups, tasks, tasks_schema_migration
```

No provider keys anywhere.

## Writable while the app runs? No — plainly

- **`setting.json`: not safe while running.** It carries no lock file, but the app keeps settings in
  memory and re-serialises the whole document. Proof: `setting.json.bak-pre-models-2026-11-04` vs the
  live file differ on keys unrelated to providers — `zcodeInteractionBehavior` `queue`->`guide`,
  `toolSearchMode` `off`->`auto`, `taskAutoArchiveEnabled` `false`->`true`, `desktopWindowSize.height`
  `641`->`647`. The pre-models backup also still holds the pre-reset roster (14 ids):
  `["glm","deepseek","grok-build-oauth","openrouter","new-provider","new-provider-2",
  "kilocode-free","minimax-m-plan","opencode-zen-chat-2","zai-standard-api","openai","anthropic",
  "codex-oauth","claude-code-oauth"]`. A hand edit will be overwritten.
- **Binary LOCK:** `C:/Users/VIVIM.inc/AppData/Roaming/ZCode/session/Local Storage/leveldb/LOCK`
  (0 bytes, dir mtime Nov 4 19:56) and `C:/Users/VIVIM.inc/AppData/Roaming/ZCode/lockfile` are held by
  the running app. LevelDB takes an exclusive lock; writing it under a live process corrupts the store
  and the app ignores out-of-band edits anyway. It is also moot: that store holds no provider list.
- **Even with the app closed it is not writable for this key.** `avr`'s `.transform(()=>[...dge])` and
  `Ics` force `enabledBuiltinAgentCliProviders` to `["glm"]` on the very next parse of the file.

## Restore path, ranked

1. **UI only — for the desktop Model Settings provider list.** The domain is a closed two-value enum
   (`zai`/`bigmodel` over six `account:*` ids) and the file is re-serialised from memory; there is no
   file a user can write that the page will honour. A file write is not a restore path here, at any
   time, running or stopped.
2. **For the model providers that actually matter (the 8 usable ones): no desktop action needed.**
   They live in `C:/Users/VIVIM.inc/.zcode/v2/provider_config.json` (65291 bytes, Nov 4 19:54) as the
   personal provider overlay, resolved as `Dit="provider_config.json"` via
   `ZCODE_PERSONAL_PROVIDER_CONFIG_FILE` in `resolveNodeProviderRuntimePaths`. Writes must go through
   `replacePersonalConfig(t)` / `savePersonalProviderOverlay(t,n,o,i)`, which is what the app itself
   uses. The CLI already spawns on `deepseek/deepseek-flash`, so this path is already correct.
3. **Do not** write `enabledBuiltinAgentCliProviders` with 14 or 27 ids. The transform resets it to
   `["glm"]` on the next read. The observed revert is by design in this build, and it is not evidence
   of a wrong store.

## Drift

- App version **3.17.2.9661**. The ruled-out builtin-template path in the brief,
  `runtime/provider/windows-x86_64/3.15.1/endpoint-*/zcode-builtin.json`, is a **stale copy**; the live
  one is
  `C:/Users/VIVIM.inc/.zcode/v2/runtime/provider/windows-x86_64/3.17.2/endpoint-78d7c3bef4024722642626fe3669a799/zcode-builtin.json`
  (with `zcode-builtin-refresh.json`), dir mtime Nov 4 20:09 — actively refreshed.
- `setting.json` mtime Nov 4 20:01; `provider_config.json` Nov 4 19:54. Both post-date the 3.15.1 dir
  (Oct 12), so there is no stale-store explanation for the picker.
- `provider_config.json` has 43 `.bak-*` siblings in `~/.zcode/v2/`, i.e. that file is rewritten
  frequently by the CLI/CCR path — a second reason not to hand-edit it while anything runs.
