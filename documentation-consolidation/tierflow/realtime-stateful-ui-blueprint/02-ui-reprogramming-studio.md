# Real-Time Stateful UI Reprogramming Studio

**Status:** normative greenfield architecture specification  
**Scope:** OS-agnostic mobile UI studio, declarative UI documents, safe editing, rendering, preview, publication, extensions, and synchronization  
**Transport assumption:** the mobile app reaches the host through Tailscale; connectivity and device enrollment are already established  
**Examples:** JSON examples are illustrative contracts, not a wire-final dependency

## 1. Normative language and goals

The key words **MUST**, **MUST NOT**, **SHOULD**, and **MAY** are to be interpreted as normative requirements.

The system MUST let an authorized user reprogram a running UI from the mobile UI itself without shipping or evaluating mobile application code. It MUST provide:

- a native, OS-agnostic mobile renderer and visual editor;
- an authoritative, versioned UI document and independent real-time application state;
- safe drafts, live preview, validation, atomic publication, and rollback;
- extensibility without granting user logic mobile-code execution, filesystem, or network authority;
- deterministic recovery after disconnects, duplicate commands, stale edits, and process restarts.

Tailscale provides a private, encrypted network path. It MUST NOT be treated as sufficient application authentication, document authorization, or command authorization.

Non-goals include arbitrary remote mobile-code execution, unconstrained plug-in loading, direct mobile writes to authoritative state, and treating a rendered preview as proof that publication occurred.

## 2. Architecture and authority

### 2.1 Actors

| Actor | Normative responsibility | Authority boundary |
|---|---|---|
| Mobile renderer/editor | Renders registered native components, edits declarative documents, displays preview, and submits commands | No arbitrary code evaluation; no authoritative UI or state mutation |
| TypeScript control plane | Authenticates sessions, authorizes capabilities, manages drafts and packages, orchestrates validation/preview, and exposes APIs | Cannot bypass Rust revision checks or directly publish around the authoritative log |
| Rust state/event-log service | Owns authoritative application state, published UI revisions, commands, revision checks, event ordering, atomic publication, and audit records | Sole writer of authoritative state and publication pointer |
| Pure WASM user logic | Computes deterministic, side-effect-free results for preview or approved host invocation | No network, filesystem, ambient time, randomness, native rendering, or direct state/command authority |
| Trusted host extensions | Versioned, explicitly installed host integrations or native-component implementations | Separate administrative trust class; never represented as untrusted user WASM |

The TypeScript control plane MAY validate and prepare work, but the Rust service MUST perform the final compare-and-publish operation. Authoritative application events and UI-publication events MUST share one ordering domain or carry explicitly related, monotonically increasing sequence numbers so clients cannot observe an ambiguous ordering.

### 2.2 Separate revision domains

The system MUST distinguish:

- `ui_revision`: monotonically increasing published revision of one UI document;
- `state_sequence`: monotonically increasing authoritative application event position;
- `draft_revision`: monotonically increasing revision within a draft;
- `registry_revision`: immutable component-registry version or digest;
- `extension_version` and `module_hash`: immutable extension identity.

A state update MUST NOT silently change `ui_revision`. A UI publication MUST NOT silently mutate application state. A rendered frame SHOULD identify both the UI revision and state sequence from which it was derived.

### 2.3 Trust and privilege separation

Every API request MUST be authenticated and authorized at the control plane and again at the authoritative mutation boundary when a capability is security-sensitive. Capability checks MUST include actor, document or resource scope, operation, and current policy. Network location, node possession, and knowledge of an identifier MUST NOT confer authority.

User-authored WASM and trusted host extensions MUST use different installation flows, permission models, and audit categories. Installing a trusted host extension MUST require an extension-administrator capability and MUST NOT be possible through a normal UI draft or command.

## 3. Declarative UI document

### 3.1 Document identity and compatibility

A UI document MUST contain a document identifier, document schema version, component-registry reference, root node, extension declarations, localization resources or references, and metadata needed for compatibility. Publication metadata such as `ui_revision`, content hash, actor, and timestamp MAY be carried in an envelope rather than embedded in editable content.

The following versions MUST remain independent:

1. envelope/protocol version;
2. UI document schema version;
3. component-registry version;
4. application state schema version;
5. extension manifest and WASM ABI versions;
6. mobile renderer compatibility version.

A renderer MUST reject or safely degrade a document requiring a newer mandatory feature. It MUST NOT guess semantics from an unknown version.

### 3.2 Node model

Each UI node MUST have:

- an `id`: an opaque, document-unique identifier generated by the service or a cryptographically strong client generator;
- a `type`: a component type registered in the pinned registry;
- `props`: data conforming to that component's property schema;
- ordered `children` or named `slots`, according to the component contract;
- optional accessibility, visibility, and extension metadata constrained by schema.

Node IDs MUST be treated as indivisible tokens. Clients and extensions MUST NOT derive hierarchy, array position, type, authorization, persistence keys, or business meaning from an ID. References MUST use node IDs, never child indexes or display labels. Reordering nodes MUST preserve their IDs and associated local interaction state.

The node graph MUST be finite and acyclic. Every child and slot reference MUST resolve exactly once, every non-root node MUST have exactly one parent, and all nodes MUST be reachable from the declared root. Component-specific slot cardinality and permitted child types MUST be enforced.

### 3.3 Closed component semantics with controlled extensibility

The mobile runtime ships a closed set of native component semantics, for example layout, text, icon, input, selection, list, navigation, dialog, chart, progress, and command controls. A component type MUST have one versioned meaning across operating systems, even when its native presentation differs.

Extensibility is permitted only through:

1. **composition** of existing registered components into declarative nodes;
2. **trusted native components** explicitly installed with a host/renderer release and exposed through a versioned registry;
3. **pure WASM computation** returning typed data or declarative proposals that are then validated.

WASM MUST NOT define native component behavior, inject platform widgets, supply executable UI markup, or bypass the component registry. Unknown component types MUST render an accessible, non-interactive unsupported-component placeholder and emit a diagnostic; they MUST NOT be interpreted as another type by naming similarity.

The registry MUST define, per component version:

- property and event schemas, defaults, and deprecation policy;
- allowed slots, child types, and cardinality;
- accessibility role and required accessible-name rules;
- supported state-binding types and command events;
- responsive behavior, focus behavior, and loading/empty/error states;
- minimum renderer version and migration functions where applicable.

A published document MUST pin an immutable registry digest. Registry upgrades MUST be explicit migrations, not silent semantic changes.

### 3.4 Illustrative UI document

```json
{
  "schema_version": "1.0.0",
  "document_id": "doc_01JYX7N4G4Q2PZK8WQ6W0M7K2D",
  "registry": {
    "name": "core-mobile",
    "version": "1.4.0",
    "digest": "sha256:44ee3c43d1740f0bd378b14c6b7ea04abdc2db07bb0c3de3fa63492e9910"
  },
  "default_locale": "en",
  "fallback_locale": "en",
  "root": "n_01JYX7N8Q7V3W5M6P2Q9Z0N4K8",
  "nodes": [
    {
      "id": "n_01JYX7N8Q7V3W5M6P2Q9Z0N4K8",
      "type": "core.vertical_stack@1",
      "props": {
        "spacing": "space.4",
        "padding": "space.4",
        "content_width": { "min": 0, "preferred": 640, "max": 960 }
      },
      "children": [
        "n_01JYX7NCQ5B8G0A3T4R7E2H1J6",
        "n_01JYX7NGJ9C2D6F8H1K3M5P7R4",
        "n_01JYX7NM2S8V6W4X1Y3Z5A7B9C"
      ]
    },
    {
      "id": "n_01JYX7NCQ5B8G0A3T4R7E2H1J6",
      "type": "core.text@1",
      "props": {
        "text": { "message": "dashboard.title" },
        "style": "title.large"
      },
      "a11y": { "role": "header" },
      "children": []
    },
    {
      "id": "n_01JYX7NGJ9C2D6F8H1K3M5P7R4",
      "type": "core.metric@1",
      "props": {
        "label": { "message": "temperature.label" },
        "value": {
          "bind": "/rooms/byId/room_kitchen/temperature_c",
          "type": "number",
          "fallback": null,
          "format": { "kind": "temperature", "unit": "celsius", "precision": 1 }
        },
        "availability": {
          "bind": "/rooms/byId/room_kitchen/online",
          "type": "boolean",
          "fallback": false
        }
      },
      "a11y": { "live": "polite", "value_message": "temperature.value" },
      "children": []
    },
    {
      "id": "n_01JYX7NM2S8V6W4X1Y3Z5A7B9C",
      "type": "core.button@1",
      "props": {
        "label": { "message": "refresh.action" },
        "variant": "primary",
        "confirmation": { "message": "refresh.confirm" }
      },
      "events": {
        "activate": {
          "kind": "command",
          "name": "environment.refresh",
          "payload": {
            "room_id": { "literal": "room_kitchen" }
          },
          "required_capability": "environment.refresh",
          "pending_message": "refresh.pending"
        }
      },
      "a11y": { "hint": "refresh.hint" },
      "children": []
    }
  ],
  "extensions": [
    {
      "id": "example.temperature-formatter",
      "version": "1.0.0",
      "module_hash": "sha256:fbffdc213b04b6d8718d4de3bb1c1f41c307bfaafc0d30f5449280643d9f",
      "entrypoint": "format_temperature",
      "required": false
    }
  ],
  "messages": {
    "en": {
      "dashboard.title": "Environment",
      "temperature.label": "Kitchen temperature",
      "temperature.value": "Kitchen temperature {value}",
      "refresh.action": "Refresh sensor",
      "refresh.confirm": "Request a new sensor reading?",
      "refresh.pending": "Refresh requested",
      "refresh.hint": "Requests one reading from the kitchen sensor"
    }
  }
}
```

### 3.5 State bindings

Bindings MUST be typed, declarative data references, not expressions evaluated as JavaScript. A binding MUST specify a normalized JSON Pointer or equivalent constrained path, expected type, fallback behavior, and optional registered formatter. State addressed by a binding MUST be explicitly exposed to the UI projection; arbitrary host objects, secrets, environment values, and internal event-log records MUST NOT be bindable.

The host MUST enforce binding read scopes per document and actor. Collection paths MUST avoid unbounded projections. Missing, stale, unavailable, and permission-denied values MUST have distinct presentation semantics and MUST NOT be converted into a plausible zero or false value.

Transformations MUST use registered deterministic functions with versioned input/output schemas. An optional extension failure MUST produce the declared fallback or an explicit error state; a required extension failure MUST make the candidate invalid or the affected region unavailable.

Local interaction state such as text being composed, focus, scroll offset, and expanded disclosures MUST remain local unless explicitly modeled as shared application state. It MUST NOT leak into authoritative application events merely because a UI revision changed.

## 4. Mobile rendering and editing

### 4.1 Renderer

The renderer MUST interpret the document using native registered components. It MUST NOT use `eval`, dynamically loaded JavaScript, downloaded native libraries, arbitrary WebView script, or equivalent mechanisms to realize user logic.

Rendering MUST be deterministic for a given document, registry, state projection, locale, viewport, and platform capability set. It MUST preserve node identity across updates, virtualize large collections, constrain nesting and layout work, and isolate extension failures to the smallest safe region. Dynamic content MUST NOT be able to spoof trusted system chrome, authentication prompts, or another document's controls.

The renderer SHOULD diff by opaque node ID and apply state updates without rebuilding unrelated regions. Any optimistic display MUST be visibly marked pending and reconciled against the authoritative event or command outcome.

### 4.2 Studio editor

The studio MUST expose structural and property editing through native controls: add, remove, duplicate, reorder, reparent, slot assignment, property editing, binding selection, command wiring, localization editing, and responsive/a11y inspection. Advanced users MAY edit JSON, but the same validation and transaction path MUST apply.

The editor MUST NOT offer capabilities the actor cannot legally publish as though they were available. Unavailable features MAY be visible for discoverability but MUST be identified as requiring additional capability.

All edits MUST target a draft. Direct mutation of the published document is prohibited. Autosave MUST be server-persisted, and the client MUST display draft revision, base published revision, validation state, stale/conflict state, and whether the visible canvas is draft preview or published reality.

## 5. Drafts, preview, and publication

### 5.1 Two-phase draft/preview/commit model

Publication consists of a preparation phase and an authoritative commit phase. This is not a claim of distributed two-phase commit across mobile, TypeScript, and Rust.

**Phase 1 — draft and prepare:**

1. Create a draft from published `base_revision` and immutable registry digest.
2. Apply one or more atomic draft operations; increment `draft_revision` for each accepted transaction.
3. Validate the exact candidate content and extension artifacts.
4. Execute deterministic preview computations within resource limits.
5. Return a short-lived `prepared_token` bound to actor, document, base revision, draft revision, candidate hash, registry digest, extension hashes, validation results, and expiry.

**Phase 2 — commit:**

1. The user with publish capability explicitly requests publication with the prepared token, expected base revision, expected draft revision, candidate hash, and an idempotency key.
2. The Rust service verifies that the current published revision still equals the base revision and that policy and artifacts remain valid.
3. Security-sensitive validation is repeated at commit; a preview token is not authorization.
4. The new immutable revision, publication-pointer update, audit record, and outgoing notification record are committed in one authoritative transaction.
5. Only after commit does the service acknowledge the new `ui_revision` and distribute it.

If any part of publication fails, no partial publication, audit-only “success,” or pointer-only update is permitted. Prepared tokens MUST be single-use except for idempotent retries of the identical commit request.

### 5.2 Draft transactions

Draft operations SHOULD use explicit operations such as `node.add`, `node.remove`, `node.move`, `props.set`, `binding.set`, `event.set`, and `messages.set`. Each transaction MUST include `draft_id`, expected `draft_revision`, transaction ID, actor, operations, and client timestamp for diagnostics only.

A transaction MUST be applied atomically and deterministically in declared operation order. Rejected transactions MUST leave the draft unchanged. Transaction IDs MUST be idempotency keys: a retry with identical content returns the original result; reuse with different content is an error.

Drafts MUST expire according to policy but MUST NOT silently disappear while represented as saved. Export or recovery of a superseded draft SHOULD remain possible until retention expiry.

### 5.3 Live preview

Preview MUST execute against the exact candidate hash, not whichever draft happens to be newest. It MAY subscribe to live read-only state projections, but the preview MUST label state sequence and stale status. If the draft changes, the service MUST invalidate the previous preview session or explicitly rebase it to a new preview revision.

Preview MUST use an ephemeral interaction namespace. By default, command controls MUST be disabled or simulated. If command testing is enabled, it MUST use a separately authorized sandbox target with no production side effects. Preview MUST NEVER issue production commands merely to render, validate bindings, run WASM, or measure a component.

A second editor MAY open a read-only or explicitly shared preview. Private drafts and their state projections MUST NOT be broadcast to subscribers lacking draft-read capability.

### 5.4 Optimistic concurrency and conflicts

A commit based on a stale `base_revision` MUST fail with a concurrency conflict and return the current revision plus enough metadata to begin rebase. It MUST NOT overwrite the newer publication.

Rebase SHOULD use structural three-way merge over base, local draft, and current published content:

- independent additions and edits to disjoint nodes MAY merge automatically;
- edits to the same scalar property conflict unless values are identical;
- delete versus edit conflicts MUST require explicit resolution;
- competing moves of the same node MUST conflict;
- order changes MUST be represented independently of node identity where possible;
- registry, schema, extension, capability, or localization changes MUST be revalidated after rebase.

Automatic merge MUST NOT guess intent for command payloads, required capabilities, accessibility-critical properties, or visibility rules. Conflict resolution creates a new draft revision with a new base revision; it does not rewrite either history.

### 5.5 Rollback

Rollback MUST create and publish a new revision whose content is derived from a selected historical revision. It MUST NOT delete revisions, move the publication pointer backward, reuse an old revision number, or erase subsequent audit history.

The rollback candidate MUST pass current schema, semantic, security, registry, and extension validation or go through an explicit migration. Its base revision is the revision current when rollback preparation begins, so ordinary conflict rules still apply. The audit record MUST identify the source revision, actor, reason, and superseded revision.

## 6. Action and command flow

### 6.1 Command envelope

A UI event MAY declare a versioned command name and a schema-constrained payload assembled from literals and typed bindings. It MUST NOT contain an arbitrary script, SQL, shell fragment, native symbol, URL to invoke, or arbitrary Rust/TypeScript method name.

Each submitted command MUST include at least:

- a cryptographically random `command_id` generated before the first send;
- command name and schema version;
- actor/session identity supplied by the authenticated channel, not trusted from payload;
- source document, published UI revision, and node ID for diagnostics;
- typed arguments and required capability context;
- optional expected application preconditions;
- idempotency payload hash.

The client MUST persist command IDs in its outbox until an authoritative outcome is observed. The host MUST persist deduplication records with the outcome for at least the maximum client retry/retention window.

### 6.2 Authoritative processing

For each command, the control plane MUST authenticate, authorize, validate schema and size, and route to the Rust authority. The Rust service MUST then:

1. reject an unknown command, invalid payload, insufficient capability, or failed precondition;
2. check durable idempotency state for `command_id`;
3. return the original result for an exact duplicate;
4. reject reuse of the same `command_id` with a different payload hash;
5. perform current-state business validation and capability checks;
6. atomically apply the state transition, append events, record the result, and enqueue notifications;
7. return a stable accepted/completed/rejected outcome with the resulting state sequence when available.

At-most-once authoritative application of a command MUST NOT depend on the mobile connection remaining alive. Acknowledgement loss MUST be recoverable by querying the original command ID. Retries MUST use the same ID and payload, never a fresh ID.

Pure WASM MAY propose a typed command intent, but it MUST NOT execute it. Any proposed intent MUST pass the same schema, authorization, precondition, and idempotency path as a user-originated command.

## 7. Extensions and pure WASM

### 7.1 Extension classes

**Sandboxed user logic** is content-addressed WASM executing in a restricted host. It MUST be deterministic and pure: identical canonical input, ABI version, and resource policy produce identical output or a deterministic resource-limit result. It MUST NOT receive ambient clock, random seed, network, filesystem, environment, process, platform UI, or mutable global host state.

The host MUST provide deterministic inputs when time or randomness is required as data. Resource exhaustion MUST terminate the invocation and mark the affected validation/preview/result as failed; it MUST NOT be converted into a partial successful value.

**Trusted host extensions** run outside that purity guarantee and MAY integrate with explicitly granted host services. They MUST be versioned, signed or otherwise provenance-verified, administratively installed, permission-scoped, and independently audited. Their APIs MUST NOT be implicitly exposed to user WASM.

### 7.2 Verification and execution

Before use, the host MUST verify module hash, size, manifest schema, ABI compatibility, signatures/provenance where required, imports, exports, memory declarations, and prohibited instructions or capabilities. Only declared deterministic host imports MAY be linked. WASI or any general network/filesystem capability MUST NOT be granted to sandboxed user logic.

Each invocation MUST have bounded fuel/instructions, execution time, memory, stack, input/output bytes, and call depth. Outputs MUST be canonical typed data validated against the declared schema. Floating-point behavior, collection ordering, and other sources of nondeterminism MUST be constrained by the ABI or excluded.

A required module MUST be replicated or durably available before publication can depend on it. A document and preview MUST pin the exact module hash; version labels alone are insufficient.

### 7.3 Illustrative extension manifest

```json
{
  "manifest_schema": "1.0.0",
  "extension_id": "example.temperature-formatter",
  "version": "1.0.0",
  "class": "sandboxed-pure-wasm",
  "module": {
    "url": "content://sha256/fbffdc213b04b6d8718d4de3bb1c1f41c307bfaafc0d30f5449280643d9f",
    "sha256": "fbffdc213b04b6d8718d4de3bb1c1f41c307bfaafc0d30f5449280643d9f",
    "bytes": 18342
  },
  "abi": {
    "name": "ui-pure-logic",
    "version": "1.0.0",
    "determinism": "strict"
  },
  "entrypoints": [
    {
      "name": "format_temperature",
      "input_schema": "example.temperature-input@1",
      "output_schema": "core.localized-text@1",
      "limits": {
        "fuel": 1000000,
        "memory_bytes": 8388608,
        "input_bytes": 4096,
        "output_bytes": 4096
      }
    }
  ],
  "permissions": [],
  "compatibility": {
    "document_schema": ">=1.0.0 <2.0.0",
    "registry": "core-mobile@1"
  }
}
```

An empty `permissions` list is meaningful: this module receives no host capabilities. Declaring a permission in a manifest MUST NOT grant it; policy and an authorized administrator MUST independently grant any non-default capability.

## 8. Validation, migration, and versioning

### 8.1 Validation layers

Every draft prepare, publish, migration, and rollback MUST perform the applicable layers below. JSON Schema alone is not sufficient.

1. **Envelope and structural schema:** required fields, types, string/byte limits, no unrecognized security-sensitive fields, valid versions and hashes.
2. **Graph semantics:** unique opaque IDs, one root, resolved references, no cycles or orphans, valid parent/child and slot rules.
3. **Registry semantics:** component versions, props, defaults, event types, renderer compatibility, and deprecation constraints.
4. **State semantics:** exposed paths, expected types, formatter signatures, collection limits, fallback behavior, and read scopes.
5. **Command semantics:** known commands, payload schemas, required capabilities, confirmation policy, and prohibited preview side effects.
6. **Extension semantics:** module availability and hash, ABI, imports/exports, purity restrictions, entrypoint types, and resource limits.
7. **UX semantics:** accessible names/roles, focus order, contrast and target requirements, localization coverage/fallbacks, responsive constraints, and nonvisual alternatives.
8. **Policy and abuse controls:** actor permissions, document quotas, package quotas, command policy, and administrative restrictions.

Validation diagnostics MUST be machine-coded and human-readable, reference the node ID and property/path, distinguish errors from warnings, and avoid exposing secret state. Publication MUST fail on every error. Warnings MUST NOT silently become ignored errors; exceptions require an explicit, auditable policy decision.

The prepare result MUST include a candidate hash calculated over canonical content and all semantic dependencies. Commit MUST reject a mismatch between the validated and submitted candidate.

### 8.2 Migration

Migrations MUST be explicit, deterministic, version-to-version transformations executed by the trusted host on a copy of immutable content. They MUST NOT execute on the mobile renderer during ordinary display. A migration chain MUST declare source and target versions, preserve opaque node IDs and audit provenance, and finish with full validation at the target version.

Migration MUST NOT silently discard unknown nodes, command capabilities, bindings, accessibility metadata, or extension requirements. If loss is unavoidable, migration MUST fail or require an explicit reviewed loss declaration.

Published historical revisions remain immutable. Migrating an old document for editing or rollback creates a new candidate and eventually a new revision. A renderer that cannot support the current required schema MUST display a safe unavailable state rather than an approximation.

Backward-compatible schema additions SHOULD use optional fields with defined defaults. Breaking changes to component behavior, binding types, command semantics, or ABI MUST use a new major/versioned contract.

## 9. Real-time synchronization and reconnect

Clients SHOULD receive revisioned updates over an authenticated stream, with a snapshot/delta fallback. Every update MUST identify document or projection, revision/sequence, and predecessor when delta-based. Clients MUST discard stale duplicates and MUST detect gaps.

On reconnect, the client sends its last committed `ui_revision`, last applied `state_sequence`, active draft ID/revision, preview token if any, and outbox command IDs. The service MUST respond according to these rules:

- send deltas only if the complete requested range is available; otherwise send a fresh snapshot and new baseline;
- never ask the client to infer missing events;
- resolve the current published revision independently from draft edits;
- mark a preview stale if its candidate, base revision, policy, registry, or required extension changed;
- return durable command outcomes for outstanding IDs; an unknown outcome means retry with the same ID or explicitly mark it uncertain, never generate a new ID automatically;
- preserve server-saved drafts, while re-running conflict and policy checks before further editing or publication;
- resubscribe with the actor's current capabilities and remove projections no longer authorized.

An offline editor MAY queue structural edits, but it MUST label them as based on a potentially stale revision and reconcile through normal draft transactions. Offline display MUST NOT imply that a command or publication was accepted.

## 10. Roles, capabilities, and audit

### 10.1 Minimum role model

Roles are convenience bundles; authorization MUST ultimately be capability- and resource-scoped.

| Role | Typical capabilities |
|---|---|
| Viewer | Render published documents and read permitted state projections |
| Operator | Viewer capabilities plus explicitly allowed operational commands |
| Editor | Create/edit drafts and run non-side-effecting previews |
| Publisher | Publish prepared candidates and create rollback revisions within scope |
| Extension administrator | Install, grant, disable, and audit extensions; manage trusted host integrations |
| Auditor | Read revision, command, and audit history according to data policy |

Draft read/write, publish, command execution, state projection, extension installation, and audit export MUST be separable capabilities. An editor MUST NOT gain publish authority by possessing a prepared token. Delegation, revocation, and expiry MUST be enforced at prepare and again at commit/execution.

### 10.2 Audit history

The authoritative audit log MUST record:

- actor and authenticated session/proxy identity as applicable;
- operation, resource scope, capabilities evaluated, and policy version;
- base and resulting revisions, draft/transaction/command IDs, and candidate/content hashes;
- registry digest, extension hashes, validation/migration versions, and diagnostics summary;
- trusted timestamp, result, reason, correlation/causation IDs, and rollback source where relevant;
- source network metadata only according to privacy policy.

Audit history MUST be append-only and tamper-evident or tamper-resistant according to deployment requirements. It MUST NOT contain raw secrets, tokens, private state payloads, or sensitive localization/user content unless explicitly classified and protected. Publication, rollback, extension changes, authorization failures, and command outcomes MUST be auditable without relying on mobile logs.

## 11. Accessibility, localization, and responsive behavior

These requirements are validation and release criteria, not optional renderer enhancements.

### 11.1 Accessibility

- Every interactive control MUST have a programmatic accessible name, role, state, and action; icon-only controls MUST use localized labels.
- Meaningful text MUST meet WCAG AA contrast: at least 4.5:1 for normal text and 3:1 for large text; meaningful non-text UI indicators MUST meet at least 3:1 against adjacent colors.
- Touch targets SHOULD be at least 44 by 44 logical points or the platform-equivalent accessible minimum, with adequate separation.
- Focus order MUST be logical and explicitly derivable from structure; focus MUST remain stable by node ID across state updates and be restored sensibly after deletion or navigation.
- Screen readers, keyboard/switch navigation, platform text scaling, reduced motion, high contrast, and non-color-only status cues MUST be supported.
- Dynamic status updates MUST use appropriate polite/assertive announcements without repeatedly announcing high-frequency values.
- Validation MUST detect missing names, invalid roles, inaccessible custom composition, hidden-focusable content, and motion without reduced-motion alternatives.

### 11.2 Localization

- User-visible strings MUST use message keys and locale resources; node IDs and command names MUST NOT contain localized display text.
- Messages SHOULD use an ICU-compatible, versioned message format with typed placeholders and plural/select rules.
- Locale fallback MUST be explicit, for example regional locale to base locale to document fallback. Missing required messages MUST produce validation diagnostics, not blank controls.
- Layout MUST support text expansion, line wrapping, bidirectional text, locale-aware number/date/unit formatting, and mirrored directional affordances where appropriate.
- A binding or extension MUST NOT return unlocalized markup that bypasses message and accessibility policy.

### 11.3 Responsive behavior

Layout MUST respond to available logical width/height, orientation, safe areas, text scale, and input capabilities rather than OS name or device model. Breakpoints and constraints MUST be declarative and testable. Content MUST remain operable at narrow widths, large text, split-screen sizes, and platform insets without horizontal trapping or clipped primary actions.

Components MUST define minimum/maximum content constraints and overflow behavior. Lists MUST define loading, empty, error, and large-result behavior. No essential action MAY depend only on hover, fine pointer precision, color, or animation.

## 12. Limits and abuse controls

The service MUST enforce configurable limits before parsing expensive content, linking modules, running layout, or executing commands. The following are safe initial defaults; deployments MAY tighten them and MUST expose effective values to the studio.

| Resource | Initial default limit |
|---|---:|
| Canonical UI document size | 1 MiB |
| Nodes per document | 10,000 |
| Graph depth | 64 |
| Children per node | 1,024 |
| Draft operations per transaction | 500 |
| Command payload | 64 KiB |
| Binding result per node | 256 KiB |
| Active private drafts per actor | 20 |
| Prepared-token lifetime | 5 minutes |
| Sandboxed WASM module size | 2 MiB |
| WASM linear memory per invocation | 64 MiB maximum |
| WASM input or output per invocation | 1 MiB maximum |
| WASM invocation fuel | Entrypoint-specific and mandatory |
| Preview computations | 10 per second per actor, bounded concurrency |
| Commands | Per-command policy; default 30 per second per actor with bounded burst |

Additional controls MUST include bounded concurrent drafts/previews/WASM instances, queued work with backpressure, maximum event replay and snapshot sizes, command-specific rate limits, duplicate suppression, payload/schema complexity limits, and cancellation on disconnect where work has no durable purpose.

Limits MUST NOT be enforced only by the mobile UI. Oversized or excessive requests MUST receive deterministic, machine-readable errors. Abuse controls MUST NOT cause partially applied transactions or ambiguous command execution. Trusted host extensions MUST have separate resource and privilege quotas.

## 13. Edit–preview–publish sequence

```mermaid
sequenceDiagram
    autonumber
    actor U as Authorized user
    participant M as Mobile studio/renderer
    participant T as TypeScript control plane
    participant W as Pure WASM sandbox
    participant R as Rust authority/event log

    U->>M: Edit UI from current published document
    M->>T: Create draft(base_revision, registry_digest)
    T->>R: Read immutable base and actor policy
    R-->>T: Base content + ui_revision + capabilities
    T-->>M: draft_id + base content + draft_revision

    loop Atomic editing transactions
        M->>T: Operations(draft_id, expected_draft_revision, transaction_id)
        T->>T: Authorize, validate envelope, apply or deduplicate
        T-->>M: New draft_revision or conflict/error
    end

    M->>T: Prepare(candidate_hash, base_revision, draft_revision)
    T->>T: Schema + graph + registry + policy validation
    T->>W: Deterministic entrypoints(canonical typed input, limits)
    W-->>T: Typed output or deterministic failure
    T->>T: Semantic, a11y, localization, responsive validation
    T-->>M: Preview artifacts + short-lived prepared_token
    M-->>U: Exact candidate preview; production commands disabled/simulated

    U->>M: Publish
    M->>T: Commit(prepared_token, expected_base, candidate_hash, idempotency_key)
    T->>R: Authorized commit request with full dependency hashes
    R->>R: Recheck policy + current ui_revision == base + critical validation
    alt Commit succeeds
        R->>R: Atomically append revision + audit + notification; swap pointer
        R-->>T: Committed new ui_revision
        T-->>M: Committed new ui_revision
        R-->>M: Sequenced publication update
        M-->>U: Published reality replaces draft preview
    else Stale base or validation/policy failure
        R-->>T: Conflict/error; no publication mutation
        T-->>M: Current revision and rebase/revalidation details
        M-->>U: Resolve conflicts; published UI remains unchanged
    end
```

## 14. Failure semantics

The following distinctions MUST be preserved in APIs and UI:

- **validated:** candidate passed the stated checks but is not published;
- **prepared:** immutable candidate and dependencies are ready for commit;
- **committed:** authoritative transaction completed and new revision exists;
- **displayed:** a particular client rendered a revision, possibly stale;
- **command accepted/completed:** authoritative command outcome exists;
- **command uncertain:** connection failed before outcome was learned; reconciliation is required.

Process restart MUST recover published content, drafts, idempotency records, audit history, and event sequence from durable storage. A cached preview or prepared token MUST NOT be treated as durable authority. Notification delivery MAY be repeated, so every consumer MUST deduplicate by revision, sequence, transaction, or command ID.

## 15. Acceptance criteria

The architecture is acceptable only when all of the following are demonstrated with automated tests plus platform accessibility checks where noted:

1. **Native closed rendering:** a document containing script, an unknown component, or a downloaded-code reference is rejected or rendered as an accessible unsupported region; no mobile code evaluation occurs.
2. **Opaque identity:** reordering, moving, or restyling nodes preserves IDs and local focus/interaction state; no component relies on ID contents or child indexes.
3. **Independent revisions:** application state updates do not change `ui_revision`, and UI publication does not create application side effects.
4. **Draft isolation:** edits and preview are invisible as published UI to ordinary viewers and cannot issue production commands by default.
5. **Exact preview:** preview hash, registry digest, and extension hashes match the candidate submitted for commit; changing any dependency invalidates preparation.
6. **Atomic publication:** injected failure at revision write, pointer update, audit write, or notification enqueue leaves either the complete new committed revision or the complete old revision, never a partial state.
7. **Optimistic concurrency:** two publishers using the same base cannot both succeed; the stale commit receives a conflict and the newer published content is not overwritten.
8. **Structural conflict handling:** disjoint edits can merge, while same-property edits, delete/edit, competing moves, and security/a11y-sensitive ambiguities require resolution.
9. **Idempotent commands:** duplicate delivery with the same command ID and payload returns the original result and applies side effects once; same ID with different payload fails.
10. **Uncertain-send recovery:** a connection drop after authoritative command execution but before acknowledgement is reconciled by command ID after reconnect without a second state transition.
11. **Capability enforcement:** editor-only actors cannot publish, viewer/operator restrictions are enforced on host, preview tokens do not grant authority, and revoked capabilities fail at commit/execution.
12. **Rollback history:** rollback produces a higher new `ui_revision`, preserves all intervening history, identifies its source revision, and obeys base-revision conflict checks.
13. **Schema and semantic validation:** cyclic graphs, duplicate IDs, invalid slots, bad bindings, unknown commands, missing extension hashes, and missing accessible names are rejected with node/path diagnostics.
14. **WASM purity:** modules attempting ambient time, randomness, network, filesystem, undeclared imports, excess memory, or unbounded execution are denied or deterministically terminated; no production side effect results.
15. **Trusted-extension separation:** a normal editor cannot install or invoke an ungranted trusted host extension, and extension-administrator actions are independently audited.
16. **Registry stability:** a published revision continues to resolve its pinned registry semantics; unknown newer mandatory semantics fail safely rather than being guessed.
17. **Migration fidelity:** version migrations preserve node IDs and provenance, validate the migrated copy, and fail rather than silently discard unknown security- or accessibility-relevant content.
18. **Reconnect correctness:** available deltas resume in order, missing ranges trigger snapshots, stale updates are ignored, drafts recover, and unauthorized projections are removed.
19. **Accessibility:** manual and automated platform checks confirm accessible names/roles/states, logical focus, contrast, text scaling, reduced motion, keyboard/switch operation, non-color status cues, and dynamic announcements.
20. **Localization/responsiveness:** tests cover fallback locales, plural/select messages, bidirectional text, text expansion, narrow/split-screen layouts, safe areas, and large text without loss of primary actions.
21. **Resource controls:** oversized documents, deep graphs, excessive draft operations, command floods, and WASM resource exhaustion produce bounded deterministic errors without partial transactions or host starvation.
22. **Audit completeness:** publish, rollback, extension changes, command outcomes, and relevant authorization failures can be reconstructed by actor, content hash, revision, and correlation ID without exposing stored secrets.
23. **Crash recovery:** after host restart, published content, durable drafts, event ordering, audit records, and command idempotency outcomes remain consistent.
24. **Network independence:** the complete edit–preview–publish and command security model passes with a non-Tailscale test transport, proving that authorization does not rely on network membership alone.
