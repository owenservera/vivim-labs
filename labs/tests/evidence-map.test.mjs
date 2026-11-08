// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-011: the capability to evidence linking map.
 * @hq:owner labs
 * @hq:task LABS-W3-011
 * @hq:why the program doc asks for a queryable relation that is "not a new independent KB" and should
 *   "link to Repos HQ's canonical graph, not copy fact authority". The test refuses a map that holds
 *   authority inside Labs or that omits one of the seven links.
 * @hq:see labs/evidence/map/index.json
 * @hq:see docs/ENGINEERING-EVIDENCE-FACTORY.md:107-118
 */

import { expect, test } from "bun:test";

import { readSeed } from "./_helpers.mjs";

const map = readSeed("evidence/map/index.json");
const LADDER = [
  "discovered",
  "specified",
  "unit verified",
  "integrated",
  "environment verified",
  "production verified",
];

test("every row carries all seven links of the relation", () => {
  for (const row of map.rows) {
    expect(row.capability.length).toBeGreaterThan(0);
    expect(row.implementation.startsWith("/labs/")).toBe(true);
    expect(row.contract.repo.length).toBeGreaterThan(0);
    expect(row.contract.path.length).toBeGreaterThan(0);
    expect(row.consumer.repo.length).toBeGreaterThan(0);
    expect(row.test.startsWith("/labs/")).toBe(true);
    expect(LADDER).toContain(row.evidence.level);
    expect(row.evidence.task_id.startsWith("LABS-W3-")).toBe(true);
    expect(row.platform.repo.length).toBeGreaterThan(0);
  }
});

test("the map holds no authority of its own", () => {
  for (const row of map.rows) {
    expect(row.contract.repo).not.toBe("vivim-labs");
    expect(row.consumer.repo).not.toBe("vivim-labs");
    expect(row.platform.repo).not.toBe("vivim-labs");
  }
  expect(map.counts.authority_held_inside_labs).toBe(0);
  expect(map.counts.rows).toBe(map.rows.length);
  expect(map.relation).toBe("Capability -> Implementation -> Contract -> Consumer -> Test -> Evidence -> Platform");
});

test("the map points at the canonical graph instead of duplicating it", () => {
  expect(map.canonical_graph.repo).toBe("repos_hq");
  expect(map.canonical_graph.path.length).toBeGreaterThan(0);
  expect(map.known_gaps.length).toBeGreaterThan(0);
});

test("no row claims more than the ladder allows for a Labs artefact", () => {
  for (const row of map.rows) {
    expect(LADDER.indexOf(row.evidence.level)).toBeLessThanOrEqual(LADDER.indexOf("unit verified"));
  }
});
