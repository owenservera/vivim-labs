# Additive comment meta-language — v1+v2 now, typed v3 extension later
## Compatibility
Use existing **v1** `@hq:context, why, invariant, see, todo, owner, task, risk, deprecated` and **v2** `@hq:type, capability, effect, evidence, contract, emits, consumes, action, test`. Do NOT break `hq.annotation/2` or private parser behavior. Proposed `@hq:semantic/3` is opt-in and **DESIGN_ONLY** until owner-reviewed grammar/fixtures/negative corpus. Existing v2 parser ignores unsupported tags; do not install these in product source before compiler activation.

## SCF minimum
Each first-party non-generated source file has a file-scope header:
- `@hq:context` short human explanation of responsibility;
- `@hq:owner` canonical source owner;
- 1..N `@hq:capability` registered feature IDs, each `name@numeric.version`; an unclassified file needs PR-level exemption, not fabricated tags.
- Optional `@hq:contract`, `@hq:test`, `@hq:invariant`, `@hq:why`, `@hq:effect` as meaningful.
Specific exported API/symbol may carry narrower v2 annotation. Compiler must distinguish FILE_SCOPE and SYMBOL_SCOPE via actual syntax/comment attachment, not substring matching or guessed line ranges.

## Proposed registered v3 extension (reference-only, new owner-reviewed grammar)
| Tag | Typed value | Intended meaning | No authority to |
|---|---|---|---|
| `@hq:unit` | registered stable `name@version` | identity of file/symbol unit across relocation | override actual Git blob/symbol |
| `@hq:role` | registered role ref | unit is adapter/owner/consumer/projection/test, etc. | change canonical runtime owner |
| `@hq:relation` | registered relation record ref | typed directed relationship; target/role resolved in registry | create graph mutation or permission |
| `@hq:migration` | registered lineage record ref | old pin/unit → new pin/unit + replacement rationale | confirm equivalence without evidence |
| `@hq:wiki-topic` | registered concept ref | human/machine wiki navigation anchor | create alternate wiki writer |
| `@hq:utterance,frame,slot,constraint,visual,example,disclosure` | registered refs from SWC-001 | curated deterministic language/UI extension | install Host handler or arbitrary markup |

The SCF-specific `unit`/`role` additions are **proposals to the existing SWC v3 draft**, not an alternate language version or a competing parser. All values refer to owned typed registry records; do not inline JS/SQL/shell/URL executors or untrusted expressions.

## Illustrative opt-in future v3
```rust
/// @hq:context Native graph observations for a bounded compare query.
/// @hq:owner native-hqkg
/// @hq:capability vivim.hvi.compare-revisions@1
/// @hq:contract vivim.hvi.compare-request@1
/// @hq:effect host:read-only
/// @hq:invariant Native graph work may not own Host effects.
/// @hq:unit vivim.native.hvi.comparator@1
/// @hq:role vivim.unit.pure-compute@1
/// @hq:relation vivim.relation.hvi-uses-source-manifest@1
/// @hq:migration vivim.migration.hvi-native-read-port@1
fn compare_native(/* ... */) { /* ... */ }
```
**Do not paste this into live code as if v3 is accepted.** Registry records and parser don't yet exist; the example uses illustrative IDs.

## Compiler rules
- Every `@hq:...` must be lexed from a real native comment, never a string/regex/template/data payload.
- Typed refs must exist, have acceptable status/owner and resolve under a SHA-pinned source tree; incomplete source returns `unknown`.
- Source unit identity != file path != semantic capability identity != Host action ID. Separate registries and explicit crosswalk preserve meaning.
- Duplicate `@hq:capability` identical record is a warning, not a second contribution. Contradictory owner/contract/effect refs are hard errors.
- `@hq:effect` describes potential class, not authorization. `@hq:action` is a descriptive link to owner catalog only.
- `@hq:test` is a declared relationship; test path existence, actual exercised coverage and passed runtime receipt are three separate facts.
- Never auto-register unknown semantic IDs, promote deprecated IDs, or infer equivalence from identical names.
- No user-specific memory/DCB/credentials in source comments. Private memory/owner overlays remain separately authorized, revocable and non-exportable by HQ.
- Bound the number/size of annotations and deny secret-path refs and self-recursive relation bombs; collect compiler diagnostics without executing source text.
