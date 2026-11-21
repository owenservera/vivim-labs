# DevAgent integration: R005 semantic truth -> existing Wiki PWA surface

**No ARBOR edits were made by Planning.** The R005 files are staged directly in the **existing** Labs R003 crate. Re-pin ARBOR and coordinate with active ChatGPT Dev claims before applying. First integrate or reconcile [R003/R004](../R003/DEVAGENT-APPLY.md), then apply this additive module set.

## 1. Register three small Rust modules in existing core

After receiver ACK, copy from Labs `DESIGN/AUTOWIKI-FOUNDATION-R001/src/` into the **existing ARBOR** `core/programmability/src/autowiki/`:

```text
semantic_evidence.rs    # proof decision -> existing SentenceClaim
page_assembly.rs        # authorized page/links/unknown state
read_pipeline.rs        # one bounded vertical read-only composition
```

The Labs `src/lib.rs` already declares `pub mod semantic_evidence;`, `pub mod page_assembly;`, `pub mod read_pipeline;`. If Dev has already installed R003/R004, add just these declarations to existing `autowiki/mod.rs`. **Do not duplicate** `core/programmability` into a new product crate or copy the Lab Cargo.toml to ARBOR.

## 2. Exact API sequence

```text
Existing human/authorized-agent composer source (Dev-owned)
 -> Existing Rust LiveSystemState/SourceChanged/Draft (no new parser)
 -> existing context_lens::Lens state
 -> R004 issue_read(source-attested Wiki digest, graph epoch)
 -> existing Host owner read + native HQKG query_scoped + R004 authorization
 -> R003 LensProjection + exact page aliases
 -> source-owner current grant + independent semantic revision/source commit
 -> R005 project_human_wiki_page(
      lens, ticket, context, current_graph_epoch, current_wiki_digest,
      current_scope, proposed_claims, proof_reader, hub_map, allowed_hubs)
    -> R004 readback_guard::accept_readback (reject stale)
    -> R005 semantic_evidence::admit_semantic_sentences
    -> EXISTING R003 narrative::realize
    -> R005 page_assembly::assemble_human_page
 -> existing Host bounded read port + existing Dev-owned PWA Wiki renderer
```

`project_human_wiki_page` returns `HumanWikiPage`: exact committed canonical page ID, seven hub navigation entries (**None** for unavailable hubs), current Wiki revision and graph epoch, explanation state and claim-backed paragraph. In the browser use **`textContent`**, existing keyboard/IME handling, existing accessible links and the same unchanged PWA URL. Never construct HTML from the sentence.

## 3. Proof reader activation is explicitly gated

```rust
// Until an independently implemented/checker-authenticated SV exists:
let proof_reader: Option<&dyn IndependentProofReader> = None;
```

That default intentionally produces a truthful unavailable explanation for capability claims, not a fabricated verified paragraph. No adapter may set `independently_authenticated`, `model_scope_checked` or `explanation_checked` from user JSON, generated code comments or a self-run fixture. See [source truth](SOURCE-TRUTH-AND-ABI.md).

## 4. Source and security fences

- Refetch current Host permission/grant + revision immediately before query **and again before reply**; a revoked grant must invalidate results even if graph/Wiki revisions did not change.
- Keep graph epoch, semantic source revision, Wiki journal revision, Wiki SHA-256 digest and input request sequence separate. Do not collapse to a single timestamp or reuse expected revision as trusted.
- `allowed_hubs` and `ExactHubMap` must come from current committed Wiki and trusted exact entity IDs, not proposed display slugs. Missing canonical pages are unavailable, never generated as facts.
- R004 `query_admission` requires a complete current graph exclusion set; current owner does not yet expose a proven generator. If incomplete: **refuse read**, not prefix-only fallback.
- `IndependentProofReader` is a trusted Rust trait injection boundary, not an authentication engine. Do not wire browser data directly to it.
- All R005 page assemblies are read-only. Existing Host Control remains sole effect authority; no autonomous Send, Wiki write, provider access, trace grant, task creation or private development-index API.

## 5. Acceptance and rollback

Run [ACCEPTANCE](ACCEPTANCE.md) from a real cloud Rust checkout, then the owner-granted Host/PWA acceptance through Dev's existing release playbook. Record actual commands, exits and separate GPUI/browser/phone observations; do not confuse mock proofs with real SV deployment.

To revert, revert only the three new Rust modules and their `pub mod` lines plus the explicitly integrated read consumer; preserve current Wiki, HQKG and Host journals, in-flight Dev PWA patches, grants and source history. Coordinate before any rollback because Dev's product files are active.
