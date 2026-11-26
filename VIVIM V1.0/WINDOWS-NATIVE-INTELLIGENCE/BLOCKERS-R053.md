# R053 — Windows Native Intelligence blocker register

**Date:** 2026-11-26 · **Reviewed Labs main:** `c719350c4a15ed79e9fe5c7dc4dd6093b053529b`
**Baseline:** [R052 acceptance checkpoint](https://github.com/owenservera/vivim-labs/commit/1e2d3c7c162ee9632fe534a9f90e4409cdc5d7c1)

## 1. Acceptance state

Five bounded priorities (1, 2, 4, 7, 9) are accepted; four (3, 5, 6, 8) remain partial. Seven distinct open blockers are listed below. No production ARBOR/Product writes, deployments, grants, or platform repairs authorized by this register. Owners are proposed roles, not confirmed assignments. Original priority ordering remains intact.

| ID | Priority | Blocker | Severity | Status |
|---|---:|---|---|---|
| WNI-B03-A | 3 | Intermittent actual ACP session/new rejection | high | open |
| WNI-B03-B | 3 | Actual approved-model readback, provider prompt and cancellation | high | open |
| WNI-B05-A | 5 | Live Host/MCP registration and GPUI interaction parity | high | open |
| WNI-B06-A | 6 | Windows SystemIndex provider query unavailable | medium | open |
| WNI-B06-B | 6 | Shell Property System failure on exact source fixture | medium | open |
| WNI-B08-A | 8 | Full approved application semantic observation | high | open |
| WNI-B08-B | 8 | Existing Studio/Host action receipt and recovery binding | high | open |

## 2. Blocker dossiers

### WNI-B03-A — Intermittent actual ACP session/new rejection

**Proposed owner:** Windows native terminal and ACP adapter; coordinate with SDK L05-HARNESS. **Classification:** implementation / runtime compatibility.

**Observed evidence**
- [EVIDENCE-R052-ACP-RECOVERY.json](EVIDENCE-R052-ACP-RECOVERY.json) — source `791cc7f3929df7c543b9246ec055374ddd76f427` — receipt `ctl-704f7cd3cb994bcc`: 26/27 passed; session/new rejected in first actual agent test; actual reconnect passed.
- [EVIDENCE-R052-ACP-RECOVERY.json](EVIDENCE-R052-ACP-RECOVERY.json) — receipt `ctl-77eaddb0eda951f5`: Exact failed test passed in isolation, 1/1, 26 filtered.

**Unknown:** Agent rejection reason and reproducibility; no confirmed root cause.

**Dependencies:** Exact installed OpenCode executable/version and isolated workspace/config; Existing OwnedProcess/ACP transport.

**Remediation:**
- Capture bounded sanitized RPC error code and stage, agent exit and correlation; do not publish raw provider errors or paths.
- Reproduce sequentially across fresh owned sessions and distinguish server startup, config, workspace, and protocol failures.
- Correct proven cause in existing adapter; retain generation fencing, loss rejection and no auto-resend.

**Closure criteria:**
- [ ] Clean 27/27 exact-pin Windows suite in one run.
- [ ] Repeat actual initialize/new/load/reconnect across three independently started owned processes or sessions; record each result and no orphaned jobs.
- [ ] Negative cases continue refusing stale generation and unknown state.

### WNI-B03-B — Actual approved-model readback, provider prompt and cancellation

**Proposed owner:** SDK L05-HARNESS with ACP adapter owner and existing model registry. **Classification:** provider compatibility / authorization.

**Observed evidence**
- [EVIDENCE-R052-ACP-RECOVERY.json](EVIDENCE-R052-ACP-RECOVERY.json) — receipt `ctl-704f7cd3cb994bcc`: Fixture model/cancel/replay passed; actual model configuration not verified; no actual provider prompt.

**Unknown:** Exact actual model config shape and availability, approved provider credential/budget binding, actual cancellation behavior.

**Dependencies:** WNI-B03-A; Existing approved model/provider registry and explicit data/budget grant; Real ACP capability advertisement.

**Remediation:**
- Inspect actual advertised configuration shape and version using bounded metadata only; preserve unknown metadata as unavailable.
- Bind exact approved model and verify current value after selection; no implicit fallback to another model.
- Run a small non-private read-only prompt with no tool access in an isolated session only after grant and exact model readback; verify correlated stream/settlement and an independently authorized cancellation case.

**Closure criteria:**
- [ ] Record exact provider/model identity, readback, scope, bounded prompt and settlement receipt without secrets.
- [ ] Actual in-flight cancellation settles or is explicitly reconciled, never treated as success on notification alone.
- [ ] No unexpected model, tool effect, duplicate prompt, or unauthorized charge; missing prerequisites remain blocked.

### WNI-B05-A — Live Host/MCP registration and GPUI interaction parity

**Proposed owner:** Incumbent Host/MCP/GPUI owners; coordinate SDK L04-ASSURANCE and P06-MCP-GRAPH. **Classification:** integration / owner authorization.

**Observed evidence**
- [EVIDENCE-R042-CAPABILITY.json](EVIDENCE-R042-CAPABILITY.json) — source `659058219a73c3bffc57b8b07e85dedae1dbc05a` — receipt `ctl-36c65bbfd3f66185`: 33 candidate rows, 132 typed projections, 33 GPUI presentation calls, 132 AI candidate refusals, zero ID collisions; no live registration or renderer.

**Unknown:** Current canonical registry drift, activation-ready subset, actual interactive and receipt parity.

**Dependencies:** Current canonical capability registry and schemas; Existing Host dispatcher, grants, receipts and GPUI surface; Per-adapter native acceptance; WNI-B08-B for semantic effects; Explicit approval before production activation.

**Remediation:**
- Reconcile exact current canonical registry; classify each of 33 as supported, unavailable, pending or rejected without widening grants.
- In isolated staging, use existing dispatcher and registration path; do not introduce a parallel registry or renderer.
- Exercise actual MCP discovery/invocation and GPUI interaction against a reviewed supported subset; verify denial for disabled candidates and shared receipt correlation. Extend coverage per candidate before any live acceptance claim.

**Closure criteria:**
- [ ] 33/33 dispositions and schema/source bindings recorded; no ID collisions.
- [ ] For each activated candidate, live MCP and GPUI route to same Host contract with grant, denial, approval where required, effect receipt and recovery readback.
- [ ] Unready candidates stay disabled and correctly reported; owner reviews production adoption separately.

### WNI-B06-A — Windows SystemIndex provider query unavailable

**Proposed owner:** Windows file intelligence owner; workstation administrator only for explicitly approved repairs. **Classification:** workstation platform dependency.

**Observed evidence**
- [EVIDENCE-R044-HQ-JOIN.json](EVIDENCE-R044-HQ-JOIN.json) — source `b3d3e7f9896f1f369463019aa2a158ac9b066e3d` — receipt `ctl-ff60fc66f7fe7f4c`: execute-query returned HRESULT -2147221164 (0x80040154); zero accepted hits means unavailable, not empty index. HQ add/change/delete and stale refusal passed.

**Unknown:** Whether COM registration, process context/bitness, provider availability or another platform condition causes failure.

**Dependencies:** Windows Search service/provider in actual execution context; Existing scoped HQ fallback and file identity checks.

**Remediation:**
- Read-only diagnose service, provider availability/bitness and execution context; preserve stage-specific HRESULT.
- Test actual scoped query on known indexed owned fixture, not global search; do not repair COM registry or service without explicit authorization.
- Reuse native handle/digest and HQ fallback on unavailable provider; prepare native OLE DB adapter only against proven provider behavior.

**Closure criteria:**
- [ ] Positive scoped indexed hit for known fixture with correct identity and provenance, plus explicit empty-result and unavailable distinction.
- [ ] Verify edit/delete reconciliation and stale hit refusal; no arbitrary file access.
- [ ] If OS provider cannot be restored within scope, owner must explicitly approve a documented fallback-only scope change; not automatic closure.

### WNI-B06-B — Shell Property System failure on exact source fixture

**Proposed owner:** Windows file intelligence owner. **Classification:** optional platform compatibility.

**Observed evidence**
- [EVIDENCE-R044-HQ-JOIN.json](EVIDENCE-R044-HQ-JOIN.json) — receipt `ctl-ff60fc66f7fe7f4c`: SHGetPropertyStoreFromParsingName returned -2147024809 (0x80070057/E_INVALIDARG); DOS path normalization did not resolve earlier failure.
- [EVIDENCE-R043-PROPERTY.json](EVIDENCE-R043-PROPERTY.json): Separate owned .txt property readback passed.

**Unknown:** File-type/path/provider-specific reason; native handle evidence remains valid.

**Dependencies:** Owned source fixture and exact file identity; Existing Shell COM adapter.

**Remediation:**
- Compare approved file types, normalized paths and property-store flags using stage-specific errors.
- Keep optional property failure separate from native handle/digest and HQ result.

**Closure criteria:**
- [ ] Successful native property readback on the exact scoped source fixture, or explicitly documented supported/unsupported property matrix reviewed by owner.
- [ ] No fabricated property values or silent identity substitution.

### WNI-B08-A — Full approved application semantic observation

**Proposed owner:** Windows native app semantics and incumbent Studio/Harness owners; coordinate SDK L03-SEMANTICS. **Classification:** implementation / coverage.

**Observed evidence**
- [EVIDENCE-R051-HCU-INTEGRITY.json](EVIDENCE-R051-HCU-INTEGRITY.json) — source `707d6fb5410148581f94599e28debc7eb2afe156` — receipt `ctl-7fae2c0310ddea8b`: 13/13 passed; two selected HWND controls, one delta, ACK, forged/stale/destroyed refusals.
- [EVIDENCE-R046-HCU.json](EVIDENCE-R046-HCU.json): Hidden root enumeration returned zero children; explicit selected group passed.

**Unknown:** Coverage and provider behavior beyond explicit selected HWND group, including non-HWND descendants and provider timeouts.

**Dependencies:** Existing UIA probe, semantic Registry/Delivery and authorized window scope; Supervised helper deadlines; generation/identity fencing.

**Remediation:**
- Implement bounded scoped tree observation using existing UIA and semantic primitives; handle virtualization, partial/foreign trees, password controls and timeouts without leaking values.
- Use owned application fixture with nested and dynamic controls; validate semantic identity across changes and provider restart.
- Project only complete observations as complete; no absence inference from partial enumeration.

**Closure criteria:**
- [ ] Actual bounded full-tree observation within declared fixture scope, including non-HWND descendants where provider supports them; coverage limitations explicit.
- [ ] Create/change/remove controls produce correct revisions/deltas and acknowledgment; timeout/partial/foreign/sensitive cases fail closed.
- [ ] No unapproved window access or private text projection.

### WNI-B08-B — Existing Studio/Host action receipt and recovery binding

**Proposed owner:** Incumbent Studio/Host owners; coordinate SDK L04-ASSURANCE and P05-HOST-RECEIPTS. **Classification:** integration / owner authorization.

**Observed evidence**
- [EVIDENCE-R051-HCU-INTEGRITY.json](EVIDENCE-R051-HCU-INTEGRITY.json) — receipt `ctl-7fae2c0310ddea8b`: Selected-control observation and state changes verified; no real Studio/Host action receipt or production binding.

**Unknown:** Actual canonical dispatcher binding, approval flow and postcondition correlation across stale/refreshed UI.

**Dependencies:** WNI-B08-A for full-app coverage; selected-group effects can be isolated earlier; Existing Host grant, action identity, effect receipt and recovery contracts; Reviewed safe owned control fixture.

**Remediation:**
- Resolve scoped reference against fresh observation immediately before dispatch; bind operation to existing Host grant/approval and idempotency contract.
- In isolated incumbent Studio/Host path, exercise harmless owned control action and independent postcondition readback.
- On uncertain outcome reconcile by original request ID; no fallback or retry without verified pre-dispatch refusal.

**Closure criteria:**
- [ ] Actual Host receipt correlates exact scope, revision, action, postcondition and resulting semantic delta/ACK.
- [ ] Stale/disabled/foreign/forged references and missing grants cannot cause effects.
- [ ] Restart/uncertain receipt reconciliation proves no duplicate effect; no production activation without owner review.

## 3. Residual scope limits (not counted as additional blockers for bounded acceptance)

- **Priority 1:** Global/continuous watcher coverage and production HQ/AutoWiki adoption; bounded R032 acceptance unchanged. [Evidence](EVIDENCE-R031-PRODUCT.json).
- **Priority 2:** Durable authenticated cross-campaign Host receipts, restart reconciliation and quota enforcement; bounded R039 acceptance unchanged. [Evidence](EVIDENCE-R039-SDK-DOMAIN.json).
- **Priority 4:** Always-on supervised instrumentation, system-wide coverage and real OS buffer-loss stress not verified; R049 deliberate sequence gap is distinct. [Evidence](EVIDENCE-R049-ETW-LIVE.json).
- **Priority 7:** Real TUI agent, installed Host endpoint and wedged ClosePseudoConsole outer-helper deadline acceptance not verified; bounded terminal fixture passed. [Evidence](EVIDENCE-R045-CONPTY.json).
- **Priority 9:** DirectX ExplicitDevice HRESULT -2147023728, NPU unverified/not routed, trained-model/LLM performance and production registry/quotas not verified. CPU reference acceptance unchanged. [Evidence](EVIDENCE-R048-WINML-DEVICES.json).

## 4. Execution and closure

Suggested remediation order: B03-A → B03-B; B08-A → B08-B → B05-A activation; B06-A/B can proceed independently. B05-A catalogue reconciliation can run in parallel. This is dependency sequencing, not a replacement for the original priority list.

Every closure needs exact source/build pins, actual execution receipts, reproducible acceptance, explicit scope limits and owner review where scope or activation changes. A passing isolated rerun does not erase a prior flaky suite. Do not mark missing provider output as empty or unknown hardware as supported. Do not silently widen grants, retry uncertain effects or activate another authority.

Machine-readable companion: [BLOCKERS-R053.json](BLOCKERS-R053.json).
