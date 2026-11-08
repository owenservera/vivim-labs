// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-006: the read-only A/B/D differential baseline against TierFlow.
 * @hq:owner labs
 * @hq:task LABS-W3-006
 * @hq:why a differential baseline is only useful while it still describes the source. This test pins the
 *   baseline to a commit that is still in TierFlow's history and confirms the canonical surfaces it names
 *   are real files. It reads TierFlow and never writes it.
 * @hq:see labs/evidence/differential/baseline.json
 */

import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";

import { readSeed, siblingFileExists } from "./_helpers.mjs";

const baseline = readSeed("evidence/differential/baseline.json");
const TIERFLOW_ROOT = "C:/0-BlackBoxProject-0/vivim-ui";

/**
 * Run a read-only git command inside TierFlow. `rev-parse` and `merge-base` read refs; they write nothing.
 * @param {string[]} args
 * @returns {{status: number | null, stdout: string}}
 */
function gitReadOnly(args) {
  const run = spawnSync("git", ["-C", TIERFLOW_ROOT, ...args], {
    encoding: "utf8",
    timeout: 30_000,
    windowsHide: true,
  });
  return { status: run.status, stdout: typeof run.stdout === "string" ? run.stdout.trim() : "" };
}

test("the baseline is pinned to a commit that is still in TierFlow's history", () => {
  expect(baseline.source_sha).toMatch(/^[0-9a-f]{40}$/);
  const ancestor = gitReadOnly(["merge-base", "--is-ancestor", baseline.source_sha, "HEAD"]);
  expect(ancestor.status).toBe(0);
});

test("the read-only inspection command actually reads TierFlow's HEAD", () => {
  const head = gitReadOnly(["rev-parse", "HEAD"]);
  expect(head.status).toBe(0);
  expect(head.stdout).toMatch(/^[0-9a-f]{40}$/);
  expect(baseline.read_only_inspection).toContain("rev-parse HEAD");
});

test("every canonical surface the baseline names is a real file", () => {
  let checked = 0;
  for (const surface of baseline.canonical_surfaces) {
    const present = siblingFileExists(baseline.source_repo, surface);
    if (present === null) continue;
    expect(present).toBe(true);
    checked += 1;
  }
  expect(checked).toBeGreaterThan(0);
});

test("the baseline lists all sixteen reserved lane paths and says where they came from", () => {
  expect(baseline.reserved_lane_paths.length).toBe(16);
  expect(baseline.ownership_source).toContain("WAVE-3-OWNERSHIP.md");
  for (const entry of baseline.reserved_lane_paths) {
    expect(entry.path.startsWith("/")).toBe(true);
    expect(typeof entry.present_at_capture).toBe("boolean");
    expect(["A", "B", "C", "D", "E", "F", "G", "H"]).toContain(entry.lane);
  }
});

test("the reserved lane presence is reported, not asserted: a lane landing is a real change, not a test failure", () => {
  for (const entry of baseline.reserved_lane_paths) {
    const present = siblingFileExists(baseline.source_repo, entry.path);
    if (present === null) continue;
    if (present !== entry.present_at_capture) {
      console.log(
        `  drift: ${entry.path} was ${entry.present_at_capture ? "present" : "absent"} at the pinned sha ` +
          `and is ${present ? "present" : "absent"} now; re-capture the baseline.`,
      );
    }
  }
  expect(true).toBe(true);
});
