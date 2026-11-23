/**
 * ZCode dynamic-workflow read-only status audit.
 * Never changes ZCode state, launches a model, or reads prompt/event payloads.
 * Node 24+ built-in node:sqlite; use --experimental-strip-types.
 */
import { DatabaseSync } from "node:sqlite";
import { existsSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

type Run = {
  id: string;
  status: string;
  time_updated: number;
  parent_session_id: string | null;
};
type Count = { status: string; n: number };
type LastEvent = { last_event: number | null; n: number };
type OwnerPid = { process_id: number };

export function classifyRun(ageMinutes: number | null, processAlive: boolean, thresholdMinutes = 10): string {
  if (ageMinutes === null) return "UNVERIFIED_NO_EVENT";
  if (!processAlive) return "ORPHANED_NO_OWNER_PROCESS";
  if (ageMinutes > thresholdMinutes) return "STALE_NO_RECENT_EVENT";
  return "RECENT_PROCESS_CANDIDATE_NOT_EXECUTION_PROOF";
}

function processExists(pid: number): boolean {
  if (!Number.isSafeInteger(pid) || pid < 1) return false;
  try { process.kill(pid, 0); return true; } catch { return false; }
}

const dbPath = process.env.ZCODE_DB_PATH || join(homedir(), ".zcode", "cli", "db", "db.sqlite");
if (!existsSync(dbPath)) {
  console.error("ZCODE_SQLITE_NOT_FOUND; no database created");
  process.exitCode = 2;
} else {
  const db = new DatabaseSync(dbPath, { readOnly: true });
  try {
    const schema = db.prepare("SELECT name FROM sqlite_master WHERE type='table'").all() as { name: string }[];
    const tables = new Set(schema.map(x => x.name));
    const required = ["dwf_run", "dwf_event", "session_runtime_presence"];
    if (required.some(x => !tables.has(x))) {
      console.log(JSON.stringify({ schema: "vivim.zcode-workflow-readonly-audit/1", state: "UNSUPPORTED_SCHEMA", missing: required.filter(x => !tables.has(x)) }));
      process.exitCode = 2;
    } else {
      const now = Date.now();
      const counts = db.prepare("SELECT status, COUNT(*) AS n FROM dwf_run GROUP BY status ORDER BY status").all() as Count[];
      const running = db.prepare("SELECT id,status,time_updated,parent_session_id FROM dwf_run WHERE status='running' ORDER BY time_updated DESC").all() as Run[];
      const queryEvent = db.prepare("SELECT MAX(time_created) AS last_event, COUNT(*) AS n FROM dwf_event WHERE run_id=?");
      const queryOwner = db.prepare("SELECT process_id FROM session_runtime_presence WHERE session_id=?");
      const entries = running.map(run => {
        const ev = queryEvent.get(run.id) as LastEvent;
        const ownerPids = run.parent_session_id ? (queryOwner.all(run.parent_session_id) as OwnerPid[]).map(x => x.process_id) : [];
        const ownerProcessAlive = ownerPids.some(processExists);
        const ageMinutes = ev.last_event === null ? null : Math.round((now - ev.last_event) / 60000);
        return { id: run.id, databaseStatus: run.status, lastEventUtc: ev.last_event === null ? null : new Date(ev.last_event).toISOString(), eventCount: ev.n, ageMinutes, ownerProcessAlive, classification: classifyRun(ageMinutes, ownerProcessAlive) };
      });
      console.log(JSON.stringify({
        schema: "vivim.zcode-workflow-readonly-audit/1",
        observedAtUtc: new Date(now).toISOString(),
        totals: { total: counts.reduce((n, x) => n + x.n, 0), byStatus: counts },
        counts: { markedRunning: running.length, recentProcessCandidates: entries.filter(x => x.classification.startsWith("RECENT_PROCESS")).length, staleOrOrphaned: entries.filter(x => !x.classification.startsWith("RECENT_PROCESS")).length },
        entries,
        note: "RECENT_PROCESS_CANDIDATE requires independent progress/readback; SQLite running alone never proves live execution. Counts do not attest quality or model cost."
      }, null, 2));
    }
  } finally { db.close(); }
}
