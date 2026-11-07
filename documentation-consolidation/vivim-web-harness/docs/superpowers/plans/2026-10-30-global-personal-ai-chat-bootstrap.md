# Global Personal AI and Chat Bootstrap Implementation Plan

> **For agentic workers:** Implement task-by-task using the executing-plans workflow. The owner requested direct work without subagents; do not automatically dispatch implementation lanes. UI work remains owned by a designer, and any required independent code-file review must be arranged explicitly rather than silently skipped.

**Goal:** Establish the always-reachable global Personal AI, populate its persistent personal-context workspace during pairing from the user's connected AI, then use that foundation to prime ongoing work and preview, activate and undo supported Chat configuration proposals.

**Architecture:** Extend the existing `primary` Personal Agent and `VersionedStore`, not a second agent store or browser controller. The existing relay worker owns web effects, while Studio owns durable primary/configuration state and validates commands. Build and the protected recovery entrance reach the same global identity; ordinary Talk remains independently usable.

**Tech Stack:** Existing Node >=22 ESM modules, native PowerShell/NTFS support, plain browser JavaScript, existing Studio HTTP/tool registry, outbound Vercel relay and journaled web-app harness. No new framework or runtime dependency is planned.

**Spec:** `docs/superpowers/specs/2026-10-30-prepared-builder-chat-design.md`.

**Lean delivery priority:** See `docs/ip/2026-10-30-bootstrap-core-scaffolding.md`. The tasks below are seams/checks for a thin vertical mobile loop, not separate platform engines to finish before phone iteration. Reuse primary/memory/configuration/receipts; implement only the fields and Chat operations needed for one real suggestion → preview → activation → revision → undo cycle. Full historical memory/context/rights/Fabric infrastructure is not a prerequisite.

**Cross-repo adoption is now bounded:** the same scaffolding note maps the main `251286e` harvest to source-pinned owner/evidence decisions. W1 is a point-of-use import check, not a new exhaustive pre-build project. Add W2's small capability manifest to the existing host projection; use AION evidence semantics and VOMEGA typed-command boundaries without importing their runtimes. Keep existing `studio/` World/primary state and Web Harness as owners. PC HQ diagnostics are optional read-only facts, never a second host administrator. No further broad donor research precedes this mobile slice.

## Global Constraints

- AI providers are used only through authorized web sessions and the existing harness; no model-provider APIs.
- Reuse the existing single Studio/Chrome effect owner. Do not start a second browser actor, server, or poller.
- Installation authorization covers bounded background priming in this dev flow; no extra priming approval prompt. The model cannot authorize its own effects.
- Preserve exact-thread binding, draft protection, provider attribution, and no automatic resend after uncertain effects.
- The global Personal AI is protected from app configuration, layout, optional-feature removal and rollback. Missing providers or an offline host still limit responses honestly.
- Build remains separate from Talk. Existing pairing, conversations and chat are not reset or gated on bootstrap completion.
- First-version customization uses supported configuration, not arbitrary generated code. Chat is first; custom agents, modes and full time travel follow later.
- Secrets, cookies, pairing material, lockfiles and private runtime records never enter commits or diagnostic receipts.
- Current owner instruction: no new unit-test suites or suite sweeps. Use bounded disposable runtime checks, then real-phone acceptance. Existing red-suite evidence is not a live provider failure or a green release claim.
- Windows and Linux are required. Use existing path/default helpers and file-URL imports; preserve Linux defaults.
- Proposed execution constants: readiness TTL 30 seconds; priming deadline 120 seconds; one automatic priming send per generation; brief maximum 12,000 characters; 2–3 initial proposals. These are implementation defaults, not owner-specified preferences.

## Review Focus

1. A second phone or process restarts during priming: reuse the same identity/generation/request ID; never issue a second send. Task 5 owns the crash/reconnect check.
2. A provider becomes signed out after an earlier ready sample: expire readiness and block effects without switching providers. Tasks 2 and 5 own that check.
3. The user hides all Chat controls, resets the app or supplies corrupt configuration: the protected global and recovery entrance remain reachable. Tasks 1, 3 and 6 own those checks.
4. A model emits an unknown capability, JavaScript, wrong base version or a claim of approval: reject it without activation. Tasks 3 and 4 own those checks.
5. A provider thread disappears after a possibly delivered send: keep local primary/memory/Work, mark the binding unavailable or uncertain, and reconcile read-only. Task 5 owns that check.

## Code map and dependency gate

- `studio/src/agents/personal-agent-service.mjs:49–111,315–358`: existing `primary` identity, CAS writes, inventory, memory, continuity and Work. Extend this authority; do not duplicate it.
- `studio/src/agents/personal-agent-bootstrap.mjs`: existing evidence-backed bootstrap stages. Keep their completion rules separate from new provider-preparation state.
- `studio/src/store/versioned.mjs`: immutable versions, cross-process CAS and fail-closed locks. Use it through existing services; do not invent a second locking scheme.
- `studio/src/studio/personal-agent-routes.mjs`: existing same-owner endpoints and tool registration; preserve read-only status semantics.
- `studio/src/studio/personal-app-{blueprint,service,routes}.mjs`: inert blueprint DSL, protected base screens, preview and CAS activation. Extend for Chat rather than inventing another app configuration store.
- `studio/src/studio/server.mjs:265–268`: existing independent `/personal-agent` document route. Use it as the host recovery foundation.
- `harness/src/interface/router.mjs:16–151` on PR #36: injectable router, exact-thread effects, journal, approved sends, model cache. `status()` is a descriptor, not signed-in proof.
- `harness/src/providers/*.mjs`: existing auth-wall and positive signed-in selectors; unknown must remain unknown when positive evidence is missing.
- `studio/bin/commands/relay.mjs` on PR #36: model manifest currently asserts `signedIn:true`; replace that unsupported claim for bootstrap readiness.
- `studio/bin/relay-chat-mvp.mjs` on PR #36: current worker, per-provider serialization and thread bindings. Put priming on that same effect-owner queue.
- `studio/src/studio/relay-agent.mjs`: existing outbound status/command transport; keep transport distinct from effect authorization.
- `apps/mobile-relay/lib/relay-core.mjs`: authenticated phone/host separation; currently accepts only `mode:relay-chat`. Extend with validated bootstrap operations, not arbitrary tool names.
- PR #35 (`b577714`): additive Build tab in `apps/mobile-relay/{app.js,index.html}`. Its cards/history are device-local and its prompt moves to Talk; they are not host-backed bootstrap.
- PR #36 (`e7234d4`): provider fixes, mobile-relay dependency and Windows compatibility. Both PRs were OPEN when mapped; neither is implicitly approved for merge by this plan.

### Task 0: Assemble an isolated implementation baseline and acceptance receipts

**Files:** existing source from PR #36; the two Build files from PR #35; the approved spec; `docs/roadmap.json`, `docs/ROADMAP.md`; canonical live/investigation ledgers.

**Interfaces:** produces an isolated branch containing compatible dependency snapshots and the approved design. No running worktree changes during assembly.

- [ ] Inspect refs/status and preserve all local changes. Create a separate implementation worktree; do not switch or rewrite the running `vwh-relay` or `vwh-fixes` trees.
- [ ] Review and integrate pinned dependency snapshots locally, preserving both compatibility fixes and Build scaffold. Do not merge upstream PRs or deploy implicitly. Account for any later remote changes before proceeding.
- [ ] Amend B01's blanket no-automatic-send wording to record installation-authorized priming explicitly. Reference the protected global requirement in B01/H03/H04/H08; regenerate Markdown through the existing roadmap process, not an independent task list.
- [ ] Read local identity with `node studio/bin/studio.mjs personal-agent status --json`. Separately discover the existing owner's registered status and `conversation_targets` operations before invoking them; local Personal Agent status alone is not browser-owner proof. If those operations are absent, report the missing seam rather than substituting raw CDP or another controller. Leave the owner untouched during offline work. If live chat is not proven, record it as NOT_TESTED, not as a satisfied V1 entry gate.
- [ ] Record exact refs and run only syntax/module-loading checks with no provider effects. Commit the dependency reconciliation separately from new behavior; do not claim feature completion.

### Task 1: Protect the existing global primary and its local recovery contract

**Files:** modify `studio/src/agents/personal-agent-service.mjs`, `studio/src/studio/personal-agent-routes.mjs`; create pure `studio/src/agents/personal-ai-contracts.mjs`.

**Interfaces:** keep `agentId:'primary'` and the existing local-directory scope. Add `agent.globalStatus() -> GlobalStatus`, `agent.builderState() -> BuilderState`, and `agent.updateBuilder({expectedVersion, generation, patch, idempotencyKey}) -> BuilderState`. `GlobalStatus` contains identity/scope, `protected:true`, preparation phase, preferred target, capability-summary references and recovery availability—never private memory or credentials.

- [ ] Define schema `vivim.personal-ai/1` and phases `unprepared|preparing|ready|blocked|failed|uncertain`; add builder metadata additively to the existing primary record. Retain current stage evidence, memory and Work untouched.
- [ ] Add same-owner `GET /personal-agent/global` and `GET /personal-agent/builder` routes under the existing `/api/v1` integration prefix. Reads never initialize state or contact a provider. Initialization uses existing `initialize({expectedVersion:null})`, treating an existing same-scope primary as a read-only replay.
- [ ] Use existing CAS for mutations. Disallow patches to identity, protection, raw memory, credentials or completed bootstrap-stage evidence; do not expose generic write-patch endpoints to the model/phone.
- [ ] With a disposable data root, initialize twice, preserve existing memory/Work, simulate two stale writers and restore an old personal-app version. Verify one primary, one winning CAS update, unchanged memory/Work and `protected:true` throughout. Check Windows and Linux path projections without using live data.
- [ ] Review the complete new contract file and its consumers under the repository's required review policy before marking the task complete. Commit only its named source/documentation paths.

### Task 2: Produce conservative provider readiness through the existing router

**Files:** create `harness/src/interface/readiness.mjs`; modify `harness/src/interface/router.mjs`, relevant provider descriptors, `studio/bin/commands/relay.mjs`, and `studio/src/studio/relay-agent.mjs`.

**Interfaces:** add `router.readiness({provider=null, now=Date.now()}) -> Promise<ReadinessManifest>`. Schema `vivim.provider-readiness/1`: `observedAt`, `expiresAt`, and provider entries `{id,label,state,reason,modelCapabilities}`; state is `ready|signed_out|unavailable|unknown`. Model cache discovery and freshness remain distinguishable from authentication evidence.

The adjacent bootstrap capability projection carries `capabilityId`, `source`, `readiness`, `inputs`, `effects`, `approvalScope`, `reversible`, `evidence`, `version`, plus distinct maturity/enabled fields. Reuse one registry and only advertise real registered first-slice handlers; missing or source-only capabilities remain unavailable. Read-only host diagnostics carry source/observation/expiry, not unqualified green health.

- [ ] Inspect existing owned targets through the router's verified connection and existing driver. A readiness probe may read visible authentication/composer indicators; it may not type, click dismissals/model controls, send, scrape conversation text, or open a second browser/controller. No matching tab or unreadable/ambiguous DOM means unknown.
- [ ] Require positive sign-in evidence, no visible authentication wall and a usable composer before ready. Start with a provider whose positive evidence can be verified live; other descriptors remain unknown until supported. A signed-out marker overrides cached ready data. Expire observations after 30 seconds.
- [ ] Publish the versioned manifest in the existing host status snapshot, within the current 32 KB limit. Keep legacy Talk transport working; do not treat its fallback provider chips or cached `signedIn:true` as Build authorization.
- [ ] Exercise injected page snapshots: positive sign-in, guest composer, visible auth wall, hidden markers, inaccessible target, stale observation and target/profile mismatch. Verify zero side-effect calls and no conversation text in the output. Run one read-only native probe using the existing owner and append a metadata-only live receipt.
- [ ] Review the new readiness module and commit this independently usable truthful-readiness slice. No automatic priming is enabled yet.

### Task 3: Extend the existing personal-app model for configurable Chat

**Files:** modify `studio/src/studio/personal-app-blueprint.mjs`, `personal-app-service.mjs`, `personal-app-routes.mjs`; designer owns the supported Chat presentation/control catalog and its browser renderer.

**Interfaces:** preserve existing preview/apply contracts and CAS versions. Add capability ID `auto-chat`, screen kind `chat`, and proposal operation `configure-chat` with separate `presentation`, `placement`, and `controls` fields. Export a versioned `CHAT_CONFIG_CATALOG` plus `validateChatConfig(value)`; the same catalog drives AI proposals and the fixed renderer. Add `app.applyChatChange({id,expectedVersion,proposal,approval,idempotencyKey})` and `app.restoreChatConfiguration({id,expectedVersion,targetVersion,approval,idempotencyKey})`, returning `{app,replayed,receipt}`; restore changes Chat configuration only, not unrelated state. No HTML, CSS strings, script, URLs, arbitrary tool names or filesystem paths are accepted configuration.

- [ ] Have the designer define the smallest genuinely implemented presentation, structure and control-selection vocabulary. This is a dependency for proposal generation and UI work: do not silently substitute main-thread visual design or claim unsupported configuration knobs work. Preserve the existing product identity rather than redesigning Talk.
- [ ] Extend the current blueprint validator/preview compiler, not a second app store. Validate supported catalog keys and values; reject any operation that removes/replaces the protected global, its recovery entrance, or the existing required base screens.
- [ ] Apply a user-selected proposal through existing explicit activation semantics and CAS. Replaying the same activation returns its receipt, not another mutation. Restoring a supported prior Chat configuration appends a new version; it must not roll back Personal AI identity/memory/Work or external sends.
- [ ] With a disposable app and hand-authored supported proposals, preview without mutation, activate once, replay once, reopen and restore. Verify stable conversation IDs, no implicit provider/model changes, stale-version rejection, and continued recovery access after hiding all optional controls or loading malformed custom configuration. This task does not depend on an AI-generated proposal.
- [ ] Preserve generic blueprint operations and existing app records through additive normalization/migration. Commit the Chat configuration model only after complete-file review where required.

### Task 4: Define bounded briefs, global context inventory and Chat proposal validation

**Files:** extend `studio/src/agents/personal-ai-contracts.mjs`; create `studio/src/agents/personal-ai-brief.mjs`; reuse `personal-agent-{service,self-knowledge,continuity}.mjs` and `studio/src/studio/personal-app-blueprint.mjs`.

**Interfaces:** `buildPersonalAiBrief({globalStatus,readiness,catalog,configuration,preferences}) -> {text,digest,catalogVersion}`; `parsePersonalAiProposals(value,{catalog,appId,baseVersion}) -> Proposal[]`; `compilePersonalAiBaseline(value,{agentId,scopeFingerprint,requestId,observedAt}) -> BaselinePatch`; `agent.ingestBaseline({expectedVersion,idempotencyKey,patch}) -> {version,coverage,replayed}`. The initial response envelope carries structured `baseline` and optional `proposals`. Baseline sections cover owner profile, preferences, communication/working style, roles, projects, goals, routines, tools/environment and constraints, each with explicit unknowns and host-attached provenance. Preserve primary identity/role separately from the owner's personal profile. Schema `vivim.personal-ai-proposals/1` carries 2–3 bounded proposals. Do not promote provider-reported statements into owner-confirmed memory automatically.

- [ ] Inventory all existing registered capabilities with explicit availability/authorization qualifications; distinguish discoverable tools from executable Chat/bootstrap operations. Do not create a second tool registry or claim that every registered tool has been exercised.
- [ ] Construct the brief locally, maximum 12,000 characters. Include supported operations, current app state and explicit relevant preferences; omit credentials, raw session archives and unrelated memory. If the bounded brief cannot be assembled, block preparation rather than silently truncate required instructions.
- [ ] Request broad structured personal-context intake from what the selected AI actually knows, not just Chat setup preferences. Explain that provider-managed personalization is unknown, missing facts must remain unknown, and the model does not approve its own proposals. Validate field lengths/types and attach source/request/time/basis on the host; discard fabricated authority, executable fields and credentials rather than accepting a free-form dossier.
- [ ] Implement `ingestBaseline` on the existing primary/CAS authority with replay-safe merges into owner/profile, preference and active-context sections. Keep confirmed context, record conflicting provider assertions separately and compute coverage from accepted facts/explicit unknowns. Human-readable master-workspace/identity-folder files, if exposed, are read-only generated views of the same versions—not another editable authority or a new master identity.
- [ ] Persist the validated context foundation before deriving or accepting personalized Chat proposals against it. A combined first response is allowed, but one-message convenience must not replace broad intake with a small configuration questionnaire. Partial/unknown intake still permits honestly labeled generic proposals and must not keep the user waiting for a complete profile.
- [ ] Parse only the specified JSON schema, optionally inside one complete JSON fence. Reject extra fields, malformed/oversized output, duplicate proposal IDs, unsupported capability IDs, executable content, wrong app/version, more than three proposals and fabricated execution/approval fields. Several alternatives may configure the same supported Chat capability; they are not installed together automatically. No automatic model-repair prompt.
- [ ] Exercise valid and malicious in-memory samples without provider calls. Verify no activation callback is invoked, credential sentinels are absent from the brief, and unsupported capabilities produce a useful limitation rather than a fake active feature. Round-trip broad structured intake across service recreation, retain confirmed facts under conflicting/replayed intake, and ensure the next context packet uses the persisted workspace rather than only provider conversation memory.
- [ ] Commit the pure brief/compiler slice after required complete-file review. Concrete visual configuration values come from the designer-owned catalog in Task 3, not guesses in this task.

### Task 5: Prime one fresh provider binding, with restart-safe recovery

**Files:** create `studio/src/agents/personal-ai-preparation.mjs`; modify `studio/bin/relay-chat-mvp.mjs`, `studio/src/studio/personal-agent-routes.mjs`, `studio/src/studio/bridge-commands.mjs`, `harness/src/interface/router.mjs` as narrowly required; add CLI worker wiring alongside the existing relay handler.

**Interfaces:** `createPersonalAiPreparation({agent,router,authority,clock}) -> {prepare(command),reconcile(command),continue(command)}`. Commands include authenticated owner scope, `generation`, `expectedVersion`, and `idempotencyKey`. Preferred target is `{provider,model:null|string,effort:null|string}`. No provider is selected from an LLM-supplied authority field.

- [ ] Persist selected/preferred target and the installation-authorization basis on the PC. Trigger baseline collection and preparation as soon as pairing, fresh provider readiness and target selection are all established, before Build opens. Pairing alone does not send; multiple unselected providers require a builder choice rather than fan-out. Unavailable preferred providers wait/block without silent fallback. Permit explicit target changes only through the authenticated owner command path.
- [ ] CAS-persist the preparation generation, brief digest and deterministic request ID before dispatch. Record an effect intent and its deadline before the existing authority issues a request-bound approval. The model/relay payload cannot approve itself; do not invoke the owner's interactive `wh approve` command.
- [ ] Send one fresh-thread request through the existing router and shared per-provider worker queue, which must serialize both Talk and priming. Use a 120-second overall deadline and at most one automatic send per generation. No new daemon/browser owner and no replacement of legacy Talk command semantics.
- [ ] Compile and persist the baseline into the primary workspace, then persist validated proposals and the exact provider binding before ready. Never forward the raw onboarding response through the legacy relay result path or diagnostic logs; return a bounded coverage/proposal projection. Add router read-only result/binding access as necessary; never infer a thread URL from display text or bind to an arbitrary tab. Current in-memory router replies are not restart durability: after a crash use the original journal ID, exact target and stored reply boundary to reconcile read-only.
- [ ] If a receipt exists or dispatch might have occurred, do not recreate the job or resend. Unknown submission is uncertain; no exact thread is blocked/uncertain, not a new conversation. Only a provably absent/pre-effect request may proceed using its original ID. A complete receipt can be read/reconciled without another send.
- [ ] Continue the user's first Build message in that prepared thread with a new request ID. If the thread has disappeared, retain primary/memory/Work, surface the binding loss and offer explicit re-preparation; do not move the message automatically to a fresh conversation.
- [ ] In a disposable runtime with fake router/authority, interrupt before dispatch, after intent, after send and before proposal-save. Recreate the service and verify one request ID, at most one send, preserved state, stale-generation rejection, no repeated initialization and read-only uncertain recovery. Exercise competing phone commands, repeated pairing/status events, a never-opened Build screen, and a sign-out between readiness and dispatch. Verify baseline collection starts before Build, unselected providers are not contacted, and provider-reported baseline never overwrites confirmed preferences.
- [ ] Review the complete new effect-owner module before a single live authorized priming attempt. Record that attempt immediately in the canonical live/investigation ledgers; no resend loop after a failed or uncertain attempt. Commit the verified scope and its actual evidence classification.

### Task 6: Connect Build and a protected recovery entrance to the same global

**Files:** modify `apps/mobile-relay/{app.js,index.html,vercel.json}`, `apps/mobile-relay/lib/relay-core.mjs`, `studio/src/studio/{relay-agent,bridge-commands,server,personal-agent-routes}.mjs`; designer owns UI work and isolated recovery assets at `apps/mobile-relay/{personal-ai.html,personal-ai.js}`. Reuse the existing Studio `/personal-agent` recovery foundation rather than creating a second server.

**Interfaces:** extend the relay with strict schema `vivim.personal-ai-command/1` operations `select-builder|prepare|continue|preview|activate|undo|reconcile`. Carry request identity/version fields through the existing command transport. Publish a bounded `personalAi` projection with schema `vivim.personal-ai-phone/1`, `agentId:'primary'`, scope fingerprint, protected flag, phase, selected target, readiness expiry, context-workspace version and coverage/unknown-section counts, configuration version, at most three validated proposals and recovery path `/personal-ai`. Keep the complete status within 32 KB; the full personal baseline, detailed briefs, memory, secrets, native provider URLs and raw runtime records stay on the PC.

- [ ] Register explicit host dispatch handlers for those operations; do not accept arbitrary method/tool execution through the relay. Existing `relay-chat` commands, pairing cookies and host-secret separation remain intact. The host, not the relay or model, resolves owner scope and authorization.
- [ ] Wire Build to real manifest/preparation/proposal state. Retire local-only activation as an authority: an existing `vivim.bootstrap.v1` cache is an optional local draft, never proof of host activation. Retain legacy Talk conversations and localStorage keys without reset.
- [ ] Make the global Personal AI reachable from the base harness shell, outside customizable navigation. Add the dedicated phone recovery route and existing host recovery URL, independent of the custom layout/parser/renderer and feature toggles. Both reach the same primary and same command/effect owner.
- [ ] The recovery surface may load minimal authenticated status, explain offline/provider-unavailable states, continue the same global conversation when available, and request an explicit restore of last-known-good configuration. It must not silently wipe corrupt data, recover by launching a new browser, or require the broken custom app to load.
- [ ] Designer verifies compact readable availability, preferred-AI selection, preparation progress and 2–3 proposals, plus supported Chat customization, focus/keyboard behavior and Android portrait. These are implementation deliverables, not visuals preselected by this plan.
- [ ] Perform bounded local runtime checks with fake relay/store for duplicate commands, unauthorized callers, >32 KB status, stale responses and lost activation acknowledgements. Verify replay-safe results and no memory/secret sentinel in status. Then commit transport/UI changes with their actual evidence limits.

### Task 7: Prove the real-phone path and preserve the global through failures

**Files:** canonical append-only live/investigation ledgers; `docs/roadmap.json`, generated `docs/ROADMAP.md`; a short task-specific evidence report with IDs and outcome only.

**Interfaces:** consumes the same paired phone, single effect owner and provider selected in Task 5. Produces an honest V1 slice receipt, not a fabricated full-roadmap pass.

- [ ] Inspect current owner status and conversation targets again; record exact code revisions and verify one effect owner. Coordinate handover before restarting a worker; do not start a second relay/controller. Never treat a historical port/profile as identity proof.
- [ ] On the phone, verify availability, select the preferred ready AI and observe bounded automatic priming. Confirm suggestions came from that provider and that the user's first message continues the same prepared native conversation.
- [ ] Preview and activate one supported Chat presentation/structure/control change; reopen the app and undo the local configuration change. Confirm existing Talk still works and conversation history/primary memory/Work are unchanged.
- [ ] Hide optional navigation, load a deliberately invalid disposable configuration, and enter through the protected recovery URL. Verify same global identity and a restore preview without data deletion. Simulate an unavailable provider/host and verify honest unavailable state, not a fake reply or silently selected provider.
- [ ] Kill/restart only the coordinated worker during a controlled preparation attempt, preserving original keys. Reconcile read-only; verify no duplicate priming or resend. If the outcome is uncertain, stop live effects and record uncertainty instead of manufacturing a passing retry.
- [ ] Append each attempt immediately with timestamp, exact revision, objective, authorization, request IDs, outcome and limitations; no secrets, screenshots of pairing material or raw private conversation text. Mark roadmap items in progress/complete only to the extent demonstrated; keep full modes, custom-agent creation and time travel queued.
- [ ] Commit reviewed source/evidence only, push the feature branch and open a review PR when requested. Do not merge or deploy automatically. Report native Windows evidence separately from Linux verification; do not claim cross-OS parity from Windows fixtures.

## Execution and self-review

- Host identity/global authority is reused from `primary`; optional app changes cannot create a substitute global.
- The existing versioned store is the sole CAS owner; the relay remains transport and the existing worker remains the sole web-effect owner.
- AION/VOMEGA/PC HQ remain contract references or deferred integrations; no runtime imports, extra controllers, registries, stores or remediators are created. The original harvest and historical IP registry remain unchanged opportunity/source indexes, not alternative roadmaps.
- Tasks 1–2 can deliver protected host identity and readiness independently. Task 3 supplies the catalog consumed by Task 4; Task 5 uses those contracts; Task 6 connects the phone. Task 3's catalog and Task 6's interface work require designer ownership; the current direct/no-subagents preference must not cause that specialist requirement to be silently bypassed.
- New risky code-file review is a gate before live operation. The most recently supplied AGENTS.md instructions take precedence over older routing text in GOAL.md. Verify the named model IDs against the live catalog before any authorized review dispatch; do not guess IDs or skip review. No reviewer or implementation agent was dispatched to write this plan.
- All five Review Focus conditions have explicit checks above. Evidence uses bounded local runtime checks and a coordinated phone walkthrough, not unit-suite sweeps, in accordance with the latest supplied instructions.
- No production behavior, provider send, activation, dependency merge, or deployment is performed by this planning change.
