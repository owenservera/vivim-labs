import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

type Lane = { id: string; folder: string; issueIds: string[]; execution: string; workflowRefs: string[] };
type Manifest = { schema: string; version: string; autostart: boolean; ownsScheduler: boolean;
  maxUnverifiedModelWorkers: number; maxInitialAdmittedReadOnlyWorkers: number;
  snapshot?: { open: number; checked: number }; lanes: Lane[] };
type IssueRow = { id: string; checked: boolean };

const root = dirname(fileURLToPath(import.meta.url));
const required = ["README.md", "AGENTS.md", "DIRECTIVE.md", "LANE-BOARD.md", "STRATEGIC-AUDIT-R002.md",
  "LAUNCH-FIVE-CHATS.md", "manifest.json", ...["01-chatgpt-provider", "02-gpui-mobile-surface",
  "03-work-oracle-autowiki", "04-vlang-visual-gestures", "05-verification-models-release"]
    .flatMap(x => ["AGENTS.md", "START-PROMPT.md", "HANDOFF.md", "CHATGPT-LAUNCH.md"].map(y => join(x,y)))];

export function expandIssueIds(range: string): string[] {
  const match = /^([A-Z])([0-9]{2})(?:-([A-Z])?([0-9]{2}))?$/.exec(range);
  if (!match) throw new Error("INVALID_RANGE:" + range);
  const [, prefix, first, endPrefix, last] = match;
  if (endPrefix && endPrefix !== prefix) throw new Error("CROSS_PREFIX_RANGE:" + range);
  const start = Number(first), end = last ? Number(last) : start;
  if (end < start || end > 99) throw new Error("INVALID_RANGE_BOUNDS:" + range);
  return Array.from({ length: end - start + 1 }, (_,i) => prefix + String(start + i).padStart(2,"0"));
}

export function parseIssueRows(body: string): IssueRow[] {
  const matches = [...body.matchAll(/^\s*[-*]\s*\[([ xX])\]\s*\*\*([A-Z][0-9]{2})\*\*/gm)];
  return matches.map(m => ({ id: m[2], checked: m[1].toLowerCase() === "x" }));
}

export function checkCoverage(doc: Manifest, issueBody?: string): { errors: string[]; open: number; verifiedAgainstIssue: boolean; checked?: number } {
  const errors: string[] = [];
  const assigned = new Map<string,string>();
  for (const lane of doc.lanes) {
    for (const range of lane.issueIds || []) {
      let ids: string[] = [];
      try { ids = expandIssueIds(range); } catch (e) { errors.push(String(e)); continue; }
      for (const id of ids) {
        if (assigned.has(id)) errors.push("DUPLICATE_PRIMARY:" + id + ":" + assigned.get(id) + ":" + lane.id);
        assigned.set(id, lane.id);
      }
    }
  }
  if (doc.snapshot && assigned.size !== doc.snapshot.open) errors.push("SNAPSHOT_OPEN_COUNT_MISMATCH:" + assigned.size + ":" + doc.snapshot.open);
  if (issueBody === undefined) return { errors, open: assigned.size, verifiedAgainstIssue: false };
  const rows = parseIssueRows(issueBody);
  const canonical = new Map<string,boolean>();
  for (const row of rows) {
    if (canonical.has(row.id)) errors.push("DUPLICATE_CANONICAL:" + row.id);
    canonical.set(row.id,row.checked);
  }
  if (!rows.length) errors.push("EMPTY_OR_UNPARSEABLE_ISSUE_BODY");
  const openIds = rows.filter(x=>!x.checked).map(x=>x.id);
  const checked = rows.filter(x=>x.checked).length;
  for (const id of openIds) if (!assigned.has(id)) errors.push("UNASSIGNED_OPEN:" + id);
  for (const id of assigned.keys()) {
    if (!canonical.has(id)) errors.push("NOT_IN_CANONICAL:" + id);
    else if (canonical.get(id)) errors.push("ALREADY_CHECKED:" + id);
  }
  if (doc.snapshot && doc.snapshot.checked !== checked) errors.push("SNAPSHOT_CHECKED_COUNT_MISMATCH:" + checked + ":" + doc.snapshot.checked);
  return { errors, open: assigned.size, verifiedAgainstIssue: errors.length === 0, checked };
}

export function validate(doc: Manifest, issueBody?: string, rootDir = root) {
  const problems: string[]=[];
  if (doc.schema !== "vivim.five-lane-operation/1") problems.push("UNEXPECTED_SCHEMA");
  if (doc.autostart || doc.ownsScheduler) problems.push("UNAPPROVED_AUTOSTART_OR_SCHEDULER");
  if (doc.maxUnverifiedModelWorkers !== 0 || doc.maxInitialAdmittedReadOnlyWorkers > 2)
    problems.push("UNSAFE_MODEL_ADMISSION");
  if (doc.lanes.length !== 5 || new Set(doc.lanes.map(x=>x.id)).size !== 5)
    problems.push("EXPECTED_FIVE_UNIQUE_LANES");
  for (const f of required) if (!existsSync(join(rootDir,f))) problems.push("MISSING_FILE:" + f);
  for (const lane of doc.lanes) {
    if (!/^L0[1-5]$/.test(lane.id) || lane.execution !== "READY_NOT_LAUNCHED")
      problems.push("INVALID_LANE:" + lane.id);
    if (!lane.workflowRefs?.length || lane.workflowRefs.some(p=>!p.startsWith(".zcode/workflows/")))
      problems.push("INVALID_EXISTING_DWF_REFERENCE:" + lane.id);
  }
  const coverage = checkCoverage(doc,issueBody);
  return { schema:doc.schema, version:doc.version, lanes:doc.lanes.map(x=>x.id),
    filesChecked:required.length, coverage, issues:[...problems,...coverage.errors] };
}

if (process.argv[1] && fileURLToPath(import.meta.url).toLowerCase() === process.argv[1].toLowerCase()) {
  const pathIndex=process.argv.indexOf("--issue-body");
  const issueBody=pathIndex>=0 && process.argv[pathIndex+1]
    ? readFileSync(process.argv[pathIndex+1],"utf8") : undefined;
  const doc=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8")) as Manifest;
  const report=validate(doc,issueBody);
  console.log(JSON.stringify(report,null,2));
  if (report.issues.length) process.exitCode=1;
}
