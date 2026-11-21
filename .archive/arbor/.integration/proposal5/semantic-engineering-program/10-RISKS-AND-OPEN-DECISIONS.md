# Risk and decision register
| Issue | Default resolution for deep prep |
|---|---|
| Compiler grading itself | independently rederive syntax facts, independently checked spec/oracle |
| Same agent writes code + tests + oracle | require separate trusted reviewer/CI attestation |
| Universal claim from finite tests | SUPPORTED_BY_TESTS, not PROVED_UNDER_MODEL |
| No events equated to no effects | UNKNOWN unless complete authorized observation/static proof |
| Semantics fabricated in prose | claim-bound reviewed explanation templates and modality |
| Tags too heavy on legacy code | minimal touched-file/export protocol and waivers |
| Version/migration ID reuse mistaken equivalence | HVI source/build/behavior proof delta; explicit unknown |
| Source SHA vs actual running artifact | pinned build dependency/toolchain and output digest |
| HVI branches actively changing | don't merge SCF or add adapter before accepted gate |
| Private data or secrets from source | owner grants/redaction/denial at read and publish |
| New verifier creates second execution authority | pure isolated tests only; Host owner receipts for effects |
| Resource exhaustion, ENOSPC, cancellation | bounded workers/cache, preserve originals and last verified snapshot |
| SDK scope too early | kernel/falsification first, opt-in SDK after accepted contracts |

Open deep-prep decisions: checker in Rust vs JS (actual TCB constraints); CI/oracle approval ownership; initial metadata coverage threshold; formal model checker bounded domain; runtime witness completeness semantics; first supported language/runtime targets; SDK package boundaries; independent release QA; explicit cost/performance budgets; optional TierFlow/IDE UX scope.
