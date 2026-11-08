// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-008: the provenance, redaction and tombstone corpus.
 * @hq:owner labs
 * @hq:task LABS-W3-008
 * @hq:why provenance without redaction leaks, and redaction without a tombstone silently drops knowledge.
 *   The test checks both, and checks that no secret-looking string reached the artifact.
 * @hq:see labs/fixtures/provenance/cases.json
 */

import { expect, test } from "bun:test";

import { readSeed } from "./_helpers.mjs";

const corpus = readSeed("fixtures/provenance/cases.json");
const SECRET_PATTERNS = [
  /sk-[A-Za-z0-9]{16,}/,
  /AKIA[0-9A-Z]{16}/,
  /ghp_[A-Za-z0-9]{20,}/,
  /-----BEGIN [A-Z ]*PRIVATE KEY-----/,
  /Bearer\s+[A-Za-z0-9._-]{20,}/,
];

test("every case carries provenance: source, actor and time", () => {
  for (const item of corpus.cases) {
    expect(item.source.length).toBeGreaterThan(0);
    expect(item.actor.length).toBeGreaterThan(0);
    expect(item.at).toMatch(/^\d{4}-\d{2}-\d{2}T/);
    expect(["public", "internal", "private"].includes(item.sensitivity)).toBe(true);
    expect(["live", "tombstoned"].includes(item.state)).toBe(true);
  }
});

test("a deleted case keeps a tombstone instead of vanishing", () => {
  const tombstoned = corpus.cases.filter((item) => item.state === "tombstoned");
  expect(tombstoned.length).toBeGreaterThan(0);
  for (const item of tombstoned) {
    expect(item.tombstone_reason.length).toBeGreaterThan(0);
  }
});

test("a private case states what was redacted", () => {
  const privateCases = corpus.cases.filter((item) => item.sensitivity === "private");
  expect(privateCases.length).toBeGreaterThan(0);
  for (const item of privateCases) {
    expect(Array.isArray(item.redacted)).toBe(true);
    expect(item.redacted.length).toBeGreaterThan(0);
  }
});

test("no secret-looking string reached the artifact", () => {
  const text = JSON.stringify(corpus);
  for (const pattern of SECRET_PATTERNS) {
    expect(pattern.test(text)).toBe(false);
  }
});

test("the manifest counts match the cases", () => {
  const cases = corpus.cases;
  const withRedaction = cases.filter((item) => Array.isArray(item.redacted) && item.redacted.length > 0);
  const tombstoned = cases.filter((item) => item.state === "tombstoned");
  expect(corpus.counts.cases).toBe(cases.length);
  expect(corpus.counts.with_provenance).toBe(cases.length);
  expect(corpus.counts.with_redaction).toBe(withRedaction.length);
  expect(corpus.counts.tombstoned).toBe(tombstoned.length);
  expect(corpus.counts.passed).toBe(cases.length);
});
