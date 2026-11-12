# VIVIM Labs — NLP Visual Command UI Lab

**Scope:** UI research / experimental compiler projection, **not** another parser, command registry, Host, permission layer, or production UI publisher.

This is the isolated, locally runnable workbench for translating natural-language user commands into **symbols, icons, structured slot menus, token annotations, canonical command readings, effect previews, and longform English explanations**. It starts with the recovered Omega/NLCL and VIVIM Final implementations, while **reading current local Host and HQ source registries** instead of publishing a new canonical inventory.

## Run (Windows PowerShell / Node 22+)

Place `nlp-command-ui-lab` in the Labs checkout. Configure the locations of the CURRENT local repos if they are not siblings of `vivim-labs`:

```powershell
$env:VIVIM_HARNESS = 'C:\0-BlackBoxProject-0\vivim-web-harness'
$env:VIVIM_HQ = 'C:\0-BlackBoxProject-0\repos-hq'
cd C:\0-BlackBoxProject-0\vivim-labs\nlp-command-ui-lab
bun run start
# Open http://127.0.0.1:4387
```

Without locally accessible Host/HQ checkouts, the GUI **still starts** but shows those connections as unavailable and uses a strictly labelled fictional contact fixture for the email menu. No account contact lookup or email send is silently simulated as a real provider action. `VIVIM_TIERFLOW` may locate a non-sibling TierFlow checkout. `LAB_CONTACTS_PATH` may point to a **local, explicitly curated, noncommitting read-only JSON projection**, structured like `fixtures/demo-contacts.json`. Never commit private contacts to this public Labs repository.

```powershell
bun run test
```

### Round 2 — Living Composer and HQ knowledge lens

- **Independent full-stack UI lab:** the composer, symbol/slot preview, styling controls and source-connected wiki dock run in one loopback-only app. Four palette options, three type styles and four outline modes make the entire interaction language inspectable.
- **Read-only contextual knowledge:** `POST /api/context` calls the existing HQ `kbQuery()` and `searchContent()` directly against the actual local `.hq/index.db`, opened using the existing HQ `dbPath()` with SQLite **readOnly**. It builds an **ephemeral, evidence-linked working page**, not another wiki writer.
- **As-you-type updates:** topic extraction and project focus; ~260ms debounce, AbortController and stale response suppression. Shows source kind, project, excerpt, index file revision and known public GitHub file links. A missing HQ DB returns a labelled unavailable state.
- **Personal sessions and notes are opt-in:** disabled until the user checks the toggle. This is only a developmental HQ read-model experiment; the production personal corpus must come via a Host-authorized personal evidence read interface and cannot be dumped into HQ's development database.
- **Expanded command atlas:** HQ action/CLI, current Host typed actions, and TierFlow `UiCommandKind` source declarations remain visibly distinct. The visual affordance mapping adds role icons, evidence outline hints and accessible labels **without introducing a new command registry**.
- **Read-only whole-space census:** `bun run census` summarizes source counts/coverage/gaps; `bun run census:full` prints every discovered source descriptor. It cannot certify that unindexed provider/Studio/MCP/skills commands exist or are executable.

Read [Living Composer vision](docs/LIVING-COMPOSER-VISION.md) and [source census](docs/PLATFORM-COMMAND-CENSUS-ROUND2.md). All reported counts are scoped to observed source registries; do not interpret zero when the local repos are absent.

### What works now

- Loopback-only web UI, debounced keystroke feedback, source revision guards and stale response cancellation.
- Host's existing `createControlPlanner().planText()` (pure, **non-executing**) is dynamically imported from the existing Harness checkout when available. Host catalog is read fresh from disk on each request.
- HQ Wave 1 Lane C `inventory()` is dynamically called from the existing HQ repository. **Source observations and the Host action catalog remain different namespaces.** CLI roots are separately labeled. Inventory is recalculated, not mirrored.
- Visual `email` prototype opens a contacts menu instantly; names narrow by typing; Up/Down selects and Tab inserts exact selected span. Contact fixture is fictional and not a real authority.
- Under-composer, optional longform English interpretation; symbolic action/entity chips; source-token role view; typed slots; readiness/effect-free output; trace-only chat-like history; command atlas; HQ gap display.
- UI switches four feedback experiments on/off independently. No build dependencies, no provider APIs, no signed-in browser automation, no Playwright, no Puppeteer.

### Important limitations

- **No commands are executed by this Lab.** Enter/arrow only records a compiler observation within the temporary in-browser session. No send endpoint is implemented.
- The visual email flow is an experimental UX interaction until Host registers and verifies an email action, scoped contact read port, required grants and actual receipts. The fixture is never authoritative.
- No claim of platform-wide command completeness or production availability; see [HQ gap register](docs/HQ-GAPS.md).
- The original Omega interpreter code **is not copied/re-forked** here; current Host already vendors its pure NLCL engine and provides interpretation. The Lab consumes the existing Host path instead.
- Dynamic imports assume the HQ/Harness working trees are locally present. In the absence of those checkouts, the GUI honestly reports missing source connections.

## Design and provenance

See [Architecture](docs/ARCHITECTURE.md), [Recovered history](docs/RECOVERY.md), [HQ gaps](docs/HQ-GAPS.md), and [Feedback experiments](docs/EXPERIMENTS.md). No private corpora, sessions, API keys or real contact records belong here.

**Authority invariant:** HQ owns semantic discovery and source provenance; Host owns actual action ID, permission, effect, runtime receipt; TierFlow owns production rendering; Labs owns the experiments and their evidence. Matching names do not transfer authority.

## R005 progressive sprint — in-composer semantic density (2026-11-12)

The v0.3 prototype adds a keyboard-first **semantic spine inside the composer**, slash command discovery from source registries, generated per-command sigils, quiet/assisted/X-ray density, source/permission truth separation, meaningful change hints, conservative negation, Host refusal precedence and reversible Tab selections. Menus rank choices selected *in this browser session only*; no private preference storage is created.

Run `node --test tests/*.test.mjs` and `node scripts/semantic-bench.mjs` for synthetic acceptance. See [`docs/R005-PROGRESSIVE-SPRINT.md`](docs/R005-PROGRESSIVE-SPRINT.md) for the recorded results and remaining local production gates; see [`docs/100-SIGNAL-DENSE-COMPOSER-INNOVATIONS-R004.md`](docs/100-SIGNAL-DENSE-COMPOSER-INNOVATIONS-R004.md) on the research branch for the 100 design ideas. **No execution or authorization flow has been added.**

## R008 — AutoWiki screen-state mirror research

The AutoWiki design makes every eligible VIVIM screen a reversible, source-linked conceptual wiki view, with current work and recent verified source evidence prioritized. Read [R008 vision](docs/R008-AUTOWIKI-SCREEN-STATE-MIRROR-VISION.md). The source-backed HQ audit, full PRD, architecture, draft JSON contracts and 48 integration work orders live in the [HQ AutoWiki research branch](https://github.com/owenservera/repos-hq/tree/research/autowiki-screen-mirror-20261112/research/autowiki-screen-mirror). **Research only: no product integration or private history ingestion is claimed.**


## R009 — Rock Core, adaptive bounded intelligence

Storage capacity becomes a first-class VIVIM user experience: a per-owner protected **Rock Core** plus byte-capped reconstructible HQ index, graph, generated wiki and transient AutoWiki caches. Auto/Survival/Lean/Balanced/Full/Custom modes protect a physical disk reserve; the original personal corpus and authoritative journals are never automatically deleted. Read [R009 vision](docs/R009-ROCK-CORE-STORAGE-BOUNDED-INTELLIGENCE.md) and the canonical research [HQ Rock Core PRD + pure policy/membership prototype](https://github.com/owenservera/repos-hq/tree/research/rock-core-storage-bounds-20261112/research/rock-core-storage). No production storage writer has been enabled by these drafts.


## R010 — Semantic Wiki Compiler and deterministic capability language

R010 extends Living Composer + VIEW ALL + AutoWiki + Rock Core into a source-backed **compiled semantic service**: HQ code comments and versioned registry IDs describe meaning; deterministic typed language frames and Rust graph compute produce intent previews; Host remains the only authority for its actions, approval and receipts; TierFlow renders the exact source-linked trace. Read [R010 vision](docs/R010-SEMANTIC-WIKI-COMPILER-VISION.md) and the detailed [HQ architecture/PRD and pure research compiler](https://github.com/owenservera/repos-hq/tree/research/semantic-wiki-compiler-20261112/research/semantic-wiki-compiler). The 90% of supported frequent requests goal is a **benchmark hypothesis**, not achieved coverage. HQ compilation is inert; no new execution authority or ARBOR migration changes.
