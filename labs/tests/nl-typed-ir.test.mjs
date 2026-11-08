// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-001: the typed-IR conformance corpus slice must be internally honest.
 * @hq:owner labs
 * @hq:task LABS-W3-001
 * @hq:why a corpus that lies about its own counts is worse than no corpus. This test recomputes every
 *   count from the cases themselves, so the manifest cannot claim coverage it does not have.
 * @hq:see labs/fixtures/nl/typed-ir/cases.json
 */

import { expect, test } from "bun:test";

import { readSeed } from "./_helpers.mjs";

const corpus = readSeed("fixtures/nl/typed-ir/cases.json");
const ALLOWED_OUTCOMES = ["accepted", "clarification", "refusal"];

test("every case carries a typed expectation or a stated refusal", () => {
  for (const item of corpus.cases) {
    expect(typeof item.id).toBe("string");
    expect(item.utterance.length).toBeGreaterThan(0);
    expect(ALLOWED_OUTCOMES).toContain(item.expected_outcome);
    if (item.expected_outcome === "accepted") {
      expect(item.expected_command).toBeTruthy();
      expect(typeof item.expected_command.op).toBe("string");
      expect(typeof item.expected_command.version).toBe("number");
    } else if (item.expected_outcome === "refusal") {
      expect(item.refusal_reason.length).toBeGreaterThan(0);
    } else {
      expect(item.clarification_prompt.length).toBeGreaterThan(0);
    }
  }
});

test("the manifest counts match the cases, so the corpus cannot inflate itself", () => {
  const cases = corpus.cases;
  const utterances = new Set(cases.map((item) => item.utterance));
  const independent = cases.filter((item) => item.label_source !== item.generator);
  const executable = cases.filter((item) => item.expected_command !== null && item.expected_command !== undefined);

  expect(corpus.counts.generated).toBe(cases.length);
  expect(corpus.counts.deduplicated).toBe(utterances.size);
  expect(corpus.counts.independently_labelled).toBe(independent.length);
  expect(corpus.counts.executable).toBe(executable.length);
  expect(corpus.counts.passed).toBe(corpus.counts.executable);
  expect(utterances.size).toBe(cases.length);
  expect(independent.length).toBe(cases.length);
});

test("no case mints an authority, and every accepted command names a known namespace", () => {
  const namespaces = ["workspace.panel.move"];
  for (const item of corpus.cases) {
    if (item.expected_command) {
      expect(namespaces).toContain(item.expected_command.op);
      expect(item.expected_command.version).toBe(1);
    }
    const text = `${item.utterance} ${item.refusal_reason || ""}`;
    expect(text.toLowerCase().includes("grant me")).toBe(false);
  }
});
