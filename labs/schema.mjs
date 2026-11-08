// @ts-check
/**
 * @hq:context The Labs work-item and evidence-record model: the twelve required record fields, the
 *   six-stage maturity ladder, the evidence levels, the twelve factory ids and the nine booster waves.
 *   Every Labs work item is validated against this model and nothing else.
 * @hq:owner labs
 * @hq:task LABS-W3-000
 * @hq:why this is Labs-local scaffolding and NOT a platform contract. The field list is taken verbatim from
 *   docs/ENGINEERING-EVIDENCE-FACTORY.md ("Candidate task evidence record"), which itself says it is "a
 *   discussion-level data list, not a mandatory schema". Labs validates it against itself; no other system
 *   may read a Labs work item as authority, and Labs cannot mint authority from one.
 * @hq:invariant the maturity claim can never exceed the evidence held. validateMaturityClaim is the only
 *   place a claim is judged, so a fixture-only result can never be described as real device or Host proof.
 * @hq:invariant the forbidden path list is data (labs/forbidden-paths.mjs), never a rule hardcoded here, so
 *   an ownership change in the owning repository is visible without editing Labs.
 * @hq:see labs/forbidden-paths.mjs
 * @hq:see docs/ENGINEERING-EVIDENCE-FACTORY.md:62-70,94
 * @hq:see docs/WAVE-BOOSTERS.md:34,41
 */

import {
  FORBIDDEN_PATHS,
  KNOWN_REPOS,
  findForbiddenPath,
  isLabsWritablePath,
  isRepoAbsolutePath,
} from "./forbidden-paths.mjs";

/**
 * The twelve fields every Labs work item carries. This is the evidence record from
 * docs/ENGINEERING-EVIDENCE-FACTORY.md:94, in the order that record lists them.
 * @type {readonly string[]}
 */
export const WORK_ITEM_FIELDS = Object.freeze([
  "task_id",
  "source_sha",
  "owner",
  "contract_version",
  "inputs",
  "expected_output",
  "allowed_paths",
  "test_command",
  "evidence_level",
  "result",
  "artifact_hash",
  "reviewer",
]);

/**
 * Fields that are not part of the twelve but that a work item may carry. `maturity` is the stage the item
 * currently claims; `maturity_target` is the stage the item is aiming at. A target is a plan and is not
 * judged against the evidence; a claim is.
 * @type {readonly string[]}
 */
export const OPTIONAL_WORK_ITEM_FIELDS = Object.freeze([
  "factory",
  "wave",
  "repo",
  "maturity",
  "maturity_target",
  "evidence_ceiling",
  "booster_guidance",
  "results",
  "last_result",
  "updated",
]);

/**
 * The six-stage capability maturity ladder from docs/ENGINEERING-EVIDENCE-FACTORY.md:62-68. The order is
 * the ladder: a later stage is only reachable by holding the evidence of the earlier one.
 * @type {readonly {level: number, stage: string, meaning: string}[]}
 */
export const MATURITY_LADDER = Object.freeze([
  Object.freeze({
    level: 1,
    stage: "discovered",
    meaning: "Item identified. Nothing has been specified or executed yet.",
  }),
  Object.freeze({
    level: 2,
    stage: "specified",
    meaning: "Contract, inputs, outputs, risk and boundaries are defined.",
  }),
  Object.freeze({
    level: 3,
    stage: "unit verified",
    meaning: "Isolated deterministic tests pass. Mocks and fixtures do not count as integration.",
  }),
  Object.freeze({
    level: 4,
    stage: "integrated",
    meaning: "Actual neighboring production modules interoperate. Mocked neighbors do not count.",
  }),
  Object.freeze({
    level: 5,
    stage: "environment verified",
    meaning: "The relevant real browser, OS or device executed the item.",
  }),
  Object.freeze({
    level: 6,
    stage: "production verified",
    meaning: "Owner-authorized release and real operational receipts exist.",
  }),
]);

/**
 * The evidence levels a work item may hold. They are the ladder stage names: evidence is claimed by
 * naming the stage that was actually reached.
 * @type {readonly string[]}
 */
export const EVIDENCE_LEVELS = Object.freeze(MATURITY_LADDER.map((step) => step.stage));

/**
 * The twelve factories from docs/ENGINEERING-EVIDENCE-FACTORY.md:18-31, with the canonical owner each
 * one answers to. Labs proposes and verifies inside these factories; it never owns the canonical surface.
 * @type {readonly {id: string, name: string, priority: string, proposed_scale: string, canonical_owner: string, authority_ref: string}[]}
 */
export const FACTORY_IDS = Object.freeze([
  Object.freeze({
    id: "01",
    name: "Natural Language Command Factory",
    priority: "P0",
    proposed_scale: "100k+ utterances",
    canonical_owner: "repos_hq (semantics) + vivim-ui (command runtime)",
    authority_ref: "vivim-ui src/visual-runtime/command/ir.ts",
  }),
  Object.freeze({
    id: "02",
    name: "Catalogue of Catalogues",
    priority: "P0",
    proposed_scale: "1,000+ integration candidates",
    canonical_owner: "repos_hq",
    authority_ref: "repos_hq hq/lib/canon.mjs",
  }),
  Object.freeze({
    id: "03",
    name: "Universal Asset Library",
    priority: "P0",
    proposed_scale: "1,000+ reusable assets",
    canonical_owner: "vivim-ui",
    authority_ref: "vivim-ui src/visual-runtime/components/catalog.ts",
  }),
  Object.freeze({
    id: "04",
    name: "Adversarial Security Factory",
    priority: "P0",
    proposed_scale: "10k+ scenarios",
    canonical_owner: "vivim-web-harness (Host grants) + repos_hq (approvals)",
    authority_ref: "repos_hq hq/lib/approvals.mjs",
  }),
  Object.freeze({
    id: "05",
    name: "Integration Compatibility Lab",
    priority: "P0",
    proposed_scale: "All critical interfaces",
    canonical_owner: "vivim-ui",
    authority_ref: "vivim-ui src/visual-runtime/integration-2/compose.ts",
  }),
  Object.freeze({
    id: "06",
    name: "Production Scaffold Factory",
    priority: "P0",
    proposed_scale: "Full release topology",
    canonical_owner: "owner-approved release pipeline (not Labs)",
    authority_ref: "repos_hq docs/integration/contracts/naming.md",
  }),
  Object.freeze({
    id: "07",
    name: "Knowledge & Context Evaluation Lab",
    priority: "P1",
    proposed_scale: "Large synthetic corpora",
    canonical_owner: "repos_hq",
    authority_ref: "repos_hq hq/lib/kernel/store.mjs",
  }),
  Object.freeze({
    id: "08",
    name: "Provider Compatibility Observatory",
    priority: "P1",
    proposed_scale: "Broad app coverage",
    canonical_owner: "vivim-web-harness",
    authority_ref: "vivim-web-harness studio/src/studio/chat-channel.mjs",
  }),
  Object.freeze({
    id: "09",
    name: "State & Failure Simulation Lab",
    priority: "P1",
    proposed_scale: "Thousands of fault runs",
    canonical_owner: "vivim-ui",
    authority_ref: "vivim-ui src/visual-runtime/revisions/fixture-store.ts",
  }),
  Object.freeze({
    id: "10",
    name: "UX & Accessibility Factory",
    priority: "P1",
    proposed_scale: "Many devices/viewports",
    canonical_owner: "vivim-ui",
    authority_ref: "vivim-ui src/visual-runtime/components/accessibility.ts",
  }),
  Object.freeze({
    id: "11",
    name: "Performance & Optimization Lab",
    priority: "P1",
    proposed_scale: "Continuous benchmarks",
    canonical_owner: "repos_hq",
    authority_ref: "repos_hq hq/lib/telemetry.mjs",
  }),
  Object.freeze({
    id: "12",
    name: "Documentation & Recipe Factory",
    priority: "P2",
    proposed_scale: "Thousands of recipes",
    canonical_owner: "repos_hq",
    authority_ref: "repos_hq hq/lib/context.mjs",
  }),
]);

/**
 * The nine booster waves from docs/WAVE-BOOSTERS.md:9-19.
 * @type {readonly {id: string, name: string, booster_focus: string, status: string}[]}
 */
export const BOOST_WAVES = Object.freeze([
  Object.freeze({
    id: "W1",
    name: "audit (historical)",
    booster_focus: "Catalogue active code paths, duplicate authority map, capability inventory, testability matrix.",
    status: "historical",
  }),
  Object.freeze({
    id: "W2",
    name: "universal kernel (I2-C accepted)",
    booster_focus: "Re-run clean-checkout environment, package test recipes, preserve real-vs-mock provenance.",
    status: "closed",
  }),
  Object.freeze({
    id: "W3",
    name: "core convergence",
    booster_focus: "01 NL typed-IR conformance; 02 federated catalogue intake; 03 reusable component matrix; 04 adversarial cases; 05 differential compatibility; 07 provenance corpus; 09 fault matrix; 12 exact recipes.",
    status: "active",
  }),
  Object.freeze({
    id: "W4",
    name: "existing PC + Android",
    booster_focus: "Cross-version device contract matrix, provider observation, phone faults, a11y suites, real latency measurement, clean build reproducibility.",
    status: "planned",
  }),
  Object.freeze({
    id: "W5",
    name: "programmable UI",
    booster_focus: "Multi-turn and multilingual authoring corpus, template trust evaluation, injection cases, Studio/NL/TierFlow parity.",
    status: "planned",
  }),
  Object.freeze({
    id: "W6",
    name: "production cutover",
    booster_focus: "Reproducible build/release, exhaustive contract compatibility, authority-bypass negatives, power-loss faults, operator runbooks.",
    status: "planned",
  }),
  Object.freeze({
    id: "W7",
    name: "Day One (proposed)",
    booster_focus: "Onboarding funnel and support recipes, provider smoke coverage, install/uninstall regression.",
    status: "proposed",
  }),
  Object.freeze({
    id: "W8",
    name: "Ecosystem expansion (proposed)",
    booster_focus: "Candidate classification, provider workflow catalogue, compatibility and safe-adapter fixtures.",
    status: "proposed",
  }),
  Object.freeze({
    id: "W9",
    name: "Oracle maturity (proposed)",
    booster_focus: "Context graph retrieval and memory evaluation, agent/skill recipes, model routing benchmarks, autonomy denial scenarios.",
    status: "proposed",
  }),
]);

/**
 * @typedef {Object} ValidationIssue
 * @property {string} code    Stable machine-readable code.
 * @property {string} field   The field the issue belongs to, or "" when it is about the whole item.
 * @property {string} message Human-readable explanation.
 */

/**
 * The numeric level of a ladder stage, or null when the stage is unknown.
 * @param {unknown} stage
 * @returns {number | null}
 */
export function maturityLevel(stage) {
  if (typeof stage !== "string") return null;
  const found = MATURITY_LADDER.find((step) => step.stage === stage);
  return found ? found.level : null;
}

/**
 * The single rule that stops a fixture-only result from being described as real device or Host proof.
 *
 * A claimed maturity stage may never exceed the evidence the item actually holds. Claiming "integrated",
 * "environment verified" or "production verified" while holding only "unit verified" is refused, with the
 * reason stated, because "Never call fixture-only tests real Android/Host/browser proof"
 * (docs/ENGINEERING-EVIDENCE-FACTORY.md:70).
 * @param {{claimed?: unknown, evidence_level?: unknown}} input
 * @returns {{ok: boolean, errors: ValidationIssue[], warnings: ValidationIssue[], claimed_level: number | null, evidence_level_number: number | null}}
 */
export function validateMaturityClaim(input) {
  /** @type {ValidationIssue[]} */
  const errors = [];
  /** @type {ValidationIssue[]} */
  const warnings = [];
  const claimed = input && typeof input === "object" ? input.claimed : undefined;
  const evidenceLevel = input && typeof input === "object" ? input.evidence_level : undefined;

  const claimedLevel = maturityLevel(claimed);
  const evidenceLevelNumber = maturityLevel(evidenceLevel);

  if (claimedLevel === null) {
    errors.push({
      code: "maturity_unknown",
      field: "maturity",
      message: `maturity claim "${String(claimed)}" is not a ladder stage (${EVIDENCE_LEVELS.join(", ")})`,
    });
  }
  if (evidenceLevelNumber === null) {
    errors.push({
      code: "evidence_level_unknown",
      field: "evidence_level",
      message: `evidence_level "${String(evidenceLevel)}" is not a ladder stage (${EVIDENCE_LEVELS.join(", ")})`,
    });
  }
  if (claimedLevel !== null && evidenceLevelNumber !== null && claimedLevel > evidenceLevelNumber) {
    errors.push({
      code: "maturity_claim_exceeds_evidence",
      field: "maturity",
      message:
        `maturity claim "${String(claimed)}" (level ${claimedLevel}) exceeds the evidence held ` +
        `("${String(evidenceLevel)}", level ${evidenceLevelNumber}). Fixture-only results are never real ` +
        `device, browser or Host proof.`,
    });
  }
  return {
    ok: errors.length === 0,
    errors,
    warnings,
    claimed_level: claimedLevel,
    evidence_level_number: evidenceLevelNumber,
  };
}

/**
 * Validate one work item against the Labs model.
 *
 * Refused, with a named code for each:
 *  - a missing required field (`missing_field`);
 *  - an allowed path that is not a repository-anchored absolute path (`allowed_path_not_repo_absolute`);
 *  - an evidence level that is not on the ladder (`evidence_level_unknown`);
 *  - a maturity claim above the evidence held (`maturity_claim_exceeds_evidence`);
 *  - an empty test command at evidence level 3 or above (`test_command_required`);
 *  - a missing reviewer at evidence level 4 or above (`reviewer_required`);
 *  - an allowed path inside a reserved path of the item's repository (`forbidden_path`);
 *  - an allowed path outside the Labs write scope (`write_scope_outside_labs`).
 * @param {any} item
 * @returns {{ok: boolean, errors: ValidationIssue[], warnings: ValidationIssue[], item: any}}
 */
export function validateWorkItem(item) {
  /** @type {ValidationIssue[]} */
  const errors = [];
  /** @type {ValidationIssue[]} */
  const warnings = [];

  if (!item || typeof item !== "object" || Array.isArray(item)) {
    return {
      ok: false,
      errors: [
        {
          code: "not_an_object",
          field: "",
          message: "a work item must be a plain object",
        },
      ],
      warnings,
      item,
    };
  }

  for (const field of WORK_ITEM_FIELDS) {
    const value = item[field];
    if (value === undefined || value === null) {
      errors.push({
        code: "missing_field",
        field,
        message: `required field "${field}" is missing`,
      });
      continue;
    }
    if (field === "inputs" || field === "allowed_paths") {
      if (!Array.isArray(value)) {
        errors.push({
          code: "field_type",
          field,
          message: `field "${field}" must be an array of strings`,
        });
      }
      continue;
    }
    if (typeof value !== "string") {
      errors.push({
        code: "field_type",
        field,
        message: `field "${field}" must be a string`,
      });
    }
  }

  const taskId = typeof item.task_id === "string" ? item.task_id : "";
  if (taskId.length > 0 && !/^LABS-[A-Z0-9]+-\d{3}$/.test(taskId)) {
    errors.push({
      code: "task_id_format",
      field: "task_id",
      message: `task_id "${taskId}" must look like LABS-W3-001`,
    });
  }

  const repo = typeof item.repo === "string" && item.repo.length > 0 ? item.repo : "vivim-labs";
  if (!KNOWN_REPOS.includes(repo)) {
    errors.push({
      code: "unknown_repo",
      field: "repo",
      message: `repo "${repo}" is not one of ${KNOWN_REPOS.join(", ")}`,
    });
  }

  const evidenceLevel = typeof item.evidence_level === "string" ? item.evidence_level : "";
  const evidenceLevelNumber = maturityLevel(evidenceLevel);
  if (evidenceLevel.length > 0 && evidenceLevelNumber === null) {
    errors.push({
      code: "evidence_level_unknown",
      field: "evidence_level",
      message: `evidence_level "${evidenceLevel}" is not a ladder stage (${EVIDENCE_LEVELS.join(", ")})`,
    });
  }

  if (evidenceLevelNumber !== null && evidenceLevelNumber >= 3) {
    if (typeof item.test_command !== "string" || item.test_command.trim().length === 0) {
      errors.push({
        code: "test_command_required",
        field: "test_command",
        message: `evidence_level "${evidenceLevel}" needs a real test_command with an exit code`,
      });
    }
  }

  if (evidenceLevelNumber !== null && evidenceLevelNumber >= 4) {
    if (typeof item.reviewer !== "string" || item.reviewer.trim().length === 0) {
      errors.push({
        code: "reviewer_required",
        field: "reviewer",
        message: `evidence_level "${evidenceLevel}" needs a named reviewer`,
      });
    }
  }

  if (typeof item.source_sha === "string" && item.source_sha.length > 0) {
    if (!/^[0-9a-f]{7,40}$/.test(item.source_sha) && item.source_sha !== "pending") {
      warnings.push({
        code: "source_sha_shape",
        field: "source_sha",
        message: `source_sha "${item.source_sha}" is not a git object id`,
      });
    }
  }

  if (Array.isArray(item.inputs) && item.inputs.length === 0) {
    errors.push({
      code: "inputs_empty",
      field: "inputs",
      message: "inputs must name at least one input the item consumes",
    });
  }
  if (Array.isArray(item.inputs)) {
    for (const input of item.inputs) {
      if (typeof input !== "string" || input.trim().length === 0) {
        errors.push({
          code: "inputs_empty",
          field: "inputs",
          message: "every entry of inputs must be a non-empty string",
        });
        break;
      }
    }
  }

  if (Array.isArray(item.allowed_paths)) {
    if (item.allowed_paths.length === 0) {
      errors.push({
        code: "allowed_paths_empty",
        field: "allowed_paths",
        message: "allowed_paths must name at least one path the item may write",
      });
    }
    for (const allowed of item.allowed_paths) {
      if (!isRepoAbsolutePath(allowed)) {
        errors.push({
          code: "allowed_path_not_repo_absolute",
          field: "allowed_paths",
          message: `allowed path "${String(allowed)}" is not a repository-anchored absolute path (it must start with "/")`,
        });
        continue;
      }
      if (!isLabsWritablePath(allowed)) {
        errors.push({
          code: "write_scope_outside_labs",
          field: "allowed_paths",
          message:
            `allowed path "${allowed}" is outside the Labs write scope (/labs, /work, /tests, plus the ` +
            `root package.json and tsconfig.json). Labs may not write another repository's source.`,
        });
      }
      const forbidden = findForbiddenPath(repo, allowed);
      if (forbidden) {
        errors.push({
          code: "forbidden_path",
          field: "allowed_paths",
          message:
            `allowed path "${allowed}" falls inside "${forbidden.path}" reserved by ${forbidden.owner} ` +
            `(source: ${forbidden.source})`,
        });
      }
    }
  }

  if (item.maturity !== undefined && item.maturity !== null && item.maturity !== "") {
    const claim = validateMaturityClaim({ claimed: item.maturity, evidence_level: item.evidence_level });
    for (const issue of claim.errors) errors.push(issue);
    for (const issue of claim.warnings) warnings.push(issue);
  } else if (evidenceLevelNumber !== null && evidenceLevelNumber >= 3) {
    warnings.push({
      code: "maturity_claim_missing",
      field: "maturity",
      message: `evidence_level "${evidenceLevel}" without a maturity claim: state the stage actually reached`,
    });
  }

  if (
    item.maturity_target !== undefined &&
    item.maturity_target !== null &&
    item.maturity_target !== "" &&
    maturityLevel(item.maturity_target) === null
  ) {
    errors.push({
      code: "maturity_target_unknown",
      field: "maturity_target",
      message: `maturity_target "${String(item.maturity_target)}" is not a ladder stage`,
    });
  }

  return { ok: errors.length === 0, errors, warnings, item };
}

/**
 * Do two repository paths claim the same files? A path claims the files under its own prefix, so
 * `/labs/a` and `/labs/a/b` overlap, and `/labs/a` and `/labs/ab` do not.
 * @param {string} left
 * @param {string} right
 * @returns {boolean}
 */
export function pathsOverlap(left, right) {
  const leftParts = left.split("/").filter((part) => part.length > 0);
  const rightParts = right.split("/").filter((part) => part.length > 0);
  const shared = Math.min(leftParts.length, rightParts.length);
  for (let index = 0; index < shared; index += 1) {
    const leftPart = leftParts[index];
    const rightPart = rightParts[index];
    if (leftPart.includes("*") || rightPart.includes("*")) return true;
    if (leftPart !== rightPart) return false;
  }
  return true;
}

/**
 * Find write-scope conflicts between work items. Disjoint scopes are mandatory: two items that may
 * write the same file are a concurrency defect, not a race to be discovered later.
 * @param {any[]} items
 * @returns {{task_id: string, other_task_id: string, path: string, other_path: string}[]}
 */
export function detectScopeConflicts(items) {
  /** @type {{task_id: string, other_task_id: string, path: string, other_path: string}[]} */
  const conflicts = [];
  const list = Array.isArray(items) ? items : [];
  for (let left = 0; left < list.length; left += 1) {
    const a = list[left];
    if (!a || typeof a !== "object" || !Array.isArray(a.allowed_paths)) continue;
    for (let right = left + 1; right < list.length; right += 1) {
      const b = list[right];
      if (!b || typeof b !== "object" || !Array.isArray(b.allowed_paths)) continue;
      for (const leftPath of a.allowed_paths) {
        for (const rightPath of b.allowed_paths) {
          if (typeof leftPath !== "string" || typeof rightPath !== "string") continue;
          if (pathsOverlap(leftPath, rightPath)) {
            conflicts.push({
              task_id: String(a.task_id),
              other_task_id: String(b.task_id),
              path: leftPath,
              other_path: rightPath,
            });
          }
        }
      }
    }
  }
  return conflicts;
}

/**
 * A one-line description of a work item, used by the CLI tables.
 * @param {any} item
 * @returns {string}
 */
export function describeItem(item) {
  if (!item || typeof item !== "object") return "(not an object)";
  return [
    item.task_id ?? "?",
    item.factory ?? "--",
    item.wave ?? "--",
    item.maturity ?? "--",
    item.evidence_level ?? "--",
  ].join(" | ");
}

/** The reserved path data, re-exported so a caller needs one import. */
export { FORBIDDEN_PATHS, KNOWN_REPOS };
