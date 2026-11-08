// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-003: the federated catalogue intake must link owners, not copy them.
 * @hq:owner labs
 * @hq:task LABS-W3-003
 * @hq:why the program doc is explicit: "Do not merge into one giant JSON file, monolithic DB or another
 *   independent authoritative registry" (docs/ENGINEERING-EVIDENCE-FACTORY.md:50). A catalogue that copies
 *   an owner's content is a second registry wearing a different name, so the test refuses copied payloads.
 * @hq:see labs/catalogue/candidates.json
 * @hq:see docs/ENGINEERING-EVIDENCE-FACTORY.md:48-54
 */

import { expect, test } from "bun:test";

import { readSeed, siblingFileExists } from "./_helpers.mjs";

const catalogue = readSeed("catalogue/candidates.json");
const LADDER = [
  "discovered",
  "specified",
  "unit verified",
  "integrated",
  "environment verified",
  "production verified",
];

test("every entry links a canonical owner and a source path", () => {
  for (const entry of catalogue.entries) {
    expect(entry.id.length).toBeGreaterThan(0);
    expect(catalogue.domains).toContain(entry.domain);
    expect(entry.canonical_owner_repo.length).toBeGreaterThan(0);
    expect(entry.canonical_owner_path.startsWith("/")).toBe(false);
    expect(entry.canonical_owner_path.length).toBeGreaterThan(0);
    expect(entry.source_registry.length).toBeGreaterThan(0);
    expect(LADDER).toContain(entry.capability_maturity);
    expect(LADDER).toContain(entry.evidence.level);
    expect(entry.evidence.labs_task_id.startsWith("LABS-W3-")).toBe(true);
  }
});

test("the catalogue links owners instead of copying their content", () => {
  const copyableKeys = ["content", "payload", "definition", "schema_body", "source_text"];
  for (const entry of catalogue.entries) {
    for (const key of copyableKeys) {
      expect(entry[key]).toBeUndefined();
    }
    // A linked entry is a pointer, so its whole body stays small.
    expect(JSON.stringify(entry).length).toBeLessThan(900);
  }
});

test("ids are unique and the declared counts are true", () => {
  const ids = catalogue.entries.map((entry) => entry.id);
  expect(new Set(ids).size).toBe(ids.length);
  expect(catalogue.counts.candidates).toBe(catalogue.entries.length);
  expect(catalogue.counts.linked_owners).toBe(catalogue.entries.length);
  expect(catalogue.counts.duplicate_ids).toBe(0);
});

test("each linked owner path really exists in the owning repository", () => {
  let checked = 0;
  for (const entry of catalogue.entries) {
    const present = siblingFileExists(entry.canonical_owner_repo, entry.canonical_owner_path);
    if (present === null) continue; // the repository is not on this machine: not measured, not failed
    expect(present).toBe(true);
    checked += 1;
  }
  expect(checked).toBeGreaterThan(0);
});
