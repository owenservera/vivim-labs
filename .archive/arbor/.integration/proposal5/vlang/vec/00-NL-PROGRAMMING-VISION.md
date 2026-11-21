# VEC product vision: the user programs VIVIM through natural language

## Central correction
VEC is not mainly an autonomous software engineer rewriting the application's repository. Its most valuable role is **ordinary-user programmability**. A natural-language request should result in a durable personal behavior or UI change as readily as a request for information. Default to the simplest local deterministic implementation when current SDK capability contracts can express the request; use LLMs only for requests needing semantic reasoning or genuinely missing capabilities. Do not promise zero-LLM comprehension of unrestricted language.

## Three execution paths
**A: Recognized intent, zero LLM.** Versioned phrase/grammar rule + registered synonyms/aliases + typed entity/slot validation -> one capability invocation or a user program template. Detect ambiguities and conflicts; ask a concise deterministic question.
**B: Composed capability graph, zero LLM where grammar/IR supports it.** Trigger + condition + query/transform + action + approval/receipt edges generated from known capability manifests and compatible types, not guessed chains.
**C: Complex or new capability, opt-in LLM assistance.** The model proposes a typed plan, authoring grammar pattern or capsule SDK addition that must pass the same registry, owner approval, tests and rollout gates. Do not use model prose as an executable authority. Later identical or parameterized requests can follow A/B without model calls.

## Concrete user experiences
- 'Every morning show my unfinished tasks': create personal scheduled trigger -> Host-authorized task query -> TierFlow view.
- 'When local storage is below 5 GB suggest what I could clean': threshold event -> filesystem metadata read within grant -> deterministic classification -> non-destructive recommendation UI; actual deletions are separate approvals.
- 'Put frequent tools at the bottom': versioned user layout preference -> approved TierFlow projection update; never write publisher source by default.
- 'Make recent conversations accessible on my phone': only available if approved local source + paired-device sync capabilities exist; request missing grants, do not invent a connector or unsupported capability.
- 'Whenever I download a PDF categorize it': local folder event -> allowed parser/classifier -> designated file action (requires suitable approved classification implementation and filesystem grant).
- 'Undo my previous change': revert a specific user program or UI preference revision via existing Host journal and recovery; do not infer arbitrary Git/source rollback.

## What user gets
A preview in everyday language plus exact triggers, data sources, permissions, user-visible effects, resource cost and reversibility; Save/Enable/Disable/Edit/Undo; dry-run receipts; predictable running behavior and explanation 'why this happened.' User can share a redacted *recipe template* without accidentally sharing private source roots, grants, tokens or personal data.

## Success measures (future benchmarks)
Intent coverage for a declared grammar corpus; zero-LLM A/B hit rate measured against eligible supported requests (not all arbitrary language); clarification frequency; correct first-program compilation; p50/p95 parsing+compile latency; execution determinism; task success; revoked-grant refusal; UI revision consistency; user undo success; local CPU/RSS/SQLite footprint; time to add a new SDK capability. Define target values only after measured baseline.

## Explicit non-goals
Unlimited NL with perfect deterministic semantic understanding; automatic execution of instructions in user data, webpages or comments; arbitrary code from text; separate personal-action authority; implicit network/provider API usage; hidden scheduled surveillance; permanent grant escalation; automatically writing product source for routine user preferences.
