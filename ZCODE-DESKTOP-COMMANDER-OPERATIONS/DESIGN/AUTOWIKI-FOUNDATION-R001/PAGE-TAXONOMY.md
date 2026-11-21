# First canonical page set
Use canonical semantic IDs for links, not English slug as identity. Slugs are human-readable aliases and can change.

| Page ID | Page | Purpose | Primary child/link types |
|---|---|---|---|
| wiki.home | Home | what is this system, active state, what changed, where to go | all hubs, recent changes |
| wiki.system | System | one Rust graph fabric, execution boundaries, major expressions | runtime, capabilities, data, command |
| wiki.capabilities | Capabilities | what can currently be done vs declared/blocked | capability detail, operations, tests |
| wiki.knowledge | Knowledge | concepts, entities, sources, evidence, relationships | concept detail, source detail |
| wiki.commands | Commands | what user can ask; preview/approval/effect status | command detail, capability, permission |
| wiki.activity | Activity | recent verified changes, revision, receipts, pending unknowns | event detail, source, task |
| wiki.development | Development | current integration state, tracker, gaps and proposed work | milestone, file, verification |

Each hub contains: 1-sentence definition; 1-3 short explanatory paragraphs; 3-7 meaningful links; source-backed key facts; current coverage/status; last source revision; drill-down links; explicit unknowns. Empty hubs display a truthful scaffold rather than invented content.

## Page anatomy
Title + summary + 'How it fits' + Key facts + Connected concepts + Available commands (no implicit execution) + Sources + Revision/quality footer.
Provide 'human', 'semantic', 'trace' views. Human is the default. Semantic exposes typed claims and references. Trace exposes compact compiler notation and raw source IDs for developers only.

## Example
### Commands
Commands are the ways people and authorized agents request actions through ARBOR. A command is first interpreted as a typed operation. Only a registered, wired and permitted operation may progress to an approved effect.

Related: [Capabilities](wiki.capabilities), [System](wiki.system), [Activity](wiki.activity).
This paragraph is illustrative copy, not evidence that any specific command currently executes.

## Incremental update
Typing creates ephemeral, revision-keyed suggestions: no durable wiki writes from partial keystrokes. Stable/evidenced knowledge is committed only via the existing authorized revision pipeline. Pages reflect accepted revisions, while speculative cards remain visibly distinct and expire when superseded.
