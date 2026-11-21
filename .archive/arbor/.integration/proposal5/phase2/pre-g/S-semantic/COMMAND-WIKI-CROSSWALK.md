# COMMAND-WIKI-CROSSWALK — authority-free data flow

**Exact status:** an inert candidate proof only. The only actual emitted schema here is the original HQ `vivim.semantic-wiki-ir/0.1` (schema version 1); later stages are **post-G handoffs**, not wired code.

| Stage | Existing owner/source | Input → output | Admission condition / refusal |
| --- | --- | --- | --- |
| 1. Code comment grammar | D: `hq/lib/annotation-v2.mjs`, `hq.annotation/2` | real JS comments `@hq:action`, `@hq:capability`, `@hq:contract`, `@hq:effect`, `@hq:evidence` → typed records & diagnostics with `line` | refuse error diagnostics; strings are not comments; v1 explanatory tags remain separate |
| 2. Accepted semantic ID | D: `hq/lib/semantic-registry.mjs` | versioned `vivim.status.show@1` → registry `lookup` and current owner state | owner-promoted **current** semantic ID required post-G; unknown/deprecated/alias conflict fail closed; current implementation needs stricter adapter |
| 3. Read-only machine wiki IR | HQ research SWC `semantic-wiki-compiler.mjs` | records + explicit registry and Host crosswalk → `semantic-ir.json` nodes, source lines, edges, action sourceRefs, version/risk/readiness | original schema preserved; `authority:none`, `executable:false`, `executionPerformed:false`, `sourceDigestVerified:false`; no wiki write |
| 4. Inert Rust candidate | A native graph/Node0 (future) | source-attested immutable IR + typed original-text event → bounded candidate/ambiguous/invalid trace | no effects, grants, scheduling, source read escalation, cached stale text overrides or duplicate graph; Rust integration **UNRUN** |
| 5. Host effect admission | B `intent-planner.mjs`, `command-intent.mjs`, catalog v6 and existing dispatcher | candidate + **original user text** + actor/session/revision → Host recomputation and exact risk/approval/grant decision | Host alone chooses action; a wiki/comment/Candidate/Tab/sigils cannot authorize; stale catalog/world refuses |
| 6. Durable receipt reference | B existing exclusive action journal/claims/receipts | accepted Host action result → externally sourced receipt ID/state (including `uncertain`) | no receipt is created by SWC; unknown or uncertain send must not auto-retry; reference must stay `null` without actual Host result |
| 7. Human wiki explanation | D existing HQ wiki read projection; E TierFlow R005-style visual lens (future) | same source refs / accepted Host disposition / real receipt ref → safe explanation and VIEW ALL trace | explanation must distinguish source-declared, candidate, unavailable, approved and applied; AutoWiki/Rock Core R008/R009 are design-only |

## Synthetic end-to-end row, no execution

`fixtures/status.example.mjs` lines **2–6** → `vivim.status.show@1` (synthetic current-registry test lookup) → IR node `synthetic.status:2:action` and four typed edges → explicit B catalog `vivim.status.show` (owner `host`, risk `R0`, `source_wired`; B catalog v6 blob `c423d5933c28ca9f534028aeaffd7929f767c269`) → **Rust UNRUN** → **Host admission UNRUN** → **receiptRef null** → **human page not generated**.

A real source identity is **repo + immutable commit/tree/blob + path + content digest + symbol/span + trust/owner scope**. The sample only proves source bytes SHA-256 self-consistency and a selected catalog identity. A full trusted repository walker, current owner semantic IDs, exact Host catalog freshness and revision-coherent original-text semantics remain separate requirements.

## What is explicitly refused

- No `@hq:semantic/3` runtime grammar; unsupported proposed `@hq:utterance`, `@hq:frame`, `@hq:slot` etc refuse in the conservative local boundary adapter. The HQ v2 parser itself ignores unknown names, so it is **not sufficient alone** for v3 compatibility.
- No natural-language parsing or routing in the adapter. `guardHumanRequest` is negative-only: negation, quotation-only, ambiguous meaning and Host refusal are **not interpreted into actions**. Even one meaning requires Host original-text re-interpretation.
- No new wiki writer, semantic registry, action executor, user permissions, private corpus ingestion or production DCB memory.
- No source/digest, action/readiness, receipt or device acceptance is inferred from the synthetic fixture.
