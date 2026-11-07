# Master integration — context without another authority

**Status:** design for local implementation; runtime not claimed. **Baseline:** repo main inspected 2026-11-01.  
**Core invariant:** context is a read/selection/delivery plane; action authority stays with existing Host/Studio/agent tools.

## 1. Five planes and directional dependencies

1. **Truth plane:** Git HEAD/diff; roadmap.json tasks; dependency/activation graph; source symbol index; code/test receipts; existing Personal World/primary and Studio journals. Each item has an owner, revision and trust class.
2. **Observation/index plane:** proposed Context Lake ingests **metadata** of Git commits and source paths into append-only, locally reconstructible events; reports drift, stale docs and missing task attribution. It does not write task status, Personal World content or browser state.
3. **Atomic retrieval plane:** existing note units plus proposed verified chat segments with stable message/span refs. Immutable source evidence, current delivery/provenance state and per-principal ACL are checked *before* selection.
4. **Composition plane:** existing createDCBComposer and createDcbSourceAdapters. Build bounded, deterministic manifests by task, audience, budget, trust, freshness, security and provider disclosure; record omissions. Separate dev/task profiles from private/personal profiles; never use Git metadata as owner personal memory.
5. **Consumption plane:** agent CLI startup brief; Studio/Host Personal Agent; eventually desktop/paired-phone World and explicit context preview. Consumption grants **no permissions**. Approved actions enter canonical Host/Studio typed contracts and journals separately.

Flow: repo events + roadmap + test receipts → derived developer feed → DCB source adapter → agent-session brief.  
Flow: owner-authorized World records + source-verified message atoms → DCB source adapter → owner preview → already-approved provider/harness send.  
**Not**: context data → automatic browser action or source claims → confirmed memories.

## 2. Authority and data-boundary matrix

| Data | Owner | Can be auto-read? | Can be exported to external model? |
|---|---|---|---|
| Git commit metadata, public source identifiers | Git + local Context Lake projection | Yes, within repo and local workspace | Only under explicit destination/policy |
| Roadmap item status | docs/roadmap.json | Yes | Project-policy scoped |
| Test results/host process live state | Original runner/receipt | Only from current verified receipt | Avoid secrets; label provenance/time |
| Personal identity, preferences and hard NOs | Host/primary/owner consent | Per local principal and approved scope | Source-by-source, destination-specific owner approval |
| Chat body/span | Exact captured/imported private source | Only after source access + provenance validation | Explicit approval and preview; secret/hard NO always excluded |
| Provider sent/reply/delivery state | Studio journal + Host source verification | Yes, metadata-scoped | Not inferred from bridge completion |
| Capability registry | Existing atomic registry | Yes | Catalog is requirement space, not current runtime claim |

## 3. Unified machine contracts (candidate fields; adapt existing implementations)

- **ContextSource**: sourceRef, ownerAuthority, principalRef, projectRef, class, sensitivity, trust, evidenceRef, revision or sha256, range, observedAt, validUntil, originId, policy.
- **ContextRequest**: purpose, taskRef, profile, principalRef, audience/destination, allowed refs/prefixes, maxSensitivity, byte/token budget, exact source approvals, sourceHead, current prompt query.
- **ContextManifest**: schema version, request key, source selector/version, selected refs+ranges+revisions, trusted provenance, omissions with reason, source freshness, budget estimate, ordered render, manifest digest.
- **ContextFeed**: throughCommit (not current live PID!), Git-backed orphan/unrecorded/stale indicators, active phase reference; deterministic regeneration from pinned Git tree.
- **SessionBootstrapResult**: repo identity, branch/HEAD/dirty, active owner scope, suggested next task(s) with pointers, known blockers and acceptance, read-only probe receipts, stale/missing flags, bounded output.
- **ChatAtom**: see CHAT-ATOM-CONTRACT.md. No content digest can attest a provider send or human read.

Normalize terms at adapter seams. Do not silently mix the AION schema, VIVIM Final ten layers and current composer nine source layers. In current composer, L7UserQuery is effectively the caller query, not an extra stored source layer.

## 4. Incremental architecture — no duplicate systems

**Stage C0 — reconcile local session draft.** Inspect uncommitted docs/architecture/CONTEXT-LAKE-V1.md and PRD 13. Compare current main, preserve local agent changes, register the real roadmap IDs once. Correct stale START-HERE freeze. Do not rewrite broad AGENTS unless needed.

**Stage C1 — Git evidence feed.** Implement a read-only bounded brief over existing roadmap/source/capability catalogs and Git first. Then add append-only local event records, integrity/recovery, determinism, drift and a generated committed fallback feed. Local store must not save file bodies/diffs/secrets. Hook auto-staging must detect manually edited artifacts and never overwrite them silently. Avoid GitHub Actions.

**Stage C2 — DCB adapter.** Plug Context Lake snapshot into existing dcb-sources (new source adapter), with exact commit provenance, no second composer. Demonstrate reproducible bundle digest for same HEAD/request/source revisions, clear budget and explicit omission reasons. Cache keyed by source revisions and policy; invalidates on meaningful changes.

**Stage C3 — agent bootstrap.** Root AGENTS.md gives a tiny safe first-interaction command path; preflight returns bounded context/status and **next steps** on greeting. Optional OpenCode event/plugin may optimize speed but must not inject surprise prompts or execute arbitrary effects. Do not rely on nonstandard slash commands. Test actual fresh OpenCode session on Windows and alternate agent CLI separately.

**Stage C4 — chat atom retrieval.** Add versioned atom/span indexing over *authorized* local conversation captures without affecting send journal. Prove re-import/dedup/revision handling, partial span citations, content integrity and private ACL. Connect via DCB source adapter only after source evidence can be verified.

**Stage C5 — Personal World + cross-provider context.** Owner sees bundle preview/omissions, can suppress sources, and grants exact provider disclosure before a send; Host retains approvals/receipts/undo. Phone reflects state only via existing paired sessions. Hard NOs/secret fields stay denied.

**Stage C6 — continuous correctness.** Reconcile Git feed after checkout/amend, index generated files, invalidate DCB caches, and re-run deep cross-source drift tests. Measure quality/time/token savings against normal targeted repo reading. Retire any machinery without measurable benefit.

## 5. Non-negotiable tests

- Fresh greeting must produce concrete next actions grounded in roadmap + current HEAD (not a generic response). If no safe task is known, offer 2–3 specific choices instead of inventing one.
- Pure read paths perform zero browser/provider/phone actions; unclear state stays unknown.
- No automatic provider send; one Studio owner; no uncertain resend; no Playwright/Puppeteer, API provider sends or Vercel deployment.
- Same request, source revisions, policy and deterministic time anchor → same manifest/content digest; omissions are reproducible.
- Source changed mid-composition → reject or retry read-only with fresh revision; never present mixed-HEAD bundle as current.
- Invalid/stale/poisoned chat atom or prompt-injection content remains **untrusted text**, never instruction authority.
- No accidental private personal data, emails, credentials, prompts, replies or cookies in committed context feed or agent startup logs.
- Docs and indexes are a discovery layer, not runtime acceptance.

## 6. Precise implementation outputs

Use existing paths unless the local checkout proves alternatives. Likely interfaces: tools/context/lake.mjs; tools/context/lib/*; docs/context/context-feed.v1.json; harness/src/context/dcb-sources.mjs; harness/src/context/dcb-composer.mjs; a thin tools/context/session-brief.mjs or lake.mjs brief; code for chat atoms adjacent to existing local capture/World units; tests under tools/context/test and affected Studio/Host suites. Do not spawn a fresh agent orchestrator, independent scheduler, new database or permanent daemon as a prerequisite for the first "hi" test.
