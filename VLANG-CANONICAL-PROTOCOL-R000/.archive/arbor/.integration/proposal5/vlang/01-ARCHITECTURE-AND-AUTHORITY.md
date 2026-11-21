# Architecture, data flow and ownership

## Functional pipeline
```text
Agent edits ordinary source code
  + small versioned @hq: declarations (untrusted)
       |
existing HQ v1/v2 lexer  -> proposed opt-in v3 grammar (NOT YET IMPLEMENTED)
       |
source SHA + blob verification + independent symbol/AST observation
       |
existing HQ semantic registry/index/SQLite/HQKG boundaries
       |
proposed SCF incremental compiler -> typed SC-ABI fact ledger
       |                                      |
       |                               independent SV
       |                          reviewed ClaimSpec/oracle
       |                          artifact-bound test/proof/trace
       |                          checker + counterexample
       |                                      |
       +------ scoped ClaimVerdicts / ProofReceipts --+
                              |
            existing Wiki / HVI read projection / agent diagnostic API
                              |
              Host-authenticated read boundary (future)
                              |
                 TierFlow inert display projection (future)
```

## Authority matrix
| Concern | Canonical owner | VLang role | Forbidden |
|---|---|---|---|
| Dev index/source access | `hq/lib/arbor-service.mjs`, indexer | Query/compile authorized dev roots | New source reader bypass, personal corpus |
| Semantic IDs | `hq/lib/semantic-registry.mjs` | Reviewed additive kinds/crosswalk | Auto-register IDs from comments |
| Storage | Existing HQ SQLite | Additive versioned SCF side tables | Second graph/registry database, destructive migration |
| Graph computation | Existing in-repo HQKG Rust | Optional bounded compute | New graph authority |
| Wiki | `hq/lib/wiki.mjs`, wiki-program | Derived pages and checked claims | New writer or independent truth |
| Comparisons | HVI accepted native interfaces | Optional semantic enrichment | Real donor migration audit at install time |
| Effects / grants | `apps/vivim-host` | Authorized read request, revocation checks | Effect/approval from metadata |
| Browser sends | Studio signed-in browser owner | None in compilation | Provider APIs, Playwright/Puppeteer |
| UI | TierFlow publisher | Read-only revision-bound inspector | New publisher, stale claims as current |
| Verification | Future independent SV under approved trust root | Check source, specs, artifacts and explanations | Compiler self-certification |

## Distinguish identities
A Git source pin is not a file path; a file path is not a symbol/unit; a symbol/unit is not a semantic capability; a capability is not an executable Host action. Persist typed crosswalks with owner evidence, not string-equality guesses.

## Data/trust contracts
- Provenance: repo+SHA+blob+content digest+dependency/toolchain/build artifact+claim spec+checker version+scope.
- Fact lifecycle: DECLARED -> PARSED/DERIVED -> independently REVIEWED/TESTED/PROVED_UNDER_MODEL or REFUTED; UNKNOWN/BLOCKED always possible.
- SV verdicts: PROVED_UNDER_MODEL, REFUTED, SUPPORTED_BY_TESTS, UNKNOWN, BLOCKED. `REFUTED` must include bound counterexample.
- Explanations: no unsupported claim clauses; finite tests never universal proof; partial telemetry never proves no effects.
- HVI 2..N: compare only immutable synthetic invented Git source fixtures during tooling acceptance; real-world comparison requires later explicit request and granted source roots.
- Security: fail closed for invalid IDs, secret paths, symlinks, scope/revocation, malformed tags, injected commands and stale cache.

## Minimal first vertical slice
One pure first-party function -> opt-in annotation -> independently re-derived source witness -> owner-reviewed ClaimSpec -> intentionally wrong code sample -> independently witnessed counterexample -> REFUTED receipt bound to exact source/build -> safe Wiki explanation. No Host effect, no browser send, no real donor comparison. This is an **implementation acceptance objective**, not a completed demo.
