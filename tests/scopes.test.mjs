// @ts-check
/**
 * @hq:context Acceptance for write-scope conflict detection: two items may never claim the same file.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why "Disjoint scopes are mandatory" is what makes parallel Labs work safe. A scope overlap is a
 *   silent data race between two workers, so it has to be found before the work starts, not after.
 * @hq:see labs/schema.mjs detectScopeConflicts
 */

import { expect, test } from "bun:test";

import { detectScopeConflicts, pathsOverlap } from "../labs/schema.mjs";

test("two items with overlapping allowed paths are detected", () => {
  const items = [
    { task_id: "LABS-W3-001", allowed_paths: ["/labs/corpus/nl/**", "/labs/tests/a.test.mjs"] },
    { task_id: "LABS-W3-002", allowed_paths: ["/labs/corpus/nl/ambiguity/cases.json"] },
  ];
  const conflicts = detectScopeConflicts(items);
  expect(conflicts.length).toBeGreaterThan(0);
  expect(conflicts[0].task_id).toBe("LABS-W3-001");
  expect(conflicts[0].other_task_id).toBe("LABS-W3-002");
  expect(conflicts[0].path).toBe("/labs/corpus/nl/**");
  expect(conflicts[0].other_path).toBe("/labs/corpus/nl/ambiguity/cases.json");
});

test("two items with the same path are detected", () => {
  const items = [
    { task_id: "LABS-W3-001", allowed_paths: ["/work/work-items.json"] },
    { task_id: "LABS-W3-002", allowed_paths: ["/work/work-items.json"] },
  ];
  expect(detectScopeConflicts(items).length).toBe(1);
});

test("disjoint items are not reported", () => {
  const items = [
    { task_id: "LABS-W3-001", allowed_paths: ["/labs/corpus/nl/typed-ir/**"] },
    { task_id: "LABS-W3-002", allowed_paths: ["/labs/corpus/nl/ambiguity/**"] },
    { task_id: "LABS-W3-003", allowed_paths: ["/labs/tests/nl-typed-ir.test.mjs"] },
  ];
  expect(detectScopeConflicts(items).length).toBe(0);
});

test("path overlap is segment aware, not string aware", () => {
  expect(pathsOverlap("/labs/corpus", "/labs/corpus/nl")).toBe(true);
  expect(pathsOverlap("/labs/corpus", "/labs/corpus2")).toBe(false);
  expect(pathsOverlap("/labs/a/b", "/labs/a/b/c")).toBe(true);
  expect(pathsOverlap("/labs/a/b", "/labs/c")).toBe(false);
});
