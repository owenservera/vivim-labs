# VEC compiler/0.1 — one utility, three *different* products

```text
Trusted caller chooses intended product DOMAIN (wiki / command / ui)
    + bounded natural-language text or source
    + pinned current registry/surface/World revisions
    -> VEC frontends: source/semantic, NLCL/command, UI grammar
    -> typed candidate only
       wiki: `wiki-projection` -> incumbent HQ Wiki program validation/writer (until takeover)
       command: `command-plan` -> existing Host dispatcher with current grant/journal (never takeover)
       ui: `ui-revision-proposal` -> existing PWA compileUiProposal + Host apply-proposal
```

**The distinctions are essential:**
- "Show me my unfinished tasks" means **execute/query an existing capability** (command domain). Result may be rendered but does not rewrite the UI definition.
- "Put my tasks below Projects on the Home screen" means **edit an interface program/document** (UI domain), returning a proposed UI revision. Nothing executes merely from compiling the request.
- "Show this dependency group as a Wiki page" means **compile a knowledge projection** (Wiki domain) and only existing owner-confirmed Wiki writer may activate configuration.

**No intent-based domain guessing at this stage.** Trusted product context chooses which frontend is active; generic chat orchestration must later use an independently reviewed intent-routing policy, not silently reinterpret a message as UI mutation.

## VEC preview contract
`vivim.vec-compile/0.1` returns `domain, product, outcome, code, authorization:none, executed:false, persisted:false` and (only for candidates) canonical `proposal`, `preview`, `summary`. `legacyOwner` denotes planned but unported domains. A candidate is not Host authorization, a permission grant, successful publish or source-of-truth revision.

## Current PWA implementation
UI grammar accepts exact patterns such as:
- "Add a card called Research" / "Add a heading saying My work" / "Create text named Notes".
- "Rename card Research to Projects".
- "Remove card Research".
- "Move card Today to the top".

The compiler **never constructs arbitrary HTML/JS** and delegates all structure, source references, private branches and stale revisions to the existing `compileUiProposal` validator. Unknown/complex requests preserve the pre-existing AI design path, but the model's JSON must pass exactly the same validator. Ambiguous UI targets ask clarification; refused UI commands do NOT fall back silently to a model. Final Activate goes through existing paired Host and existing undo.

## Migration contracts
Wiki source of truth: HQ index and owner-confirmed Wiki-config writer; VEC may take compiler ownership only after byte-reproducible baseline and differential tests.
Command source of truth: Host catalog + personal rule store; VEC will own typed compilation only. Host dispatcher/approvals/journal never migrate.
UI source of truth: current Host-published `vivim.ui-revision/1`, and the guarded PWA projection. Do not activate TierFlow's historical independent SQLite store as a parallel writer.

## Trust and portability
The first ESM module is pure, browser-safe, no network/fs/process/clock. On-device UI preview is inert and can run unpaired but cannot publish unpaired. A future internal shared module/package must expose exactly the same source bytes/semantics to desktop/Node/phone without copy/paste fork. Rust native transform validation is an **optional later backend**, with explicit ABI and JS differential parity. Source comments/Rust capsules are reviewed build inputs, never executable grants.