# Lane A — Universal Frontmatter Contract: Delivery and Handoff
**Date:** 2026-11-07  
**State:** CONTRACT PUBLISHED / STATIC CONSISTENCY CHECK PASSED / INDEPENDENT PARSER AND RUNTIME VERIFICATION PENDING  
**Canonical source baseline:** Repos HQ `acad98697575c2f3fe2fcbf847e19a8dcd3670db`.

## Shipped artifacts
| Artifact | Contract role |
|---|---|
| [schemas/frontmatter.v0.1.schema.json](schemas/frontmatter.v0.1.schema.json) | Typed read-only semantic IR; 35 fields, 29 required; no unknown extension fields; state, privacy and format guards |
| [schemas/field-ontology.v0.1.json](schemas/field-ontology.v0.1.json) | 35 named fields, evidence classes, stable canonicalization, owner/Host boundaries |
| [schemas/provider-adapter-matrix.json](schemas/provider-adapter-matrix.json) | 19 exact file patterns → 217 deterministic sidecar/native-route decisions |
| [schemas/shared-section-crosswalk.json](schemas/shared-section-crosswalk.json) | 13 shared concepts, eight observed families, file-local unique variant examples and 168 pinned unclassified headings |
| [fixtures/A-contract-cases.json](fixtures/A-contract-cases.json) | 3 valid and 12 negative **test declarations**, not executed schema test receipts |
| [schemas/VALIDATOR-CONTRACT.md](schemas/VALIDATOR-CONTRACT.md) | Reader/serializer, policy, native precedence, cache revision and refusal rules |

## 217-row format routing
- **55 conditional native YAML candidates:** 52 `SKILL.md` + 3 `.opencode/agents/*.md`; native keys documented, exact local loader runtime verification **not complete**. Any adapter can fall back to a sidecar when current policy permits.
- **151 sidecar-first Markdown:** no speculative new YAML headers in AGENTS, CLAUDE, GEMINI, SITREP, README, HANDOFF, etc.
- **11 JSON sidecars:** source JSON unchanged; no `---` insertion or unknown properties.
- Every route is tied to `sourceRepository/sourcePath/sourceBlobSha`; identical file titles across repos are separate sources.

## Shared/unique section safety
- 13 observed original section types are preserved with per-family meanings, distinct from reusing literal headings.
- The additional **168** heading IDs remain `null`-typed, with original path, blob, section ID, title, starting line and confidence `null`. They are a review queue, not confidently classified.
- 12 private-origin source bodies and private content remain outside Labs. The public crosswalk contains **zero private source bodies**.
- Source headings alone never become permission, binding policy or current state.

## What has been checked
Read-back static audit fetched all six artifacts from `main` and established: 217 unique matrix records route exactly once; 19 patterns; 55/151/11 formats; all 13 shared concepts and all 168 unresolved headings preserved; 35 ontology fields; 3 expected-positive and 12 expected-negative fixtures; all section review SHA references match matrix; no unresolved sections auto-classified; no private-origin source bodies copied. **Zero failures in this static audit.**

## What has NOT been checked
This lane did NOT execute JSON Schema validator against test fixtures, parse all original YAML with actual provider loaders, inspect all 217 source bodies, validate 168 full spans, run Windows/Android Host delivery or any provider instruction. It did NOT install, activate or mutate any source agentic file. A fixture definition and GitHub commit are not a test receipt.

## Lane handoff
**B (60):** apply exact schema and adapter mapping for agent instructions; record original loader, nested precedence, authority and scope evidence. No default YAML on ordinary Markdown.  
**C (79):** apply skill and tool schemas, unique triggers and verified handler status; no imaginary tools; preserve all 11 JSON.  
**D (78):** map context, dated SITREP and no-resend HANDOFF states; compose read-only DCB metadata; invalidate on source/policy/graph/evidence changes, do not rewrite in-flight requests.  
**E (217):** execute independent JSON Schema and source-specific semantic validation, parsed provider-native fixtures, negative/ACL tests, all-source hash checks, heading-span review and DCB/LRU goldsets. Publish receipts and denominator-based blocked/pass status.  
**Integrator:** compare B/C/D outputs against this contract, require any schema changes as explicit deltas; do not certify live state without separate runtime and owner authorization.

**Primary source docs:** [Agent Skills](https://agentskills.io/specification); [OpenCode agents](https://opencode.ai/docs/agents/). Published docs describe format candidates, not installed-loader compatibility.
