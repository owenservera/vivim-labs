// @ts-check
/**
 * @hq:context Acceptance for LABS-W3-005: the adversarial case slice.
 * @hq:owner labs
 * @hq:task LABS-W3-005
 * @hq:why every case must expect a denial and must not be mistaken for a live permission proof. The
 *   program doc is blunt: "no synthetic proof inflated to live permission".
 * @hq:see labs/adversarial/cases.json
 * @hq:see docs/WAVE-BOOSTERS.md:28
 */

import { expect, test } from "bun:test";

import { readSeed } from "./_helpers.mjs";

const cases = readSeed("adversarial/cases.json");

test("every adversarial case expects a denial and states the reason", () => {
  for (const item of cases.cases) {
    expect(item.expected_outcome).toBe("denied");
    expect(item.reason.length).toBeGreaterThan(0);
    expect(item.attack_class.length).toBeGreaterThan(0);
    expect(item.actor_state.length).toBeGreaterThan(0);
  }
});

test("the slice is not presented as a live permission proof", () => {
  expect(cases.live_proof).toBe(false);
  expect(cases.live_proof_note.length).toBeGreaterThan(0);
  expect(cases.counts.expected_live_permission_proof).toBe(0);
  expect(cases.counts.cases).toBe(cases.cases.length);
  expect(cases.counts.expect_denial).toBe(cases.cases.length);
});

test("the attack classes cover the distinct failure families", () => {
  const classes = new Set(cases.cases.map((item) => item.attack_class));
  for (const expected of ["cross-owner leak", "forged receipt", "replay", "privilege escalation", "revocation", "prompt injection"]) {
    expect(classes.has(expected)).toBe(true);
  }
});

test("ids are unique", () => {
  const ids = cases.cases.map((item) => item.id);
  expect(new Set(ids).size).toBe(ids.length);
});
