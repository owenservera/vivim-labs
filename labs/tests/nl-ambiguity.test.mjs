// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-002: the ambiguity, clarification and refusal slice.
 * @hq:owner labs
 * @hq:task LABS-W3-002
 * @hq:why an ambiguity corpus that quietly accepts ambiguous text is the exact failure the program doc
 *   names: a wrong target and a wrong scope are recorded outcomes, not accidents.
 * @hq:see labs/fixtures/nl/ambiguity/cases.json
 */

import { expect, test } from "bun:test";

import { readSeed } from "./_helpers.mjs";

const corpus = readSeed("fixtures/nl/ambiguity/cases.json");

test("every case is a clarification or a refusal, never a silent acceptance", () => {
  for (const item of corpus.cases) {
    expect(["clarification", "refusal"]).toContain(item.expected_outcome);
    expect(item.expected_command).toBeUndefined();
    if (item.expected_outcome === "clarification") {
      expect(item.ambiguity_dimension.length).toBeGreaterThan(0);
      expect(item.clarification_prompt.length).toBeGreaterThan(0);
    } else {
      expect(item.refusal_reason.length).toBeGreaterThan(0);
    }
  }
});

test("the manifest counts match the cases", () => {
  const cases = corpus.cases;
  const utterances = new Set(cases.map((item) => item.utterance));
  const clarifications = cases.filter((item) => item.expected_outcome === "clarification");
  const refusals = cases.filter((item) => item.expected_outcome === "refusal");
  const independent = cases.filter((item) => item.label_source !== corpus.generator);

  expect(corpus.counts.generated).toBe(cases.length);
  expect(corpus.counts.deduplicated).toBe(utterances.size);
  expect(corpus.counts.independently_labelled).toBe(independent.length);
  expect(corpus.counts.ambiguity_cases).toBe(clarifications.length);
  expect(corpus.counts.refusal_cases).toBe(refusals.length);
  expect(corpus.counts.passed).toBe(cases.length);
  expect(independent.length).toBe(cases.length);
});
