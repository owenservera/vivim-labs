# LOCAL AGENT MASTER MISSION — Recover the complete automation and agentic system into VIVIM Labs

**Assignment:** AUTQ-01 · Forensic discovery, lossless safe intake of eligible sources, unified system atlas, future Automation Queen blueprint.
**Start condition:** Owner has requested inventory and copying into Labs. This is **not** authority to execute automations, stop/start processes, publish private assets, change repo ownership, or migrate runtime.
**Repo:** owenservera/vivim-labs. **Public visibility warning:** do not publish contents of private repos or Windows-local sources. Read README.md and SCHEMA-v1.json first.

## End goal
An automation *design architect above individual automations*: given intent and authority, it can understand the installed capabilities, derive a Workflow/Plan/DAG, pick agents/tools/models, handle scheduling and resources, supervise execution through the canonical runtime, request approvals, verify receipts, recover safely, maintain knowledge and recommend improvements. **This PR only captures the real system and produces an evidence-based blueprint.** It does not appoint itself as live scheduler or provider-effect executor.

## Current leads — verify, don't assume
- Existing Repos HQ canonical Rust HQKG, supervisor, agent bus/protocol, Work and execution contracts; follow its multiwave admission and evidence policy.
- Harness Host/bootstrap and Studio/provider action surface, typed permission gates, tools/plugins/skills, browser execution and mobile relay; preserve one Host-owned provider effect authority.
- SOFTWARE_HQ automation and agent assets, OpenCode configuration, model policy/doctor, orchestration/discovery tools.
- VIVIM Labs already has agentic_excelence, orphaned-work automation research and previous code references. Incorporate, do not create a second taxonomy or replace history.
- TierFlow World/typed commands and other checked-out repos are potential consumers or previous automation origins.
- Historical named research candidates: Automation Design Architect (ADA); Architecture Steward + Agent Commons + 10 CFA; agentctl; autonomous Work/Plan/Step/Attempt; temporal dispatcher, leases, ledger, workflows, scheduled automations, self-repair. Look for their actual files/commits before classifying.
- Start with the expected Windows working root C:\0-BlackBoxProject-0 and user-scoped OpenCode/ZCode locations; *discover existence* rather than treating any assumed path as fact.

## Exact scope: twelve automation families
1. Agent roles, teams, delegation, supervisor hierarchy and agent communications.
2. Automation/workflow definitions, DAGs, runbooks, scripts, repeatable recipes.
3. Triggers, watchers, timers, schedulers, event subscriptions and startup.
4. Tool/capability catalogs, MCP/ACP bridges, CLI, adapters, plugins and skills.
5. Work/Plan/Step/Attempt/Execution orchestration, job queues, context and events.
6. Effect/authorization policy, approvals, browser interaction, send-once handling.
7. Models/routing/concurrency/token/cost/budget and resource policies.
8. State, checkpoints, memory/context, durable ledgers, retries and idempotency.
9. Observability, telemetry, audits, tests, failure diagnosis, repair and release gates.
10. UI/NL command language, phone/desktop control surfaces, notifications.
11. Bootstrap, installer, scheduled task/service/Windows process and sync infrastructure.
12. Historical branches, archived research, deleted/orphaned implementations and uncommitted local code.

Mark any class with no observed source UNKNOWN or NOT_FOUND_IN_SCOPE; never imply a complete scan from a search hit.

## Execution protocol: five independently scoped lanes

### Lane A — Repository and commit archaeology (read-only)
Enumerate all available user-owned VIVIM-related repos/checked-out worktrees; record current HEAD, branch, dirty state, remote, visibility, shallow clone status and accessible Git history. Explore known and plausible automation subdirectories with bounded file-content search. Inspect old branches, archived/orphaned source and historical commits for genuinely reusable assets. Record source file path/hash and classification in local PRIVATE-SOURCE-MANIFEST.jsonl, never in public catalog. Collect exact artifacts only after D approves copy classification. Output A-SOURCE-COVERAGE.private.json and public-safe aggregate counts.

### Lane B — Windows and live automation archaeology (read-only)
Within project-related roots only, inventory user-scoped configuration: OpenCode/ZCode, agents/skills/hooks/plugins/MCP config *keys without secret values*, scripts, local CLI launchers, Windows Task Scheduler registrations, owned services, startup entries, known project watchers and running processes. Capture name/state/dependency/launcher metadata privately with automatic redaction; avoid dumping full command lines where they may contain tokens. Determine configured vs observed running vs stale vs docs-only. Do not stop jobs, change registry/services, launch agents, alter profiles or open browser sessions. Expand search outside these roots only with owner-reviewed scope. Output B-LIVE-INVENTORY.private.json and B-UNKNOWN-ROOTS.private.md.

### Lane C — Semantic system map (read-only)
Extract how parts work, not just filenames: source→definition→trigger→workflow→agent→dispatcher→capability→authorization→effect→verification→receipt→memory/world update. Derive contract edges, owners, version boundaries, shared memory/state stores, failure paths and app-facing entry points. Find bit-identical SHA duplicates and near/semantic duplicates; preserve all original provenance links. Map all records into the 217-type file taxonomy where applicable. Flag contradictory authority, ghost agents, dead jobs, unsafe retries, missing approvals and orphaned source. Output C-DEPENDENCY-GRAPH.private.jsonl, C-DUPLICATES.private.jsonl and redacted architecture summary.

### Lane D — Safe, immutable copy and lineage (single writer; gated)
This lane creates Labs central view only after the A/B source inventory is classified:
- Before copies, inspect free disk space, projected bytes and name/path collisions. Deduplicate by content hash, with provenance for every source pointer, commit, timestamp, author and license.
- PUBLIC_SAFE: only copy text from already-public and appropriately licensed origins, *after* credential/security scan and manual/agent review; tracked into 03-corpus-public with attribution and source hash.
- PRIVATE_CODE or LOCAL_SENSITIVE: copy eligible non-credential code/config into gitignored `automation-queen/.private-local/corpus/<source-id>/...`; exact paths & SHA in PRIVATE manifest; enforce restrictive filesystem ACLs where supported.
- SECRET_REF_ONLY / CREDENTIAL_DENY: never copy bytes (including git history dumps, token-bearing values, cookie/browser profiles, credential databases, private keys or raw personal conversation archives); record existence, type and secure location pointer privately only.
- UNKNOWN_REVIEW: don't copy or publish until classified.
- Large binaries, DBs, caches, package installations and generated directories: reference/size/hash-only by default; selectively inspect small safe schemas.
- Never overwrite originals. Never alter original git state. Verify each created copy by SHA256. Preserve an immutable inventory even for copies that are BLOCKED, SKIPPED, INACCESSIBLE or NOT_FOUND.
- Install/verify .gitignore and staged-file secret/visibility checks *before* any snapshot; stop if accidental private staging detected. Never use `git add .` or `git add -f` on private intake.
- Do not perform a push of harvested corpus without a separate PUBLIC_RELEASE_REVIEW gate; a secret scan alone is not proof that private code may be published.

### Lane E — Synthesis and future Queen design
Using evidence from A–D, produce a single private current-state SYSTEM-ATLAS that shows all automation assets and an *aggregated public-safe* PUBLIC-ATLAS. Build capability coverage, lifecycle state, source ownership, deployment/user entrypoints, duplicate clusters and keeper candidates. Propose one future Queen control-plane on existing Repos HQ/Host/TierFlow boundaries (not a second scheduler/graph/memory/browser executor). Identify what's missing for intent→design→simulate→approval→run→verify→repair→learn, with 20 prioritized upgrade candidates, alternatives and tradeoffs. Produce migration waves, adapter contracts, unification decisions, tests, rollback plan; **do not execute a migration**.

## Serial gates across lanes
G0 / BASELINE: scan set and all source SHAs/PC roots pinned; privacy state confirmed; active running jobs identified.
G1 / COVERAGE: 12 classes inventoried with source references or explicit not-found/blockers; history and local dirty artifacts checked.
G2 / SECURITY: public safe set separated from private; no secrets, private source code or sensitive metadata staged.
G3 / COPY INTEGRITY: every copied file hash matches original; no original path changed; all skipped items have reasons.
G4 / LINEAGE: file↔agent↔workflow↔trigger↔runtime↔authority↔evidence relations recorded; duplicates mapped.
G5 / REALITY: implemented vs currently running vs historical vs claims differentiated; no invented proof.
G6 / SYSTEM VIEW: an operator can find the owner and evidence for each automation, follow dependents, detect gaps and locate a copy.
G7 / QUEEN BLUEPRINT: single authority matrix, interoperable contracts, top 20 opportunities, migration and risk gates.
G8 / SAFE PUBLICATION: only reviewed public information, approved public copies and sanitized summaries are committed.
G9 / NO DISRUPTION: existing sessions, startup tasks, provider profiles, repos and production effects untouched.
Every gate is PASS/FAIL/UNRUN/BLOCKED with raw evidence, exact checks and responsible owner. Do not collapse UNRUN to PASS.

## Core invariants to carry forward
- Deterministic-first governed Work substrate: Trigger → Work → Plan → Authority/Gate → Step/Attempt → Capability → Checkpoint → Verification → Evidence → World Update → Attention/Continuity.
- Agent ≠ Work ≠ Plan ≠ Run ≠ Attempt ≠ Capability. Models and agents never grant permission.
- Single effect owner: Host/browser sender stays authoritative. Never auto-resend an unknown external effect. Never ingest credentials as knowledge.
- Reuse canonical HQKG, supervisor, software model-policy and existing agent messaging; Labs remains a **research and consolidation workspace** until a separate runtime migration grant.
- All future Queen capabilities should be operable through natural language on desktop and paired mobile via Host-owned contracts. Portable core, Windows/Android adapters.
- No direct APIs to provider webapps. No Playwright/Puppeteer. No GitHub Actions, force-push, profile reset or autonomous Git repository deletion.
- Concurrency ceiling is 5 waves / 10 lanes per wave / 50 total, **not a launch target**. Intake discovery starts read-only and bounded, no nested agent spawn without supervisor registration.
- Main-branch/fast-forward is the runtime integration policy; this document is held on a temporary proposal branch exclusively for PR review. Avoid conflict with Repos HQ intelligence integration PR #9.

## Concrete artifacts to leave in Labs and locally
Public/version-controlled (when independently reviewed):
- README.md, SCHEMA-v1.json, PUBLIC-ATLAS.md, PUBLIC-MANIFEST.jsonl (aggregate/surrogate only),
- redacted taxonomy crosswalk, dependency diagram, QUEEN-BLUEPRINT.md,
- GATE-REPORT.md, redacted GAP-REGISTER.md, licensed public corpus selected for copy.

Private/local gitignored:
- .private-local/PRIVATE-SOURCE-MANIFEST.jsonl and raw immutable source corpus with accurate origin refs,
- .private-local/SYSTEM-ATLAS.private.md, installed automation service/process index,
- .private-local/DUPLICATES.private.jsonl, exact-path dependency graph, copy receipts, blocked files and unresolved secrets references.

Finish with counts: total discovered candidates, exact copied public, exact mirrored private, duplicates, references-only, secret-excluded, inaccessible, historical, observed-running, current-source-verified, uncertain. Include bytes/copy hashes and catalog completeness per scope. Never invent counts.

## Recovery and rollbacks
Treat every scan/copy job as idempotent with a unique scanId and append-only receipts. Re-run must detect already-copied hashes and only add new provenance. Temporary files are atomic rename into private staging after digest verification. On interrupted collection, resume from last verified receipt without deleting any existing data. On a leak or suspicious staged file: abort all commits/pushes, remove staged content, retain only a local incident receipt, require owner review. No destructive cleanup of originals.

## Final local-agent handoff
At completion, return:
1. exact scopes scanned and what remained inaccessible,
2. local-private catalogue path and public-safe Labs PR/commit link,
3. evidence of cryptographic copy integrity and zero source modifications,
4. cross-system current-state map and distinct QUEEN-BLUEPRINT,
5. top 20 priorities and five future implementation lanes, sorted by dependency,
6. complete PASS/FAIL/UNRUN/BLOCKED gate board and what needs owner authorization.

**Do not say 'everything copied' if any candidate is skipped, inaccessible, private-only or excluded for safety.**
