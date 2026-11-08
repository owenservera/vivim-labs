// @ts-check
/**
 * @hq:context Acceptance for the Labs CLI: the capacity probe must measure, and the plan must be clean.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why a capacity profile with invented numbers is worse than no profile, because it is believed. The
 *   test parses the JSON the command actually emits and compares it with the machine it is running on.
 * @hq:see labs/cli.mjs
 */

import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { cpus, freemem, totalmem } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const REPO_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");

/**
 * Run the Labs CLI and capture its output.
 * @param {string[]} args
 * @returns {{status: number | null, stdout: string, stderr: string}}
 */
function runCli(args) {
  const run = spawnSync(process.execPath, [join("labs", "cli.mjs"), ...args], {
    cwd: REPO_ROOT,
    encoding: "utf8",
    timeout: 180_000,
    windowsHide: true,
  });
  return {
    status: run.status,
    stdout: typeof run.stdout === "string" ? run.stdout : "",
    stderr: typeof run.stderr === "string" ? run.stderr : "",
  };
}

test("labs:capacity exits 0 and emits JSON that carries the measured fields", () => {
  const run = runCli(["capacity"]);
  expect(run.status).toBe(0);

  const profile = JSON.parse(run.stdout);
  expect(typeof profile.measured_at).toBe("string");
  expect(typeof profile.machine.cpu_logical_cores).toBe("number");
  expect(profile.machine.cpu_logical_cores).toBeGreaterThan(0);
  expect(typeof profile.machine.memory_total_bytes).toBe("number");
  expect(profile.machine.memory_total_bytes).toBeGreaterThan(0);
  expect(typeof profile.machine.memory_free_bytes).toBe("number");
  expect(typeof profile.machine.disk_c_free_bytes).toBe("number");
  expect(profile.machine.disk_c_free_bytes).toBeGreaterThan(0);
  expect(typeof profile.toolchain.bun.version).toBe("string");
  expect(typeof profile.toolchain.node.version).toBe("string");
  expect(typeof profile.toolchain.git.version).toBe("string");
  expect(typeof profile.local_fleet.seat_count).toBe("number");
  expect(typeof profile.local_fleet.agent_definition_count).toBe("number");
  expect(profile.spawn_caps.source).toContain("hq");
  expect(profile.spawn_caps.default_limits).toBeTruthy();
  expect(profile.spawn_caps.default_limits.text).toContain("maxChildren");
  expect(typeof profile.source_revisions.tierflow_head).toBe("string");
});

test("the capacity numbers are measured now, not stored", () => {
  const profile = JSON.parse(runCli(["capacity"]).stdout);
  expect(profile.machine.cpu_logical_cores).toBe(cpus().length);
  expect(profile.machine.memory_total_bytes).toBe(totalmem());
  const measuredFree = profile.machine.memory_free_bytes;
  const liveFree = freemem();
  // The free memory moves between two reads; only a stored estimate would be exactly equal twice.
  expect(Math.abs(measuredFree - liveFree) / liveFree).toBeLessThan(0.5);
});

test("labs:plan runs clean and reports no overlapping write scopes", () => {
  const run = runCli(["plan"]);
  expect(run.status).toBe(0);
  expect(run.stdout).toContain("scope check:");
  expect(run.stdout).toContain("0 overlapping allowed_paths");
  expect(run.stdout).toContain("LABS-W3-001");
});

test("labs:validate runs clean over the shipped work items", () => {
  const run = runCli(["validate"]);
  expect(run.status).toBe(0);
  expect(run.stdout).toContain("items validated");
  expect(run.stdout).not.toContain("FAIL");
});

test("labs:status prints the table header and one row per work item", () => {
  const run = runCli(["status"]);
  expect(run.status).toBe(0);
  expect(run.stdout).toContain("task_id");
  expect(run.stdout).toContain("evidence_level");
  expect(run.stdout).toContain("LABS-W3-001");
});

test("labs:evidence emits a linking index that names the canonical owner", () => {
  const run = runCli(["evidence"]);
  expect(run.status).toBe(0);
  expect(run.stdout).toContain("capability");
  expect(run.stdout).toContain("canonical owner");
  expect(run.stdout).toContain("vivim-ui");
  expect(run.stdout).toContain("repos_hq");
});

test("labs:report emits the markdown summary", () => {
  const run = runCli(["report"]);
  expect(run.status).toBe(0);
  expect(run.stdout).toContain("# Labs readiness report");
  expect(run.stdout).toContain("Counts by factory");
  expect(run.stdout).toContain("Counts by maturity");
  expect(run.stdout).toContain("Counts by evidence level");
  expect(run.stdout).toContain("## Blockers");
});
