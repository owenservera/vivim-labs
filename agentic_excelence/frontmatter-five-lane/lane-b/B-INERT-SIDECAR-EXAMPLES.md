# Lane B — Inert native / sidecar source-to-target examples

**Status:** DESIGN EXAMPLES ONLY · **Activation:** prohibited · **Consumer runtime:** unverified.

These are side-by-side examples of source observations versus *proposed* out-of-band semantic metadata. They are not deployable agent instructions and no original or provider config was changed. All source examples below are from migrated public-origin snapshots, not the four private source references. The example sidecars shown are **virtual** and have not been written to provider-recognized paths.

| Source file | Observed existing content | Target proposal | Delta |
|---|---|---|---|
| `file:001` OpenCode agent YAML | native-yaml-observed; heading “Do real work” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:003` Repository AGENTS.md | plain-markdown-observed; heading “AION — Execute the work” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:004` BOOTSTRAP.md | plain-markdown-observed; heading “AION BFF bootstrap” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:098` WORKSPACE.md | plain-markdown-observed; heading “Files” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:021` AGENT.MD | plain-markdown-observed; heading “Agent Instructions” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:017` CLAUDE.md | plain-markdown-observed; heading “cdpilot” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:018` GEMINI.md | plain-markdown-observed; heading “curl / wget — BLOCKED” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |
| `file:213` Misclassified `docs/shape-atlas/claude.md` | plain-markdown-observed; heading “Provider-native shape contract — Claude WEB (`claude-web-shapes/1`)” | Inert JSON sidecar | Metadata added **outside** original; active file unchanged |

## Example 1 — OpenCode agent YAML (`file:001`)

**Before (source observation):** `owenservera/aion/.opencode/agents/aion-chief.md`; source blob `91fbedf6e6f8750a03e1fe97567d29c74feaffb8`.

Observed YAML keys: `description`, `mode`; loader acceptance **not** tested.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:001",
  "sourceRepository": "owenservera/aion",
  "sourcePath": ".opencode/agents/aion-chief.md",
  "sourceBlobSha": "91fbedf6e6f8750a03e1fe97567d29c74feaffb8",
  "fileType": ".opencode/agents/*.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [
    "section_type:context"
  ],
  "uniqueSectionIds": [
    "section:001:002",
    "section:001:003",
    "section:001:004"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "opencode-native-header-observed-plus-sidecar-proposal"
}
```

**Difference and restriction:** The original file already has YAML; `description` and `mode` were observed, not loader-verified. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 2 — Repository AGENTS.md (`file:003`)

**Before (source observation):** `owenservera/aion/AGENTS.md`; source blob `e1cfc546876a92835ad0b1ff6ec943d3c48111d0`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:003",
  "sourceRepository": "owenservera/aion",
  "sourcePath": "AGENTS.md",
  "sourceBlobSha": "e1cfc546876a92835ad0b1ff6ec943d3c48111d0",
  "fileType": "AGENTS.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [],
  "uniqueSectionIds": [
    "section:003:001",
    "section:003:002",
    "section:003:003",
    "section:003:004"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** Historical owner assertions must not become current VIVIM owner policy. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 3 — BOOTSTRAP.md (`file:004`)

**Before (source observation):** `owenservera/aion/workspace/aion/bff/BOOTSTRAP.md`; source blob `58db31c086c8f1944aab509ba053682644421bc1`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:004",
  "sourceRepository": "owenservera/aion",
  "sourcePath": "workspace/aion/bff/BOOTSTRAP.md",
  "sourceBlobSha": "58db31c086c8f1944aab509ba053682644421bc1",
  "fileType": "BOOTSTRAP.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [],
  "uniqueSectionIds": [
    "section:004:001",
    "section:004:002",
    "section:004:003",
    "section:004:004"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** A historical restart command is a source observation, not a reproducible current startup. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 4 — WORKSPACE.md (`file:098`)

**Before (source observation):** `owenservera/aion/workspace/aion/sessions/roles/06-tooling-integration-engineer/WORKSPACE.md`; source blob `a1e7744761991448d6cfc3ca7b82cd0e5652940c`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:098",
  "sourceRepository": "owenservera/aion",
  "sourcePath": "workspace/aion/sessions/roles/06-tooling-integration-engineer/WORKSPACE.md",
  "sourceBlobSha": "a1e7744761991448d6cfc3ca7b82cd0e5652940c",
  "fileType": "WORKSPACE.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [
    "section_type:tools",
    "section_type:lifecycle"
  ],
  "uniqueSectionIds": [
    "section:098:002",
    "section:098:003",
    "section:098:004"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** Workspace names and ownership are research metadata, not present-tense permissions. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 5 — AGENT.MD (`file:021`)

**Before (source observation):** `owenservera/aion/workspace/aion/ops/chrome-relay/harvest/external/landing/donors/chatgpt-bridge/AGENT.MD`; source blob `1b2d5e915fbdfb1cd6a4f03df14adfa21f7cd568`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:021",
  "sourceRepository": "owenservera/aion",
  "sourcePath": "workspace/aion/ops/chrome-relay/harvest/external/landing/donors/chatgpt-bridge/AGENT.MD",
  "sourceBlobSha": "1b2d5e915fbdfb1cd6a4f03df14adfa21f7cd568",
  "fileType": "AGENT.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [
    "section_type:verification",
    "section_type:workflow",
    "section_type:recovery"
  ],
  "uniqueSectionIds": [
    "section:021:001",
    "section:021:002",
    "section:021:003",
    "section:021:005",
    "section:021:007"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** The actual filename uses uppercase `.MD`; no loader recognition is assumed. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 6 — CLAUDE.md (`file:017`)

**Before (source observation):** `owenservera/aion/workspace/aion/ops/chrome-relay/harvest/external/landing/donors/cdpilot/CLAUDE.md`; source blob `443e56aa25587975b66773aaa6d8c9a4331e237c`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:017",
  "sourceRepository": "owenservera/aion",
  "sourcePath": "workspace/aion/ops/chrome-relay/harvest/external/landing/donors/cdpilot/CLAUDE.md",
  "sourceBlobSha": "443e56aa25587975b66773aaa6d8c9a4331e237c",
  "fileType": "CLAUDE.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [],
  "uniqueSectionIds": [
    "section:017:001",
    "section:017:002",
    "section:017:003",
    "section:017:004",
    "section:017:005"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** Treat historical global Claude precedence claims as unverified observations. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 7 — GEMINI.md (`file:018`)

**Before (source observation):** `owenservera/aion/workspace/aion/ops/chrome-relay/harvest/external/landing/donors/cdpilot/GEMINI.md`; source blob `0df443418eeb68b743aadc3b98c3c78947013fdd`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:018",
  "sourceRepository": "owenservera/aion",
  "sourcePath": "workspace/aion/ops/chrome-relay/harvest/external/landing/donors/cdpilot/GEMINI.md",
  "sourceBlobSha": "0df443418eeb68b743aadc3b98c3c78947013fdd",
  "fileType": "GEMINI.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [
    "section_type:context",
    "section_type:workflow",
    "section_type:tools"
  ],
  "uniqueSectionIds": [
    "section:018:003",
    "section:018:004",
    "section:018:005",
    "section:018:007",
    "section:018:008"
  ],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** Commands marked blocked in a donor file do not install current VIVIM tool restrictions. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Example 8 — Misclassified `docs/shape-atlas/claude.md` (`file:213`)

**Before (source observation):** `owenservera/vivim-harness/docs/shape-atlas/claude.md`; source blob `5f5c2428c0cd3a08e71c4ea0e1902ffcbaad5c3e`.

No leading native YAML header observed; native insertion **not** permitted.

**After (virtual sidecar, NOT published as an active file):**

```json
{
  "schema": "vivim.agentic-frontmatter/0.1",
  "fileId": "file:213",
  "sourceRepository": "owenservera/vivim-harness",
  "sourcePath": "docs/shape-atlas/claude.md",
  "sourceBlobSha": "5f5c2428c0cd3a08e71c4ea0e1902ffcbaad5c3e",
  "fileType": "CLAUDE.md",
  "role": "instruction",
  "sourceClass": "migrated-historical-snapshot",
  "authorityClass": "observed-source-not-policy",
  "purpose": null,
  "scope": {
    "projectRef": null,
    "taskRef": null,
    "actualInstructionScope": null
  },
  "sharedSectionTypes": [],
  "uniqueSectionIds": [],
  "generatedState": "proposal-not-runtime",
  "formatAdapter": "sidecar-first-no-native-insertion"
}
```

**Difference and restriction:** Matrix type `CLAUDE.md` conflicts with observed lowercase provider-shape documentation; do not route to instruction loader. The source file remains byte-for-byte unchanged; no runtime permissions, tool registrations, provider state, or owner approvals are inferred.

## Field-level diff invariant

| Field group | Source observation | Proposed sidecar field | Runtime consequence |
|---|---|---|---|
| Immutable identity | Original repo/path/blob SHA | `fileId`, `sourceRepository`, `sourceBlobSha` | Retrieval lookup only |
| Section semantics | Atlas IDs and source headings | `sharedSectionTypes`, `uniqueSectionIds` | Read-only graph links |
| Effective scope | Historical content; current state unknown | `scope.actualInstructionScope = null` | No authorization |
| Provider syntax | Existing Markdown or observed YAML | `formatAdapter` | No header rewrite |
| Tool and policy claims | Untrusted imported instructions | `authorityClass = observed-source-not-policy` | Denied as owner grants |

### Specific regression traps

- `file:154` and `file:213` are labelled `CLAUDE.md` in the matrix but physically reside as lowercase `docs/shape-atlas/claude.md`, describing Claude web transport shapes. They must not silently be promoted to native Claude instruction files.
- `file:021` is named `AGENT.MD` in the source; filename canonicalization must not silently imply consumer compatibility.
- An `AGENTS.md` under a nested donor directory cannot override a root instruction in another current repository without a loader-specific, scope-specific authority check.
- Native OpenCode `description` and `mode` are **observed** in three recovered profile files; exact installed loader/version evidence is still missing.

**Lane E action:** inspect these projections as untrusted data, replay path/precedence and refusal cases, and compare against the pinned source and actual provider loaders before granting validation status.
