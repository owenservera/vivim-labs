# Shared engines must compose without duplicated computation

Authoritative machine DAG and property owners: machine/engine-dependencies.v1.json.
Dependencies flow from a consuming engine to the typed supplier. This graph was checked acyclic at authoring.

Examples: Paint consumes Color and Assets; Border consumes Color; Theme consumes Color/Typography/Spacing/Corners;
Charts consume Data/Color/Typography; Forms consume Input/Focus/States. These dependencies are typed, not imports
that secretly grant network, filesystem or Host data access.

## Configuration cascade
- 100: default
- 200: brand
- 300: theme
- 400: workspace
- 500: component-variant
- 600: component-state
- 700: owner-workspace
- 800: owner-component
- 900: accessibility-user
- 1000: system-safety

System safety and accessibility are hard constraints, not cosmetic lowest-priority tokens.
Multiple same-layer writers of one canonical property must produce a conflict refusal, not silent last-writer wins.
Explicit owner change requires review and exact current Host revision. A UI undo cannot reverse an external send.

## Token model
Use explicit token types, nested groups, full-value aliases, type matching, cycle and unknown-reference refusal.
Mode overlays and complete DTCG 2025.10 support remain worker deliverables; do not claim current implementation is complete.
Theme orchestration resolves typed source tokens; Color still owns color space/conversion and Border stroke properties.

## Dependency acceptance
The approved DAG is a design scaffold. Agent A012 must produce actual executable graph resolver, cycle/refusal tests
and source compatibility proof. No automatic live engine loading happens from these edges.
