# Lane A — Universal frontmatter ontology, syntax adapters and shared/unique semantics

**Owns:** shared contract, validators' expected behavior and field ontology. **Per-file assignment:** no exclusive instance; validates the vocabulary for all 217. **Cannot:** change original files, provider instructions or owner policy.

## Core objective
Define an internal `vivim.agentic-frontmatter/0.1` semantic IR for 19 observed types and extensible 21-family taxonomy. Derive a minimal canonical field set for every file but keep external representations provider-native. Distinguish identity from current task state, an observed claim from a permission, and a section's shared meaning from a file-specific variant.

## Required design
1. **Identity:** stable file ID, original repo/path/blob hash, generated target identity, intrinsic file kind and current schema revision. Never derive identity from mutable title alone.
2. **Ownership and policy:** policy owner, scope, principal/destination, evidence authority class, lifecycle, sensitivity, export restrictions; no metadata grants execution approval.
3. **Retrieval:** semantic purpose, role, task affinity, triggers, symbols/capabilities referenced, dependencies, auto-exposure roles, context freshness and token budget hints. Descriptive only.
4. **Section links:** original `section:...` IDs, 13 shared `section_type` categories, file-specific section variants, explicit unresolved/unclassified class. Design a rubric for 168 supplemental headings with original spans and confidence.
5. **Compatibility:** adapters for native Agent Skills YAML, OpenCode agent YAML, ordinary Markdown and JSON; sidecar fallback always allowed where private/current policy permits. Never introduce a YAML header to consumer JSON.
6. **Change rules:** source hash and schema version pinned; normalized field order/digest; stable diffs, no hardcoded local paths, no secrets, no synthesis of unobserved capabilities.

## Artifact contract
- `schemas/frontmatter.v0.1.schema.json` and `schemas/field-ontology.v0.1.json`
- `schemas/provider-adapter-matrix.json`: verified/unknown provider-native keys, precedence and supported placement
- `schemas/shared-section-crosswalk.json`: per-family section-type meanings and unique variants; evidence class for each mapping
- `fixtures/A-*.json`: positive/negative cases, including stale hash, private scope and conflicting owner authority

## Gate
All 217 rows route deterministically to a valid adapter; all 19 observed type patterns mapped; 168 headings remain visibly unclassified until reviewed; native keys never inferred solely from a Markdown extension; schema validation does not turn untrusted text into authority.
