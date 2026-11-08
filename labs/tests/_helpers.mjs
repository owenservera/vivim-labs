// @ts-check
/**
 * @hq:context Shared helpers for the per-item booster acceptance tests: reading a Labs seed artefact and
 *   resolving a sibling repository file for a read-only existence check.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why the booster tests must be able to say "the canonical file this row points at is really there".
 *   That is a read, never a write, and it is what separates a link from a wish.
 * @hq:see labs/tests
 */

import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** The labs directory. @type {string} */
export const LABS_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");

/** The repository root: the parent of `labs/`. @type {string} */
export const REPO_ROOT = join(LABS_ROOT, "..");

/** The sibling repositories on this machine, for read-only inspection. @type {Record<string, string>} */
export const SIBLING_ROOTS = {
  "vivim-ui": "C:/0-BlackBoxProject-0/vivim-ui",
  "repos_hq": "C:/0-BlackBoxProject-0/repos_hq",
  "vivim-web-harness": "C:/0-BlackBoxProject-0/vivim-web-harness",
};

/**
 * Read a Labs seed artefact.
 * @param {string} relativePath Path under `labs/`.
 * @returns {any}
 */
export function readSeed(relativePath) {
  return JSON.parse(readFileSync(join(LABS_ROOT, relativePath), "utf8"));
}

/**
 * Does a file exist in a sibling repository? Returns null when that repository is not on this machine, so
 * a test can record "not measured here" instead of a false failure.
 * @param {string} repo
 * @param {string} relativePath
 * @returns {boolean | null}
 */
export function siblingFileExists(repo, relativePath) {
  const root = SIBLING_ROOTS[repo];
  if (!root || !existsSync(root)) return null;
  return existsSync(join(root, relativePath));
}
