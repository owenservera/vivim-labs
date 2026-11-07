# Wave 1.5 — Frontmatter design across all 217 agentic files

**State:** architecture and per-file work plans only, no installed headers, sidecars or modified provider instruction files. **Date:** 2026-11-07.
**Source baseline:** Repos HQ `acad98697575c2f3fe2fcbf847e19a8dcd3670db` plus the migrated Labs [agentic-file corpus](../agentic-file-system/) and [Wave 1.5 DCB/agentic drafting design](../context-corpus-wave-1-5/).

## Objective
Give every inventoried agentic-file *instance* a usable machine-readable semantic identity, file-role metadata, provenance and rich context-routing signals. Design **one common metadata contract** but preserve original semantics, provider-native frontmatter compatibility and per-file unique requirements. Do not mass-edit the recovered 217 source bodies.

## The five lanes (one owner per file)
| Lane | Purpose | Exclusive per-file design owners | Cross-cutting scope |
|---|---|---:|---|
| [A: Contract and ontology](LANE-A-CONTRACT.md) | Universal envelope, shared vs unique sections, schema and precedence | 0 | all 217 |
| [B: Agent instruction surfaces](LANE-B-AGENTS.md) | AGENTS, BOOTSTRAP, provider rules, agents, workspaces | 60 | families 01 + 02 |
| [C: Skills, capabilities and config](LANE-C-SKILLS-TOOLS.md) | SKILL, BEST-PRACTICES, tools, JSON configuration | 79 | families 09 + 08 + 10 |
| [D: Context, status and continuity](LANE-D-CONTEXT-STATE.md) | CONTEXT/README, SITREP, HANDOFF | 78 | families 04 + 12 + 19 |
| [E: Independent verifier](LANE-E-VERIFICATION.md) | Schema conformance, loader safety, DCB retrieval, simulations | 0 | all 217 |

**Exact work allocation:** `FRONTMATTER-217-WORK-MATRIX.json` assigns each of the 217 file IDs once to B/C/D and provides a proposed frontmatter encoding, common fields, file-role fields and the file's original unique section evidence. A and E review every file. Historical 8 donor snapshots remain inert.

## Important distinction: frontmatter is not always YAML
- `SKILL.md` and observed `.opencode/agents/*.md` support provider-native YAML in the inspected exemplars; preserve the loader's actual supported keys.
- 151 Markdown documents/instruction files get **semantic sidecar-first** metadata, not speculative YAML inserted into `AGENTS.md`, `CLAUDE.md`, `SITREP.md` etc. A target-specific loader may support frontmatter after proof.
- 11 JSON files remain valid JSON; place universal metadata in a **versioned sidecar** or an already supported schema property. Never prepend `---` to JSON.
- **55 native-YAML candidates** (52 skills + 3 OpenCode agent profiles), **151 sidecar-first Markdown**, **11 JSON sidecar**; these are *routing proposals* from filenames, not verification of every source's exact existing header.

## Shared contract vs unique file
Lanes must not use one generic human-readable header. Model the same semantic axes (identity, purpose, scope, source/authority, lifecycle, input/output, context behavior, privacy, dependencies, verification) through family-specific vocabulary and surface-specific native syntax. Link the file's original `section:NNN:NNN` IDs to the shared [13 section types](../agentic-file-system/recovered/section-types.json); keep the 168 newly added unclassified headings under review. Repeated literal headings are not sufficient proof of shared semantics.

## One proposed metadata projection
```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:002",
  "fileType": "SKILL.md",
  "sourceRepository": "owenservera/aion",
  "sourcePath": ".opencode/skills/aion-skill-builder/SKILL.md",
  "sourceBlobSha": "aa6a9302a93f7fb6a91dd876839f749f7f634418",
  "purpose": "source-grounded skill research",
  "scope": {"projectRef": null, "roleRef": null, "taskRef": null},
  "lifecycle": "historical-research-copy",
  "authorityClass": "observed-source-not-policy",
  "consumerSurfaces": ["lab-search", "context-dcb"],
  "sharedSectionTypes": ["section_type:tools"],
  "uniqueSectionIds": ["section:002:002"],
  "provenanceRefs": ["repos-hq:file:002"],
  "generatedState": "metadata-proposal"
}
```
This is an **illustrative sidecar projection**, not the actual content of the source file or authorized provider-native YAML. Machine-readable fields are explicit typed values; the composer may generate task-dependent context projections without modifying the source.

## Interfaces and integration
1. A publishes `frontmatter.schema.json`, vocabulary and an exact reader/serializer contract.
2. B/C/D each publish file-by-file metadata plans and *inert* native/sidecar examples, exclusively for their assigned IDs.
3. E runs parser/validator/read-only simulated consumer tests. The Wave 1 Rust graph can index read-only `FILE_HAS_FRONTMATTER`, `FRONTMATTER_HAS_SECTION`, `APPLIES_TO_CONTEXT`, `SOURCE_PROVENANCE` edges, but does not acquire write or permission authority.
4. The dynamic context composer selects relevant metadata **as data**, not automatic instructions. The LRU keys on source hash, schema revision, policy, scope, graph epoch and provider adapter version. File change invalidates dependent projections.
5. A master integration checks 217/217 coverage, zero duplicate ownership and zero unsupported native keys. There is no automatic activation or cross-repo migration.

## Completion criteria and reporting
- A: normative schema and error codes; all 19 observed file type patterns represented; preserve 13 shared section types and 168 unclassified additions.
- B: 60/60 files specified; provider-native precedence, nested scope and owner-grant constraints.
- C: 79/79 specified; 52 skill native YAML candidates and 11 JSON configs remain syntactically valid; no unverified tools.
- D: 78/78 specified; source-revision and scope-aware context, fresh status evidence, no duplicate handoff effects.
- E: 217/217 independently checked, at least five representative inert fixtures per relevant format/family combination, adversarial tests and baseline DCB retrieval comparison.
- **Gate:** provider loading, native instruction execution, Windows/Android behavior, or any Host effect remain **UNTESTED** unless independent real-run receipts exist.

Every lane reports `sourceSha`, `matrixVersion`, `attempted`, `designed`, `blocked`, `validated`, `missingEvidence`, `testReceipts` and `handoff`. Zero files may be marked active merely by producing the design.

## Execution coordination
Run A first to publish a minimal contract; B/C/D can concurrently research and draft against that **pinned** contract. E develops tests in parallel and independently verifies their outputs. Each lane may propose contract changes via a documented delta, not write another lane's owned files. Integration is a separate accountable pass, not one of the five lanes.
