# VCL — Compositional Visual Expression SDK R001

Status: DESIGN CANDIDATE, not native implementation. Date 2026-11-25. Companion [axis registry](VISUAL-AXIS-REGISTRY-R001.json) and [241-candidate inventory](MASTER-VISUAL-INVENTORY.md). U09/U08/U10 under ARBOR #33 remain canonical engineering tracking.

## 1. Principle
One visual instruction set, arbitrary compositional expression, a single GPUI renderer, and Host sole authority over effects. The semantic compiler provides stable typed nodes, source ranges, relationships, alternatives and state. A visual compiler turns those into declarative expressions through a replaceable policy. An inert renderer resolves geometry and displays them. User events return typed draft edits or inspection intents to the existing compiler; they never directly execute commands.

## 2. Factorized grammar
VisualExpression = Target x Topology x Geometry x Stroke x Fill x Typography x Layout x Adornment x Interaction x Motion x State x Accessibility. Continuous parameters are bounded. The registry enumerates axes, not every product of axes. Expressions are reusable recipes with inheritance and sparse overrides. Validate capability, structural, budget, accessibility and state constraints before drawing. Examples: same border can target one word, multiple discontinuous words, sentence, nested clauses, entire composer, or outside edge. Solid/dashed/dotted/double/wavy are presentation, not intrinsic authorization semantics.

## 3. Semantic scene graph and addressability
SourceSnapshot: draft_id, actor_id, monotonic draft_revision, text_digest, catalog_revision, world_revision, scope_revision.
SemanticNode: stable id within snapshot; semantic role; source span set; optional resolved typed value, evidence, alternatives and state.
SemanticEdge: stable id; source/target node; kind (contains, argument_of, provider_of, account_of, attachment_of, depends_on, sequence, alternative, condition, negates, references, shared_target); provenance.
Group is a semantic relation, not necessarily a bounding rectangle. Permit nested, overlapping and discontinuous groups without mutating text. Explicit relations distinguish containment from execution order. "Email James through Gmail and attach report.pdf" may describe one draft operation with attachment argument or multiple dependent stages; compiler decides. No inference of a send merely from a visual group.
Text ranges are half-open UTF-8 byte ranges internally with validated grapheme boundaries for visual decoration. Explicit adapters convert UTF-16 offsets from existing surfaces; reject invalid/stale offsets. Shaped glyph layout maps logical spans to visual rectangles per line and bidi run. Reflow recomputes geometry, not semantics. Preserve original text, selection, IME and clipboard. Multi-line geometry modes: per-run, per-line fragments, joined contour, union hull (avoid including unrelated text), explicit connectors between disjoint fragments.

## 4. Compiler-facing Rust design sketch
Types below are illustrative API, not compiled or integrated.
```rust
struct RevisionFence {
    draft_id: DraftId, draft_revision: u64, text_digest: Digest,
    catalog_revision: Revision, world_revision: Revision,
    scope_revision: Revision,
}
enum Target {
    SpanSet(Vec<TextRange>), Node(NodeId), Group(GroupId),
    Relation(EdgeId), Surface(SurfaceId),
    Edge { surface: SurfaceId, side: Side, anchor: AnchorId },
}
struct VisualExpression {
    id: ExpressionId, target: Target, recipe: RecipeId,
    geometry: GeometrySpec, style: StylePatch,
    placement: PlacementSpec, layer: Layer,
    binding: Option<SemanticBinding>, interactions: Vec<InteractionSpec>,
    accessibility: AccessibilitySpec,
}
struct VisualFrame {
    fence: RevisionFence, schema_version: Version,
    expressions: Vec<VisualExpression>, diagnostics: Vec<Diagnostic>,
}
enum UiIntent {
    Inspect { node: NodeId },
    OpenChoices { slot: SlotId },
    StageChoice { slot: SlotId, choice: ChoiceId },
    CycleChoice { slot: SlotId, direction: Direction },
    Dismiss { expression: ExpressionId },
    ReorderDraft { group: GroupId, before: GroupId },
}
trait VisualPolicy {
    fn project(&self, semantic: &SemanticSnapshot, theme: &Theme,
        viewport: &Viewport) -> Result<VisualFrame, ProjectionError>;
}
trait VisualRenderer {
    fn layout(&self, frame: &VisualFrame, text: &ShapedText)
        -> Result<ResolvedScene, LayoutError>;
}
```
Events include frame fence, expression ID, target binding and event sequence; validate again against current draft and catalog. The host planner separately admits submitted semantic commands. No visual expression may contain arbitrary code, shell commands, URLs to execute, or executable callbacks. Compiler inputs may use semantic roles, not literal colors; policy maps to theme tokens. Colors can be personalized without changing command interpretation.

## 5. Spatial attachment engine
Treat the composer and each semantic node as anchor hosts with an ID, bounding geometry, clipping region, edge coordinates and available placement slots.
Anchor forms: inline, baseline, above/below range, group leading/trailing edge, editor inside/outside top/right/bottom/left, editor corner, gutter, docked panel, floating overlay.
Adornment = anchor + offset + alignment + component + binding + event contract + collision policy. Maintain deterministic ordering and lane allocation. Anchor stays bound during typing/reflow; stale or deleted targets detach; no jitter. Multi-adornment edges can stack, compact, overflow or move to inspector. Preserve cursor, focus and scroll. Phone uses same GPUI-rendered scene; touch/wheel intents return to corresponding control.

## 6. Interaction engine
Separate component shape from interaction pattern. A Gmail icon could be a decorative provider label, a button, a picker trigger, or a scoped account carousel. Supported gestures: tap/click, hover, keyboard, wheel, swipe, long press, drag, focus. Bind them to typed UI intents, not effects. Every gesture has an accessible alternative; wheel must not hijack normal composer scrolling. Account options must come from current authorized registry; show masked/display identity, no secrets. Switching account is a staged edit, causes re-interpretation, may invalidate a plan; never sends mail, signs in, or switches external account automatically. Provider selection and account selection are distinct typed slots. Prefer explicit focus/click to engage cycling. Escape cancels; undo reverses draft edit. Alternatives remain revision-bound.

## 7. Example semantics
Input: Email James through Gmail and attach report.pdf.
Semantic nodes: email action, James recipient, Gmail provider, unresolved/selected Gmail account, report.pdf file reference, attachment relation. Illustrative only; actual capability and accounts must be verified. Outer outline identifies command group. Solid outline around resolved recipient/action; dashed file wrapper if unresolved; connector joins attachment to email without implying a separate send. Gmail icon anchored outside composer right/top edge, linked to provider node; press to open scoped account picker, scroll while focused, confirm staged account. If no account registry, show unavailable with explanation, never fabricate choices.
Distinct state axes: interpretation, capability availability, authorization, execution, freshness. Policy assigns redundant cues; no green check to indicate completed action while still typing.

## 8. Renderer design and validation
Resolve geometry in order: source snapshot -> shaped runs -> target anchor bounds -> grouped contours -> relation routing -> adornment lanes -> collision handling -> hit regions -> render layers -> accessibility tree. Composite layers background/group fill/span fill/text/decorations/group strokes/relations/adornments/selection/caret/popovers. Ensure text and caret remain legible; handle long lines, bidi, ligatures, Unicode, emoji, IME, scale, theme, tiny viewports. Validate graph cycles only where relation forbids them; allow reference cycles. Bounded expression count, depth, span sets and overlays; deterministic fallback to text/inspector. Dirty-region incremental updates, cache by source/style/layout revision. No LLM per keystroke.
Test axis boundaries, pairwise covering arrays, targeted 3-way interactions (wrap x bidi x selection; edge x reflow x touch; stale x choice x approval), semantic journey tests and physical GPUI/phone acceptance. Test real command outputs and unavailable capabilities; illustrative design samples are not runtime receipts. No exhaustive Cartesian enumeration.

## 9. Integration sequence
1. Audit existing semantic spans/slot correction, Rust projection, GPUI text and VEC/TierFlow source; do not add competing parser or renderer.
2. Create Rust typed registry, validated expressions, version negotiation, source fences, unit tests.
3. Implement geometry resolver and span/wrapper/connector painter using GPUI shaped text, with reflow and cursor preservation.
4. Add generic anchor adornment layout and typed interaction dispatcher; first provider/account selector only after real catalogue support.
5. Add visual policy mappings and accessibility/density themes, then integrate existing AutoWiki.
6. Test compiler-to-native visual scene on real registered command journeys; compare physical phone viewport; add source-linked acceptance.
7. Expand recipe registry without changing command space. Policy maps command roles and states to visual expressions, not individual bespoke widgets.

Open decisions: exact crate placement after source audit, GPUI text-shaping API, style token palette, account registry contract and availability, geometry limits measured on workstation, semantic policy choices. No production source changes or runtime claims in this design.
