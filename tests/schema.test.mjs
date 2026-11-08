// @ts-check
/**
 * @hq:context Acceptance for the Labs work-item model: what must be accepted, and what must be refused.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why the model only earns its keep when it refuses. These cases are the refusals that stop a
 *   fixture-only result from being described as real device, browser or Host proof.
 * @hq:see labs/schema.mjs
 * @hq:see docs/ENGINEERING-EVIDENCE-FACTORY.md:62-70
 */

import { expect, test } from "bun:test";

import { MATURITY_LADDER, validateMaturityClaim, validateWorkItem } from "../labs/schema.mjs";

/**
 * A work item that is valid in every respect, so each test can break exactly one thing.
 * @param {Record<string, unknown>} [overrides]
 * @returns {any}
 */
function validItem(overrides) {
  return {
    task_id: "LABS-W3-900",
    source_sha: "074fd48fd86d690c6133df9ddb4bc3fe288155cd",
    owner: "labs",
    contract_version: "labs.work-item/1",
    inputs: ["labs/corpus/nl/typed-ir/cases.json"],
    expected_output: "a checked corpus slice with honest counts",
    allowed_paths: ["/labs/corpus/nl/typed-ir/**"],
    test_command: "bun test labs/tests/nl-typed-ir.test.mjs",
    evidence_level: "specified",
    result: "pending",
    artifact_hash: "pending",
    reviewer: "",
    maturity: "specified",
    repo: "vivim-labs",
    ...(overrides || {}),
  };
}

/**
 * The codes of every error the validator reported.
 * @param {any} verdict
 * @returns {string[]}
 */
function codes(verdict) {
  return verdict.errors.map((issue) => issue.code);
}

test("a valid work item validates", () => {
  const verdict = validateWorkItem(validItem());
  expect(verdict.ok).toBe(true);
  expect(verdict.errors.length).toBe(0);
});

test("a missing required field is refused", () => {
  const item = validItem();
  delete item.reviewer;
  const verdict = validateWorkItem(item);
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("missing_field");
});

test("a maturity claim of integrated on unit-verified evidence is refused", () => {
  const verdict = validateWorkItem(
    validItem({ maturity: "integrated", evidence_level: "unit verified" }),
  );
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("maturity_claim_exceeds_evidence");
  const issue = verdict.errors.find((entry) => entry.code === "maturity_claim_exceeds_evidence");
  expect(issue.message).toContain("Fixture-only");
});

test("environment verified and production verified are refused on unit-verified evidence too", () => {
  for (const claimed of ["environment verified", "production verified"]) {
    const verdict = validateWorkItem(validItem({ maturity: claimed, evidence_level: "unit verified" }));
    expect(verdict.ok).toBe(false);
    expect(codes(verdict)).toContain("maturity_claim_exceeds_evidence");
  }
});

test("an allowed path into a TierFlow Wave 3 reserved lane is refused", () => {
  const verdict = validateWorkItem(
    validItem({
      repo: "vivim-ui",
      allowed_paths: ["/src/visual-runtime/document/wave3-action-semantics.ts"],
    }),
  );
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("forbidden_path");
  const issue = verdict.errors.find((entry) => entry.code === "forbidden_path");
  expect(issue.message).toContain("tierflow-wave3-lane-a");
  expect(issue.message).toContain("WAVE-3-OWNERSHIP.md");
});

test("an allowed path into a repos_hq shared-serial file is refused", () => {
  const verdict = validateWorkItem(
    validItem({ repo: "repos_hq", allowed_paths: ["/hq/bin/hq.mjs"] }),
  );
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("forbidden_path");
});

test("an allowed path that is not a repository-anchored absolute path is refused", () => {
  for (const bad of ["labs/schema.mjs", "C:/0-BlackBoxProject-0/vivim-ui/src/x.ts", "/labs/../etc/passwd"]) {
    const verdict = validateWorkItem(validItem({ allowed_paths: [bad] }));
    expect(verdict.ok).toBe(false);
    expect(codes(verdict)).toContain("allowed_path_not_repo_absolute");
  }
});

test("an evidence level that is not on the ladder is refused", () => {
  const verdict = validateWorkItem(validItem({ evidence_level: "probably fine" }));
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("evidence_level_unknown");
});

test("an empty test command at unit-verified evidence is refused", () => {
  const verdict = validateWorkItem(
    validItem({ evidence_level: "unit verified", maturity: "unit verified", test_command: "" }),
  );
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("test_command_required");
});

test("a missing reviewer at integrated evidence is refused", () => {
  const verdict = validateWorkItem(
    validItem({ evidence_level: "integrated", maturity: "integrated", reviewer: "" }),
  );
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("reviewer_required");
});

test("a missing reviewer at specified evidence is allowed", () => {
  const verdict = validateWorkItem(validItem({ reviewer: "" }));
  expect(verdict.ok).toBe(true);
});

test("an allowed path outside the Labs write scope is refused", () => {
  const verdict = validateWorkItem(validItem({ allowed_paths: ["/src/visual-runtime/render/dom.ts"] }));
  expect(verdict.ok).toBe(false);
  expect(codes(verdict)).toContain("write_scope_outside_labs");
});

test("the maturity ladder is ordered and complete", () => {
  expect(MATURITY_LADDER.length).toBe(6);
  expect(MATURITY_LADDER.map((step) => step.stage)).toEqual([
    "discovered",
    "specified",
    "unit verified",
    "integrated",
    "environment verified",
    "production verified",
  ]);
  MATURITY_LADDER.forEach((step, index) => {
    expect(step.level).toBe(index + 1);
  });
});

test("validateMaturityClaim is the single rule that blocks an inflated claim", () => {
  const claim = validateMaturityClaim({ claimed: "production verified", evidence_level: "unit verified" });
  expect(claim.ok).toBe(false);
  expect(claim.errors[0].code).toBe("maturity_claim_exceeds_evidence");
  expect(claim.claimed_level).toBe(6);
  expect(claim.evidence_level_number).toBe(3);

  const honest = validateMaturityClaim({ claimed: "unit verified", evidence_level: "unit verified" });
  expect(honest.ok).toBe(true);
});
