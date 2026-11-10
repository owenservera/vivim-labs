// @ts-check
/**
 * @hq:context The Labs command line: plan, validate, verify, status, evidence, capacity and report. Every
 *   command reads the Labs work-item model and writes only inside vivim-labs.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why the CLI is the operational face of the model: a maturity claim is only worth as much as the exit
 *   code that produced it, so `labs:verify` runs the real command and records the real exit code.
 * @hq:invariant `labs:verify` never marks an item verified above the stage its command actually justifies.
 *   The default ceiling is `unit verified`; a Labs-local fixture run is never device, browser or Host proof.
 * @hq:invariant `labs:capacity` measures the machine at run time. No number in its output is a stored
 *   estimate, and a source that cannot be read is reported as null with the reason, never guessed.
 * @hq:see labs/schema.mjs
 * @hq:see labs/workitems.mjs
 * @hq:see package.json
 */

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, statfsSync } from "node:fs";
import { homedir, platform as osPlatform, release as osRelease, totalmem, freemem, cpus } from "node:os";
import { join } from "node:path";

import {
  BOOST_WAVES,
  EVIDENCE_LEVELS,
  FACTORY_IDS,
  MATURITY_LADDER,
  WORK_ITEM_FIELDS,
  detectScopeConflicts,
  validateWorkItem,
} from "./schema.mjs";
import { FORBIDDEN_PATHS } from "./forbidden-paths.mjs";
import { REPO_ROOT, find, load, update } from "./workitems.mjs";

/** The W3 booster plan. @type {string} */
export const BOOST_PLAN_PATH = join(REPO_ROOT, "work", "boost-plan.json");

/** Resolve read-only capacity probe paths, retaining the original Windows fallbacks. */
function firstExisting(candidates) {
  for (const candidate of candidates) {
    if (candidate && existsSync(candidate)) return candidate;
  }
  return null;
}
const SIBLING_DIR = join(REPO_ROOT, "..");
const HQ_ROOT = firstExisting([join(SIBLING_DIR, "repos-hq"), join(SIBLING_DIR, "repos_hq"), "C:/0-BlackBoxProject-0/repos_hq"]) || "C:/0-BlackBoxProject-0/repos_hq";
const TIERFLOW_ROOT = firstExisting([join(SIBLING_DIR, "vivim-ui"), join(SIBLING_DIR, "tierflow"), "C:/0-BlackBoxProject-0/vivim-ui"]) || "C:/0-BlackBoxProject-0/vivim-ui";
const SEATS_PATH = firstExisting([join(SIBLING_DIR, "channel", "seats.json"), "C:/VIVIM/channel/seats.json"]) || "C:/VIVIM/channel/seats.json";
/** Measure the system disk on Windows and the actual repo volume elsewhere. */
const DISK_VOLUME = osPlatform() === "win32" ? "C:\\" : REPO_ROOT;

/** How many lines of command output a receipt keeps. @type {number} */
const OUTPUT_TAIL_LINES = 40;

/**
 * The Labs evidence records this scaffold links to instead of duplicating. Each entry names the owning
 * file; the numbers below are read out of that file at run time, so this table can never drift from the
 * evidence it points at. Labs copies none of their content.
 */
const LINKED_EVIDENCE = [
  {
    path: "evidence/repro/clean-build-reproducibility.md",
    owner: "labs clean-build lane",
    factory: "06",
    measures: [
      { label: "pinned source sha", pattern: /Source SHA \| `([0-9a-f]{40})`/ },
      { label: "bun suite", pattern: /\*\*(\d+) pass \/ 0 fail\*\*/ },
      { label: "rust sidecar tests", pattern: /(\d+) passed; 0 failed/ },
      { label: "sidecar build seconds", pattern: /Finished release profile in ([\d.]+)s/ },
      { label: "maturity claimed", pattern: /\*\*Maturity: ([^*]+)\*\*/ },
    ],
  },
  {
    path: "evidence/adversarial/adversarial-suite-results.md",
    owner: "labs adversarial lane",
    factory: "04, 05",
    measures: [
      { label: "pinned source sha", pattern: /Source SHA \| `([0-9a-f]{40})`/ },
      { label: "bun suite", pattern: /\| `bun test tests\/` \| 0 \| (\d+) pass/ },
      { label: "adversarial negatives", pattern: /\*\*(\d+) of those are `adversarial:`/ },
      { label: "real A-H", pattern: /real-a-h\.test\.ts` \| 0 \| (\d+) pass/ },
      { label: "compose", pattern: /compose\.test\.ts` \| 0 \| (\d+) pass/ },
      { label: "sqlite lane C", pattern: /ui-store\.test\.ts.*?\*\*(\d+) pass \/ 0 fail\*\*/ },
      { label: "maturity claimed", pattern: /the maturity is\s+`([a-z ]+)`/ },
    ],
  },
  {
    path: "evidence/catalogue/federated-catalogue-findings.md",
    owner: "labs catalogue lane",
    factory: "02, 03",
    measures: [
      { label: "catalogue domains", pattern: /catalogue-index\.json` \((\d+) domains\)/ },
      { label: "assets catalogued", pattern: /asset-inventory\.json` \((\d+) assets\)/ },
      { label: "duplicate authority collisions", pattern: /duplicate-authority\.json` \((\d+) collisions\)/ },
      { label: "high severity collisions", pattern: /(\d+) high severity/ },
      { label: "atomic registry rows", pattern: /atomic-registry\.json` \((\d+)\)/ },
      { label: "live index rows", pattern: /\*\*all (\d+) rows\*\*/ },
      { label: "maturity ceiling", pattern: /\*\*Maturity max (\d+)\*\*/ },
      { label: "maturity claimed", pattern: /\*\*Maturity: ([^*]+)\*\*/ },
    ],
  },
];

/**
 * Read a JSON file, or return null when it is absent or unreadable.
 * @param {string} path
 * @returns {any | null}
 */
function readJson(path) {
  if (!existsSync(path)) return null;
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    return null;
  }
}

/**
 * The last lines of a block of text, for a receipt that stays readable.
 * @param {string} text
 * @param {number} [lines]
 * @returns {string}
 */
function tail(text, lines) {
  const limit = lines || OUTPUT_TAIL_LINES;
  const value = typeof text === "string" ? text : "";
  const parts = value.split(/\r?\n/);
  if (parts.length <= limit) return value.trimEnd();
  return parts.slice(parts.length - limit).join("\n").trimEnd();
}

/**
 * The booster plan, or an empty plan.
 * @returns {{schema: string, wave: string, notes: string, items: any[]}}
 */
function readPlan() {
  const plan = readJson(BOOST_PLAN_PATH);
  if (!plan || typeof plan !== "object") {
    return { schema: "labs.boost-plan/1", wave: "", notes: "", items: [] };
  }
  return {
    schema: typeof plan.schema === "string" ? plan.schema : "labs.boost-plan/1",
    wave: typeof plan.wave === "string" ? plan.wave : "",
    notes: typeof plan.notes === "string" ? plan.notes : "",
    items: Array.isArray(plan.items) ? plan.items : [],
  };
}

/**
 * Every work item Labs knows about: the plan and the ledger, de-duplicated by task id. The ledger wins,
 * because it holds the recorded results.
 * @returns {any[]}
 */
function allItems() {
  const byId = new Map();
  for (const item of readPlan().items) {
    if (item && typeof item === "object" && item.task_id) byId.set(item.task_id, item);
  }
  for (const item of load().items) {
    if (item && typeof item === "object" && item.task_id) byId.set(item.task_id, item);
  }
  return [...byId.values()];
}

/**
 * `labs:plan` — the W3 booster set with its write scopes, and any scope that two items share.
 * @returns {number} exit code
 */
function cmdPlan() {
  const plan = readPlan();
  if (plan.items.length === 0) {
    console.error(`no booster plan found at ${BOOST_PLAN_PATH}`);
    return 1;
  }
  console.log(`Labs booster plan — ${plan.schema} — wave ${plan.wave}`);
  if (plan.notes) console.log(`notes: ${plan.notes}`);
  console.log("");
  for (const item of plan.items) {
    const factory = FACTORY_IDS.find((entry) => entry.id === item.factory);
    console.log(`${item.task_id}  factory=${item.factory} ${factory ? factory.name : "(unknown factory)"}`);
    console.log(`  owner:            ${item.owner}`);
    console.log(`  contract_version: ${item.contract_version}`);
    console.log(`  maturity:         ${item.maturity}  ->  target ${item.maturity_target}`);
    console.log(`  evidence_level:   ${item.evidence_level}`);
    console.log(`  source_sha:       ${item.source_sha}`);
    console.log(`  allowed_paths:    ${item.allowed_paths.join(", ")}`);
    console.log(`  test_command:     ${item.test_command}`);
    console.log("");
  }
  const conflicts = detectScopeConflicts(plan.items);
  if (conflicts.length > 0) {
    console.error(`WRITE-SCOPE CONFLICTS (${conflicts.length}) — disjoint scopes are mandatory:`);
    for (const conflict of conflicts) {
      console.error(
        `  ${conflict.task_id} [${conflict.path}] overlaps ${conflict.other_task_id} [${conflict.other_path}]`,
      );
    }
    return 1;
  }
  console.log(`scope check: ${plan.items.length} items, 0 overlapping allowed_paths`);
  return 0;
}

/**
 * `labs:validate` — every work item against the Labs model.
 * @returns {number} exit code
 */
function cmdValidate() {
  const items = allItems();
  if (items.length === 0) {
    console.error("no work items found in work/boost-plan.json or work/work-items.json");
    return 1;
  }
  let failures = 0;
  for (const item of items) {
    const verdict = validateWorkItem(item);
    const label = item && item.task_id ? String(item.task_id) : "(no task_id)";
    if (verdict.ok) {
      console.log(`OK   ${label}  maturity=${item.maturity} evidence=${item.evidence_level}`);
    } else {
      failures += 1;
      console.error(`FAIL ${label}`);
      for (const issue of verdict.errors) {
        console.error(`       [${issue.code}] ${issue.field ? `${issue.field}: ` : ""}${issue.message}`);
      }
    }
    for (const issue of verdict.warnings) {
      console.log(`  warn ${label} [${issue.code}] ${issue.message}`);
    }
  }
  const conflicts = detectScopeConflicts(items);
  for (const conflict of conflicts) {
    failures += 1;
    console.error(
      `FAIL scope conflict: ${conflict.task_id} [${conflict.path}] overlaps ${conflict.other_task_id} [${conflict.other_path}]`,
    );
  }
  console.log("");
  console.log(`${items.length} items validated, ${failures} failing`);
  return failures === 0 ? 0 : 1;
}

/**
 * The highest stage a command can justify. A work item may declare `evidence_ceiling`; without one the
 * ceiling is `unit verified`, because a Labs-local fixture run is never device, browser or Host proof.
 * @param {any} item
 * @returns {string}
 */
function justifiedStage(item) {
  const declared = item && typeof item.evidence_ceiling === "string" ? item.evidence_ceiling : "";
  if (EVIDENCE_LEVELS.includes(declared)) return declared;
  return "unit verified";
}

/**
 * `labs:verify` — run one item's test command for real and record what happened.
 * @param {string} taskId
 * @returns {number} exit code
 */
function cmdVerify(taskId) {
  if (!taskId) {
    console.error("usage: labs:verify -- <task_id>");
    return 2;
  }
  const item = find(taskId);
  if (!item) {
    console.error(`no work item with task_id "${taskId}" in the ledger`);
    return 2;
  }
  const command = typeof item.test_command === "string" ? item.test_command.trim() : "";
  if (command.length === 0) {
    console.error(`${taskId} has no test_command to run`);
    return 2;
  }
  console.log(`verifying ${taskId}`);
  console.log(`  command:  ${command}`);
  console.log(`  cwd:      ${REPO_ROOT}`);
  const started = Date.now();
  const run = spawnSync(command, {
    cwd: REPO_ROOT,
    shell: true,
    encoding: "utf8",
    timeout: 300_000,
    windowsHide: true,
  });
  const durationMs = Date.now() - started;
  const exitCode = typeof run.status === "number" ? run.status : 1;
  const stdout = typeof run.stdout === "string" ? run.stdout : "";
  const stderr = typeof run.stderr === "string" ? run.stderr : "";
  const artifactHash = createHash("sha256")
    .update(`${command}\n${exitCode}\n${stdout}\n${stderr}`)
    .digest("hex");

  const stage = exitCode === 0 ? justifiedStage(item) : "specified";
  const status = exitCode === 0 ? "pass" : "fail";
  const receipt = {
    at: new Date().toISOString(),
    command,
    cwd: REPO_ROOT,
    exit_code: exitCode,
    duration_ms: durationMs,
    stdout_tail: tail(stdout),
    stderr_tail: tail(stderr),
    artifact_hash: artifactHash,
    justified_stage: stage,
    status,
  };

  update(taskId, (stored) => {
    const history = Array.isArray(stored.results) ? stored.results : [];
    history.push(receipt);
    const kept = history.slice(-10);
    return {
      ...stored,
      result: `${status} (exit ${exitCode})`,
      artifact_hash: artifactHash,
      evidence_level: stage,
      maturity: stage,
      last_result: status,
      results: kept,
      updated: receipt.at,
    };
  });

  console.log("");
  console.log(`  exit code: ${exitCode}`);
  console.log(`  duration:  ${durationMs} ms`);
  console.log(`  artifact:  sha256:${artifactHash}`);
  console.log(`  evidence:  ${stage}`);
  if (stdout.trim().length > 0) {
    console.log("  --- stdout tail ---");
    console.log(tail(stdout, 20));
  }
  if (stderr.trim().length > 0) {
    console.log("  --- stderr tail ---");
    console.log(tail(stderr, 20));
  }
  if (exitCode !== 0) {
    console.error(`FAIL ${taskId}: the command failed with exit ${exitCode}. Recorded as a failure, not as verified.`);
    return 1;
  }
  console.log(`OK ${taskId}: recorded at stage "${stage}".`);
  return 0;
}

/**
 * `labs:status` — one table over every work item.
 * @returns {number} exit code
 */
function cmdStatus() {
  const items = allItems();
  if (items.length === 0) {
    console.error("no work items found");
    return 1;
  }
  const header = ["task_id", "factory", "wave", "maturity", "evidence_level", "last_result", "source_sha", "updated"];
  const rows = items.map((item) => [
    String(item.task_id ?? "?"),
    String(item.factory ?? "--"),
    String(item.wave ?? "--"),
    String(item.maturity ?? "--"),
    String(item.evidence_level ?? "--"),
    String(item.last_result ?? (item.result ? String(item.result) : "pending")),
    String(item.source_sha ?? "--").slice(0, 12),
    String(item.updated ?? "--"),
  ]);
  const widths = header.map((title, column) =>
    Math.max(title.length, ...rows.map((row) => row[column].length)),
  );
  const line = (cells) => cells.map((cell, column) => cell.padEnd(widths[column])).join("  ");
  console.log(line(header));
  console.log(widths.map((width) => "-".repeat(width)).join("  "));
  for (const row of rows) console.log(line(row));
  return 0;
}

/**
 * `labs:evidence` — the linking index. It links capability, implementation, contract, consumer, test,
 * evidence and platform to the owner that holds the authority. It copies no authority: every row names
 * the canonical owner and where that owner's authority lives.
 * @returns {number} exit code
 */
function cmdEvidence() {
  const items = allItems();
  if (items.length === 0) {
    console.error("no work items found");
    return 1;
  }
  console.log("# Labs capability / evidence linking index");
  console.log("");
  console.log(
    "This index LINKS owners. It does not copy them: the canonical owner column is where the authority " +
      "actually lives, and nothing in this table grants source, Host, KB or release authority.",
  );
  console.log("");
  const header = [
    "capability",
    "factory",
    "implementation (labs)",
    "contract (canonical owner)",
    "consumer (canonical owner)",
    "test (labs)",
    "evidence",
    "platform (canonical owner)",
  ];
  const rows = items.map((item) => {
    const factory = FACTORY_IDS.find((entry) => entry.id === item.factory);
    const guidance = item.booster_guidance && typeof item.booster_guidance === "object" ? item.booster_guidance : {};
    const target = guidance.target_canonical_owner || {};
    const contract = guidance.canonical_contract || {};
    const implementation = (item.allowed_paths || []).filter((path) => typeof path === "string")[0] || "--";
    const testPath = (item.allowed_paths || []).find(
      (path) => typeof path === "string" && path.includes("test"),
    );
    return [
      String(item.task_id ?? "?") + " " + String(item.expected_output ?? "").slice(0, 48),
      String(item.factory ?? "--"),
      implementation,
      contract.path ? `${contract.repo || "?"}:${contract.path}` : factory ? factory.authority_ref : "--",
      target.repo || (factory ? factory.canonical_owner : "--"),
      testPath || "--",
      `${item.evidence_level ?? "--"} / ${item.last_result || "pending"}`,
      target.repo || "--",
    ];
  });
  const widths = header.map((title, column) =>
    Math.max(title.length, ...rows.map((row) => row[column].length)),
  );
  const line = (cells) => cells.map((cell, column) => cell.padEnd(widths[column])).join(" | ");
  console.log(line(header));
  console.log(widths.map((width) => "-".repeat(width)).join("-+-"));
  for (const row of rows) console.log(line(row));
  console.log("");
  console.log(`rows: ${rows.length} (linking index, not an authority copy)`);
  console.log("");
  console.log("Linked Labs evidence records — linked, not copied. The owning file is the source of truth:");
  for (const record of readLinkedEvidence()) {
    if (!record.present) {
      console.log(`  ${record.path}  [${record.factory}]  owner=${record.owner}  file not present`);
      continue;
    }
    const measured = record.measures.map((measure) => `${measure.label}=${measure.value}`).join(", ");
    console.log(`  ${record.path}  [${record.factory}]  owner=${record.owner}`);
    console.log(`      ${measured}`);
  }
  return 0;
}

/**
 * Run a version command and return its trimmed output.
 * @param {string} command
 * @param {string[]} args
 * @returns {{version: string | null, command: string, error?: string}}
 */
function probeVersion(command, args) {
  const run = spawnSync(command, args, { encoding: "utf8", timeout: 20_000, windowsHide: true });
  if (run.error) return { version: null, command: `${command} ${args.join(" ")}`, error: run.error.message };
  const value = typeof run.stdout === "string" ? run.stdout.trim() : "";
  if (value.length === 0) return { version: null, command: `${command} ${args.join(" ")}` };
  return { version: value.split(/\r?\n/)[0], command: `${command} ${args.join(" ")}` };
}

/**
 * Read one line out of a file and report where it was found.
 * @param {string} path
 * @param {RegExp} pattern
 * @returns {{path: string, line: number, text: string} | null}
 */
function findLine(path, pattern) {
  if (!existsSync(path)) return null;
  const lines = readFileSync(path, "utf8").split(/\r?\n/);
  for (let index = 0; index < lines.length; index += 1) {
    if (pattern.test(lines[index])) return { path, line: index + 1, text: lines[index].trim() };
  }
  return null;
}

/**
 * Read the linked Labs evidence records at run time. The owning file is the source of truth: the numbers
 * are extracted from it, so this table cannot drift from the evidence it points at.
 * @returns {{path: string, owner: string, factory: string, present: boolean, measures: {label: string, value: string}[]}[]}
 */
function readLinkedEvidence() {
  return LINKED_EVIDENCE.map((record) => {
    const absolute = join(REPO_ROOT, record.path);
    if (!existsSync(absolute)) {
      return { path: record.path, owner: record.owner, factory: record.factory, present: false, measures: [] };
    }
    const text = readFileSync(absolute, "utf8");
    const measures = record.measures.map((measure) => {
      const found = text.match(measure.pattern);
      if (!found) return { label: measure.label, value: "not found" };
      const captured = found[1] !== undefined ? found[1] : found[0];
      return { label: measure.label, value: captured.trim() };
    });
    return { path: record.path, owner: record.owner, factory: record.factory, present: true, measures };
  });
}

/**
 * `labs:capacity` — a measured capacity profile of this machine and this local fleet. Every number is
 * read at run time from a real command or a real file; nothing here is an estimate.
 * @returns {number} exit code
 */
function cmdCapacity() {
  const cpuList = cpus();
  const total = totalmem();
  const free = freemem();

  let disk = { path: DISK_VOLUME, total_bytes: null, free_bytes: null, source: "node:fs statfsSync" };
  try {
    // statfsSync is a real measurement of the volume, not a guess.
    const stats = statfsSync(DISK_VOLUME);
    disk = {
      path: DISK_VOLUME,
      total_bytes: stats.blocks * stats.bsize,
      free_bytes: stats.bavail * stats.bsize,
      source: "node:fs statfsSync",
    };
  } catch (error) {
    disk = {
      path: DISK_VOLUME,
      total_bytes: null,
      free_bytes: null,
      source: "unavailable",
      error: error && typeof error === "object" ? error.message : String(error),
    };
  }

  const seatsDocument = readJson(SEATS_PATH);
  const seats = seatsDocument && Array.isArray(seatsDocument.seats) ? seatsDocument.seats : [];
  const reservedSeats =
    seatsDocument && Array.isArray(seatsDocument.reserved) ? seatsDocument.reserved : [];

  const agentsDir = join(homedir(), ".zcode", "agents");
  let agentDefinitions = [];
  let agentDefinitionError = null;
  try {
    agentDefinitions = readdirSync(agentsDir).filter(
      (name) => name.endsWith(".md") && !name.includes(".bak") && !name.startsWith("_backup"),
    );
  } catch (error) {
    agentDefinitionError = error && typeof error === "object" ? error.message : String(error);
  }

  const spawnSource = join(HQ_ROOT, "hq", "lib", "spawn.mjs");
  const defaultLimits = findLine(spawnSource, /DEFAULT_LIMITS\s*=/);
  const fleetLimits = findLine(spawnSource, /FLEET_LIMITS\s*=/);
  const laneFile = join(HQ_ROOT, "docs", "integration", "archive", "lanes.v1.json");
  const laneDocument = readJson(laneFile);

  const tierflowHead = spawnSync("git", ["-C", TIERFLOW_ROOT, "rev-parse", "HEAD"], {
    encoding: "utf8",
    timeout: 20_000,
    windowsHide: true,
  });

  const profile = {
    measured_at: new Date().toISOString(),
    note: "Every value below was read at run time. Nothing is a stored estimate.",
    machine: {
      os: `${osPlatform()} ${osRelease()}`,
      cpu_logical_cores: cpuList.length,
      cpu_model: cpuList.length > 0 ? cpuList[0].model.trim() : null,
      cpu_speed_mhz: cpuList.length > 0 ? cpuList[0].speed : null,
      memory_total_bytes: total,
      memory_free_bytes: free,
      memory_used_bytes: total - free,
      disk_c_free_bytes: disk.free_bytes,
      disk_c_total_bytes: disk.total_bytes,
      disk_c_source: disk.source,
    },
    toolchain: {
      bun: probeVersion("bun", ["--version"]),
      node: probeVersion("node", ["--version"]),
      git: probeVersion("git", ["--version"]),
      cargo: probeVersion("cargo", ["--version"]),
    },
    local_fleet: {
      seats_file: SEATS_PATH,
      seats_file_readable: seatsDocument !== null,
      seats: seats.map((seat) => (seat && seat.id ? seat.id : "(unnamed)")),
      seat_count: seats.length,
      reserved_seats: reservedSeats,
      reserved_seat_count: reservedSeats.length,
      agent_definition_dir: agentsDir,
      agent_definition_count: agentDefinitions.length,
      agent_definitions: agentDefinitions.sort(),
      agent_definition_error: agentDefinitionError,
    },
    spawn_caps: {
      source: spawnSource,
      default_limits: defaultLimits,
      fleet_limits: fleetLimits,
      lane_file: laneFile,
      lane_max_concurrent: laneDocument ? laneDocument.maxConcurrent : null,
      lane_shared_serial_paths: laneDocument && laneDocument.sharedSerial
        ? laneDocument.sharedSerial.paths.length
        : null,
    },
    source_revisions: {
      tierflow_head:
        typeof tierflowHead.stdout === "string" ? tierflowHead.stdout.trim() : null,
      tierflow_command: `git -C ${TIERFLOW_ROOT} rev-parse HEAD`,
    },
  };
  console.log(JSON.stringify(profile, null, 2));
  return 0;
}

/**
 * `labs:report` — the readiness and status summary in markdown.
 * @returns {number} exit code
 */
function cmdReport() {
  const items = allItems();
  const plan = readPlan();
  const counts = (key) => {
    const tally = new Map();
    for (const item of items) {
      const value = String(item[key] ?? "--");
      tally.set(value, (tally.get(value) || 0) + 1);
    }
    return [...tally.entries()].sort((left, right) => right[1] - left[1]);
  };
  const conflicts = detectScopeConflicts(items);
  const invalid = items.filter((item) => !validateWorkItem(item).ok);
  const failures = items.filter((item) => item.last_result === "fail");
  const pending = items.filter((item) => !item.last_result || item.last_result === "pending");

  const lines = [];
  lines.push("# Labs readiness report");
  lines.push("");
  lines.push(`Generated: ${new Date().toISOString()}`);
  lines.push("");
  lines.push(`- work items: **${items.length}** (plan wave ${plan.wave || "--"})`);
  lines.push(`- plan file: \`${BOOST_PLAN_PATH}\``);
  lines.push(`- invalid items: **${invalid.length}**`);
  lines.push(`- scope conflicts: **${conflicts.length}**`);
  lines.push(`- failed verifications: **${failures.length}**`);
  lines.push(`- not yet verified: **${pending.length}**`);
  lines.push("");
  lines.push("## Counts by factory");
  lines.push("");
  lines.push("| factory | name | items |");
  lines.push("|---|---|---|");
  for (const [factory, count] of counts("factory")) {
    const known = FACTORY_IDS.find((entry) => entry.id === factory);
    lines.push(`| ${factory} | ${known ? known.name : "(unknown)"} | ${count} |`);
  }
  lines.push("");
  lines.push("## Counts by maturity");
  lines.push("");
  lines.push("| maturity | items |");
  lines.push("|---|---|");
  for (const [maturity, count] of counts("maturity")) lines.push(`| ${maturity} | ${count} |`);
  lines.push("");
  lines.push("## Counts by evidence level");
  lines.push("");
  lines.push("| evidence_level | items |");
  lines.push("|---|---|");
  for (const [level, count] of counts("evidence_level")) lines.push(`| ${level} | ${count} |`);
  lines.push("");
  lines.push("## Blockers");
  lines.push("");
  if (invalid.length === 0 && conflicts.length === 0 && failures.length === 0) {
    lines.push("- none: every work item validates, no write scope overlaps, and no verification has failed.");
  }
  for (const item of invalid) {
    const verdict = validateWorkItem(item);
    lines.push(
      `- **invalid** \`${item.task_id}\`: ${verdict.errors.map((issue) => `[${issue.code}] ${issue.message}`).join("; ")}`,
    );
  }
  for (const conflict of conflicts) {
    lines.push(
      `- **scope conflict**: \`${conflict.task_id}\` [${conflict.path}] overlaps \`${conflict.other_task_id}\` [${conflict.other_path}]`,
    );
  }
  for (const item of failures) {
    lines.push(`- **failed**: \`${item.task_id}\` — ${item.result || "the test command failed"}`);
  }
  if (pending.length > 0) {
    lines.push(
      `- **not verified yet**: ${pending.length} item(s) have no passing run: ${pending.map((item) => `\`${item.task_id}\``).join(", ")}`,
    );
  }
  lines.push("");
  lines.push("## Linked Labs evidence records");
  lines.push("");
  lines.push(
    "These records are owned by other Labs lanes. This report links them and reads their measured numbers " +
      "at run time; it copies none of their content and asserts none of their claims.",
  );
  lines.push("");
  lines.push("| record | factory | owner | measured |");
  lines.push("|---|---|---|---|");
  for (const record of readLinkedEvidence()) {
    if (!record.present) {
      lines.push(`| \`${record.path}\` | ${record.factory} | ${record.owner} | file not present |`);
      continue;
    }
    const measured = record.measures
      .map((measure) => `${measure.label}: ${measure.value}`)
      .join("; ");
    lines.push(`| \`${record.path}\` | ${record.factory} | ${record.owner} | ${measured} |`);
  }
  lines.push("");
  lines.push("## Model reference");
  lines.push("");
  lines.push(`- required work-item fields (${WORK_ITEM_FIELDS.length}): ${WORK_ITEM_FIELDS.join(", ")}`);
  lines.push(`- maturity ladder: ${MATURITY_LADDER.map((step) => `${step.level}. ${step.stage}`).join(" | ")}`);
  lines.push(`- reserved paths Labs refuses to write: ${FORBIDDEN_PATHS.length}`);
  lines.push(`- booster waves: ${BOOST_WAVES.map((wave) => wave.id).join(", ")}`);
  lines.push("");
  console.log(lines.join("\n"));
  return invalid.length === 0 && conflicts.length === 0 ? 0 : 1;
}

/**
 * The CLI entry point.
 * @param {string[]} argv
 * @returns {number} exit code
 */
export function main(argv) {
  const args = argv.filter((value) => value !== "--");
  const command = args[0] || "help";
  switch (command) {
    case "plan":
      return cmdPlan();
    case "validate":
      return cmdValidate();
    case "verify":
      return cmdVerify(args[1]);
    case "status":
      return cmdStatus();
    case "evidence":
      return cmdEvidence();
    case "capacity":
      return cmdCapacity();
    case "report":
      return cmdReport();
    case "help":
    case "--help":
      console.log(
        [
          "labs — the VIVIM Labs operating scaffold",
          "",
          "  bun labs/cli.mjs plan       show the W3 booster plan and check the write scopes are disjoint",
          "  bun labs/cli.mjs validate   validate every work item against the Labs model",
          "  bun labs/cli.mjs verify <id>  run an item's test_command and record the real result",
          "  bun labs/cli.mjs status     one table over every work item",
          "  bun labs/cli.mjs evidence   the capability -> evidence linking index",
          "  bun labs/cli.mjs capacity   the measured machine and fleet capacity profile (JSON)",
          "  bun labs/cli.mjs report     the markdown readiness summary",
        ].join("\n"),
      );
      return 0;
    default:
      console.error(`unknown command "${command}". Run "bun labs/cli.mjs help".`);
      return 2;
  }
}

const invokedDirectly = process.argv[1] ? /(?:^|[\\/])cli\.mjs$/.test(process.argv[1]) : false;
if (invokedDirectly) {
  const exitCode = main(process.argv.slice(2));
  if (exitCode !== 0) process.exit(exitCode);
}
