# Command-space taxonomy — 279 candidate capabilities, 23 domains

**Source:** generated from research-only [machine-readable registry](registry/command-space.v0.json), which is *not* the ARBOR action catalog. All 279 records are `status: proposed` and must not be treated as shipped features. These are **semantic intent classes**, not implementation contracts, independent OS APIs, or user-frequency metrics.

## Coverage map

| Domain | Candidates | User outcome | Representative utterance |
|---|---:|---|---|
| `files.find` | 14 | Locate/read/search files and storage without changing them | Find my latest tax PDF |
| `files.change` | 14 | Create/move/rename/patch/trash scoped files with previews | Create a folder for my invoices |
| `documents` | 12 | Read, summarize, compare and transform PDFs and documents | Summarize this PDF in five points |
| `archives` | 10 | Inspect, unpack, compress, back up and restore archives | Show the contents of this ZIP |
| `apps.process` | 13 | App launching, windows, supervised processes and jobs | What applications are running |
| `terminal` | 10 | Constrained developer CLI and bounded interactive sessions; raw shell withheld | Show installed Rust and Bun versions |
| `browser.tabs` | 12 | Navigation, tab state, page reading and downloads | What tabs are open in my browser |
| `browser.workflow` | 12 | Signed-in browser form actions through existing Studio lease only | Open this project's issue tracker |
| `knowledge.wiki` | 12 | Host Wiki CRUD, revision/source and knowledge projection | Search my Wiki for device pairing |
| `notes.tasks` | 14 | Notes, tasks, checklists and voice-to-tasks preparation | Remember this thought as a note |
| `calendar.time` | 12 | Schedule, events, time expressions and reminders | What's on my calendar today |
| `communications` | 12 | Messages, email, contacts and external send proposals | Write a message to the team without sending it |
| `research.search` | 10 | Public information retrieval, synthesis and citations | Find the official API documentation |
| `media` | 12 | Local images/audio/video, conversions and transcripts | Find pictures taken in June |
| `system.settings` | 12 | System information, volume, display settings and high-risk OS changes | Show my PC system information |
| `devices.network` | 13 | Paired devices, local connectivity and permitted remote operations | Which devices are paired |
| `privacy.security` | 12 | Scope/grant discovery, audit, revocation and sensitive exports | Show who can access my data |
| `automation` | 12 | Scheduled, conditional, recurring and reversible workflows | What automations do I have |
| `developer.git` | 12 | Repository status/diffs and guarded version control operations | Show Git changes in ARBOR |
| `developer.build` | 13 | Build/test/server operations through approved developer commands | Run focused Rust tests for this crate |
| `ui.workspace` | 12 | Command discovery, previews, navigation, approvals and receipts | Open the Wiki screen |
| `data.tables` | 12 | CSV/Excel inspect, transform, export and charts | Describe the columns in this CSV |
| `personal.context` | 12 | Personal preferences, aliases, context and trusted language rules | What preferences have I saved |

**Counts checked against JSON:** 279 unique IDs, 23 domains, 18 adapter families; R0 **118**, R1 **89**, R2 **60**, R3 **12**. Planned implementation phases: **P0 65**, **P1 98**, **P2 98**, **P3 18**. P0 is intended prioritization only; it does **not** mean functional.

## Primitive composition, not 279 code paths

The user-visible noun/verb space is much larger than the implementation primitive set. A single secure Rust `filesystem.list` or `filesystem.find` broker can support many semantic commands, while compound queries (“find all old invoices and zip them”) compile to validated primitive DAGs. The dominant engineering challenge is not memorizing phrases: it is **grounding users, sources and targets, enforcing OS permissions, preserving a single Host effect path, and providing usable results**.

## 20 high-frequency journey hypotheses (not measured)

1. “What's taking up so much space?” → device storage scan → ranked folder/file list with source timestamps.
2. “Find my five biggest Downloads files” → scoped file index → sort → exact metadata.
3. “Organize my Downloads” → draft grouping proposal → collision/preview → approved batch move.
4. “Find that PDF from last week” → time-bounded file search → ambiguity chooser → open.
5. “Put these PDFs together” → verify selected PDFs and order → produce new file.
6. “What did I change in this document?” → source/selected version pair → diff or summary.
7. “Open the app I was using” → recent foreground-session evidence → exact app choice.
8. “Close the frozen program” → inspect process + owner → explicit termination preview.
9. “Run the Rust tests” → resolve actual repository → approved cargo binary + arguments → stream output → receipt.
10. “Why did the build fail?” → read real test/build receipt → extract error frames and paths.
11. “Open the Wiki beside my chat” → pure PWA/GPUI layout projection, no OS effect.
12. “Remember this as a task” → capture typed task from utterance → explicit persistence.
13. “Show me everything due tomorrow” → scoped tasks/reminders/calendar read.
14. “Summarize my notes about ARBOR” → source-backed Wiki/note retrieval → grounded synthesis.
15. “Send this document to Marta” → resolve selected document + contact/channel + scope → exact-content approval → external effect receipt.
16. “Look up this error in the official docs” → source retrieval → compare current local issue and provenance.
17. “Move these photos into a folder” → concrete selection → preview → approved move.
18. “What happened to the file I moved?” → inspect receipt + historical file identity.
19. “When this folder gets a receipt, save it to my expenses” → durable watch + conditional workflow proposal → activation approval.
20. “Undo the last change” → find compatible receipt → preview inverse effect → separate authorized compensation.

The list deliberately mixes queries, local reversible effects, external effects and composed workflows. It is the candidate evaluation corpus—not a statement that ARBOR already performs those tasks.

## Intent language features that must be orthogonal to domains

- **Nouns and references:** files, folder, process, window, app, URL, tab, email, contact, device, Wiki page, task, current selection.
- **Selection/filter:** newest, biggest, duplicates, modified after, matching regex, from sender, only PDFs, less than size; typed predicates with resource caps.
- **Ordering/limits:** top 5, descending, newest first, only three results, first page, all *within bounds*.
- **Time:** yesterday, every Friday, two hours ago, after the call, next month; capture timezone/precision.
- **Relations:** in/under/near/from/to, within this folder, on my laptop, current tab, previously opened.
- **Conditionals:** if/when/unless; durable event source required.
- **Sequences:** then, before, after, also; deterministic DAG and step-level receipts.
- **Comparisons:** compare A/B, changes since snapshot, summarize differences, ignore duplicates.
- **Reversal:** undo, restore, stop, cancel; only when an actual supported inverse exists.
- **Personalization:** alias/phrase learning cannot enlarge approval or capability scope.
- **Languages:** English initially; plan Castilian Spanish and Mallorquí as locale-specific grammar/evaluation, not silent assumptions of verified parity.

## Risk patterns

- `R0`: content visibility and scope always controlled; read-only is not unlimited disclosure.
- `R1`: reversible local change, exact preview/approval, receipt.
- `R2`: sensitive writes, remote effects, personal data export, communications, loss-prone action.
- `R3`: privileged/destructive action; **not executable under current ARBOR risk rules**.

Commands such as “clean up” and “fix my PC” have no safe default effect. Discover state, produce a proposed plan, clarify missing scope and never infer permission from imperative language.

## Candidate prioritization basis

P0: file discovery/metadata, storage overview, existing Wiki/personalization, capability/help, system read diagnostics, receipt/approval UI; native value without a broad new OS privilege surface. P1: reversible file operations and scoped developer/process jobs, better search, sessions, workflows preview. P2: browser/communications/media/docs/spreadsheets and connected accounts after verified brokers and grants. P3: destructive/privileged/high-sensitivity operations only when independent safety requirements are met.

**Metric reminder:** 279 classes are not “279 implemented commands,” and certainly do not imply **90% of actual user requests**. See [measurement](06-90PCT-MEASUREMENT.md).
