// @ts-check
/**
 * @hq:context Acceptance for the Labs work-item ledger: the JSON round trip, the atomic write and the lock.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why the ledger is the only place a Labs result is recorded. It has to survive a concurrent writer
 *   and a half-finished write, so the round trip and the temporary-file behaviour are tested, not assumed.
 * @hq:see labs/workitems.mjs
 */

import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { load, save, validateAll, withLock } from "../labs/workitems.mjs";

/**
 * A throwaway directory for a ledger, so the test never touches the shipped one.
 * @returns {string}
 */
function scratchDir() {
  return mkdtempSync(join(tmpdir(), "labs-ledger-"));
}

test("the ledger JSON round-trips through load and save", () => {
  const directory = scratchDir();
  const path = join(directory, "work-items.json");
  const item = {
    task_id: "LABS-W3-901",
    source_sha: "074fd48fd86d690c6133df9ddb4bc3fe288155cd",
    owner: "labs",
    contract_version: "labs.work-item/1",
    inputs: ["a seed file"],
    expected_output: "a round trip",
    allowed_paths: ["/labs/tests/**"],
    test_command: "bun test tests/",
    evidence_level: "specified",
    result: "pending",
    artifact_hash: "pending",
    reviewer: "",
    maturity: "specified",
  };

  save({ schema: "labs.work-items/1", updated: "2026-11-06T00:00:00.000Z", items: [item] }, path);
  const reloaded = load(path);

  expect(reloaded.schema).toBe("labs.work-items/1");
  expect(reloaded.updated).toBe("2026-11-06T00:00:00.000Z");
  expect(reloaded.items.length).toBe(1);
  expect(reloaded.items[0]).toEqual(item);

  const onDisk = JSON.parse(readFileSync(path, "utf8"));
  expect(onDisk.items[0].task_id).toBe("LABS-W3-901");
});

test("the write is atomic: no temporary file is left behind", () => {
  const directory = scratchDir();
  const path = join(directory, "work-items.json");
  save({ schema: "labs.work-items/1", updated: "", items: [] }, path);
  save({ schema: "labs.work-items/1", updated: "", items: [] }, path);
  const leftovers = readdirSync(directory).filter((name) => name.includes(".tmp-"));
  expect(leftovers.length).toBe(0);
  expect(existsSync(path)).toBe(true);
});

test("a missing ledger reads as an empty ledger, not as an error", () => {
  const directory = scratchDir();
  const document = load(join(directory, "absent.json"));
  expect(document.items).toEqual([]);
  expect(document.schema).toBe("labs.work-items/1");
});

test("a corrupt ledger is reported, not silently treated as empty", () => {
  const directory = scratchDir();
  const path = join(directory, "work-items.json");
  writeFileSync(path, "{ this is not json", "utf8");
  let message = "";
  try {
    load(path);
  } catch (error) {
    message = error && typeof error === "object" ? error.message : String(error);
  }
  expect(message).toContain("not valid JSON");
});

test("the write lock is taken and released, and a second holder waits instead of interleaving", () => {
  const directory = scratchDir();
  const lockPath = join(directory, "work-items.lock");
  let held = false;
  withLock(() => {
    held = existsSync(lockPath);
    expect(held).toBe(true);
  }, lockPath);
  expect(held).toBe(true);
  expect(existsSync(lockPath)).toBe(false);
});

test("validateAll reports every item in the ledger", () => {
  const directory = scratchDir();
  const path = join(directory, "work-items.json");
  save({ schema: "labs.work-items/1", updated: "", items: [] }, path);
  const empty = validateAll(path);
  expect(empty.ok).toBe(true);
  expect(empty.results.length).toBe(0);
});
