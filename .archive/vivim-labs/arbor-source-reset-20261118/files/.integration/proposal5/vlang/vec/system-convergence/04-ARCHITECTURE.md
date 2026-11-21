# One proposed end-to-end contract

```text
Developer source + opt-in comment Rust capsule (untrusted)
    -> HQ/SCF source identities + source digest
    -> independent SV verification + reviewed SDK module manifest
    -> Host installed capability catalog (versioned; NO private HQ graph)

User natural language / voice / gesture
    -> Host NLCL + known SDK grammar (deterministic)
    -> typed VEC Program Proposal (data only; no effect grant)
    -> Product Rust validator; optional HQKG scoped pure transformations
    -> Preview: approved Host snapshot -> TierFlow World/Visual projection
    -> Owner review + Host personal-program revision/trigger/journal
    -> Host effect dispatcher ONLY (if necessary) -> receipt
    -> TierFlow desktop + PWA view (same source/World/Program revision)
```

## Concrete user example
**"Every weekday morning, show unfinished tasks from my active project at the top of my phone."**
1. NLCL resolves known schedule, task query, project filter and UI view capabilities. Missing time/timezone/project or ambiguity must be clarified, never guessed. Missing SDK capability -> CAPABILITY_UNAVAILABLE, not fake result.
2. VEC emits a versioned typed Program IR referencing the installed scheduler/query/filter/view descriptors; no newly generated Rust code or model call if all grammar/capabilities are already known.
3. Product Rust validates a bounded *pure* transform DAG with data classes and risk. HQKG may compute scoped graph dependencies under authenticated caller inputs. Neither grants permissions.
4. TierFlow builds a responsive preview and declares data sources and permissions; no private source payload is stored in development HQ.
5. Host obtains review, stores the personal program in its existing owner-controlled source of truth and binds the approved clock/trigger. Current grants are rechecked on execution, and actual receipt state is projected, including stale/unknown/revoked.
6. On the next morning, deterministic compiled SDK components run locally; phone receives authorized revision projection. User saying "weekends too" edits the personal program graph; dragging the card edits the UI revision. Each has its own review and undo semantics.
7. If task summarization itself needs an LLM, that is a **model-dependent task step**, distinct from needing a model to reinterpret the identical schedule every day.

## The three IRs to keep distinct
- **VEC personal Program IR:** events, conditions, reads, transforms, *action proposals*, view bindings and lifecycle. Host-owned.
- **Rust pure Transform IR:** bounded executable computation under current caller constraints; validated but unprivileged.
- **TierFlow Visual IR:** World object and component projection, a11y/interaction, revisioned UI edits. Never action permit.

Existing IDs are linked through an explicit versioned adapter. No blanket "universal JSON DSL" should implicitly erase schema-specific meaning, risk or source permissions.

## Ownership table
| Thing | Exclusive authority | Allowed output |
|---|---|---|
| Development source/pinned capability definitions | HQ/SCF + independent SV | Reviewed manifest + evidence |
| Installed SDK and personal rules | Existing Host owner stores/catalog | Candidate program, grant/revision/receipt |
| Pure typed transforms | Product Rust/HQKG under scoped adapter | Data-only results and diagnostics |
| User-facing UI | Existing Host publisher + TierFlow inert renderer | Revision-bound visual projection |
| Browser provider send | Existing Studio/Host signed-in Chrome owner | Observed send-once receipt |
| Runtime security and rollback | Existing Host/native trust roots | Actual approvals, revocation and recovery |

No direct provider APIs, no arbitrary OS/network from capsules, no second scheduler/graph/publisher/World writer.