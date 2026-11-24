// Planning evidence classifier. Does not execute, authorize or certify external work.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

type Row = Record<string, any>;
const obj = (v: unknown): Row => v && typeof v === 'object' && !Array.isArray(v) ? v as Row : {};
const sha = (v: unknown) => typeof v === 'string' && /^[a-f0-9]{40}$/.test(v);
const text = (v: unknown) => typeof v === 'string' && v.trim().length > 0;
const task = (v: unknown) => typeof v === 'string' && /^(A(?:0[1-9]|1[0-6])|S(?:0[1-9]|1[0-4])|U(?:0[1-9]|1[01])|O0[1-6]|C0[1-5]|G0[1-5]|Z0[1-7]|R0[1-7])$/.test(v);
const commitLink = (v: unknown, repo: string, id: unknown) => sha(id) && v === `https://github.com/owenservera/${repo}/commit/${id}`;
const dated = (v: unknown) => typeof v === 'string' && Number.isFinite(Date.parse(v));
const count = (v: unknown) => Number.isSafeInteger(v) && Number(v) >= 0;

export function classifyRound(input: unknown) {
  const r = obj(input), p = obj(r.planning), work = Array.isArray(r.work) ? r.work : [];
  const identity = r.schema === 'vivim.dev-round/1' && /^L0[1-5]$/.test(String(r.lane)) && text(r.runId) && sha(r.baseSha) && dated(r.startedAt) && dated(r.finishedAt) && Date.parse(r.finishedAt) >= Date.parse(r.startedAt);
  const source = work.filter((v: unknown) => {
    const a = obj(v);
    return task(a.taskId) && a.kind === 'SOURCE_COMMIT' && a.repository === 'owenservera/ARBOR' && sha(a.beforeSha) && a.beforeSha !== a.commitSha && commitLink(a.evidence, 'ARBOR', a.commitSha) && Array.isArray(a.paths) && a.paths.length > 0 && a.paths.every(text);
  });
  const tests = work.filter((v: unknown) => {
    const a = obj(v);
    return task(a.taskId) && a.kind === 'TEST_EXECUTED' && sha(a.sourceSha) && text(a.command) && text(a.runId) && Number.isInteger(a.exitCode) && count(a.passed) && count(a.failed) && a.passed + a.failed > 0 && ((a.exitCode === 0 && a.failed === 0) || (a.exitCode !== 0 && a.failed > 0)) && text(a.evidence);
  });
  const verification = work.filter((v: unknown) => {
    const a = obj(v);
    return task(a.taskId) && a.kind === 'OBJECTIVE_VERIFICATION' && sha(a.sourceSha) && text(a.command) && text(a.runId) && text(a.result) && text(a.acceptanceCriterion) && text(a.evidence);
  });
  const next = obj(p.next);
  const planned = task(next.taskId) && text(next.scope) && text(next.action) && text(next.proof) && (p.state === 'PUBLISHED' ? commitLink(p.evidence, 'vivim-labs', p.commitSha) : p.state === 'REVALIDATED_UNCHANGED' && sha(p.blobSha) && text(p.evidence));
  const countWork = source.length + tests.length + verification.length;
  const fallback = Array.isArray(r.attempts) && r.attempts.some((v: unknown) => { const a = obj(v); return a.executor === 'GITHUB_CLOUD' && text(a.operation) && text(a.result); });
  const issues = [];
  if (!identity) issues.push('INVALID_ROUND_IDENTITY');
  if (!countWork) issues.push('NO_ENGINEERING_RECEIPT');
  if (!planned) issues.push('NO_PERSISTED_NEXT_WORK_ORDER');
  if (r.pc === 'OFFLINE' && !fallback) issues.push('NO_CLOUD_FALLBACK_ATTEMPT');
  return { schema: 'vivim.dev-round-classification/1', state: issues.length ? 'INCOMPLETE' : 'STRUCTURALLY_COMPLETE', sourceCommits: source.length, executedTests: tests.length, objectiveVerifications: verification.length, planningRecorded: planned, issues, caveat: 'Receipt shape only. Independently fetch commits, logs and work orders before crediting actual progress; never closes canonical tasks.' };
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    if (process.argv.length !== 3) throw new Error('USAGE: round-admission.ts <round.json>');
    const raw = readFileSync(process.argv[2], 'utf8');
    if (Buffer.byteLength(raw) > 128 * 1024) throw new Error('RECEIPT_TOO_LARGE');
    const result = classifyRound(JSON.parse(raw));
    console.log(JSON.stringify(result, null, 2));
    if (result.state !== 'STRUCTURALLY_COMPLETE') process.exitCode = 3;
  } catch (e) { console.error('INVALID_RECEIPT:', String(e).slice(0, 160)); process.exitCode = 2; }
}
