# UI programmability is a first-class data system

The six-wave visual runtime and final-wave work are explicitly IN SCOPE.

Map the complete chain:
natural language/typed intent → command/schema validation → UI operation/IR → document/World model → transaction/CAS/revision journal → binding/data-flow projection → render/layout/accessibility → Windows/Android/Studio consumer.

Include component and template registries, node identities, data bindings, source references, graph object relations, undo/history, drafts, previews, diff, renderer contracts, publishing, device sync, offline state, migration, rollback, and World↔visual bridge.

Separate at least five concepts:
1. proposed UI command;
2. authoritative committed UI document;
3. read-only rendered projection;
4. Host approval/grant;
5. actual external effect and receipt.

These must NOT be conflated. Repos HQ indexes and relates all five, but does not inherit their write authority. TierFlow's document writer and Host's security/effect writer retain their roles unless an explicit verified migration changes them.

Scouts must identify old and new UI stores, overlapping writer paths, transformations and format loss, version pinning, serialization contracts, revision consistency, reader/consumer coverage, and device-specific projections. Label planned vs implemented vs runtime observed evidence. Preserve unsupported cases as refusals, never silent semantic loss.
