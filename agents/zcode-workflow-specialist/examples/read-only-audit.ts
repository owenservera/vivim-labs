// Dynamic workflow TypeScript BODY example — submit via CreateWorkflow { name, script }.
// Source-reviewed against bundled dynamic-workflows SKILL.md §16; NOT runtime-executed.
// Use only in an authorized scratch workspace; prompt-only read-only is not a tool sandbox.

interface Inventory {
  paths: string[];
  observations: string[];
}
interface Verification {
  supported: boolean;
  evidence: string[];
  gaps: string[];
}

const scout = agent("Repository scout", {
  system: "Inspect the repository and return exact evidence. Do not modify files or run mutating commands."
});
const verifier = agent("Independent verifier", {
  system: "Verify claims independently against repository evidence. Do not modify files. State unknowns."
});

phase("Inventory the workspace");
const paths = await files.glob("src/**/*.ts");
const inventory = await scout.ask<Inventory>(
  "Read the current workspace and inspect relevant TypeScript implementation paths. " +
  "Use precise path evidence. Candidate paths: " + JSON.stringify(paths.slice(0, 30))
);
report({ kind: "inventory", examined: inventory.paths.length, observations: inventory.observations });

phase("Independently check the findings");
const checked = await verifier.ask<Verification>(
  "Independently check this inventory. Do not assume the claims are right. " +
  JSON.stringify(inventory)
);
report({ kind: "verification", supported: checked.supported, evidence: checked.evidence, gaps: checked.gaps });

const summary = [
  "# Repository audit",
  "",
  "Run ID: " + run.id,
  "Files reported by scout: " + inventory.paths.length,
  "Independent support: " + String(checked.supported),
  "",
  "## Observations",
  ...inventory.observations.map(x => "- " + x),
  "",
  "## Verification evidence",
  ...checked.evidence.map(x => "- " + x),
  "",
  "## Open gaps",
  ...checked.gaps.map(x => "- " + x)
].join("\n");

await artifact.markdown("audit-report", summary, {
  title: "Repository audit report",
  description: "Read-only findings with independent verification and open gaps.",
  primary: true
});

return { runId: run.id, inventory, checked };
