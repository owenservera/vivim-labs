// Candidate pure helpers for existing ARBOR consumers. Not an authority or interpreter.
import { createHash } from 'node:crypto';

// Exact JSON data, not VSL annotation-text normalization. Preserve literal user text.
export function canonical(value) {
  if (value === null || typeof value === 'boolean' || typeof value === 'string') return JSON.stringify(value);
  if (typeof value === 'number' && Number.isSafeInteger(value) && !Object.is(value, -0)) return String(value);
  if (Array.isArray(value)) {
    if (Object.keys(value).length !== value.length || Array.from({ length: value.length }, (_, i) => i).some(i => !Object.hasOwn(value, i))) throw new TypeError('Non-dense array');
    return '[' + value.map(canonical).join(',') + ']';
  }
  if (!value || typeof value !== 'object' || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) throw new TypeError('Non-JSON data');
  return '{' + Object.keys(value).sort().map(k => {
    const d = Object.getOwnPropertyDescriptor(value, k);
    if (!d || !Object.hasOwn(d, 'value')) throw new TypeError('Accessor');
    return JSON.stringify(k) + ':' + canonical(d.value);
  }).join(',') + '}';
}
export function fingerprint(domain, value) {
  if (typeof domain !== 'string' || !/^[a-z][a-z0-9.-]{0,63}$/.test(domain)) throw new TypeError('Domain');
  return createHash('sha256').update('arbor/vlang-candidate/1\0' + domain + '\0').update(canonical(value)).digest('hex');
}
export function changedAnchors(previous, current) {
  return [...new Set([...Object.keys(previous), ...Object.keys(current)])].sort().filter(k => !Object.hasOwn(previous, k) || !Object.hasOwn(current, k) || previous[k] !== current[k]);
}
export function anchorsCurrent(required, current) {
  const keys = Object.keys(required);
  return keys.length > 0 && keys.every(k => Object.hasOwn(current, k) && typeof required[k] === 'string' && required[k].length > 0 && required[k] === current[k]);
}
// Caller must establish actual verifier authenticity, scope and evidence validity.
export function planningClaim(claim, currentAnchors, { authorized = false, evidenceChecked = false } = {}) {
  return authorized === true && evidenceChecked === true && claim?.state === 'verified' && claim?.conflict === false && ['extracted','declared','observed','derived'].includes(claim.origin) && anchorsCurrent(claim.anchors ?? {}, currentAnchors);
}
// Caller provides the real renderer and existing compiler/parser for this action.
// No approval is issued. Caller binds result into the current Host plan digest.
export function previewBinding({ observable, manifest, versions, render, parse }) {
  if (!versions || !['catalog','compiler','contract','capability','policy','rules'].every(k => Object.hasOwn(versions, k) && (typeof versions[k] === 'string' && versions[k].length > 0 || Number.isSafeInteger(versions[k]) && versions[k] >= 0))) throw new TypeError('Version pins required');
  const sentence = render(observable);
  if (typeof sentence !== 'string' || !sentence.trim()) throw new TypeError('Missing canonical sentence');
  if (canonical(parse(sentence)) !== canonical(observable)) throw new Error('ROUND_TRIP_MISMATCH');
  return { sentence, bindingDigest: fingerprint('approval-preview', { sentence, observable, manifest, versions }) };
}
export function reusableCommandKey({ scope, template, parameters, versions }) {
  if (!scope || !template || !parameters || !versions || !['catalog','compiler','contract','capability','policy','rules','locale'].every(k => Object.hasOwn(versions, k) && (typeof versions[k] === 'string' && versions[k].length > 0 || Number.isSafeInteger(versions[k]) && versions[k] >= 0))) throw new TypeError('Incomplete reusable command key');
  return fingerprint('reusable-command', { scope, template, parameters, versions });
}
// Measure the entire actual rendered packet, not independently estimated cards.
export function boundedPacket({ required, optional, budget, render, measure }) {
  if (!Number.isSafeInteger(budget) || budget < 1 || !Array.isArray(required) || !Array.isArray(optional)) throw new TypeError('Invalid budget');
  function size(value) { const n = measure(value); if (!Number.isSafeInteger(n) || n < 0) throw new TypeError('Invalid measured cost'); return n; }
  let packet = render(required), cost = size(packet);
  if (cost > budget) return { outcome: 'budget_exceeded', minimum: cost };
  const included = [...required];
  for (const item of optional) {
    const next = render([...included, item]), n = size(next);
    if (n <= budget) { included.push(item); packet = next; cost = n; }
  }
  return { outcome: 'ready', packet, cost };
}
export function currentDraft(response, draft) {
  return ['requestId','draftRevision','worldRevision','catalogRevision'].every(k => Object.hasOwn(response, k) && Object.hasOwn(draft, k) && response[k] === draft[k]);
}
