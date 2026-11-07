# Lane B — Agent identity, bootstrap and provider-native instruction design: handoff

**Lane:** B (families 01/02) · **Status:** DESIGN DRAFTS READY FOR A/E REVIEW; **not** validated for native use or activated.

## Required lane report

| Field | Actual result |
|---|---|
| `sourceSha` | Repos HQ `acad98697575c2f3fe2fcbf847e19a8dcd3670db` |
| `matrixVersion` | `vivim.agentic-frontmatter-work-matrix/0.1` |
| `attempted` | 60 |
| `designed` | 60/60 structural per-file proposals, including private-source stubs |
| `blocked` | 4 private bodies absent; 2 taxonomy anomalies; 1 path-casing anomaly; native loader evidence missing |
| `validated` | 10/10 local structural checks; **0** independent Lane E acceptances; **0** native loader confirmations |
| `missingEvidence` | See blockers below |
| `testReceipts` | [Structural receipts](B-STRUCTURAL-CHECK-RECEIPTS.json), [11 inert negative cases](B-ADVERSARIAL-FIXTURES.json), [source inspections](#deliverables) |
| `handoff` | Lane A contract alignment and Lane E independent source/loader/context validation |

## Deliverables

1. [Per-file catalogue](B-60-FILE-DESIGNS.json): one source-pinned typed design record per assigned file, including original identity, exact section links, unique signposts, adapter proposal, forbidden authority fields, status and missing evidence.
2. [Source inspections 01](inspection-01.json), [02](inspection-02.json), [03](inspection-03.json), [04](inspection-04.json): read-only summaries for 56 public-origin recovered source files, pinned to 56 matching Git blob SHAs. Four private references explicitly lack public source bodies.
3. [Eight inert side-by-side examples](B-INERT-SIDECAR-EXAMPLES.md): native-preserving versus virtual sidecar projection for OpenCode, AGENTS, BOOTSTRAP, WORKSPACE, AGENT, CLAUDE, GEMINI, and a misclassified `claude.md` case.
4. [Eleven adverse scenarios](B-ADVERSARIAL-FIXTURES.json): nested precedence, cross-repository collisions, provider switching, stale environment, source injection, owner revocation, shared workspace, contradictory rules, permission spoofing, false Claude-native routing and case sensitivity.
5. [Structural self-check receipts](B-STRUCTURAL-CHECK-RECEIPTS.json): 10 checks passed; structural self-checks do **not** establish native loader semantics, safe live execution or independent verification.

## Measured coverage

| Metric | Count |
|---|---:|
| Lane B distinct file IDs | 60 / 60 |
| Recoverable source snapshots inspected and blob SHA matched | 56 / 56 |
| Private source-body references not inspected | 4 |
| Source-to-atlas section links | 399 |
| Still-unclassified sections (Lane B subset only) | 94 |
| Existing native YAML headers observed | 3 |
| Native keys emitted, originals overwritten, agents activated | **0 / 0 / 0** |
| Lane E independent accepted | **0** |

**Coverage interpretation:** All 60 now have a concrete *design proposal*. This does not mean the semantic content of all 60 has undergone independent manual review. The four private references are deliberately stubs, and all runtime-dependent semantic fields remain unknown until verified.

## Blockers and findings

- **Private:** `file:196`, `file:197`, `file:198`, `file:199` have private references with no source body in this Labs corpus. Their source text must not be reconstructed or copied into public fixtures.
- **Classification conflict:** `file:154` and `file:213` are typed `CLAUDE.md` by the work matrix but their paths end in lowercase `docs/shape-atlas/claude.md`, describing a provider web-shape contract. They are not to be loaded as Claude instructions. Lane A / taxonomy owner must resolve this without silently overwriting the matrix.
- **Filename case:** `file:021` is `AGENT.MD` in the source; do not assume `AGENT.md` path discovery in a provider or a case-sensitive filesystem.
- **Provider native compatibility:** All 3 OpenCode profile source copies have observed YAML keys `description` and `mode`. The installed OpenCode runtime version, actual recognition of those keys and exact precedence have not been live-verified. The same holds for Claude/Gemini path/precedence.
- **Authority:** All current effective owner, tool and environment permissions are unresolved. Historical donor instructions are indexed as untrusted research data, never reclassified as active VIVIM owner policy.
- **Atlas:** 94 section headings in B remain explicitly unclassified. Do not invent shared section semantics to improve coverage statistics.
- **Dependency:** The independently published Lane A normative schema/adapter files must be pinned and conformance checked. The matrix is an initial contract, not equivalent to verified Lane A artefacts.

## Acceptance request to Lane E

1. Confirm 60/60 row ownership and source SHA, including privacy exclusion of 4 private-source bodies.
2. Test native loader compatibility per observed provider/version and native placement; never activate from a test fixture.
3. Run all 11 adverse cases; fail closed on historical instruction spoof, nested policy conflict, owner revocation and source injection.
4. Reconcile section variant mappings and both `claude.md` classification exceptions.
5. Compare DCB retrieval with and without sidecar metadata against a pinned corpus. Record correctness, context budget, cache freshness, ACL enforcement and token efficiency.
6. Return independent pass/fail/blocked receipts, not a generic approval.

## Change boundaries

Only new, inert **Lane B** design, inspection and testdata artifacts were added in VIVIM Labs. One catalogue file was subsequently updated to make taxonomy anomalies explicit. No `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, original recovered body, provider config, runtime agent or live host setting was modified.

**Exit decision:** **Ready for A schema reconciliation and E independent review**. **Not accepted for runtime installation or effectful execution**.
