/**
 * @hq:context Natural-language typed-IR conformance corpus — case format, labels and category vocabulary.
 * @hq:owner vivim-labs
 * @hq:task ENGINEERING-EVIDENCE-FACTORY/W3-01
 *
 * Factory 01 of the VIVIM Labs Engineering Evidence Factory.
 *
 * This module defines ONLY the shape of a case and the vocabulary of labels.
 * It contains no expectations of its own: `cases.mjs` holds the hand-labelled
 * corpus, `independent-oracle.mjs` re-derives the outcome from the pinned
 * TierFlow source and compares it against those labels.
 *
 * Source of truth for the vocabulary:
 *   docs/ENGINEERING-EVIDENCE-FACTORY.md (VIVIM Labs) — "Natural-language
 *   conformance, not synonym accumulation" and the Factory 01 row.
 *   TierFlow (owenservera/tierflow) @ the SHA below — the command runtime.
 *
 * The corpus is FIXTURE / UNIT level. It is not device, Host, browser or
 * production evidence and must never be reported as such.
 */

/**
 * The exact TierFlow commit this corpus is pinned to.
 * Every artefact produced from this corpus records this SHA.
 */
export const TIERFLOW_SHA = 'ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7';

/** Human-facing provenance of the pin. */
export const SOURCE_PIN = Object.freeze({
  repository: 'owenservera/tierflow',
  localPath: 'C:/0-BlackBoxProject-0/vivim-ui',
  sha: TIERFLOW_SHA,
  branch: 'main',
  /** Files whose real behaviour the labels are derived from. */
  commandSurface: Object.freeze([
    'src/visual-runtime/command/compile.ts',
    'src/visual-runtime/command/crosswalk.ts',
    'src/visual-runtime/command/ir.ts',
  ]),
  /** Files whose real behaviour the expected scope is derived from. */
  scopeOwners: Object.freeze([
    'src/visual-runtime/revisions/fixture-store.ts',
    'src/visual-runtime/host/fixture.ts',
    'src/visual-runtime/host/outcome.ts',
  ]),
  spec: Object.freeze([
    'vivim-labs/docs/ENGINEERING-EVIDENCE-FACTORY.md',
    'vivim-labs/docs/WAVE-BOOSTERS.md',
  ]),
});

/**
 * The three-way outcome vocabulary required by the Factory 01 specification.
 *
 * TierFlow's compiler returns `candidate | clarification_required | refused`
 * (src/visual-runtime/command/ir.ts:38-40). The corpus uses the spec's
 * vocabulary and the oracle performs the translation explicitly:
 *   candidate            -> accepted
 *   clarification_required -> clarification
 *   refused              -> refusal
 */
export const OUTCOMES = Object.freeze(['accepted', 'clarification', 'refusal']);

/**
 * The nine categories named by the Factory 01 specification. Every category
 * must be represented by at least one case (asserted by oracle.test.mjs).
 */
export const CATEGORIES = Object.freeze([
  'synonym-paraphrase',
  'intent-classification',
  'correction',
  'chained-command',
  'ambiguity',
  'multilingual',
  'typed-ir-conformance',
  'refusal',
  'refusal-clarification-boundary',
]);

/**
 * How a label was derived.
 *
 *   derived-from-source    the expectation follows from a cited TierFlow source
 *                          line/branch with no judgement call.
 *   independent-review     the expectation follows from cited source lines plus
 *                          an explicit reading (recorded in `notes`).
 *   adversarial-mutation   the case is a deliberate mutation of another case
 *                          and the expectation is the mutated behaviour.
 *   unverifiable-in-source the source defines no behaviour for the case class;
 *                          the case records that fact instead of inventing an
 *                          expectation.
 */
export const LABEL_METHODS = Object.freeze([
  'derived-from-source',
  'independent-review',
  'adversarial-mutation',
  'unverifiable-in-source',
]);

/** The three typed-IR targets the corpus distinguishes. */
export const TARGET_OWNERS = Object.freeze(['ui_document', 'host_effect']);

/** Mirrors TierFlow's own `SEMANTIC_ID` (src/visual-runtime/command/ir.ts:53). */
export const SEMANTIC_ID = /^[a-z0-9][a-z0-9._-]{0,94}@(\d+)(?:\.\d+)*$/;

/** Mirrors TierFlow's own `ARG_KEY` (src/visual-runtime/command/ir.ts:54). */
export const ARG_KEY = /^[a-z][a-z0-9_]{0,39}$/;

/**
 * Normalisation used for per-case dedup of generated utterances.
 * Lowercase, strip combining marks, drop punctuation, collapse whitespace.
 */
export function normaliseUtterance(utterance) {
  return String(utterance)
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s]+/gu, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

function isPlainObject(value) {
  return !!value && typeof value === 'object' && !Array.isArray(value) &&
    (Object.getPrototypeOf(value) === Object.prototype || Object.getPrototypeOf(value) === null);
}

/**
 * Validate one case. Throws with a precise message on the first problem so a
 * malformed corpus fails loudly at load time rather than silently in a test.
 */
export function assertCase(testCase) {
  const where = testCase && testCase.id ? `case ${testCase.id}` : 'case';
  if (!isPlainObject(testCase)) throw new Error(`${where}: not a plain object`);
  if (typeof testCase.id !== 'string' || !/^[a-z0-9-]{3,64}$/.test(testCase.id)) {
    throw new Error(`${where}: id must match /^[a-z0-9-]{3,64}$/`);
  }
  if (!CATEGORIES.includes(testCase.category)) {
    throw new Error(`${where}: unknown category ${testCase.category}`);
  }
  if (typeof testCase.utterance !== 'string' || testCase.utterance.length === 0 || testCase.utterance.length > 2048) {
    throw new Error(`${where}: utterance must be a non-empty string of at most 2048 characters`);
  }
  if (testCase.surface !== 'text' && testCase.surface !== 'touch' && testCase.surface !== 'keyboard') {
    throw new Error(`${where}: surface must be text|touch|keyboard`);
  }
  if (typeof testCase.labelSource !== 'string' || testCase.labelSource.length === 0) {
    throw new Error(`${where}: labelSource is required`);
  }
  if (!LABEL_METHODS.includes(testCase.labelMethod)) {
    throw new Error(`${where}: unknown labelMethod ${testCase.labelMethod}`);
  }

  const expected = testCase.expected;
  if (!isPlainObject(expected)) throw new Error(`${where}: expected must be a plain object`);
  if (!OUTCOMES.includes(expected.outcome)) {
    throw new Error(`${where}: expected.outcome must be one of ${OUTCOMES.join('|')}`);
  }
  if (expected.reasonCode !== null && typeof expected.reasonCode !== 'string') {
    throw new Error(`${where}: expected.reasonCode must be a string or null`);
  }
  if (expected.outcome === 'accepted' && expected.reasonCode !== null) {
    throw new Error(`${where}: an accepted case cannot carry a refusal reason code`);
  }
  if (expected.outcome !== 'accepted' && expected.reasonCode === null) {
    throw new Error(`${where}: a ${expected.outcome} case must carry the TierFlow reason code`);
  }

  const command = expected.command;
  if (expected.outcome === 'accepted') {
    if (!isPlainObject(command)) throw new Error(`${where}: an accepted case needs expected.command`);
    if (typeof command.semanticId !== 'string' || !SEMANTIC_ID.test(command.semanticId)) {
      throw new Error(`${where}: expected.command.semanticId is not a valid TierFlow semantic id`);
    }
    const version = command.semanticId.split('@')[1];
    if (command.version !== version) {
      throw new Error(`${where}: expected.command.version must equal the semantic id version (${version})`);
    }
    if (!isPlainObject(command.target)) throw new Error(`${where}: expected.command.target is required`);
    if (!TARGET_OWNERS.includes(command.target.owner)) {
      throw new Error(`${where}: expected.command.target.owner must be one of ${TARGET_OWNERS.join('|')}`);
    }
    if (typeof command.target.actionId !== 'string' || !/^[a-z][a-z0-9._-]{1,95}$/.test(command.target.actionId)) {
      throw new Error(`${where}: expected.command.target.actionId is not a valid TierFlow action id`);
    }
    if (!Number.isSafeInteger(command.target.catalogVersion) || command.target.catalogVersion < 1) {
      throw new Error(`${where}: expected.command.target.catalogVersion must be a positive integer`);
    }
    if (!isPlainObject(command.args)) throw new Error(`${where}: expected.command.args is required`);
    for (const [key, value] of Object.entries(command.args)) {
      if (!ARG_KEY.test(key)) throw new Error(`${where}: arg key ${key} is not a valid TierFlow arg key`);
      if (typeof value !== 'string') throw new Error(`${where}: arg ${key} must be a string`);
    }
    if (typeof command.scope !== 'string' || !/^[a-z]+:[a-z]+$/.test(command.scope)) {
      throw new Error(`${where}: expected.command.scope must look like "ui:publish" or "control:execute"`);
    }
  } else if (command !== null && command !== undefined) {
    throw new Error(`${where}: a ${expected.outcome} case cannot carry an expected typed command`);
  }

  // Non-text surfaces resolve from an explicit selector; the utterance is then
  // decorative and must not be used as the resolution channel.
  if (testCase.surface === 'text') {
    if (testCase.selection !== undefined && testCase.selection !== null) {
      throw new Error(`${where}: a text-surface case cannot carry a selection`);
    }
  } else {
    const selection = testCase.selection;
    if (!isPlainObject(selection)) throw new Error(`${where}: a ${testCase.surface} case needs a selection`);
    if (typeof selection.semanticId !== 'string' || !SEMANTIC_ID.test(selection.semanticId)) {
      throw new Error(`${where}: selection.semanticId is not a valid TierFlow semantic id`);
    }
    if (!isPlainObject(selection.args)) throw new Error(`${where}: selection.args is required`);
    for (const [key, value] of Object.entries(selection.args)) {
      if (!ARG_KEY.test(key)) throw new Error(`${where}: selection arg key ${key} is not valid`);
      if (typeof value !== 'string') throw new Error(`${where}: selection arg ${key} must be a string`);
    }
  }

  // The forward contract ("what a conforming NL layer must do") is optional but
  // must itself be well formed when present.
  const requirement = testCase.requirement;
  if (requirement !== null && requirement !== undefined) {
    if (!isPlainObject(requirement)) throw new Error(`${where}: requirement must be a plain object or null`);
    if (!OUTCOMES.includes(requirement.outcome)) {
      throw new Error(`${where}: requirement.outcome must be one of ${OUTCOMES.join('|')}`);
    }
    if (typeof requirement.labelSource !== 'string' || requirement.labelSource.length === 0) {
      throw new Error(`${where}: requirement.labelSource is required`);
    }
    if (!LABEL_METHODS.includes(requirement.labelMethod)) {
      throw new Error(`${where}: requirement.labelMethod must be one of ${LABEL_METHODS.join('|')}`);
    }
    if (requirement.command !== null && requirement.command !== undefined) {
      if (!isPlainObject(requirement.command)) {
        throw new Error(`${where}: requirement.command must be a plain object or null`);
      }
      if (typeof requirement.command.semanticId !== 'string' || !SEMANTIC_ID.test(requirement.command.semanticId)) {
        throw new Error(`${where}: requirement.command.semanticId is not a valid TierFlow semantic id`);
      }
      if (!isPlainObject(requirement.command.target) || !TARGET_OWNERS.includes(requirement.command.target.owner)) {
        throw new Error(`${where}: requirement.command.target is invalid`);
      }
      if (!isPlainObject(requirement.command.args)) {
        throw new Error(`${where}: requirement.command.args is required`);
      }
      if (typeof requirement.command.scope !== 'string' || !/^[a-z]+:[a-z]+$/.test(requirement.command.scope)) {
        throw new Error(`${where}: requirement.command.scope must look like "ui:publish" or "control:execute"`);
      }
    }
  }

  if (testCase.generated !== true && testCase.generated !== false) {
    throw new Error(`${where}: generated must be a boolean`);
  }
  if (typeof testCase.notes !== 'string') throw new Error(`${where}: notes must be a string`);
  return true;
}

/** Validate a whole corpus; returns the corpus unchanged so it can be chained. */
export function validateCases(cases) {
  if (!Array.isArray(cases)) throw new Error('corpus must be an array');
  const seen = new Set();
  for (const testCase of cases) {
    assertCase(testCase);
    if (seen.has(testCase.id)) throw new Error(`duplicate case id ${testCase.id}`);
    seen.add(testCase.id);
  }
  return cases;
}

/** Counts by category, ordered as CATEGORIES. */
export function countByCategory(cases) {
  const counts = Object.fromEntries(CATEGORIES.map((c) => [c, 0]));
  for (const testCase of cases) counts[testCase.category] += 1;
  return counts;
}

/** Counts by label method, ordered as LABEL_METHODS. */
export function countByLabelMethod(cases) {
  const counts = Object.fromEntries(LABEL_METHODS.map((m) => [m, 0]));
  for (const testCase of cases) counts[testCase.labelMethod] += 1;
  return counts;
}
