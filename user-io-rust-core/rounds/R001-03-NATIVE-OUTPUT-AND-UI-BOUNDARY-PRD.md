# R001-03 — PRD: Native Output Projection ↔ High-Level UI Contract

Date: 2026-11-12
Status: read-only native projection shape source-implemented on isolated Harness branch; NOT consumed by live UI.

## Principle
Rust owns **what is believed, why, at which source revision, with what uncertainty and permission state**. The UI owns **what the user sees, how it moves, where Tab goes, which symbols appear and how accessibility describes them**.

The existing Labs UI workstream is not subordinated to a generic Rust-made UI grammar. It should be free to invent semantic ink, chip compression, fine-grained typography, menu geometry, visual microstate, caret-aware controls, Quiet/Assisted/X-ray and future TierFlow canvas as distinct views of the SAME native semantic state.

Native Rust must never pretend typography, animated outline, glyph or screen position is permission to execute.

## UI progress read on R005/R006
At Labs UI head \`20f393c...\`:
- \`src/semantic-lens.mjs\`: source-span scanning, negation handling, hints, command menus, meaning deltas and safe span replacement.
- \`src/compiler.mjs\`: read-only Host-authoritative visual projection, with refusal/clarification priority and \`executionPerformed:false\`.
- \`src/bridge.mjs\`: read-only source inventory from Host/HQ/TierFlow, simulated contacts and provenance. No effect writes.
- R005: within-composer visual controls with quiet/assisted/xray, 28/28 Node unit tests and 100 synthetic probes reported.
- R006: design catalog covers predictive project/focus menus, Rust graph fanout, prewarm, slot provenance, stale revisions and two-Tab benchmark. It does not establish a native implementation.

## Contract: Rust -> UI
**Source state** (not UI decisions):
- \`session_id\`, \`input_revision\`, \`interaction_revision\`, \`catalog_revision\`
- \`phase\`: draft / project_selected / fully_specified / submitted_inert
- source-linked derived evidence from independent native reactors, each with reactor ID and exact input revision
- \`suggested_template\`, \`suggested_project\` with ID/source revision/evidence reference, \`suggested_focus\`
- \`selected_template\`, \`selected_project\`, \`selected_focus\`
- \`ready_to_submit\` (typed completion, NOT effect permission)
- \`execution_performed:false\` and \`authorization:"not_assessed"\`

**UI projection behaviors**:
- Ghost tail may show \`Full update [XYZ?] — [Next steps?]\` but must label speculative vs grounded.
- Tab 1 commits the project, with template pinned; Tab 2 commits focus; no implicit third action.
- No source evidence -> ghost display can remain a suggestion but must not claim it is grounded.
- An unregistered catalog, stale source or ambiguous referent creates a selector/error—not a fake filled chip.
- Correct screen-reader equivalents for predicted/selected/available/requires approval.
- Stable editor/IME composition: render local text immediately, apply native semantic deltas only if matching current revision.
- UI may animate, compress, reorder or hide information but cannot invent a source status or effect success.
- A view mode change cannot alter Rust candidate identity, risk, effect authorization or learning evidence.

## Contract: UI -> Rust
R001 source-implemented JSON-only Tauri command:
\`\`\`json
{"schema":"vivim.native-user-io/1","kind":"draft_changed","session_id":"s1","sequence":1,"text":"full","composing":false}
\`\`\`
\`\`\`json
{"schema":"vivim.native-user-io/1","kind":"tab","session_id":"s1","sequence":2,"expected_revision":1}
\`\`\`
A second Tab uses the last returned projection's revision; \`submit\` is an inert test marker.

**Security**: These R001 examples contain no trusted identity. The desktop Tauri surface authentication boundary has to be implemented and checked before any private project/World data can be supplied. The native Tauri command currently installs an EMPTY source set; UI should not interpret empty sources as absent projects in the user's actual World.

## R001 response example (semantic only, values shown as synthetic)
\`\`\`json
{
  "schema":"vivim.native-user-io/1",
  "outcome":"projection",
  "projection":{
    "session_id":"s1",
    "interaction_revision":1,
    "input_revision":1,
    "catalog_revision":1,
    "phase":"draft",
    "suggested_template":"fixture.full.update",
    "suggested_project":{"id":"xyz","label":"XYZ (fixture)","source_revision":1,"evidence_ref":"fixture.source.xyz"},
    "suggested_focus":"next_steps",
    "selected_template":null,
    "selected_project":null,
    "selected_focus":null,
    "ready_to_submit":false,
    "authorization":"not_assessed",
    "execution_performed":false,
    "evidence":[]
  },
  "executionPerformed":false
}
\`\`\`
The \`evidence\` array above is deliberately elided as an example; real R001 output includes the typed derivation records from the four subscribers. Never use this sample as live fixture proof.

## Target richer schema (future phases, not implemented)
Per-token byte and UTF-16 spans; clause graph; negativity/quoted/literal boundaries; alternative grounded project candidates; missing slots; source status and freshness watermark; abstract semantic affordance hints; per-source capability check; reversible selection delta; data class/audience; accessible human-language explanation. Preserve stable IDs across view modes. Do not require Rust to produce HTML or CSS.

## Performance / consistency
- DOM/IME/caret paint is local; Rust is asynchronous suggestion computation.
- UI drops stale projections using \`input_revision\` + \`interaction_revision\` rather than relying on response timing.
- Native work can stream partial source-labelled patches in future; R001 returns one snapshot per call.
- Suppress network fetch while typing until a source-safe speculative read port is authorized; no provider sends.
- Keyboard Tab must be a user action targeted at a visible current suggestion; never auto-fire when text editing or assistive focus traversal should take precedence.
- Only explicit accepted Tab slots count for native habit learning. Undo/reject signals need future input event kinds.

## UI/Rust interface split — forbidden duplication
UI may rank presentation choices within a displayed candidate list for keyboard reach, but cannot re-rank or replace the authoritative Rust intent/slot interpretation with a different learned command. Rust may supply capability/risk/semantic role metadata but does not own colors, icons, menu animation or layout. Host remains the only effect authority.

## Acceptance contract
Same exact synthetic input+trusted source snapshot -> same native semantic candidate IDs/revision/phase independent of output completion order. Quiet/Assisted/X-ray must be source-equivalent; labels never inflate \`not_assessed\` to authorized; Tab cannot accept a stale candidate; UI preserves text and caret under IME and fast typing; no actual write, provider send or approval from the visual lab.
