// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-007: the HQ/G/H descriptor crosswalk between the two canonical owners.
 * @hq:owner labs
 * @hq:task LABS-W3-007
 * @hq:why the crosswalk is a link table between two owners. It must point at real files in both
 *   repositories, and it must not decide anything: the mismatch column records what is open.
 * @hq:see labs/evidence/descriptors/crosswalk.json
 */

import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";

import { readSeed, siblingFileExists } from "./_helpers.mjs";

const crosswalk = readSeed("evidence/descriptors/crosswalk.json");

/**
 * Read the HEAD of a sibling repository. `rev-parse` reads refs and writes nothing.
 * @param {string} root
 * @returns {string | null}
 */
function headOf(root) {
  const run = spawnSync("git", ["-C", root, "rev-parse", "HEAD"], {
    encoding: "utf8",
    timeout: 30_000,
    windowsHide: true,
  });
  if (run.status !== 0) return null;
  const value = typeof run.stdout === "string" ? run.stdout.trim() : "";
  return /^[0-9a-f]{40}$/.test(value) ? value : null;
}

test("the crosswalk names both authorities, and neither of them is Labs", () => {
  expect(crosswalk.meaning_authority.repo).toBe("repos_hq");
  expect(crosswalk.runtime_authority.repo).toBe("vivim-ui");
  expect(crosswalk.meaning_authority.owner.length).toBeGreaterThan(0);
  expect(crosswalk.runtime_authority.owner.length).toBeGreaterThan(0);
});

test("every row links a meaning surface and a runtime surface, and records what is open", () => {
  for (const row of crosswalk.rows) {
    expect(row.id.length).toBeGreaterThan(0);
    expect(row.meaning_ref).toContain(":");
    expect(row.runtime_ref).toContain(":");
    expect(row.mismatch.length).toBeGreaterThan(0);
    expect(row.labs_evidence.startsWith("LABS-W3-")).toBe(true);
    expect(row.contract_version.length).toBeGreaterThan(0);
  }
});

test("every linked surface exists in its owning repository", () => {
  let checked = 0;
  for (const row of crosswalk.rows) {
    for (const reference of [row.meaning_ref, row.runtime_ref]) {
      const separator = reference.indexOf(":");
      const repo = reference.slice(0, separator);
      const path = reference.slice(separator + 1);
      const present = siblingFileExists(repo, path);
      if (present === null) continue;
      expect(present).toBe(true);
      checked += 1;
    }
  }
  expect(checked).toBeGreaterThan(0);
});

test("both repositories can be read at a pinned commit without being written", () => {
  expect(headOf("C:/0-BlackBoxProject-0/repos_hq")).toMatch(/^[0-9a-f]{40}$/);
  expect(headOf("C:/0-BlackBoxProject-0/vivim-ui")).toMatch(/^[0-9a-f]{40}$/);
});
