/**
 * @hq:context Independent oracle — re-derives each corpus case's expected outcome from the pinned TierFlow source and compares it to the hand-written label.
 * @hq:owner vivim-labs
 * @hq:task ENGINEERING-EVIDENCE-FACTORY/W3-01
 *
 * WHY THIS FILE EXISTS
 * ENGINEERING-EVIDENCE-FACTORY.md:46 — "Generation and grading must not share
 * an unchecked model-produced oracle. Independently verify test expectations."
 *
 * This oracle does NOT read the corpus's expectations to decide what the source
 * does. It imports the REAL TierFlow modules, runs them, and then compares:
 *
 *   1. HARD DISPUTE  — the source-derived outcome or reason code differs from
 *                      the corpus label. This is a LABEL ERROR and fails the
 *                      test. The corpus is never silently realigned to the
 *                      oracle; the disagreement is reported.
 *   2. HARD DISPUTE  — an `accepted` case whose derived typed IR differs from
 *                      the recorded semantic id / version / target / arguments.
 *                      This is a WRONG-TARGET or WRONG-ARGUMENT defect.
 *   3. HARD DISPUTE  — a recorded scope that the owning TierFlow authority does
 *                      not actually enforce. This is a WRONG-SCOPE defect.
 *   4. FINDING       — the corpus `requirement` (the forward contract for a
 *                      conforming natural-language layer) differs from what the
 *                      pinned source does today. These are the real capability
 *                      gaps and they are reported, never asserted.
 *
 * The oracle imports TierFlow source READ-ONLY. It never writes into
 * C:/0-BlackBoxProject-0/vivim-ui.
 *
 * MATURITY: fixture / unit level. No Host, KB, browser, device or network.
 */

import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';

import { compileVisualCommand } from '../../../vivim-ui/src/visual-runtime/command/compile.ts';
import { VISUAL_COMMAND_SCHEMA } from '../../../vivim-ui/src/visual-runtime/command/ir.ts';
import { checkArguments } from '../../../vivim-ui/src/visual-runtime/command/crosswalk.ts';
import { UiRevisionFixtureStore } from '../../../vivim-ui/src/visual-runtime/revisions/fixture-store.ts';
import { createHostCapabilityFixture } from '../../../vivim-ui/src/visual-runtime/host/fixture.ts';
import { projectObservedHostOutcome } from '../../../vivim-ui/src/visual-runtime/host/outcome.ts';

import {
  buildContext,
  buildExpectedVersions,
  CROSSWALK_ROWS,
  HOST_CATALOG_ACTIONS,
  HOST_CATALOG_DIGEST,
  HOST_CATALOG_VERSION,
} from './fixture.mjs';
import { TIERFLOW_SHA } from './schema.mjs';

/** TierFlow's own three outcomes translated into the Factory 01 vocabulary. */
export const OUTCOME_MAP = Object.freeze({
  candidate: 'accepted',
  clarification_required: 'clarification',
  refused: 'refusal',
});

const digestOf = (value) => createHash('sha256').update(String(value), 'utf8').digest('hex');

const canonicalArgs = (args) =>
  JSON.stringify(Object.keys(args ?? {}).sort().map((k) => [k, args[k]]));

/* ------------------------------------------------------------------ */
/* Scope verification against the REAL owning authorities              */
/* ------------------------------------------------------------------ */

const REVISION_STORE = new UiRevisionFixtureStore();

const REVISION_PROBES = {
  'ui:read': (p) => REVISION_STORE.readCurrent(p, 'doc-1'),
  'ui:draft': (p) => REVISION_STORE.createDraft(p, 'draft-1', 'doc-1', { schema: 'vivim.ui-document/1' }),
  'ui:publish': (p) => REVISION_STORE.publish(p, {
    commandId: 'probe-1', documentId: 'doc-1', expectedRevision: 0,
  }),
  'ui:rollback': (p) => REVISION_STORE.rollback(p, {
    commandId: 'probe-2', documentId: 'doc-1', expectedRevision: 0, targetRevision: 1,
  }),
};

const principal = (scopes) => ({
  ownerRef: 'owner-1',
  actorRef: 'actor-1',
  deviceRef: 'device-1',
  sessionRef: 'session-1',
  grantRef: 'grant-1',
  policyRevision: 'p1',
  scopes,
});

const HOST_FIXTURE = createHostCapabilityFixture({
  actions: HOST_CATALOG_ACTIONS,
  catalogVersion: HOST_CATALOG_VERSION,
  catalogDigest: HOST_CATALOG_DIGEST,
});

const hostContext = (scopes) => ({
  schema: 'fixture.trusted-host-context/1',
  authenticated: true,
  ownerRef: 'owner-1',
  actorRef: 'actor-1',
  deviceRef: 'device-1',
  sessionRef: 'session-1',
  worldRef: 'world-1',
  surface: 'desktop',
  nowMs: 1000,
  sessionExpiresAtMs: 100000,
  worldRevision: 'world-rev-1',
  policyRevision: 'p1',
  catalogVersion: HOST_CATALOG_VERSION,
  catalogDigest: HOST_CATALOG_DIGEST,
  availabilityByAction: Object.fromEntries(HOST_CATALOG_ACTIONS.map((a) => [a.actionId, 'yes'])),
  grant: {
    grantRef: 'grant-1',
    status: 'active',
    ownerRef: 'owner-1',
    actorRef: 'actor-1',
    deviceRef: 'device-1',
    sessionRef: 'session-1',
    worldRef: 'world-1',
    expiresAtMs: 100000,
    revision: 'grant-rev-1',
    scopes,
  },
});

const hostClaim = {
  planId: 'plan-1',
  planDigest: 'digest-1',
  actionId: 'ui.theme.set_preset',
  ownerRef: 'owner-1',
  actorRef: 'actor-1',
  deviceRef: 'device-1',
  sessionRef: 'session-1',
  worldRef: 'world-1',
  worldRevision: 'world-rev-1',
  state: 'claimed',
};

/**
 * Verify that `scope` is a scope the owning TierFlow authority really
 * enforces. Returns { ok, detail }.
 *
 * `ui:*` scopes are proved against the real UI revision port: without the
 * scope the port refuses with FORBIDDEN, with it the call reaches the
 * operation itself.
 *
 * `control:*` scopes are proved twice:
 *   - vocabulary: the real Host outcome projector accepts the scope string
 *     (src/visual-runtime/host/outcome.ts:41) — a non-circular proof that the
 *     scope name exists in the source;
 *   - binding: the real Host capability fixture denies the action when the
 *     grant lacks the scope the catalogue declares for it
 *     (src/visual-runtime/host/fixture.ts:105-113).
 */
export function verifyScope(scope, target) {
  if (typeof scope !== 'string' || !/^[a-z]+:[a-z]+$/.test(scope)) {
    return { ok: false, detail: `scope ${scope} is not a scope-shaped string` };
  }
  if (scope.startsWith('ui:')) {
    const probe = REVISION_PROBES[scope];
    if (!probe) return { ok: false, detail: `no real UI revision port enforces ${scope}` };
    let without;
    let withScope;
    try {
      without = probe(principal([]));
      withScope = probe(principal([scope]));
    } catch (error) {
      return { ok: false, detail: `revision port probe for ${scope} threw: ${error.message}` };
    }
    const refusedWithout = !without.ok && without.code === 'FORBIDDEN';
    const allowedWith = withScope.ok || withScope.code !== 'FORBIDDEN';
    return {
      ok: refusedWithout && allowedWith,
      detail: `${scope}: without=${without.ok ? 'ok' : without.code} with=${withScope.ok ? 'ok' : withScope.code}`,
    };
  }
  if (scope.startsWith('control:')) {
    // 1. Vocabulary: does the real Host projector accept this scope string?
    const denied = projectObservedHostOutcome(hostClaim, null, hostContext([]));
    const granted = projectObservedHostOutcome(hostClaim, null, hostContext([scope]));
    const vocabularyOk = denied.reasonCode === 'RECONCILE_SCOPE_DENIED' &&
      granted.reasonCode !== 'RECONCILE_SCOPE_DENIED';
    // 2. Binding: does the real host catalogue enforcement require this scope
    //    for the action under test?
    const actionId = target && target.actionId;
    const without = HOST_FIXTURE.describeForPrincipal(hostContext([]), actionId);
    const withScope = HOST_FIXTURE.describeForPrincipal(hostContext([scope]), actionId);
    const declared = without.requiredScopes ?? [];
    const bindingOk = declared.includes(scope) &&
      without.permittedForActor === 'no' && without.reasonCodes.includes('SCOPE_DENIED') &&
      withScope.permittedForActor === 'yes';
    return {
      ok: vocabularyOk && bindingOk,
      detail: `${scope}: vocabulary=${vocabularyOk} binding=${bindingOk} declared=[${declared.join(',')}]`,
    };
  }
  return { ok: false, detail: `unknown scope namespace in ${scope}` };
}

/* ------------------------------------------------------------------ */
/* Derivation                                                          */
/* ------------------------------------------------------------------ */

/** Build the TierFlow CommandInput for a case, exactly as the surface allows. */
export function buildInput(testCase, expectedVersions) {
  const base = {
    commandId: `case-${testCase.id}`,
    source: { surface: testCase.surface, inputDigest: digestOf(testCase.utterance) },
    expectedVersions,
  };
  if (testCase.surface === 'text') return { ...base, text: testCase.utterance };
  return { ...base, selection: testCase.selection };
}

/**
 * Re-derive one case's outcome from the pinned TierFlow source.
 * Returns { outcome, reasonCode, command, raw }.
 */
export function deriveOutcome(testCase) {
  const context = buildContext(testCase.contextOverrides);
  const expectedVersions = buildExpectedVersions(testCase.contextOverrides);
  const input = buildInput(testCase, expectedVersions);
  const raw = compileVisualCommand(input, context);
  return {
    outcome: OUTCOME_MAP[raw.outcome] ?? raw.outcome,
    reasonCode: raw.outcome === 'candidate' ? null : raw.reasonCode,
    command: raw.outcome === 'candidate' ? raw.command : null,
    raw,
  };
}

/* ------------------------------------------------------------------ */
/* Verification of one case                                           */
/* ------------------------------------------------------------------ */

/**
 * Compare one case's hand-written label to the independently derived result.
 * Returns { id, category, ok, disputes[], gaps[], scope }.
 */
export function verifyCase(testCase) {
  const disputes = [];
  const gaps = [];
  const derived = deriveOutcome(testCase);
  const label = testCase.expected;

  if (derived.outcome !== label.outcome) {
    disputes.push({
      kind: 'outcome',
      detail: `corpus says ${label.outcome}, source produces ${derived.outcome} (${derived.reasonCode ?? 'accepted'})`,
      expected: label.outcome,
      actual: derived.outcome,
    });
  }
  if ((derived.reasonCode ?? null) !== (label.reasonCode ?? null)) {
    disputes.push({
      kind: 'reasonCode',
      detail: `corpus says ${label.reasonCode ?? 'none'}, source produces ${derived.reasonCode ?? 'none'}`,
      expected: label.reasonCode ?? null,
      actual: derived.reasonCode ?? null,
    });
  }

  let scope = null;
  if (label.outcome === 'accepted' && label.command) {
    const actual = derived.command;
    if (!actual) {
      disputes.push({ kind: 'command', detail: 'corpus expects a typed command but the source produced none', expected: 'command', actual: 'none' });
    } else {
      if (actual.schema !== VISUAL_COMMAND_SCHEMA) {
        disputes.push({ kind: 'command', detail: `wrong command schema ${actual.schema}`, expected: VISUAL_COMMAND_SCHEMA, actual: actual.schema });
      }
      if (actual.semanticRef.id !== label.command.semanticId) {
        disputes.push({ kind: 'wrong-target', detail: `semantic id ${actual.semanticRef.id} != ${label.command.semanticId}`, expected: label.command.semanticId, actual: actual.semanticRef.id });
      }
      if (actual.semanticRef.version !== label.command.version) {
        disputes.push({ kind: 'wrong-target', detail: `version ${actual.semanticRef.version} != ${label.command.version}`, expected: label.command.version, actual: actual.semanticRef.version });
      }
      const targetKey = (t) => JSON.stringify([t.owner, t.actionId, t.catalogVersion]);
      if (targetKey(actual.target) !== targetKey(label.command.target)) {
        disputes.push({ kind: 'wrong-target', detail: `target ${targetKey(actual.target)} != ${targetKey(label.command.target)}`, expected: targetKey(label.command.target), actual: targetKey(actual.target) });
      }
      if (canonicalArgs(actual.args) !== canonicalArgs(label.command.args)) {
        disputes.push({ kind: 'wrong-argument', detail: `args ${canonicalArgs(actual.args)} != ${canonicalArgs(label.command.args)}`, expected: canonicalArgs(label.command.args), actual: canonicalArgs(actual.args) });
      }
      // The argument map must also satisfy the reviewed schema of its own row.
      const row = CROSSWALK_ROWS.find((r) => r.semanticRef.id === actual.semanticRef.id);
      if (row && !checkArguments(actual.args, row.argsSchema)) {
        disputes.push({ kind: 'wrong-argument', detail: 'derived arguments do not satisfy the reviewed args schema', expected: 'schema-valid', actual: 'schema-invalid' });
      }
      const scopeResult = verifyScope(label.command.scope, label.command.target);
      scope = { scope: label.command.scope, ok: scopeResult.ok, detail: scopeResult.detail };
      if (!scopeResult.ok) {
        disputes.push({ kind: 'wrong-scope', detail: scopeResult.detail, expected: label.command.scope, actual: 'not enforced by the owning authority' });
      }
    }
  }

  // The forward contract: a finding, never a failure.
  const requirement = testCase.requirement;
  if (requirement) {
    if (requirement.outcome !== derived.outcome) {
      gaps.push({
        kind: 'capability-gap',
        detail: `requirement ${requirement.outcome}, source ${derived.outcome} (${derived.reasonCode ?? 'accepted'})`,
        required: requirement.outcome,
        actual: derived.outcome,
      });
    }
    if (requirement.command && derived.command) {
      if (canonicalArgs(derived.command.args) !== canonicalArgs(requirement.command.args)) {
        gaps.push({ kind: 'requirement-argument-mismatch', detail: 'derived arguments differ from the requirement' });
      }
    }
  }

  return {
    id: testCase.id,
    category: testCase.category,
    utterance: testCase.utterance,
    labelMethod: testCase.labelMethod,
    ok: disputes.length === 0,
    disputes,
    gaps,
    scope,
    derived: { outcome: derived.outcome, reasonCode: derived.reasonCode },
  };
}

/**
 * Run the oracle over a corpus. Returns a full report.
 * `disputes` are label errors (the test fails on them). `gaps` are the
 * recorded capability findings.
 */
export function runOracle(cases) {
  const results = cases.map((testCase) => verifyCase(testCase));
  const disputes = results.filter((r) => !r.ok);
  const gaps = results.flatMap((r) => r.gaps.map((g) => ({ id: r.id, ...g })));
  const byCategory = {};
  for (const r of results) {
    byCategory[r.category] ??= { checked: 0, confirmed: 0, disputed: 0, gaps: 0 };
    byCategory[r.category].checked += 1;
    if (r.ok) byCategory[r.category].confirmed += 1;
    else byCategory[r.category].disputed += 1;
    byCategory[r.category].gaps += r.gaps.length;
  }
  return {
    tierflowSha: TIERFLOW_SHA,
    checked: results.length,
    confirmed: results.length - disputes.length,
    disputed: disputes.length,
    results,
    disputes,
    gaps,
    byCategory,
  };
}

/* ------------------------------------------------------------------ */
/* CLI                                                                 */
/* ------------------------------------------------------------------ */

const isMain = typeof process !== 'undefined' && process.argv[1] &&
  import.meta.url === pathToFileURL(process.argv[1]).href;

if (isMain) {
  const { CASES } = await import('./cases.mjs');
  const report = runOracle(CASES);
  console.log(`TierFlow SHA: ${report.tierflowSha}`);
  console.log(`cases checked: ${report.checked}`);
  console.log(`labels confirmed by the independent oracle: ${report.confirmed}`);
  console.log(`labels disputed by the independent oracle: ${report.disputed}`);
  console.log(`capability gaps recorded (requirement vs source): ${report.gaps.length}`);
  for (const dispute of report.disputes) {
    console.log(`DISPUTE ${dispute.id}: ${dispute.disputes.map((d) => d.detail).join(' | ')}`);
  }
  process.exitCode = report.disputed === 0 ? 0 : 1;
}
