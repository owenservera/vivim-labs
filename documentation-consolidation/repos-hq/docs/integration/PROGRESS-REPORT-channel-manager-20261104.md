# Progress report — channel-manager — 2026-11-04

Session lane: channel-manager (7d24). Stopped at owner request. Scope: channel-seat bootstrap, shared-core merge, cascade, wake verification, lease check.

## 1. WHAT RAN

| Kind | Id | Time (UTC) | Result |
|---|---|---|---|
| bus send | 20261103T233459979Z-7efccf | 23:34:59 | claim sent: channel-core, TTL 90 |
| bus send | 20261103T234154448Z-6c5693 | 23:41:54 | decision sent: gemini-channel |
| bus send | 20261103T234155767Z-a4e051 | 23:41:55 | decision sent: claude-channel |
| bus send | 20261103T234156527Z-bb8879 | 23:41:56 | decision sent: atlas |
| bus send | 20261103T234157189Z-126bb2 | 23:41:57 | status sent: * |
| bus send | 20261103T234650373Z-4bf076 | 23:46:50 | status sent: CM-002 clear |
| bus send | 20261104T005810760Z-ff68bf | 00:58:10 | decision sent: CM-010/011 |
| dispatch | 3 × session recon (qwen/gemini/chatgpt) | 23:31–23:34 | completed; three reports landed |
| dispatch | forge: site-generic core | 23:36 | stopped: zero file writes, 7 min, no checkpoint (CM-008) |
| dispatch | oracle: live cascade delivery | 23:38 | stopped: no delivery report; no ACP path exists (CM-009) |
| dispatch | fixer: doctrine + prompt log | 23:47 | stopped after both edits landed |
| wake initiated | none | — | — |
| wake received | MW-20261104T005233Z-channel-manager | 00:52 | read |
| wake received | MW-20261104T011250Z-channel-manager | 01:12 | read; BUS-ACK answered |
| wake received | MW-20261104T011918Z-channel-manager | 01:19 | this report |
| wake read | MW-20261104T004708Z-claude | 00:47 | ACCEPTED; reply received |
| wake read | MW-20261104T005025Z-chatgpt | 00:50 | ACCEPTED; reply received |
| wake read | MW-20261104T005323Z-gemini | 00:53 | NOT ACCEPTED: accepted:false, pending:true, reply:null after 200 s |
| wake read | MW-20261104T011026Z-gemini | 01:10 | ACCEPTED; reply received |
| provider send | none | — | 0 receipts produced by this session |

## 2. WHAT LANDED

| Path | Change | Proof command |
|---|---|---|
| `C:/Users/VIVIM.inc/.zcode/agents/channel-manager.md` | seat contract, authority, reporting line, subagent rule | `head -6`; harness registered the agent type |
| `C:/0-BlackBoxProject-0/repos_hq/.opencode/agent/channel-manager.md` | project mirror, bodies identical | bun body-compare: `BODIES IDENTICAL (mirror note excluded)` |
| `C:/VIVIM/channel/seats.json` | seat registry + shared-surface owner | `bun -e "JSON.parse(fs.readFileSync(...))"` |
| `C:/VIVIM/channel/decisions/CM-001..CM-011.json` | 11 decisions (doctrine, merges, authorizations, takeover, delivery) | `ls C:/VIVIM/channel/decisions` |
| `docs/integration/STREAM-BOUNDARIES.md` | channel ownership section: per-site speaker, manager-owned core, reporting line | `sed -n '50,72p'` |
| `docs/OWNER-PROMPTS.md` | P34–P37 appended verbatim | `grep -n "P34-7d24\|P37-7d24"` |
| `vivim-web-harness/studio/src/studio/chat-channel.mjs` | `<site>.<action>` resolver, `SERVED_SITES`, UNCERTAIN branch, empty-OK guard, site on `createThread` and `send` | `node --test studio/test/unit/chat-channel.test.mjs` → 20/20 |
| `vivim-web-harness/studio/bin/commands/chat.mjs` | `--site`, per-site transport, site-scoped ask/work | `chat stats --site gemini`, `--site claude`, `--site gpt5` fails |
| `vivim-web-harness/schemas/chat-receipt.v1.json` | scopes pattern for eight sites, chatgpt.* preserved | node regex check: accepts chatgpt/gemini/claude, rejects gpt5 |
| `vivim-web-harness/studio/test/unit/chat-channel.test.mjs` | 20 tests (5 core + reserved-site + site-on-send) | core suite 20/20 |
| `docs/integration/PROGRESS-REPORT-channel-manager-20261104.md` | this report | `wc -l` |

## 3. WHAT IS FULLY WORKING

| Thing | Command | Result |
|---|---|---|
| core scope gate + uncertainty | `cd C:/0-BlackBoxProject-0/vivim-web-harness && node --test studio/test/unit/chat-channel.test.mjs` | 20 pass / 0 fail |
| receipt scopes for all sites | `node -e "const s=require('./schemas/chat-receipt.v1.json');const r=new RegExp(s.properties.scopes.items.pattern);console.log(r.test('gemini.send'),r.test('gpt5.send'))"` | `true false` |
| CLI site transport construction | `node studio/bin/studio.mjs chat stats --site gemini` | JSON stats, no send |
| CLI unknown-site refusal | `node studio/bin/studio.mjs chat ask --site gpt5 --thread t1 --prompt-file <file>` | `error: unsupported chat site "gpt5"` |
| bus delivery record | `bun scripts/sync.mjs log --topic channel-manager` | 7 messages, ids above |
| seat roster | `bun hq/bin/hq.mjs seat list` | chatgpt/gemini/claude/channel-manager listed |
| decision records parse | `bun -e "JSON.parse(require('fs').readFileSync('C:/VIVIM/channel/decisions/CM-010.json'))"` | valid JSON |
| 9222 lease free | `ls "C:/Users/VIVIM.inc/AppData/Local/AION/drive-leases/9222"` | No such file or directory |
| pid 24836 gone | `powershell -NoProfile -Command "Get-Process -Id 24836"` | no output (process absent) |
| seat contract mirror | bun body compare | `BODIES IDENTICAL (mirror note excluded)` |

## 4. WHAT IS PARTIALLY WORKING

| Mechanism | What is unproven | Evidence |
|---|---|---|
| Gemini cold-tab handling | cold-tab re-query and COMPOSER_UNREADABLE cases pass; the stale-composer-handle case still fails | `node --test studio/test/unit/gemini-channel.test.mjs` → 35/36, 1 fail |
| Claude core transport | the skip test `a claude.send scope reaches the transport through the policy core` is now testable but has not been flipped to a passing test | `node --test studio/test/unit/channels-claude.test.mjs` → 30 pass / 1 skip; skip text names the test |
| Wake reply capture | short wakes return replies (gemini 011026 accepted:true); a turn longer than `--wait` returns `pending:true, reply:null` and is never back-filled | `MW-20261104T005323Z-gemini.result.json`; `docs/AGENT-BUS.md:83` |
| Site on `transport.send` | unit test proves the argument; live transports were not re-run | `chat-channel.test.mjs` site-on-send test; live = UNVERIFIED |
| Live provider sends | no send was initiated by this session; receipts exist from the seats, not from here | `C:/VIVIM/channel/receipts` shows seat actors, not channel-manager |
| Thread 6aea0b20 readability | lease is free, but the thread was not re-read | UNVERIFIED: no browser read attempted |

## 5. WHAT DOES NOT WORK

| Failure | Exact error | Believed cause |
|---|---|---|
| Gemini stale composer handle | `AssertionError: 'PROMPT_MISMATCH' == 'PAGE_SHAPE_UNKNOWN'` at `studio/test/unit/gemini-channel.test.mjs:343` | stale-handle branch in `studio/src/studio/channels/gemini.mjs`; exact line UNVERIFIED (not read) |
| Wake longer than `--wait` | `accepted:false, pending:true, reply:null`, detail `the turn was still running after 200s` | `hq seat wake` has no back-fill; `docs/AGENT-BUS.md:83` |
| ACP session send | unavailable; `session/resume` times out | `docs/AGENT-BUS.md:85` |
| Two managers, one bus id | both sessions send as `channel-manager`; dispatch intents carry `requestedBy: channel-manager` | process defect: no per-session bus id; UNFIXED |
| `bun run node --test` in the seat shell | passes `node` as a script name (seat report) | environment-dependent; works in this session; root cause UNVERIFIED |
| 9222 lease as a blocker | lease directory absent; pid 24836 dead; port 9222 still owned by pid 26816 | not a live blocker: the job is gone, nothing to release |

## 6. WHAT IS STILL NOT DONE

- Sprint prompt files for the three seats; `C:/VIVIM/channel/sprints/` is empty. Superseded by the slice wakes.
- Claude `actions.mjs` composer-probe fix (authorized CM-004, not landed).
- Gemini splice-extractor fix and thread-URL evidence (authorized CM-003, not landed).
- Gemini stale-handle fix (current failing test; beyond the CM-011 cold-tab scope).
- ChatGPT `CONTRACT.md` sync, one-file round trip, `evidenceState` test (accepted CM-005, not landed).
- Re-read of thread 6aea0b20 after the lease freed.
- Commit of the `vivim-web-harness` changes (not requested; working tree only).

## 7. DEFECTS FOUND IN OTHERS' CODE

| Defect | Evidence | State |
|---|---|---|
| `chat-channel.mjs:16-18` hard-coded `chatgpt.*` scopes | gemini/claude `CHANNEL_DENIED` before transport (seat reports) | fixed CM-002 |
| `chat-channel.mjs:296-320` no UNCERTAIN branch: UNCERTAIN fell through to `OK` with an empty reply | Claude seat report; new UNCERTAIN test | fixed CM-002 |
| `chat-channel.mjs:233` hard-coded `site:'chatgpt'` at thread creation | line, pre-fix | fixed CM-002 |
| `chat.mjs:363` hard-coded `chatgpt.send` | line, pre-fix | fixed CM-002 |
| `schemas/chat-receipt.v1.json:25` chatgpt-only scopes enum | line, pre-fix | fixed CM-002 |
| `chat-channel.mjs` `transport.send` received no site | Claude seat finding; site-on-send test | fixed CM-010 |
| Reserved sites reachable as scopes | ChatGPT seat failing assertion: `qwen.send` actual `OK` | fixed CM-010 |
| `conversation-thread.mjs:15-18` no gemini thread entry | grep: only chatgpt and claude kinds | unfixed; deferred pending URL shape |
| `actions.mjs` generic fallback latches a pre-hydration textarea; `where()` at `:1422` counts ChatGPT-only selector | Claude seat report; not re-run by this session | unfixed; authorized CM-004 |
| `gemini.mjs` stale-handle mapping returns PROMPT_MISMATCH | `gemini-channel.test.mjs:343` | unfixed; current failing test |
| `AGENT-BUS.md:83` wake result never back-filled after timeout | doc + `MW-20261104T005323Z-gemini.result.json` | unfixed |
| `chat-channel.test.mjs` written by more than one seat concurrently | ChatGPT seat reply: two rejected edits, file changed between reads | unfixed process defect |

## 8. WHAT THE NEXT SESSION SHOULD DO FIRST

Fix the wake back-fill: after a `--wait` expiry, append the finished turn's reply to `C:/VIVIM/channel/dispatch/<wake-id>.result.json` instead of leaving `reply:null`. Reason: every seat turn's evidence depends on it, and it already cost the Gemini 005323 reply while the seat had in fact executed.

## 9. WHAT ONLY THE OWNER CAN DO

- Claude account credits: reset at 20:00 local; no workaround.
- ChatGPT sign-in verification: only the owner can complete it.
- Wave 3 owner-present demo: owner presence, not a manager decision.
- Confirm the 9222 `studio drive chatgpt` job is intentionally gone. Evidence says it is: pid 24836 absent, lease directory absent. Nothing needs releasing unless the owner knows of a different process.
