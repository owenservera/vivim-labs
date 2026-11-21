// VIVIM Labs research bridge. This module has no browser, account, Host or execution access.
import { createHash } from 'node:crypto';
import { readFileSync, realpathSync, statSync } from 'node:fs';
import { resolve } from 'node:path';

export const BUNDLE_FORMAT = 'craft-anything-evidence-bundle';
export const ATLAS_SCHEMA = 'vivim.application-surface-atlas/0.1';
const ID_RE = /^ev_[0-9a-f]{64}$/;
const DIGEST_RE = /^[0-9a-f]{64}$/;
const OPS = new Set(['web_site_open', 'web_site_capture', 'web_page_states_capture', 'web_page_show', 'web_walkthrough_record', 'web_snapshots_compare', 'web_pixels_compare', 'web_page_screenshot_take', 'web_walkthrough_check']);
const LIMIT_BYTES = 32 * 1024 * 1024;
const LIMIT_RECORDS = 2500;
const ROLES = new Set(['button', 'link', 'tab', 'menuitem', 'textbox', 'combobox', 'checkbox', 'radio', 'dialog', 'navigation', 'main', 'form', 'listbox', 'option', 'switch', 'searchbox', 'heading', 'summary']);
const SIGNIN = /password|secret|credential|token|api[-_]?key|authorization|bearer|cookie|session/i;
const PERSONAL = /\b[\w.+-]+@[\w.-]+\.[a-z]{2,}\b|\b\d{6,}\b|https?:\/\/|[A-Za-z0-9_-]{40,}/i;

const obj = (v) => v !== null && typeof v === 'object' && !Array.isArray(v);
function fail(code, details = '') { const error = new Error(`${code}${details ? ': ' + details : ''}`); error.code = code; throw error; }
const digest = (text) => createHash('sha256').update(text).digest('hex');
export function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(',')}]`;
  if (obj(value)) return `{${Object.keys(value).sort().filter(k => value[k] !== undefined).map(k => `${JSON.stringify(k)}:${canonicalJson(value[k])}`).join(',')}}`;
  return JSON.stringify(value);
}
export function evidenceDigest(record) {
  if (!obj(record) || !obj(record.subject)) fail('INVALID_EVIDENCE_SHAPE');
  const { id, summary, carries_target_content, subject, ...rest } = record;
  const { name, ...subjectIdentity } = subject;
  return `ev_${digest(canonicalJson({ ...rest, subject: subjectIdentity }))}`;
}
export function readJsonCapped(path, maxBytes = LIMIT_BYTES) {
  const abs = resolve(path);
  if (!statSync(abs).isFile() || statSync(abs).size > maxBytes) fail('INPUT_TOO_LARGE_OR_NOT_FILE');
  const bytes = readFileSync(abs);
  if (bytes.length > maxBytes) fail('INPUT_TOO_LARGE');
  let content;
  try { content = JSON.parse(bytes.toString('utf8')); } catch { fail('INVALID_JSON'); }
  return { content, sha256: digest(bytes), abs };
}
export function validateBundle(bundle) {
  if (!obj(bundle) || bundle.format !== BUNDLE_FORMAT || bundle.version !== 1 || !Array.isArray(bundle.records) || !Array.isArray(bundle.unknowns)) fail('UNSUPPORTED_CRAFT_BUNDLE');
  if (bundle.records.length > LIMIT_RECORDS || bundle.unknowns.length > LIMIT_RECORDS) fail('BUNDLE_LIMIT_EXCEEDED');
  const ids = new Map();
  for (const record of bundle.records) {
    if (!obj(record) || !ID_RE.test(record.id ?? '') || typeof record.operation !== 'string' || record.result === undefined || !obj(record.coverage) || !obj(record.environment)) fail('INVALID_EVIDENCE_SHAPE');
    if (!['observed', 'derived', 'inferred'].includes(record.confidence) || !['complete', 'partial', 'unknown'].includes(record.coverage.status)) fail('INVALID_EVIDENCE_QUALIFIER');
    if (evidenceDigest(record) !== record.id) fail('EVIDENCE_HASH_MISMATCH', record.id);
    if (ids.has(record.id) && canonicalJson(ids.get(record.id)) !== canonicalJson(record)) fail('EVIDENCE_ID_COLLISION');
    ids.set(record.id, record);
  }
  for (const record of ids.values()) for (const parent of record.links ?? []) {
    if (!ID_RE.test(parent)) fail('INVALID_EVIDENCE_LINK');
    // External links are allowed: a bundle may contain only a subset of an investigation.
  }
  return [...ids.values()].sort((a,b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0);
}
export function normalizedOrigin(url) {
  try {
    const u = new URL(url);
    if (!['https:', 'http:'].includes(u.protocol) || u.username || u.password) return null;
    return u.origin;
  } catch { return null; }
}
function scopedUrl(record) {
  const candidates = [record.result?.url, ...(Array.isArray(record.locations) ? record.locations.filter(x=>x.kind==='url').map(x=>x.value) : [])].filter(x=>typeof x==='string');
  const unique = [...new Set(candidates.map(normalizedOrigin).filter(Boolean))];
  return unique.length === 1 ? unique[0] : unique.length > 1 ? '__mixed__' : null;
}
function safeLabel(text, enabled) {
  if (!enabled || typeof text !== 'string') return null;
  const s = text.replace(/\s+/g, ' ').trim();
  return s && s.length <= 70 && !SIGNIN.test(s) && !PERSONAL.test(s) ? s : null;
}
function limitedCount(n) { return Number.isSafeInteger(n) && n >= 0 && n <= 100000 ? n : null; }
function captureMatch(record, dir, field) {
  if (!dir) return null;
  const expected = record.result?.[field];
  if (typeof expected !== 'string' || realpathSync(resolve(expected)) !== realpathSync(resolve(dir))) fail('CAPTURE_PROVENANCE_MISMATCH');
  return realpathSync(resolve(dir));
}
function sanitizeOutline(items, sourceId, labels) {
  if (!Array.isArray(items)) return [];
  return items.slice(0, 5000).flatMap((item, index) => {
    if (!obj(item)) return [];
    const role = [item.role, item.tag, item.type].find(x=>typeof x === 'string' && ROLES.has(x.toLowerCase()));
    if (!role) return [];
    const display = safeLabel(item.label ?? item.text ?? item.name ?? item.ariaLabel, labels);
    return [{kind:'control-observation', sourceEvidenceId:sourceId, position:index, role:role.toLowerCase(), label:display, confidence:'observed', effectAuthorized:false, locatorVerified:false}];
  });
}
function checkArtifactOrigin(content, origin) {
  if (!obj(content) || normalizedOrigin(content.url) !== origin) fail('ARTIFACT_ORIGIN_MISMATCH');
}
function constrainRecord(record, origin) {
  const sourceOrigin = scopedUrl(record);
  // Reject mismatches; don't silently pollute an atlas with another origin.
  if (sourceOrigin !== origin) fail('EVIDENCE_ORIGIN_UNVERIFIED', record.id);
  return {
    evidenceId:record.id,
    operation:record.operation,
    confidence:record.confidence,
    authority:record.authority,
    coverage:record.coverage.status,
    limitationCodes:(record.limitations ?? []).map(x=>x.code).filter(x=>typeof x==='string').slice(0,25),
    engine:typeof record.environment.engine==='string'?record.environment.engine:null,
    cleanRoom:record.environment.clean_room === true,
    carriesTargetContent:record.carries_target_content === true,
  };
}
export function buildAtlas({ bundle, siteId, origin, captureDir = null, statesDir = null, includeLabels = false, ownedTarget = false, bundleDigest = null } = {}) {
  if (!/^[a-z0-9][a-z0-9-]{0,79}$/.test(siteId ?? '') || normalizedOrigin(origin) !== origin) fail('INVALID_TARGET_SCOPE');
  if (includeLabels && !ownedTarget) fail('LABELS_REQUIRE_OWNED_TARGET');
  const records = validateBundle(bundle);
  const browser = records.filter(r => OPS.has(r.operation) && scopedUrl(r) === origin && obj(r.result));
  const foreign = records.filter(r=>OPS.has(r.operation) && scopedUrl(r) !== null && scopedUrl(r) !== origin);
  if (foreign.length) fail('MIXED_TARGET_ORIGINS', foreign.length + ' foreign browser record(s)');
  const observations = [], controls = [], knownEvidence = [];
  let usedCapture = false, usedStates = false;
  for (const r of browser) {
    const e = constrainRecord(r, origin);
    knownEvidence.push(e);
    if (r.operation === 'web_site_capture') {
      const counts = {};
      for (const name of ['outline','headings','links','forms','images']) counts[name] = limitedCount(r.result.counts?.[name]);
      const widths = Array.isArray(r.params?.widths) ? r.params.widths.filter(x=>Number.isSafeInteger(x) && x >= 320 && x <= 3840).slice(0,4) : [];
      observations.push({kind:'website-capture', evidenceId:r.id, counts, widths, status:r.result.sign_in_page?'auth-wall-observed':'captured', coverage:r.coverage.status, effectAuthorized:false});
      if (captureDir) {
        if (usedCapture) fail('MULTIPLE_CAPTURE_REFERENCES');
        const folder = captureMatch(r, captureDir, 'output_directory');
        const capture = readJsonCapped(resolve(folder, 'capture.json'), 8*1024*1024);
        checkArtifactOrigin(capture.content, origin);
        controls.push(...sanitizeOutline(capture.content.outline, r.id, includeLabels));
        observations.push({kind:'capture-artifact', evidenceId:r.id, sha256:capture.sha256, observedControls:controls.length, artifactAttestedByCraftEvidence:false, effectAuthorized:false});
        usedCapture = true;
      }
    }
    if (r.operation === 'web_page_states_capture') {
      const states = Array.isArray(r.result.states) ? r.result.states : [];
      observations.push({kind:'state-capture', evidenceId:r.id, discovered:limitedCount(r.result.found), captured:states.length, coverage:r.coverage.status, effectAuthorized:false});
      for (const [index,state] of states.slice(0,200).entries()) {
        if (!obj(state)) continue;
        const type = typeof state.kind==='string' && /^[a-z-]{1,32}$/.test(state.kind) ? state.kind : 'unknown';
        const changes = Array.isArray(state.changed) ? state.changed.filter(x=>['expanded','dialog appeared','tab selected','address changed','opened','closed','selection changed'].includes(x)) : [];
        observations.push({kind:'interaction-state',evidenceId:r.id,index,type,changes,label:safeLabel(state.label,includeLabels),effectAuthorized:false});
      }
      if (statesDir) {
        if (usedStates) fail('MULTIPLE_STATE_REFERENCES');
        const folder = captureMatch(r, statesDir, 'folder');
        const artifact = readJsonCapped(resolve(folder, 'states.json'), 8*1024*1024);
        if (!obj(artifact.content) || normalizedOrigin(artifact.content.url)!==origin || !Array.isArray(artifact.content.states)) fail('STATES_ARTIFACT_INVALID');
        observations.push({kind:'state-artifact',evidenceId:r.id,sha256:artifact.sha256, artifactAttestedByCraftEvidence:false,effectAuthorized:false});
        usedStates = true;
      }
    }
    if (r.operation === 'web_walkthrough_record') {
      const steps = Array.isArray(r.result.steps) ? r.result.steps : [];
      observations.push({kind:'walkthrough', evidenceId:r.id, steps:steps.length, failedSteps:steps.filter(s=>s.status!=='ok').length, coverage:r.coverage.status, effectAuthorized:false});
      for (const [index,step] of steps.slice(0,200).entries()) {
        if (!obj(step)) continue;
        const action=typeof step.action==='string' && ['goto','click','fill','press','select','wait'].includes(step.action) ? step.action : 'unknown';
        observations.push({kind:'walkthrough-step',evidenceId:r.id,index,action,status:step.status==='ok'?'observed-success':'unverified-or-failed',effectAuthorized:false});
      }
    }
  }
  if (captureDir && !usedCapture) fail('CAPTURE_NOT_ANCHORED');
  if (statesDir && !usedStates) fail('STATES_NOT_ANCHORED');
  const unknownIds = bundle.unknowns.map(x => x?.id).filter(x=>typeof x==='string'&&x.length<=100).sort();
  return {
    schema:ATLAS_SCHEMA,
    target:{siteId,origin,ownedTarget},
    source:{producer:'craft-anything',format:BUNDLE_FORMAT,version:1,bundleDigest:bundleDigest&&DIGEST_RE.test(bundleDigest)?bundleDigest:null,verifiedRecords:records.length,scopedBrowserRecords:browser.length},
    evidence:knownEvidence,
    observations,
    controls,
    unknownIds,
    authority:{executionAuthorized:false,activationPermitted:false,sendPermission:false},
    caveats:['Source hash verifies self-consistency, not authorization or runtime correctness','Capture artifacts carry independent digests, not Craft evidence attestation','No runtime interaction binding is promoted','Absence without complete scoped coverage is unknown']
  };
}
export function coverageMarkdown(atlas) {
  const n=atlas.observations.filter(o=>o.kind==='website-capture').length;
  const states=atlas.observations.filter(o=>o.kind==='state-capture').reduce((sum,o)=>sum+o.captured,0);
  const lines=[`# Application surface coverage — ${atlas.target.siteId}`, '',`Origin: ${atlas.target.origin}`,`Craft evidence verified: ${atlas.source.verifiedRecords}`,`Scoped browser records: ${atlas.source.scopedBrowserRecords}`,`Captured page snapshots: ${n}`,`Captured state entries: ${states}`,`Structural controls observed: ${atlas.controls.length}`,`Unknowns (IDs only): ${atlas.unknownIds.length}`,'','## Investigated operations'];
  for(const e of atlas.evidence) lines.push(`- ${e.operation} — ${e.evidenceId} (${e.coverage}; ${e.confidence})`);
  lines.push('','## Implementation status','- Import / integrity: recorded above','- Studio reconciliation: UNRUN (requires current ARBOR checkout)','- Native Rust/Host interaction validation: UNRUN','- Provider UI coverage completeness: UNKNOWN','- Capability activation: NOT AUTHORIZED','','This is a read-only developer report, not a feature certification.');
  return lines.join('\n')+'\n';
}