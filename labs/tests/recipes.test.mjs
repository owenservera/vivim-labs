// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-010: the exact command and help recipes must actually run.
 * @hq:owner labs
 * @hq:task LABS-W3-010
 * @hq:why a recipe that does not run is documentation with a command line drawn on it. This test executes
 *   every recipe in the repository and compares the real exit code with the recorded one.
 * @hq:see labs/recipes/commands.json
 */

import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";

import { REPO_ROOT, readSeed } from "./_helpers.mjs";

const recipes = readSeed("recipes/commands.json");

test("every recipe runs and returns the exit code it promised", () => {
  for (const recipe of recipes.recipes) {
    const run = spawnSync(recipe.command[0], recipe.command.slice(1), {
      cwd: recipes.cwd,
      encoding: "utf8",
      timeout: 120_000,
      windowsHide: true,
    });
    const stdout = typeof run.stdout === "string" ? run.stdout : "";
    expect(run.status).toBe(recipe.expected_exit_code);
    for (const needle of recipe.expected_stdout_contains) {
      expect(stdout.includes(needle)).toBe(true);
    }
  }
});

test("the recipe file describes itself honestly", () => {
  expect(recipes.counts.recipes).toBe(recipes.recipes.length);
  expect(recipes.counts.executable).toBe(recipes.recipes.length);
  expect(recipes.counts.passed).toBe(recipes.recipes.length);
  expect(recipes.cwd).toBe(REPO_ROOT.replace(/\\/g, "/"));
  for (const recipe of recipes.recipes) {
    expect(recipe.provenance.length).toBeGreaterThan(0);
    expect(recipe.why.length).toBeGreaterThan(0);
    expect(recipe.command.length).toBeGreaterThan(0);
  }
});
