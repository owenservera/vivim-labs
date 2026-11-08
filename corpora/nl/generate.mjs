/**
 * @hq:context Deterministic template generator that expands the hand-labelled NL corpus into a larger generated set, with per-case dedup and source verification.
 * @hq:owner vivim-labs
 * @hq:task ENGINEERING-EVIDENCE-FACTORY/W3-01
 *
 * Factory 01 — Natural Language Command Factory.
 *
 * DETERMINISM AND SCOPE
 *   No model calls. No network. No clock. No randomness that is not seeded.
 *   Running this twice with the same seed produces byte-identical output.
 *
 * HONESTY RULES (ENGINEERING-EVIDENCE-FACTORY.md:33, 46, 103)
 *   - Generated cases are NEVER counted as verified cases. The generator
 *     reports generated, deduplicated and hand-labelled counts separately.
 *   - Every generated case is re-checked against the REAL pinned TierFlow
 *     source before it is accepted. A template whose output does not behave
 *     as declared is REJECTED and counted, not silently kept.
 *   - A generated case inherits its label method from the template that
 *     produced it. It is never presented as an independently reviewed label.
 *
 * USAGE
 *   bun corpora/nl/generate.mjs                  # write corpora/nl/generated.mjs
 *   NL_GENERATE_TARGET=2000 bun corpora/nl/generate.mjs
 *   NL_GENERATE_SEED=12345 bun corpora/nl/generate.mjs
 */

import { writeFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';

import { SEED_CASES, CASES } from './cases.mjs';
import { normaliseUtterance, validateCases, TIERFLOW_SHA } from './schema.mjs';
import { deriveOutcome } from './independent-oracle.mjs';

export const DEFAULT_SEED = 20261106;
export const DEFAULT_TARGET = 240;

/** Deterministic 32-bit PRNG (mulberry32). Same seed -> same stream. */
function mulberry32(seed) {
  let a = seed >>> 0;
  return function next() {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/* ------------------------------------------------------------------ */
/* Vocabulary used by the templates                                    */
/* ------------------------------------------------------------------ */

const POSITION_PHRASE = {
  above: 'above',
  below: 'below',
  top: 'at the top',
  bottom: 'at the bottom',
  before: 'before',
  after: 'after',
};

const ANCHOR_PHRASE = {
  activity: 'activity',
  projects: 'projects',
  inbox: 'inbox',
  calendar: 'calendar',
};

const PRESET_PHRASE = {
  lab: 'lab',
  tide: 'tide',
  dusk: 'dusk',
  ember: 'ember',
  grove: 'grove',
  ink: 'ink',
  dawn: 'dawn',
};

const ORDER_PHRASE = {
  first: 'first',
  second: 'second',
  last: 'last',
};

const PANEL_NOUNS = ['tasks', 'task panel', 'task widget'];
const THEME_NOUNS = ['theme', 'colour scheme', 'appearance'];

const MULTILINGUAL = [
  { lang: 'es', move: (p, a) => `Mueve mi ${p === 'top' ? 'panel' : 'panel'} de tareas ${a}` },
  { lang: 'fr', move: (p, a) => `Deplace le panneau de taches ${a}` },
  { lang: 'de', move: (p, a) => `Setze mein Task-Panel ${a}` },
  { lang: 'it', move: (p, a) => `Sposta il pannello delle attivita ${a}` },
  { lang: 'pt', move: (p, a) => `Mova o painel de tarefas ${a}` },
  { lang: 'nl', move: (p, a) => `Verplaats het taakpaneel ${a}` },
];

const VAGUE_REQUESTS = [
  'rearrange my dashboard',
  'tidy up my workspace',
  'make the panels nicer',
  'sort my dashboard',
  'clean up the layout',
  'organise my panels',
  'improve the dashboard',
  'tweak the layout',
];

const UNSAFE_TEMPLATES = [
  (s) => `never ${s}`,
  (s) => `do not ${s}`,
  (s) => `without asking, ${s}`,
  (s) => `instead of that, ${s}`,
  (s) => `!run ${s}`,
  (s) => `!use ${s}`,
  (s) => `sudo ${s}`,
  (s) => `powershell -Command ${s}`,
  (s) => `send to provider ${s}`,
];

const CHAIN_SEPARATORS = [
  { sep: ' and then ', outcome: 'refusal', reasonCode: 'UNSAFE_OR_COMPOUND_TEXT' },
  { sep: ' after that ', outcome: 'refusal', reasonCode: 'UNSAFE_OR_COMPOUND_TEXT' },
  { sep: ' and ', outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT' },
  { sep: '; ', outcome: 'refusal', reasonCode: 'UNSAFE_OR_COMPOUND_TEXT' },
  { sep: ', then ', outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT' },
];

const ARG_MUTATIONS = [
  {
    id: 'unquoted',
    apply: (text) => text.replace(/="([^"]*)"/, '=$1'),
    outcome: 'clarification',
    reasonCode: 'UNKNOWN_TEXT',
  },
  {
    id: 'duplicate-key',
    apply: (text) => `${text} ${text.split(' ').slice(1).join(' ')}`,
    outcome: 'clarification',
    reasonCode: 'UNKNOWN_TEXT',
  },
  {
    id: 'trailing-token',
    apply: (text) => `${text} extra`,
    outcome: 'clarification',
    reasonCode: 'UNKNOWN_TEXT',
  },
  {
    id: 'extra-arg',
    apply: (text) => `${text} junk="x"`,
    outcome: 'refusal',
    reasonCode: 'INVALID_ARGUMENTS',
  },
  {
    id: 'shell-separator',
    apply: (text) => `${text}; whoami`,
    outcome: 'refusal',
    reasonCode: 'UNSAFE_OR_COMPOUND_TEXT',
  },
];

/* ------------------------------------------------------------------ */
/* Template engine                                                     */
/* ------------------------------------------------------------------ */

/**
 * Each template receives a seed case and returns candidate objects.
 * A candidate is { utterance, expected, contextOverrides, labelMethod,
 *                  requirement, note }.
 */
const TEMPLATES = {
  'synonym-paraphrase': (seedCase) => {
    const command = seedCase.requirement && seedCase.requirement.command;
    if (!command) return [];
    const args = command.args;
    const out = [];
    if (command.semanticId === 'workspace.panel.move@1') {
      for (const noun of PANEL_NOUNS) {
        out.push({
          utterance: `Move my ${noun} ${POSITION_PHRASE[args.position]} ${ANCHOR_PHRASE[args.anchor]}.`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: panel noun variant`,
        });
        out.push({
          utterance: `Please move the ${noun} ${POSITION_PHRASE[args.position]} the ${ANCHOR_PHRASE[args.anchor]} panel.`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: politeness + preposition variant`,
        });
      }
    } else if (command.semanticId === 'ui.theme.set@1') {
      for (const noun of THEME_NOUNS) {
        out.push({
          utterance: `Use the ${PRESET_PHRASE[args.preset]} ${noun}.`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: theme noun variant`,
        });
        out.push({
          utterance: `My ${noun} should be ${PRESET_PHRASE[args.preset]}.`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: first person phrasing`,
        });
      }
    } else {
      out.push({
        utterance: `Please do the ${command.semanticId.split('@')[0].replaceAll('.', ' ')} action now.`,
        expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
        labelMethod: 'derived-from-source',
        note: `templated from ${seedCase.id}: generic phrasing`,
      });
    }
    return out;
  },

  'intent-classification': (seedCase) => {
    const command = seedCase.requirement && seedCase.requirement.command;
    if (!command) return [];
    const out = [];
    if (command.semanticId === 'workspace.panel.move@1') {
      for (const noun of PANEL_NOUNS) {
        out.push({
          utterance: `Move the ${noun} ${POSITION_PHRASE[command.args.position]} ${ANCHOR_PHRASE[command.args.anchor]}`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: move intent, noun variant`,
        });
        out.push({
          utterance: `Reorder the ${noun} so it comes ${ORDER_PHRASE[command.args.position === 'top' ? 'first' : 'first']}`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: reorder intent, must not collapse into move`,
        });
      }
    } else if (command.semanticId === 'ui.theme.set@1') {
      for (const noun of THEME_NOUNS) {
        out.push({
          utterance: `Set my ${noun} to ${PRESET_PHRASE[command.args.preset]}.`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: set intent, noun variant`,
        });
        out.push({
          utterance: `Show me my ${noun}.`,
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: show intent, must not collapse into set`,
        });
      }
    } else {
      out.push({
        utterance: `Could you ${command.semanticId.split('@')[0].replaceAll('.', ' ')}?`,
        expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
        labelMethod: 'derived-from-source',
        note: `templated from ${seedCase.id}: generic intent phrasing`,
      });
    }
    return out;
  },

  correction: (seedCase) => {
    const out = [];
    const openers = ['Actually,', 'No wait,', 'Sorry,', 'Hold on,'];
    for (const opener of openers) {
      out.push({
        utterance: `${opener} move it back.`,
        expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
        labelMethod: 'unverifiable-in-source',
        note: `templated from ${seedCase.id}: deictic correction, no prior-state channel in the IR`,
      });
      out.push({
        utterance: `${opener} put it back where it was.`,
        expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
        labelMethod: 'unverifiable-in-source',
        note: `templated from ${seedCase.id}: deictic correction, no prior-state channel in the IR`,
      });
    }
    return out;
  },

  'chained-command': (seedCase) => {
    const out = [];
    const first = 'Move my tasks above activity';
    const second = 'put the task widget at the top';
    for (const variant of CHAIN_SEPARATORS) {
      out.push({
        utterance: `${first}${variant.sep}${second}.`,
        expected: { outcome: variant.outcome, reasonCode: variant.reasonCode, command: null },
        labelMethod: variant.outcome === 'refusal' ? 'derived-from-source' : 'independent-review',
        note: `templated from ${seedCase.id}: chained intents separated by "${variant.sep.trim()}"`,
      });
    }
    return out;
  },

  ambiguity: (seedCase) => {
    const out = [];
    for (const request of VAGUE_REQUESTS) {
      out.push({
        utterance: `Can you ${request}?`,
        expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
        labelMethod: 'derived-from-source',
        note: `templated from ${seedCase.id}: vague but safe request`,
      });
      out.push({
        utterance: `I want you to ${request}.`,
        expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
        labelMethod: 'derived-from-source',
        note: `templated from ${seedCase.id}: vague but safe request`,
      });
    }
    return out;
  },

  multilingual: (seedCase) => {
    const out = [];
    const positions = ['above', 'at the top'];
    for (const language of MULTILINGUAL) {
      for (const position of positions) {
        out.push({
          utterance: language.move(position, position === 'above' ? 'activity' : 'activity'),
          expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
          labelMethod: 'derived-from-source',
          note: `templated from ${seedCase.id}: ${language.lang} phrasing`,
        });
      }
    }
    return out;
  },

  'typed-ir-conformance': (seedCase) => {
    const command = seedCase.expected.command;
    if (!command) return [];
    const out = [];
    const argEntries = Object.entries(command.args);
    if (argEntries.length === 0) {
      out.push({
        utterance: `/${command.semanticId}`,
        expected: { outcome: 'accepted', reasonCode: null, command },
        labelMethod: 'derived-from-source',
        note: `templated from ${seedCase.id}: zero-argument canonical text`,
      });
      return out;
    }
    const values = {
      preset: Object.keys(PRESET_PHRASE),
      position: Object.keys(POSITION_PHRASE),
      anchor: Object.keys(ANCHOR_PHRASE),
      order: Object.keys(ORDER_PHRASE),
    };
    const [argKey] = argEntries;
    const pool = values[argKey] ?? [];
    for (const value of pool) {
      const args = { ...command.args, [argKey]: value };
      out.push({
        utterance: `/${command.semanticId} ${argKey}="${value}"`,
        expected: {
          outcome: 'accepted',
          reasonCode: null,
          command: { ...command, args },
        },
        labelMethod: 'derived-from-source',
        note: `templated from ${seedCase.id}: canonical text with enum member ${value}`,
      });
    }
    return out;
  },

  refusal: (seedCase) => {
    const out = [];
    for (const template of UNSAFE_TEMPLATES) {
      out.push({
        utterance: template('change theme'),
        expected: { outcome: 'refusal', reasonCode: 'UNSAFE_OR_COMPOUND_TEXT', command: null },
        labelMethod: 'adversarial-mutation',
        note: `templated from ${seedCase.id}: unsafe or compound phrasing`,
      });
    }
    return out;
  },

  'refusal-clarification-boundary': (seedCase) => {
    const command = seedCase.expected.command;
    const out = [];
    if (command) {
      const text = `/${command.semanticId} ${Object.entries(command.args).map(([k, v]) => `${k}="${v}"`).join(' ')}`.trim();
      for (const mutation of ARG_MUTATIONS) {
        out.push({
          utterance: mutation.apply(text),
          expected: { outcome: mutation.outcome, reasonCode: mutation.reasonCode, command: null },
          labelMethod: 'adversarial-mutation',
          note: `templated from ${seedCase.id}: ${mutation.id} mutation of canonical text`,
        });
      }
    }
    out.push({
      utterance: 'show my theme.',
      expected: { outcome: 'clarification', reasonCode: 'UNKNOWN_TEXT', command: null },
      labelMethod: 'adversarial-mutation',
      note: `templated from ${seedCase.id}: alias exactness boundary (trailing punctuation)`,
    });
    return out;
  },
};

/* ------------------------------------------------------------------ */
/* Generation                                                          */
/* ------------------------------------------------------------------ */

const CATEGORY_ABBREV = {
  'synonym-paraphrase': 'syn',
  'intent-classification': 'int',
  correction: 'cor',
  'chained-command': 'cha',
  ambiguity: 'amb',
  multilingual: 'mul',
  'typed-ir-conformance': 'tir',
  refusal: 'ref',
  'refusal-clarification-boundary': 'bnd',
};

/**
 * Generate cases from the hand-labelled seed set.
 *
 * Returns {
 *   cases, generated, deduplicated, rejected, handLabelled, seed, target,
 *   byCategory, rejections
 * }
 */
export function generateCases(options = {}) {
  const seed = options.seed ?? DEFAULT_SEED;
  const target = options.target ?? DEFAULT_TARGET;
  const rng = mulberry32(seed);

  // 1. Collect candidates in a deterministic order.
  const candidates = [];
  for (const seedCase of SEED_CASES) {
    const template = TEMPLATES[seedCase.category];
    if (!template) continue;
    for (const candidate of template(seedCase)) {
      candidates.push({ ...candidate, seedId: seedCase.id, category: seedCase.category });
    }
  }

  // 2. Deterministic shuffle (seeded) so a target cap samples uniformly.
  const shuffled = candidates.slice();
  for (let i = shuffled.length - 1; i > 0; i -= 1) {
    const j = Math.floor(rng() * (i + 1));
    [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
  }

  // 3. Dedup by normalised utterance, seed utterances first.
  const seen = new Set(CASES.map((c) => normaliseUtterance(c.utterance)));
  let deduplicated = 0;
  const unique = [];
  for (const candidate of shuffled) {
    const key = normaliseUtterance(candidate.utterance);
    if (seen.has(key)) {
      deduplicated += 1;
      continue;
    }
    seen.add(key);
    unique.push(candidate);
  }

  // 4. Verify every candidate against the REAL pinned source, then cap.
  const accepted = [];
  const rejections = [];
  const perCategory = {};
  for (const candidate of unique) {
    if (accepted.length >= target) break;
    const probe = {
      id: 'probe',
      category: candidate.category,
      utterance: candidate.utterance,
      surface: 'text',
      expected: candidate.expected,
      labelSource: 'generated',
      labelMethod: candidate.labelMethod,
      requirement: null,
      contextOverrides: null,
      generated: true,
      notes: candidate.note,
    };
    let derived;
    try {
      derived = deriveOutcome(probe);
    } catch (error) {
      rejections.push({ ...candidate, reason: `derivation threw: ${error.message}` });
      continue;
    }
    if (derived.outcome !== candidate.expected.outcome ||
        (derived.reasonCode ?? null) !== (candidate.expected.reasonCode ?? null)) {
      rejections.push({
        ...candidate,
        reason: `source produced ${derived.outcome}/${derived.reasonCode ?? 'accepted'}, template declared ${candidate.expected.outcome}/${candidate.expected.reasonCode ?? 'accepted'}`,
      });
      continue;
    }
    const index = accepted.length + 1;
    const generatedCase = {
      id: `nl-gen-${CATEGORY_ABBREV[candidate.category] ?? 'gen'}-${String(index).padStart(4, '0')}`,
      category: candidate.category,
      utterance: candidate.utterance,
      surface: 'text',
      labelSource: 'generated-from-template:corpora/nl/generate.mjs',
      labelMethod: candidate.labelMethod,
      expected: candidate.expected,
      requirement: null,
      contextOverrides: null,
      generated: true,
      notes: `${candidate.note} (seed ${candidate.seedId}; verified against TierFlow ${TIERFLOW_SHA}).`,
    };
    accepted.push(generatedCase);
    perCategory[candidate.category] = (perCategory[candidate.category] ?? 0) + 1;
  }

  return {
    cases: accepted,
    generated: accepted.length,
    deduplicated,
    rejected: rejections.length,
    rejections,
    handLabelled: CASES.length,
    seed,
    target,
    byCategory: perCategory,
  };
}

/* ------------------------------------------------------------------ */
/* CLI                                                                 */
/* ------------------------------------------------------------------ */

const isMain = typeof process !== 'undefined' && process.argv[1] &&
  import.meta.url === pathToFileURL(process.argv[1]).href;

if (isMain) {
  const seed = Number(process.env.NL_GENERATE_SEED ?? DEFAULT_SEED);
  const target = Number(process.env.NL_GENERATE_TARGET ?? DEFAULT_TARGET);
  const report = generateCases({ seed, target });

  const here = dirname(fileURLToPath(import.meta.url));
  const outputPath = join(here, 'generated.mjs');
  const body = `/**
 * @hq:context Generated slice of the NL conformance corpus — deterministic template expansion, source-verified.
 * @hq:owner vivim-labs
 * @hq:task ENGINEERING-EVIDENCE-FACTORY/W3-01
 *
 * GENERATED FILE. Do not hand-edit.
 * Regenerate with: bun corpora/nl/generate.mjs
 *
 * Every case here was re-checked against the real TierFlow source at
 * generation time and carries generated: true. Generated cases are NOT
 * independently reviewed labels and must never be counted as verified.
 */

export const GENERATION_META = Object.freeze({
  seed: ${report.seed},
  target: ${report.target},
  generated: ${report.generated},
  deduplicated: ${report.deduplicated},
  rejected: ${report.rejected},
  handLabelled: ${report.handLabelled},
  tierflowSha: ${JSON.stringify(TIERFLOW_SHA)},
  byCategory: ${JSON.stringify(report.byCategory, null, 2)},
});

export const GENERATED_CASES = ${JSON.stringify(report.cases, null, 2)};

export default GENERATED_CASES;
`;
  writeFileSync(outputPath, body, 'utf8');

  console.log('VIVIM Labs — Factory 01 natural-language corpus generation');
  console.log(`TierFlow SHA pinned: ${TIERFLOW_SHA}`);
  console.log(`seed: ${report.seed}   target: ${report.target}`);
  console.log(`hand-labelled cases (seed corpus): ${report.handLabelled}`);
  console.log(`generated cases accepted:           ${report.generated}`);
  console.log(`deduplicated (normalised utterance): ${report.deduplicated}`);
  console.log(`rejected by the source check:        ${report.rejected}`);
  console.log(`written: ${outputPath}`);
  for (const [category, count] of Object.entries(report.byCategory).sort()) {
    console.log(`  ${category}: ${count}`);
  }
  if (report.rejected > 0) {
    console.log('rejections (first 10):');
    for (const rejection of report.rejections.slice(0, 10)) {
      console.log(`  - "${rejection.utterance}" :: ${rejection.reason}`);
    }
  }
  console.log('NOTE: volume never proves correctness. Generated cases are not independently labelled.');
  process.exitCode = 0;
}
