# Platform-wide extension point map

Status: **candidate discovery register**. Rows list *plausible* extensibility surfaces, **not** verified public interfaces or present-day loaders. Every candidate needs concrete file/callsite evidence before implementation.

| ID | Candidate extension point | Existing authority / integration target | Possible plugin contribution | Non-negotiable boundary |
|---|---|---|---|---|
| EP-01 | Rust kernel computation engine nodes | Repos HQ Rust core; TierFlow Rust event sidecar where applicable | Algorithm/engine implementation via stable typed ABI or process bridge | Never replace scheduler/effect authority; resource quotas and sandboxing |
| EP-02 | Storage/index adapter | Repos HQ knowledge subsystem | Legal document index, metadata projections, search adapters | No second canonical KB; isolation, retention, provenance and deletion |
| EP-03 | Knowledge ingestion/parsers | Repos HQ ingestion; Harness capture ingress | Spanish legal-source parser, citation normalizer, document classifier | Untrusted content, source licenses, original preservation |
| EP-04 | Retrieval/ranking/context bundles | Repos HQ DCB/graph/context | Jurisdiction-aware legal ranking, context profiles and citation selection | Owner/world scope, token budget, provenance, redaction |
| EP-05 | Memory and Oracle intelligence | Repos HQ personal intelligence/Oracle contracts | Domain memories, specialized evaluators, domain agent capabilities | Oracle not superseded; consent and sensitive client-matter partitions |
| EP-06 | Capability discovery/registry | Repos HQ semantic/capability owners | New typed legal capabilities and resolver metadata | No second command registry or self-granted action |
| EP-07 | NL/typed command interpretation | HQ semantics, TierFlow typed IR, Harness NLCL | Spanish legal terms, intents, validation grammars | Interpretation only; policy and Host determine permissible effects |
| EP-08 | Agent skills and orchestration | Existing HQ bus/orchestrator/Oracle | Legal research/review/drafting skill recipes | Scoped actors, bounded delegation and approval |
| EP-09 | Workflow/automation recipes | TierFlow planning/execution and Host action broker | Review deadlines, document assembly, intake checklists | No direct external effect; idempotency, denial, no blind retry |
| EP-10 | Signed-in webapp/provider adapters | VIVIM Web Harness + Studio | Domain-specific browser affordances and capture | No provider APIs; existing Chrome owner and approvals |
| EP-11 | Document format/export | Existing artifact and capture paths; owner to verify | Spanish legal-document templates, PDF/DOCX export | No forged filing/submission/signature; user confirmation |
| EP-12 | Visual UI components/templates | TierFlow canonical visual runtime | Matter dashboard, citation card, deadline view | One UI document, one publisher, accessible and inert display |
| EP-13 | Desktop/mobile device projection | Existing Harness Windows/Android shell and TierFlow adapters | Adapt presentation, offline redaction and phone controls | One identity, one policy, no second mobile runtime |
| EP-14 | Security/compliance/policy checks | Host owner policy and scopes; HQ data governance | Confidentiality checks, privilege flags, locality requirements | Can restrict but cannot grant, relax or override Host decisions |
| EP-15 | Evaluation/test/bench suites | Labs test tooling, owning repo tests | Legal accuracy, citation integrity, prompt-injection and regression suites | Evaluation scores are not release authorization |
| EP-16 | Localization/accessibility/help | Existing canonical UI and device semantics | es-ES legal terminology, accessible help and guided onboarding | Stable semantic target across locales/devices |
| EP-17 | External data/source connectors | Existing approved data ingress / future adapters | Official legislation and case-law source readers | No invented licensing, credentials or bypass of provider terms |
| EP-18 | Lifecycle/packaging/discovery | Future approved extension manager in canonical runtime | Manifest, dependency resolution, pinning, uninstall/migration | Signed/trusted origin, policy-scoped activation, rollback |
| EP-19 | Reliability/self-healing diagnostics | TierFlow/Host diagnostics; existing recovery machinery | Domain failure classifiers, repair suggestions | Diagnose/propose only; never retry uncertain legal submissions |
| EP-20 | Telemetry/optimization | Existing logging/metrics authorities | Domain-specific latency/quality/resource metrics | Minimal data; redact client matters, no covert export |

## Discovery record required for EACH point

`id`, `status`, `owning-repo`, `source-path@commit`, `actual-caller`, `interface-version`, `input-output-schema`, `trust-boundary`, `permission-scope`, `persistence-owner`, `loading-and-unload-semantics`, `failure-mode`, `recovery`, `cross-device-contract`, `tests`, `evidence`, `recommended-action`.

Classify proposed implementation type separately: **declarative pack**, **adapter**, **process-isolated extension**, **Rust engine node**, **UI template** or **test-only plugin**. Do not assume arbitrary in-process Rust dynamic libraries are safe; assess FFI/ABI stability, crash isolation, signatures, CPU/memory bounds and platform compatibility.

## Prioritized discovery

First inspect EP-03/04/06/07/08/09/12/14/18 for composable domain experience. Then validate EP-01 Rust engine attachment, EP-02 storage adapters and EP-10/13 device/web boundaries. Inspect existing manifests/loaders before proposing any new one.

**Release gate:** no plugins load into a production process based solely on this table.
