# zcode-hub

One central hub for every ZCode agent def, skill, plugin, and tool on this machine.
The hub links to the live sources with Windows junctions, so there is exactly ONE
source of truth — edit at the source, read through the hub. Current index: agents 86 / skills 402 / user-skills 97 / plugins 7 (quarantined 0) / tools 3 / refs 2.

## Layout

```
zcode-hub/
  agents/             -> junction to C:\Users\VIVIM.inc\.zcode\agents            (86 live *.md defs; model: in frontmatter)
  skills/             -> junction to C:\Users\VIVIM.inc\.zcode\skills            (402 entries)
  user-skills/        -> junction to C:\Users\VIVIM.inc\.agents\skills           (97 entries)
  plugins/            -> junction to C:\Users\VIVIM.inc\.zcode\cli\plugins\cache  (7 plugin dirs; _quarantine-* flagged in manifest)
  tools/
    repos_hq          -> junction to C:\0-BlackBoxProject-0\repos_hq            (bus: scripts/sync.mjs, hq/bin/hq.mjs, sync/ ledger)
    vivim-web-harness -> junction to C:\0-BlackBoxProject-0\vivim-web-harness  (Capture Studio CLI: studio/bin/studio.mjs)
    bundled-skills    -> junction to C:\Program Files\ZCode\resources\glm\packages\bundled-skills\skills
  manifest.json       machine-readable index: {generatedAt, note, totals, entries[]} — one entry per item
  links.json          junction map written by the build: [{item, hubPath, sourcePath, link, verified}]
  inventory.json      source inventory written by the build: {agents[], counts, tools[], existingRefs[]}
  verify.mjs          acceptance check — recounts sources vs hub vs manifest (see below)
```

Each manifest entry is {kind, name, hubPath, sourcePath, link, model?, note?} with
kind in agent | skill-dir | user-skill-dir | plugin | tool | ref, and
link in junction | copy | ref:
- junction = Windows directory junction (cmd /c mklink /J) — one source of truth
- copy     = fallback snapshot if mklink is unavailable (flagged via sourcePath)
- ref      = pointer to a pre-existing location that is referenced, never duplicated
  (vivim-labs/agents, vivim-labs/agentic_excelence)

## Spawning 4 subagents from this hub

Send ONE assistant message containing 4 parallel Agent tool calls. Each prompt embeds
its cavecrew output contract (skill: C:\Users\VIVIM.inc\.agents\skills\cavecrew\SKILL.md):

```
Agent(subagent_type="cavecrew-investigator", prompt="...mandate...\nOutput contract: lines of `path:line — \`symbol\` — note`, then `totals: <counts>`, or `No match.`")
Agent(subagent_type="cavecrew-builder",      prompt="...mandate...\nOutput contract: `<path:line-range> — <change <=10 words>. verified: <re-read OK|mismatch>`, or terminal token too-big. / needs-confirm. / ambiguous. / regressed.")
Agent(subagent_type="cavecrew-builder",      prompt="...mandate...\n(same builder contract)")
Agent(subagent_type="cavecrew-reviewer",     prompt="...mandate...\nOutput contract: `path:line: <emoji> <severity>: <problem>. <fix>.` + `totals:`, or `No issues.`")
```

Prefer the cavecrew-* defs in agents/ (junctioned live from ~/.zcode/agents) for matching work.

## Adding a new agent or tool

1. Drop the def into C:\Users\VIVIM.inc\.zcode\agents\ — the agents/ junction reflects it
   automatically; there is no copy step and no second source of truth.
2. For a runnable tool: add a junction under tools/ (cmd /c mklink /J <hub>\tools\<name> <source>)
   and append a {kind:"tool", name, hubPath, sourcePath, link, note} entry to manifest.json.
3. Re-run the acceptance command below — verify.mjs recounts sources vs hub vs manifest.

## Acceptance command

```
bun "C:\0-BlackBoxProject-0\vivim-labs\zcode-hub\verify.mjs"
```

Must print PASS + counts and exit 0: every manifest hubPath exists on disk and each
section's source/hub/manifest counts match.
