# Dynamic context feedback loop, revision-aware LRU and safe delivery
**Status:** proposed add-on to current HQ/Web Harness composers; not installed.  
**Principle:** a DCB is an immutable snapshot. A **context controller** continuously maintains the *next* candidates and queue using events and cache, while the active agent/model request stays stable until an explicit, allowed delivery boundary.

## 1. Existing seams (read before implementing)
- HQ: `hq/lib/context.mjs` has `composeContext(db,{task,project,budgetTokens,reservedForOutput,maxFiles,now})`, source spans, deterministic knapsack, digest and `verifyContext`; `hq/lib/filetypes.mjs` has `classifyPath`, `typesForTask`, `autoExpose` but its auto-exposure is **not currently called** in `composeContext`. The `hq context` CLI and OpenCode plugin already invoke composition.
- HQ docs `DCB-PROTOCOL.md` define triggers: session.start, task.route, edit.pre, edit.post, handoff, ask, mcp.context, schedule, manual. A trigger is not a permission grant.
- Web Harness `harness/src/context/dcb-composer.mjs` and `dcb-sources.mjs` have a separate owner/provider-scoped DCB composer including sensitivity checks and consent. Preserve it as the current consumer/privacy boundary. Any shared selection engine needs adapters, not a competing writer or a merged grant model.
- Rust Wave 1 supplies a real native DEV HQKG source/semantic/command/wiki graph. Real producer→consumer streaming and product-safe bounded queries are not proven by Wave 1.
- LAB#1 real-time dogfeed specifies session event capture; Sprint 04 working-memory package provides consented, ephemeral referents. Neither is a substitute for owner-verified runtime data.

## 2. Controller state and staged data model
`ContextSession`: {scope,agentId,taskId,phase,epoch,sourceHead,graphEpoch,policyVersion,principalRef,destination,privacyScope,pinnedInstructions,activeBundleDigest,queueRevision,status}.
`ContextEvent`: {eventId,kind,scope,sourceRef?,sourceRevision?,sourceDigest?,time?,payloadClass,access,causalParent,originReceipt}; payload must be minimal and redacted.
`ContextCandidate`: {sourceRef,span,hash,sourceAuthority,evidenceClass,relevance,dependencyDistance,freshness,urgency,ttlHint,estimatedTokens,privacy,queueClass,whyIncluded,lastVerifiedAt}.
`ContextQueue`: {revision,taskGoal,hardPinned,active,warm,evictable,omissions,changeReasons,readyForNextDelivery}.
`ContextDelivery`: {deliveryId,queueRevision,bundleDigest,exactTaskRef,targetThreadOrSession,approvedScope,effectiveWindow,deliveryState:prepared|offered|consumed|refused|stale}; a prepared bundle is NOT a sent request.

## 3. Event-to-context loop
```text
User editing / Agent writing / verified tool output / source commit / failed test
               |
               v
observe -> redact -> deduplicate -> pin scope and identity
               |
               v
classify: task change? source change? policy? tool/result? stable noise?
               |
               v
invalidate dependency-indexed cache keys and derived queue items
               |
               v
retrieve: candidate spans from HQ FTS + filetype registry + typed Rust neighborhood
               |
               v
evidence/ACL gate -> rank -> coverage-diverse budgeted selection -> queue diff
               |
               v
materialize immutable DCB snapshot + omissions + digest (on safe request boundary)
               |
               v
consumer: agent session, preview, or approved Host provider send
               |
               v
outcome receipt -> retrieval miss/correction/tool failure signal -> feedback
```

Use a serializable per-scope reducer, idempotent event IDs and monotonic `queueRevision`. Coalesce bursts (e.g., typing/file writes) by debounce; important revocation/source-invalidity events bypass debounce. An event from a user typing can speculatively update **local** candidate search, but unsent text must never leak to a remote model or be treated as a commitment. No unbounded daemon or autonomous agent required to prove the algorithm.

## 4. Queue policy
- **Pinned:** owner-approved task objective and currently applicable highest-priority instructions, provenance and denied scopes. These are always verified independently and are **not** part of disposable LRU.
- **Active:** exact file lines, symbols, decisions, tests and results necessary for the next step.
- **Warm:** graph-neighbor/context hints likely needed next; prefetch only within allowed local scope.
- **Evictable:** stale, redundant, low-value or superseded projections; can be reread if still authorized.
- **Omissions:** missing/denied/oversize/superseded/uncertain source refs, never silently hidden.

Prioritize evidence *marginal utility* rather than just recency. Prefer a new failed test over repeatedly included architecture prose once the task enters verification. Preserve relevant invariants when the user changes priorities. Explicit task change can reset many warm entries but must not delete the original task journal.

## 5. Byte-bounded two-tier LRU
Initial **experimental** RAM budget: 64 MiB per local coordinator (configurable, not a measured optimal size). Two logical partitions managed under one global byte ceiling:
- **A: source fragment and graph-query cache** keyed by canonical source reference, exact range/hash, graph epoch, project/ACL context and parser/normalization version. Reusable across tasks *only within identical authorized principal/scope*.
- **B: ranked candidates and composed projection cache** keyed by normalized task hash, session phase/trigger, profile, audience+destination, permissions+consent fingerprint, policy version, corpus/type registry revision, full relevant source revision fingerprint, graph epoch, ranking configuration, budget/reserves and pinned deterministic time anchor. A permission change ALWAYS changes key eligibility.

Cache values: immutable payload, canonical digest, dependency refs, source hashes, approximate resident bytes, created/access counters, last verified epoch, eligibility and expiry hint. Hashes must be computed over canonical encoding; wall clock/access order must not change bundle digest. Entry limit and byte limit are both enforced. Keep an exact reverse-dependency index `sourceRef -> cache keys` to avoid scanning all cache entries on edit. Cache only safe redacted data; never credentials, raw private chats, cookies or owner grant tokens. A cache hit is never evidence of current permissions: recheck principal grant, revocation epoch and, when needed, source revision before delivery.

### Cache operations
```text
get(key,currentScope):
  item = lru.get(key); if absent => MISS
  if !policyAllows(item,currentScope) => evict(key), DENIED
  if invalidationEpoch(item.dependencies) changed => evict(key), STALE
  verify pinned digest / source revision as required
  move key to MRU; return immutable projection and source digest

put(key,value,dependencies):
  reject secrets; reject over-size item; canonicalize; calculate resident bytes
  replace existing entry atomically
  while usedBytes > maxBytes or items > maxItems:
      evict least-recently-used nonpinned entry
  update reverse dependency index

invalidate(change):
  map touched source refs/graph identity/policy versions to dependent keys
  evict these keys; emit queue invalidation event
  if policy revocation, clear all scoped prepared deliveries and warm candidates
```

TTL is advisory: after time expiry reverify, never force acceptance. Explicit source hash change, HEAD move, graph epoch or permissions revocation is authoritative. For noisy edits, staged speculative retrieval can be replaced; an already sent bundle must remain in the receipt log, labeled as historical, never changed under the same ID. Restart may begin with cold cache without losing task truth.

### LRU boundaries
LRU eviction is a *performance operation*; context removal is a *semantic selection operation*. Both have different reasons/receipts. A pinned context section may be dropped from the *model payload* only if it remains enforceable via the actual instruction/authority layer; do not conflate model-visible instructions with compiled policy. In simulation, compare warm-cache hit rate with actual evidence recall and user task correctness, not hit rate alone.

## 6. Feedback controller stability
Use phase-aware score updates and hysteresis to prevent rapid oscillation:
- At event `e`, recompute only affected sources and graph neighborhoods (unless major task/permission change).
- Keep an active source until another has enough marginal value to replace it, or it becomes invalid/denied/stale.
- Require new critical evidence (failed test, explicit user correction, grant revocation) to trigger immediate replacement.
- Apply stable tie-breaks and a change budget (% context slots per ordinary event).
- Record each queue transition with before/after digests, reasons, displaced refs, and cost.

No "always rewriting" of the current LLM context. An in-flight model response is immutable; the next model call or explicit agent context-read uses the latest verified snapshot. If an agent ignores newly offered evidence, log that separately from retrieval failure.

## 7. Security/freshness refusals to test
Policy revoked during warm prefetch; source modified while composing; same path but different HEAD; symlink escape; untrusted chat text containing an imperative command; donor `AGENTS.md` labeled owner policy; crossing agent/private-user scopes; two sessions racing on shared files; budget smaller than mandatory safety evidence; LRU stale hit; invalidation storm; output/response reserve exhausted.

## 8. Placement and minimum patch
Use additive read-only adapter in Repos HQ around `composeContext` and `autoExpose` plus Rust **scoped** query. Retain existing Web Harness consented DCB composer and Host delivery. Implement cache and controller as local dependent projections, not new canonical KB/Graph/World/Session stores. No change to Wave 1 acceptance or current permission model until separately reviewed. Expose `hq context explain` and `hq context simulate` only after tests verify current CLI routing, rather than inventing a second CLI.

## 9. Measurements and early gates
Proposed metrics: relevant-source recall/precision, missing critical source, stale-current claim rate (hard 0), unauthorized disclosure (hard 0), queue churn, context diff size, tokens per successful action, cache bytes/hit/rebuild/invalidations, p50/p95 event→prepared-context latency, user/agent interruption overhead and task correctness. Compare **no cache vs LRU**, and **static DCB vs dynamic DCB**, under identical scenario sequences and source revisions. A high LRU hit rate with poor evidence recall is failure.
