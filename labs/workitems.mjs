// @ts-check
/**
 * @hq:context The Labs work-item ledger: one JSON file, an atomic write, a lockfile so two Labs workers
 *   never interleave a write, and query helpers over the records.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why a single file and not a database: the ledger is a work log for one repository, and the program
 *   doc forbids another independent authoritative registry (docs/ENGINEERING-EVIDENCE-FACTORY.md:49-50).
 *   A file can be read, diffed and moved by any reviewer without a toolchain.
 * @hq:invariant the file is written atomically: a temporary file is written and renamed over the target,
 *   so a reader never sees a half-written ledger and a crashed writer never truncates the last good copy.
 * @hq:invariant writes are serialized by a lockfile, so two concurrent Labs processes cannot interleave
 *   a read-modify-write and lose one item.
 * @hq:see labs/schema.mjs
 * @hq:see labs/cli.mjs
 */

import {
  closeSync,
  existsSync,
  mkdirSync,
  openSync,
  readFileSync,
  renameSync,
  statSync,
  unlinkSync,
  writeFileSync,
  writeSync,
} from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { validateWorkItem } from "./schema.mjs";

/** The repository root: the parent of `labs/`. @type {string} */
export const REPO_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");

/** The one ledger file. @type {string} */
export const WORK_ITEMS_PATH = join(REPO_ROOT, "work", "work-items.json");

/** The write lock. @type {string} */
export const LOCK_PATH = join(REPO_ROOT, "work", ".work-items.lock");

/** How long a lock may be held before another worker treats it as abandoned. @type {number} */
const STALE_LOCK_MS = 30_000;

/** How long a worker waits for the lock before it gives up. @type {number} */
const LOCK_WAIT_MS = 15_000;

/**
 * @typedef {Object} WorkItemDocument
 * @property {string} schema
 * @property {string} updated
 * @property {any[]} items
 */

/**
 * Sleep without spinning the event loop. Used only while waiting for the lockfile.
 * @param {number} ms
 * @returns {void}
 */
function sleepSync(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

/**
 * Take the write lock. The lock is an exclusively created file; the holder writes its pid and start time
 * into it so a stalled holder can be recognised and broken by a later worker.
 * @param {{waitMs?: number, staleMs?: number, lockPath?: string}} [options]
 * @returns {void}
 */
function acquireLock(options) {
  const waitMs = (options && options.waitMs) || LOCK_WAIT_MS;
  const staleMs = (options && options.staleMs) || STALE_LOCK_MS;
  const lockPath = (options && options.lockPath) || LOCK_PATH;
  mkdirSync(dirname(lockPath), { recursive: true });
  const deadline = Date.now() + waitMs;
  for (;;) {
    try {
      const handle = openSync(lockPath, "wx");
      try {
        writeSync(
          handle,
          JSON.stringify({ pid: process.pid, at: new Date().toISOString(), lock: lockPath }),
        );
      } finally {
        closeSync(handle);
      }
      return;
    } catch (error) {
      const code = error && typeof error === "object" ? error.code : "";
      if (code !== "EEXIST") throw error;
      try {
        const stats = statSync(lockPath);
        if (Date.now() - stats.mtimeMs > staleMs) {
          unlinkSync(lockPath);
          continue;
        }
      } catch {
        // The lock vanished between the failed create and the stat; try again immediately.
      }
      if (Date.now() > deadline) {
        throw new Error(
          `timed out after ${waitMs}ms waiting for the work-items lock at ${lockPath}. ` +
            `Another Labs worker holds it.`,
        );
      }
      sleepSync(40);
    }
  }
}

/**
 * Release the write lock. A lock that is already gone is not an error: the holder's work is done either way.
 * @param {string} [lockPath]
 * @returns {void}
 */
function releaseLock(lockPath) {
  try {
    unlinkSync(lockPath || LOCK_PATH);
  } catch {
    // Already released or removed by a stale-lock breaker.
  }
}

/**
 * Run a function while holding the write lock.
 * @template T
 * @param {() => T} action
 * @param {string} [lockPath]
 * @returns {T}
 */
export function withLock(action, lockPath) {
  const target = lockPath || LOCK_PATH;
  acquireLock({ lockPath: target });
  try {
    return action();
  } finally {
    releaseLock(target);
  }
}

/**
 * Read the ledger. A missing file is an empty ledger, not an error: the file is rebuildable.
 * @param {string} [path]
 * @returns {WorkItemDocument}
 */
export function load(path) {
  const target = path || WORK_ITEMS_PATH;
  if (!existsSync(target)) {
    return { schema: "labs.work-items/1", updated: "", items: [] };
  }
  const raw = readFileSync(target, "utf8");
  if (raw.trim().length === 0) {
    return { schema: "labs.work-items/1", updated: "", items: [] };
  }
  let parsed;
  try {
    parsed = JSON.parse(raw);
  } catch (error) {
    throw new Error(
      `work-items ledger at ${target} is not valid JSON: ${
        error && typeof error === "object" ? error.message : String(error)
      }`,
    );
  }
  if (!parsed || typeof parsed !== "object" || !Array.isArray(parsed.items)) {
    throw new Error(`work-items ledger at ${target} has no "items" array`);
  }
  return {
    schema: typeof parsed.schema === "string" ? parsed.schema : "labs.work-items/1",
    updated: typeof parsed.updated === "string" ? parsed.updated : "",
    items: parsed.items,
  };
}

/**
 * Write the ledger atomically: a temporary file in the same directory, then a rename over the target.
 * A rename on the same volume is atomic, so a reader sees either the old file or the new one.
 * @param {WorkItemDocument} document
 * @param {string} [path]
 * @returns {WorkItemDocument}
 */
export function save(document, path) {
  const target = path || WORK_ITEMS_PATH;
  const payload = {
    schema: document.schema || "labs.work-items/1",
    updated: document.updated || new Date().toISOString(),
    items: Array.isArray(document.items) ? document.items : [],
  };
  mkdirSync(dirname(target), { recursive: true });
  const temporary = `${target}.tmp-${process.pid}-${Date.now()}`;
  writeFileSync(temporary, `${JSON.stringify(payload, null, 2)}\n`, "utf8");
  renameSync(temporary, target);
  return payload;
}

/**
 * Add one work item. The read-modify-write runs under the lock.
 * @param {any} item
 * @param {string} [path] The ledger file to write. Defaults to the Labs ledger.
 * @returns {any} the item that was stored
 */
export function add(item, path) {
  const target = path || WORK_ITEMS_PATH;
  const lockPath = path ? `${path}.lock` : LOCK_PATH;
  return withLock(() => {
    const document = load(target);
    const existing = document.items.findIndex(
      (candidate) => candidate && item && candidate.task_id === item.task_id,
    );
    if (existing >= 0) document.items[existing] = item;
    else document.items.push(item);
    document.updated = new Date().toISOString();
    save(document, target);
    return item;
  }, lockPath);
}

/**
 * Replace the fields of one work item. The read-modify-write runs under the lock.
 * @param {string} taskId
 * @param {(item: any) => any} mutate
 * @param {string} [path] The ledger file to write. Defaults to the Labs ledger.
 * @returns {any | null} the updated item, or null when the task id is unknown
 */
export function update(taskId, mutate, path) {
  const target = path || WORK_ITEMS_PATH;
  const lockPath = path ? `${path}.lock` : LOCK_PATH;
  return withLock(() => {
    const document = load(target);
    const index = document.items.findIndex((candidate) => candidate && candidate.task_id === taskId);
    if (index < 0) return null;
    document.items[index] = mutate(document.items[index]);
    document.updated = new Date().toISOString();
    save(document, target);
    return document.items[index];
  }, lockPath);
}

/**
 * List work items, optionally filtered.
 * @param {{factory?: string, wave?: string, maturity?: string, owner?: string, evidence_level?: string}} [filter]
 * @param {string} [path] The ledger file to read. Defaults to the Labs ledger.
 * @returns {any[]}
 */
export function list(filter, path) {
  const options = filter || {};
  return load(path).items.filter((item) => {
    if (!item || typeof item !== "object") return false;
    if (options.factory && item.factory !== options.factory) return false;
    if (options.wave && item.wave !== options.wave) return false;
    if (options.maturity && item.maturity !== options.maturity) return false;
    if (options.owner && item.owner !== options.owner) return false;
    if (options.evidence_level && item.evidence_level !== options.evidence_level) return false;
    return true;
  });
}

/**
 * Find one work item by task id.
 * @param {string} taskId
 * @param {string} [path] The ledger file to read. Defaults to the Labs ledger.
 * @returns {any | null}
 */
export function find(taskId, path) {
  const found = load(path).items.find((item) => item && item.task_id === taskId);
  return found === undefined ? null : found;
}

/**
 * Validate every work item in the ledger.
 * @param {string} [path] The ledger file to read. Defaults to the Labs ledger.
 * @returns {{ok: boolean, results: {task_id: string, ok: boolean, errors: any[], warnings: any[]}[]}}
 */
export function validateAll(path) {
  const results = load(path).items.map((item) => {
    const verdict = validateWorkItem(item);
    return {
      task_id: item && item.task_id ? String(item.task_id) : "(no task_id)",
      ok: verdict.ok,
      errors: verdict.errors,
      warnings: verdict.warnings,
    };
  });
  return { ok: results.every((result) => result.ok), results };
}
