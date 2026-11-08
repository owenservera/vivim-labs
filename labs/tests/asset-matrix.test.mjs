// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-004: the reusable asset matrix.
 * @hq:owner labs
 * @hq:task LABS-W3-004
 * @hq:why an asset that is listed but not owned is drift waiting to happen. Every row names the owner that
 *   holds it, and no row claims a verification it has not earned.
 * @hq:see labs/assets/matrix.json
 */

import { expect, test } from "bun:test";

import { readSeed, siblingFileExists } from "./_helpers.mjs";

const matrix = readSeed("assets/matrix.json");
const LADDER = [
  "discovered",
  "specified",
  "unit verified",
  "integrated",
  "environment verified",
  "production verified",
];

test("every asset names an owner outside Labs and a verification state on the ladder", () => {
  for (const asset of matrix.assets) {
    expect(asset.id.length).toBeGreaterThan(0);
    expect(asset.type.length).toBeGreaterThan(0);
    expect(asset.canonical_owner_repo).not.toBe("vivim-labs");
    expect(asset.canonical_owner_path.length).toBeGreaterThan(0);
    expect(LADDER).toContain(asset.verification);
    expect(asset.trust_note.length).toBeGreaterThan(0);
  }
});

test("nothing in the matrix is verified yet, and the manifest says so", () => {
  const verified = matrix.assets.filter((asset) => asset.verification !== "specified" && asset.verification !== "discovered");
  expect(verified.length).toBe(0);
  expect(matrix.counts.verified).toBe(0);
  expect(matrix.counts.owned_outside_labs).toBe(matrix.assets.length);
  expect(matrix.counts.assets).toBe(matrix.assets.length);
});

test("ids are unique", () => {
  const ids = matrix.assets.map((asset) => asset.id);
  expect(new Set(ids).size).toBe(ids.length);
});

test("each linked asset file exists in its owning repository", () => {
  let checked = 0;
  for (const asset of matrix.assets) {
    const present = siblingFileExists(asset.canonical_owner_repo, asset.canonical_owner_path);
    if (present === null) continue;
    expect(present).toBe(true);
    checked += 1;
  }
  expect(checked).toBeGreaterThan(0);
});
