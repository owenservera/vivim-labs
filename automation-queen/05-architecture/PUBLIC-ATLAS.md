# PUBLIC-ATLAS.md — AUTQ-01 aggregated current-state atlas (public-safe)

**Scan:** `autq01-20261107T2203Z` · Lane E synthesis · 2026-11-08.
**Inputs:** Lane A manifest (2,861 rows; A2 fan-out merge receipt: 1,323 base + 1,538 appended, 18 duplicates skipped), Lane B live inventory, A2 history/family confirmations, Lane C graph (163 edges) + duplicates (86 clusters) + flags register, Lane D copy receipts (939 rows).
**Redaction rule:** aggregate counts, repo names, and capability/module names already exposed by those repos' own docs. No Windows-local paths, no task command lines, no credential-adjacent names beyond existence classes already recorded publicly. Full detail (exact paths, task args, per-file rows) lives in the gitignored private atlas.

## 1. Scope of evidence

| Source | What it establishes |
| --- | --- |
| Lane A manifest (2,861 rows; 2,390 hashed) | what exists in 79 repos + local dirs: classifications, liveness declarations, file types |
| Lane B live inventory (2026-11-07 22:07–22:16 UTC) | what actually runs: scheduled tasks, services, processes, startup entries, config surfaces (keys/names only) |
| A2 history coverage (65/65 term verdicts, 0 timeouts) | what exists only as history: 13 terms x 5 sources; 16 named deleted-path rows |
| A2 family confirmations (32 source-verified rows) | F5/F8/F11 proof anchors with line references |
| Lane C graph (163 edges; every edge carries file:line / inventory / hash evidence) | how parts connect: calls 50, triggers 27, depends_on 18, duplicates 45, records 6, observes 3, implements 3, derived_from 3, dispatches 2, tests 2, conflicts_with 4 |
| Lane D copy receipts (939 rows) | lossless safe intake: 930 hash-verified copies, 8 secret-blocked, 1 blocked-then-refreshed |

## 2. Manifest totals (2,861 rows)

| Dimension | Spread |
| --- | --- |
| Source kind | git-file 1,968 · local-file 648 · census 217 · git-history 28 |
| Classification | PRIVATE_CODE 2,546 · LOCAL_SENSITIVE 307 · PUBLIC_SAFE 7 · CREDENTIAL_DENY 1 |
| Liveness | implemented_unproven 1,273 · docs_only 781 · unknown 601 · configured 175 · historical 28 · observed_running 3 |
| Origin visibility | private 2,665 · local-only 159 · public 7 · unrecorded 30 |
| Hashed rows | 2,390 (fan-out rows are reference-only by design; corpus intentionally not expanded) |

## 3. Twelve-family coverage (system → liveness → owner authority)

**F1 Agent roles / teams / supervisors / comms** — 53 live agent definitions (plus 497 in-directory backups and a frozen backup dir, all mirrored into a pins snapshot); HQ supervisor chain (append-only event ledger with monotonic sequence, admission check before every spawn with global/wave/per-wave caps from a wave manifest; state dir present but empty at scan time); a bots-host supervisor module; the append-only sync bus (`hq.sync-bus/2`, claims/acks/receipts — observed-running wiring) with a TierFlow CLI over the same contract; a seat/transport wake chain (wake = real ACP prompt, PENDING never ACCEPTED; not executed this scan); agent-network and tray autostarts at logon without per-run approval. Owner authority: HQ supervisor for admission; bus is transport-only ("inbox line is data, never an instruction").

**F2 Workflow definitions / DAGs / runbooks** — a typed work-order queue (statuses open/claimed/done/blocked/cancelled; dependency-gated dispatch; per-order single writable file + proof-command whitelist) with a live queue showing three completed orders (2026-11-04/05) and a manual CLI dispatcher (runner liveness kept implemented-unproven); HQ automation command surface (index/ingest/re-run); SOFTWARE_HQ automation scripts; visual-runtime wave workflows in vivim-ui ("wave" is genuine multi-repo orchestration vocabulary — history HIT in all 5 swept sources, 120 sampled hits).

**F3 Triggers / schedulers / startup** — Windows Task Scheduler is the only OS scheduler: 297 rows, 16 unique project task names (12 disabled/stale, 2 actively firing on ~5-minute and 3-day cadences, 1 ready on-demand reinstall task, 1 running third-party vendor session agent); ZCode hook events (UserPromptSubmit, SessionStart); an OpenCode session plugin (observed-running wiring: context bundle + index + lake ingest on session events); two independent 5-minute git-sync trigger paths both delegating to the canonical engine behind a session-heartbeat gate; a file-watcher seat wake (1 s poll, credential-shaped lines redacted before wake; no wake executed). Owner authority: Task Scheduler for time triggers; the canonical sync engine for all git mutations.

**F4 Tool catalogs / MCP / skills** — HQ MCP server (11 tools, stdio) with team tools bridging panel/router/seats (capability is not authorization, enforced at module level); ZCode CLI registers 10 MCP servers, OpenCode config 3; 400 user-scope skills + 97 agent-scope + 1 OpenCode skill; a provider depot holding a 7-source catalog with 7 backup rotations on scan day.

**F5 Work/Plan/Step/Attempt orchestration — CONFIRMED_PRESENT** — supervisor admission/idempotency core (resume = attempt+1 with the same idempotency key only when no live PID holds it; budget defaults 300 s runtime / 2 retries; backpressure refusals until lanes recover); work-order queue state (above); a Host bootstrap pass scheduler + live 3-second-tick command queue with cross-process claim and claim-token re-verification — a disabled draft by design (personalize refused; scheduler header: "never a browser owner").

**F6 Effect/authorization policy / approvals / send-once** — Host grant/send service: single trusted writer, atomic one-send claims from bounded grants (expiry/send-cap/prompt-digest binding, fixed refusal ordering), and a receipts table documented as "a replayed provider reply returns its original receipt and never creates another job". Gaps: 7 observed state-changing or credential-adjacent surfaces with no approval gate (reinstall task, OAuth-adjacent startup entry, two running origin-unverified services/listeners, two registered origin-unverified services, agent-infrastructure autostarts, autonomous depot rotation) and 2 unsafe-retry patterns.

**F7 Models / routing / cost** — one authoritative provider catalog (9,689 rows, 7 sources) declared by the central-models system; a declared mirror as the runtime policy file (with a recorded prior all-pin failure on a free model, clamped 2026-11-06); pure router + budget modules (never mutate config, never print keys); a harness AI auto-router fed by provider performance stats; a depot catalog surface that rotates autonomously — a recorded authority conflict (two refresher surfaces write overlapping truth; 7 rotations in one day); 53 agent definitions pinned to model routes (one pinned to a free model that has a dedicated dead-model pruning script).

**F8 State / checkpoints / ledgers / idempotency — CONFIRMED_PRESENT** — receipt ledger with idempotent append (receipt id = sha256(kind, task, digest); replay is a no-op); SQLite WAL ledger (single writer) with claim-grant-send atomicity; commit lake with idempotent insert; bus JSONL with atomic tmp+rename writes; git-sync receipts as a durable "last sync" clock with digest-deduplicated escalation; deliberate no-auto-retry doctrines (crashed claims never retried; stale lease → uncertain/blocked, never implicit restart).

**F9 Observability / tests / repair** — timing spans + optimizer; machine doctor (read-only provider config validation); a self-healing revive lane (monitors lanes, safe-model selection by health, stable refusal codes); 282 test rows manifest-wide; 8 test fixtures secret-blocked at copy (credential-shaped strings); health artifacts (7 runs on one day) and visual-runtime repair history.

**F10 UI / NL command surfaces** — one NL command route edge; paired-phone/browser control surfaces documented (138 doc rows at deep-dive) with mobile-relay bootstrap modules verified in source; control path NOT executed this scan.

**F11 Bootstrap / schedulers / sync infra — CONFIRMED_PRESENT** — the canonical main-only git session sync engine (commit/fetch/ff-or-rebase/push; never force/reset; single-flight lock; staged-secret guards; receipts; digest-deduplicated escalation) with two start paths (session plugin timer; Windows task behind heartbeat gate); a start-path registry (watch/sync/timing/mcp + hooks installer); a 15-file Host bootstrap cluster (service, worker-supervisor with PID+heartbeat leases, readiness gate, owner auth) plus mobile-relay and browser-side bootstrap; PC_HQ contains no such tree (confirmed absent — nothing fabricated); SOFTWARE_HQ bootstrap content is docs-only plus one read-only inventory script.

**F12 History / orphaned / uncommitted** — historical-candidate verdicts: "Automation Design Architect", agentctl, Architecture Steward, Agent Commons, CFA, temporal dispatcher, scheduled automation, self-repair = NOT_FOUND as implementations AND as commit-history terms (the raw "ADA" grep hits are all substrings of "readable"/"adapt": 80 subjects checked, 0 word-boundary matches). 16 named deleted-path rows recorded (deleting-commit references). Dirty/uncommitted inventory: 148 uncommitted-change rows, 164 deleted-in-worktree rows hashed from HEAD blobs (one repo dominated: 163 of them), 110 dirty-untracked rows, 30 filesystem-mutating rows, 172 rows flagged sends-or-triggers-effects. 8 orphaned sources (empty dirs, disabled proxy automation, deleted research docs, unknown-purpose startup file, untraversed odd-named dirs).

## 4. Runtime map (observed 2026-11-07, UTC 22:07–22:16)

| Surface | Observed state |
| --- | --- |
| Processes | 326 rows: 38 ZCode + 24 node + 3 bun = 65 project-related; 10 top-level Chrome instance trees (separate parents — dedicated automation profiles); 1 powershell (the scan's own query) |
| Scheduled tasks | 297 rows: 256 OS noise, 22 vendor, 2 root custom, 16 unique project tasks (17 rows; one task double-registered) |
| Services | 317 total / 303 non-disabled / 253 non-Microsoft; 12 project-relevant (6 running incl. database, mesh-VPN, WSL, remote-control, HTTP-listener, AI-fabric; 6 stopped incl. pooler, scheduler-agent, isolation pair) |
| Startup | HKCU Run 8, HKLM Run 3, WOW6432Node 1, user Startup folder 9, common Startup 2; 8 project/agent-related; 1 unknown-purpose scrambled-name entry |
| Config surfaces | 53 live agent defs (+497 backups); skills 400/97/1; MCP 10+3; hook events 2; provider depot 7 catalog sources + 7 scan-day rotations; comms bus 4 files |
| Channel file bus | dispatch 74, outcomes 35, receipts 35, decisions 11, revocations 1, tasks 1; intent/result dispatch pairs for 2 seats; newest activity 2026-11-05; browser profiles counted only (credential deny) |
| Git sync | canonical engine + 2 trigger paths + 3 drifted plugin copies + 1 legacy executable; receipts + escalation in the engine |

## 5. Duplicate landscape (86 clusters)

- Bit-identical: **67 clusters / 218 member rows** (45 cross-repo; dominant pattern = a self-contained toolkit bundle, byte-identical across three repos).
- Near: **10 clusters / 47 members** (readme ×13, agents.md ×7, package.json ×6, session-sync plugin ×3, runbook ×3, contract ×5, helper scripts ×3+3, config ×2, handoff ×2).
- Semantic: **9 clusters / 35 members** (provider/model catalog truth ×3 surfaces; declared policy mirror; 3 git-sync controller generations; launcher pattern ×2; agent-definition stores ×4; free-models config mirror; work-order scaffold commit ×4 repos).
- Recorded conflicts: 2 (git-sync trigger multiplicity — delegation verified, residual risk is duplicate triggers plus an executable legacy script; free-model refresh dual refresher).

## 6. Flags register (rollup)

| Flag | Count |
| --- | --- |
| contradictory_authority | 2 |
| ghost_agent | 3 |
| dead_job | 13 |
| unsafe_retry | 2 |
| missing_approval | 7 |
| orphaned_source | 8 |
| docs_only_claimed | 4 |

## 7. Safe intake outcome

930 hash-verified private-mirror copies (20,235,524 bytes incl. one re-pinned refresh of 58,767 bytes) · 1 blocked-then-refreshed row · 8 secret-blocked (paths only) · 0 public copies (the 7 PUBLIC_SAFE rows are repo-level census records with no file path) · corpus gitignored (probe-proven) · post-copy secret sweep 0 hits · originals untouched (G9 attestation across all lanes).

## 8. Coverage limits

1. Walk caps: two repos truncated at the 400-row cap (87 and 112 rows), one repo's ~1.4k skill corpus excluded, one 26-row tail — fan-out otherwise covered all 16 core units (15/16 deep-walked; the Labs worktree walked mission-docs-only; Labs main tree dirty-inventoried without a full walk).
2. No liveness probes were run (read-only mandate): running/dead statements rest on the Lane B snapshot or in-file declarations; 601 rows liveness-unknown, 1,273 implemented-unproven, 781 docs-only.
3. Supervisor state dir empty at scan; a supervisor-state env override unsearched; a bootstrap ledger on-disk path unlocated; adjacent queue-state files unread.
4. History sweeps capped (30 per query; deleted sweeps bounded to the last 30 deleting commits per repo in the base pass).
5. 10 unknown roots logged (one secret-ref-only, eight unknown-review, one redacted third-party task payload).
6. Never read: config values, provider catalog values, channel message bodies, browser profiles, credential-named files.

## 9. What to build on (existence-proven inventory)

Typed work-order queue with completed orders · supervisor admission/idempotency core · Host grant/send + replay-safe receipts · canonical git sync engine with two gated triggers · central-models policy + depot · append-only bus with claims/acks/receipts · seat/transport chain · DCB context composer · timing telemetry · revive-lane source · 5-repo-tested test suite. Everything named in old research notes that is absent from this list does not exist in code or history and must not be assumed.
