// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-009: the state and failure simulation matrix.
 * @hq:owner labs
 * @hq:task LABS-W3-009
 * @hq:why a fault row without an invariant is a wish, and a fault matrix that claims device evidence is a
 *   lie the program doc names explicitly. Every row keeps its ceiling honest.
 * @hq:see labs/faults/matrix.json
 */

import { expect, test } from "bun:test";

import { readSeed } from "./_helpers.mjs";

const matrix = readSeed("faults/matrix.json");

test("every row states the fault, the invariant and the safe outcome", () => {
  for (const row of matrix.rows) {
    expect(row.id.length).toBeGreaterThan(0);
    expect(row.injection.length).toBeGreaterThan(0);
    expect(row.invariant.length).toBeGreaterThan(0);
    expect(row.expected_safe_outcome.length).toBeGreaterThan(0);
  }
});

test("the matrix covers the four W3 fault families", () => {
  const families = new Set(matrix.rows.map((row) => row.fault_class));
  for (const expected of ["wal", "cas", "gap", "restart"]) {
    expect(families.has(expected)).toBe(true);
  }
  expect(matrix.counts.rows).toBe(matrix.rows.length);
  expect(matrix.counts.wal).toBe(matrix.rows.filter((row) => row.fault_class === "wal").length);
  expect(matrix.counts.cas).toBe(matrix.rows.filter((row) => row.fault_class === "cas").length);
  expect(matrix.counts.gap).toBe(matrix.rows.filter((row) => row.fault_class === "gap").length);
  expect(matrix.counts.restart).toBe(matrix.rows.filter((row) => row.fault_class === "restart").length);
});

test("no row claims device or production evidence", () => {
  expect(matrix.device_verified).toBe(false);
  expect(matrix.maturity_ceiling).toBe("unit verified");
  for (const row of matrix.rows) {
    expect(row.environment).toBe("local-bun-fixture");
  }
  expect(matrix.known_gaps.length).toBeGreaterThan(0);
});

test("ids are unique", () => {
  const ids = matrix.rows.map((row) => row.id);
  expect(new Set(ids).size).toBe(ids.length);
});
