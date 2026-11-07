# Agentic-file research — shared and unique section atlas

**Status:** migrated descriptive research; no files are auto-activated.  **Pinned source:** `acad98697575c2f3fe2fcbf847e19a8dcd3670db`.

The source research contains **13 shared section types** and a separate **per-file section variant** layer. Section-title equality does **not** imply semantic equivalence. The original `section_type:unique` means the earlier heading classifier assigned a file-specific class; it is not proof that the wording is globally unique. The supplement added new headings and replaced source-pinned headings, but supplied no classifications for its new rows: these remain **unclassified** pending inspection, not automatically relabeled unique.

## Verified migration calculations

- **217 file instances**: 205 originate in public repos, copied as inert `.source.txt` files; 12 originate in private repos, represented here by opaque metadata/reference only (bodies remain in private Repos HQ).
- Original heading research: **842** rows. Pinned supplement: **216** rows = **168 additions and 48 replacements**. Merged view: **1010 headings** across **206 files**. All replacements keep earlier types only when the same section ID existed; new additions remain unclassified.
- **802 distinct normalized heading strings**, **76 repeated strings across multiple files**; this is literal string overlap, not semantic interchangeability.

## Shared semantic type coverage, by observed family

| Family | Files | Headings | Classified shared-type | Earlier unique-type | Unclassified | Repeated headings |
|---|---:|---:|---:|---:|---:|---:|
| 01 | 53 | 376 | 130 | 168 | 78 | 102 |
| 02 | 7 | 50 | 8 | 26 | 16 | 16 |
| 04 | 46 | 127 | 40 | 74 | 13 | 0 |
| 08 | 5 | 4 | 3 | 1 | 0 | 0 |
| 09 | 67 | 168 | 66 | 77 | 25 | 42 |
| 10 | 7 | 0 | 0 | 0 | 0 | 0 |
| 12 | 20 | 205 | 97 | 101 | 7 | 77 |
| 19 | 12 | 80 | 17 | 34 | 29 | 47 |

### Shared section ontology

- `section_type:purpose` — Purpose and scope
- `section_type:prerequisites` — Prerequisites and environment
- `section_type:responsibilities` — Responsibilities and boundaries
- `section_type:workflow` — Execution workflow
- `section_type:best_practices` — Best practices
- `section_type:tools` — Tools and capabilities
- `section_type:context` — Context and memory
- `section_type:safety` — Safety and permissions
- `section_type:verification` — Tests and acceptance
- `section_type:recovery` — Recovery and handoff
- `section_type:lifecycle` — Status and lifecycle
- `section_type:examples` — Examples and templates
- `section_type:unique` — File-specific specialized section

### Unique and reused content must stay distinct

1. **Shared concept:** e.g. Verification is a common type with a stable identifier; agents can discover it across file families.
2. **File-specific variant:** in `AGENTS.md`, Verification describes behavior/acceptance rules; in `SKILL.md`, executable method/tests; in `HANDOFF.md`, existing receipts and safe restart; in `SITREP.md`, observations/time. These must never be blindly copied across files.
3. **Unique heading:** a title appears in one file but may still map to a shared semantic type. Literal uniqueness is not independent evidence of unique function.
4. **Repeated heading:** multiple files use identical or normalized-equal titles, but its body, authority and context can differ.
5. **Unclassified supplement:** newly extracted headings need evidence-based semantic classification before any automated generation.

### Source-inspected research examples

- **`AGENTS.md` — best_practices:** binding scoped instructions. Needs instruction precedence, bounded do/don't rules, test commands, conflict resolution. Risk: rules accidentally propagate to unrelated repos.
- **`SKILL.md` — best_practices:** task-specific repeatable method. Needs trigger conditions, procedure, fixtures, failure modes, references on demand. Risk: loads too broadly or implies capabilities without tools.
- **`ARCHITECTURE.md` — best_practices:** design rationale and architectural constraints. Needs invariants, alternatives, tradeoffs, failure domains, compatibility tests. Risk: design preferences mistaken for executable policy.
- **`BOOTSTRAP.md` — best_practices:** cold-start repeatability. Needs prerequisites, first commands, readiness gates, safe restoration. Risk: stale startup instructions.
- **`SITREP.md` — best_practices:** observed lessons from current work. Needs time scope, evidence, confirmed vs inferred, blockers, next checkpoint. Risk: transient observations promoted to permanent authority.
- **`HANDOFF.md` — best_practices:** resumable handover without duplicate actions. Needs state IDs, receipts, in-flight effects, no-resend gates, next owner. Risk: duplicate external effects.
- **`schema.sql` — best_practices:** database consistency and migration discipline. Needs FK constraints, versioned migrations, idempotence, backfill, rollback/read-only checks. Risk: data corruption or schema fork.
- **`TAXONOMY.json` — best_practices:** identifier and semantic authority. Needs stable IDs, parent links, provenance, alias collisions, versioning. Risk: silent semantic drift.

## The per-file work you asked for

`SECTION-ATLAS-217.json` contains one record per inventoried file, its exact source hash, type/family, the original section IDs, headings, lines, shared type assignments and how many other files reuse each heading. The 12 private file bodies and their headings are redacted from the public atlas; section counts are retained.

Use the atlas plus the 217-record drafting plans from [the Wave 1.5 design](../context-corpus-wave-1-5/) to develop a **two-pass draft compiler**: shared section ontology/requirements first, unique variant synthesis with per-file verified evidence second. Do not convert research statements into owner instructions without explicit approval.

## Next measurable checks

- Inspect each new/unclassified heading against the source text; assign shared type or document a truly file-specific variant, with provenance and confidence.
- Detect cross-file near-duplicates within **same semantic context**, not global text matches; preserve role/provider differences.
- Select one example per observed family; synthesize inert candidate section text from source spans, then validate syntax, citations, expiry and authority.
- Evaluate retrieval against held-out virtual-agent tasks, checking whether the context queue surfaces the *right section variant* rather than an irrelevant repeated heading.
