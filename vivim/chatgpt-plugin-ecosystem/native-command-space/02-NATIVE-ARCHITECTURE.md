# Native command architecture — typed semantic operating surface

## 1. Logical architecture

```text
User text / voice / PWA / GPUI / companion MCP / API
                    |
           Natural-language front end
      [trusted grammar / optional candidate model]
                    |
           Rust semantic compiler
  tokenize → intent/slots → context resolution → target evidence
                    |
      CommandFrame(version, source, plan_digest)
                    |
          existing Host planner/policy
  per-World scope + capability readiness + risk + preview
                    |
       Host exact approval + exclusive claim
                    |
        host-owned capability adapter(s)
    filesystem | process | app | studio | device
                    |
      Host journal + receipt + reconciliation
                    |
        Wiki/HQKG + PWA/GPUI VEC views
```

A user can say “Find the ten biggest files in Downloads, move the ones older than a year to Archive.” The compiler builds **two dependent** typed steps: a scoped **read** candidate list, then a **proposed** move set. “Older than a year” must bind to a definite filesystem timestamp interpretation (modified/created; ask if relevant); “Archive” must bind to one verified destination; no move without a concrete preview and approval. The Host must re-check file identities, version/hash and sandbox scope at commit time.

## 2. Separate pure meaning and effectful execution

**Pure side, in Rust programmable core:** tokens/aliases, syntactic frames, entity candidates, expected slot schemas, grounding evidence, risk projection, diff previews, deterministic validation and DAG topology. It **never** opens an arbitrary file, runs a command, creates an approval, saves receipts, or executes browser effects.

**Effect side, owned by Host:** filesystem/scoped OS handles, process/session lifetime, supervised app startup, clipboard/window control where consented, official local service operations, Studio browser adapter, credential-free device grants, signed operations, irreversible effects and journal reconciliation.

If the Host is still partly JS during migration, a Rust child helper may perform the narrowly authorized OS work but must be invoked only by the existing Host dispatcher after its verified claim; no independent MCP or PWA-facing execute endpoint. Eventually the effects implementation can itself migrate to Rust under the same Host identity.

## 3. Candidate Rust contracts (proposal, not compiled source)

```rust
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommandFrameV1 {
    pub schema: String,                   // "vivim.command-frame/1"
    pub utterance_id: String,
    pub origin: Origin,                    // text | voice | pwa | gpui | mcp
    pub command_id: String,                // registered semantic capability id
    pub operands: Vec<TypedOperand>,       // each validated against registered type
    pub targets: Vec<ResolvedTarget>,      // canonical host-scoped refs, not model paths
    pub catalog_revision: u64,
    pub world_revision: u64,
    pub expected_device_revision: Option<u64>,
    pub provenance: Provenance,
}
pub struct TypedOperand {
    pub name: String,
    pub value: OperandValue,               // file-ref | uri-ref | duration | timestamp | text | ...
}
pub struct ResolvedTarget {
    pub reference: String,                // opaque stable ref, not a raw arbitrary path grant
    pub source_revision: String,          // timestamp/version/hash as applicable
    pub scope_id: String,
}
```

Use actual enums and exhaustive serialization policy; above is a design sketch, not production ABI. Keep user-supplied surface strings separate from Host-trusted typed handles. Transport-specific wire type maps onto current catalog where exact parity exists, otherwise returns `adapter_pending` until a versioned contract is accepted.

## 4. Command primitive basis

**18 adapter families** (not 279 individual executors): `filesystem`, `document`, `process`, `studio`, `knowledge`, `tasks`, `calendar`, `communications`, `research`, `media`, `system`, `devices`, `privacy`, `automation`, `developer`, `ui`, `data`, `personal`. The catalog’s 279 intent aliases compile into these bounded adapters plus a small **primitive vocabulary**:

- `discover`, `describe`, `inspect`, `read`, `search`, `compare`, `summarize`, `watch` — evidence-producing, scoped reads.
- `create`, `copy`, `move`, `rename`, `patch`, `transform`, `set`, `submit`, `share` — effects, with previews.
- `launch`, `interact`, `observe`, `cancel`, `terminate`, `reconcile` — OS and external session state.
- `schedule`, `condition`, `compose`, `repeat`, `undo`, `restore` — lifecycle and multistep semantics.

Never define a universal `tool_execute(command: String)` as the normal executor. Specific verbs register specific schemas and OS adapters. A string that names a shell command is not executable intent.

## 5. Entity/context resolution

Support `this`, `that`, `those`, `here`, `the last download`, `my recent pictures`, `the window I am in` by looking up real **user-selected** source/selection/active-window references, captured with freshness and permission. When several targets exist, return up to a bounded choice list. Never choose merely by plausible file name. For “my PC” on PWA resolve the explicitly paired online device; if none/ambiguous, ask or refuse.

Ground time phrases through owner locale/timezone and exact clock source; a scheduled task must be a Host-owned durable record, not a browser timer. Unicode normalization and locale-specific language parsing require parity tests. English + Castilian Spanish should be planned; the seed registry currently records English examples only.

## 6. DAG composition, approvals and receipts

A multi-step command is a DAG of typed commands, but approval must bind the exact **resolved effect set**. Ex:
1. Discover `files.find.largest_files` → stable list + hashes/revisions.
2. Filter `mtime < cutoff` deterministically.
3. Preview `files.change.batch_move`: count, from/to, collisions, total bytes, rollback feasibility.
4. User approves exact digest.
5. Host obtains exclusive claim, revalidates target references and executes in deterministic order; each effect gets a receipt.
6. Partial failure returns `partial/uncertain` and individual receipts; no unapproved replay. Compensation is a **separate authorized operation**, not guaranteed atomic batch rollback.

For stream/process operations preserve process IDs + executable identity and Host session IDs; cancellation and timeout must not ambiguously claim that execution never occurred. `kill` process is never equivalent to close window, and termination of unknown processes is not learnable.

## 7. Composing low-cost intelligence

Rule-based/local Rust parser for obvious commands; optional candidate ranking for ambiguity/long utterances. Any model is **proposal-only**; Host recomputes and validates. No inference API dependency. Preserve existing NLCL and `command-ai-guard` until an evidence-based, parity-tested Rust replacement is ready. Model inference is an enhancement, not an authority.

## 8. UI and device placement

- **PWA mobile-first:** text, voice capture, browse capabilities, preview, approve, inspect receipt; native filesystem access is on paired Host, not inside browser sandbox.
- **GPUI Windows:** native command composer, quick launcher, long-running jobs and rich receipts; should use same Host plan/approve/inspect endpoints.
- **Studio:** signed-in Chrome profile, single browser lease, DOM observations and effects through Host-owned Studio. Do not create a rival browser controller or use Playwright/Puppeteer.
- **External MCP:** optional `tools/list` projection of *actually available* Host capabilities; `tools/call` maps to same typed planner; no direct OS bypass.

## 9. Contracts to prove before merge

CommandFrame validation; trust/capability provenance; structured results; allowed target refs; versioned catalog compatibility; Host receipt status; R0 data-scope protection; device online/offline; process sessions; cancellation; unknown effects; approved compensation; PWA/GPUI identical meaning. Actual code ownership must be re-pinned to latest main before implementation.

See source map for exact integration files.
