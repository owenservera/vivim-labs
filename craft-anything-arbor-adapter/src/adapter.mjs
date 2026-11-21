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
const TAG_ROLES = Object.freeze({a:'link',input:'textbox',textarea:'textbox',select:'combobox',h1:'heading',h2:'heading',h3:'heading',h4:'heading',nav:'navigation'});
const INTERACTIVE = new Set(['button','link','tab','menuitem','textbox','combobox','checkbox','radio','listbox','option','switch','searchbox','summary']);
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
export function evidenceIdentityJson(record) {
  if (!obj(record) || !obj(record.subject)) fail('INVALID_EVIDENCE_SHAPE');
  const { id, summary, carries_target_content, subject, ...rest } = record;
  const { name, ...subjectIdentity } = subject;
  return canonicalJson({ ...rest, subject: subjectIdentity });
}
export function evidenceDigest(record) {
  return `ev_${digest(evidenceIdentityJson(record))}`;
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
export function validateBundle(bundle, {nativeVerifiedIds = null} = {}) {
  if (!obj(bundle) || bundle.format !== BUNDLE_FORMAT || bundle.version !== 1 || !Array.isArray(bundle.records) || !Array.isArray(bundle.unknowns)) fail('UNSUPPORTED_CRAFT_BUNDLE');
  if (bundle.records.length > LIMIT_RECORDS || bundle.unknowns.length > LIMIT_RECORDS) fail('BUNDLE_LIMIT_EXCEEDED');
  const ids = new Map();
  for (const record of bundle.records) {
    if (!obj(record) || !ID_RE.test(record.id ?? '') || typeof record.operation !== 'string' || record.result === undefined || !obj(record.coverage) || !obj(record.environment)) fail('INVALID_EVIDENCE_SHAPE');
    if (!['observed', 'derived', 'inferred'].includes(record.confidence) || !['complete', 'partial', 'unknown'].includes(record.coverage.status)) fail('INVALID_EVIDENCE_QUALIFIER');
    if (evidenceDigest(record) !== record.id && !(nativeVerifiedIds instanceof Set && nativeVerifiedIds.has(record.id))) fail('EVIDENCE_HASH_MISMATCH', record.id);
    // Display-only fields can differ without changing Craft's evidence identity.
    // Preserve the first byte-equivalent identity and refuse any different hashed payload.
    if (ids.has(record.id) && evidenceIdentityJson(ids.get(record.id)) !== evidenceIdentityJson(record)) fail('EVIDENCE_ID_COLLISION');
    if (!ids.has(record.id)) ids.set(record.id, record);
  }
  for (const unknown of bundle.unknowns) {
    if (!obj(unknown) || !/^unk_[a-f0-9]{16}$/.test(unknown.id??'') ||
        !Number.isSafeInteger(unknown.revision) || unknown.revision<1 ||
        !['open','investigating','blocked','contradicted','resolved'].includes(unknown.status) ||
        !['low','medium','high','critical'].includes(unknown.severity)) fail('INVALID_CRAFT_UNKNOWN');
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
// Craft's blueprint uses HTML tags (a/input/textarea/select), not only ARIA roles.
// Never turn a captured node into a selector or a runnable action.
function sanitizeOutline(items, sourceId, labels) {
  if (!Array.isArray(items)) return [];
  return items.slice(0, 900).flatMap((item, index) => {
    if (!obj(item)) return [];
    const tag = typeof item.tag === 'string' ? item.tag.toLowerCase() : '';
    if (tag === 'input' && ['password','hidden','file'].includes(String(item.type || '').toLowerCase())) return [];
    const declaredRole = typeof item.role === 'string' ? item.role.toLowerCase() : '';
    const role = ROLES.has(declaredRole) ? declaredRole
      : (ROLES.has(tag) ? tag : (TAG_ROLES[tag] ?? null));
    if (!role) return [];
    const display = INTERACTIVE.has(role) ? safeLabel(item.label ?? item.text ?? item.name ?? item.ariaLabel, labels) : null;
    return [{kind:'control-observation', sourceEvidenceId:sourceId, position:index, role, label:display, confidence:'observed', effectAuthorized:false, locatorVerified:false}];
  });
}
function artifactFile(folder, filename, expectedSha) {
  // Names come from verified Craft evidence, but never treat them as paths.
  if (typeof filename !== 'string' || !/^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,90}\.png$/.test(filename) ||
      filename === '..' || !DIGEST_RE.test(expectedSha ?? '')) fail('INVALID_CAPTURE_ASSET_REFERENCE');
  const path = resolve(folder, filename);
  if (realpathSync(path) !== path) fail('CAPTURE_ASSET_PATH_UNSAFE');
  const info = statSync(path);
  if (!info.isFile() || info.size < 8 || info.size > 32*1024*1024) fail('CAPTURE_ASSET_SIZE_INVALID');
  const bytes = readFileSync(path);
  if (!bytes.subarray(0,8).equals(Buffer.from([137,80,78,71,13,10,26,10]))) fail('CAPTURE_ASSET_NOT_PNG');
  if (digest(bytes) !== expectedSha) fail('CAPTURE_ASSET_DIGEST_MISMATCH');
  return true;
}
function verifyScreenshots(folder, entries) {
  if (!Array.isArray(entries)) return 0;
  if (entries.length > 200) fail('CAPTURE_ASSET_LIMIT_EXCEEDED');
  let count = 0;
  for (const entry of entries) {
    if (!obj(entry) || typeof entry.file !== 'string') continue;
    artifactFile(folder, entry.file, entry.sha256);
    count++;
  }
  return count;
}
function verifyCaptureSummary(capture, craftRecord) {
  if (!obj(capture) || capture.url !== craftRecord.result.url) fail('CAPTURE_CONTENT_MISMATCH');
  for (const [key,path] of Object.entries({outline:'outline',headings:'headings',links:'links',forms:'forms',images:'assets.images'})) {
    const expected = craftRecord.result.counts?.[key];
    if (expected == null) continue;
    const parts = path.split('.');
    const value = parts.reduce((v,p)=>v?.[p],capture);
    if (!Array.isArray(value) || value.length !== expected) fail('CAPTURE_COUNT_MISMATCH', key);
  }
  // Craft records its design object in the content-hashed evidence result.
  if (craftRecord.result.design !== undefined && canonicalJson(capture.design ?? null) !== canonicalJson(craftRecord.result.design ?? null)) fail('CAPTURE_DESIGN_MISMATCH');
}
function a11ySummary(capture, sourceId) {
  const a = capture?.a11y;
  if (!obj(a)) return null;
  const landmarks = {};
  if (obj(a.landmarks)) for (const [key,value] of Object.entries(a.landmarks)) {
    if (/^[a-z-]{1,32}$/.test(key) && limitedCount(value) !== null) landmarks[key] = value;
  }
  return {kind:'accessibility-summary',evidenceId:sourceId,unnamedControls:limitedCount(a.unnamed_controls),imagesWithoutAlt:limitedCount(a.images_without_alt),landmarks,effectAuthorized:false};
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
export function buildAtlas({ bundle, siteId, origin, captureDir = null, statesDir = null, includeLabels = false, ownedTarget = false, bundleDigest = null, nativeVerifiedIds = null } = {}) {
  if (!/^[a-z0-9][a-z0-9-]{0,79}$/.test(siteId ?? '') || normalizedOrigin(origin) !== origin) fail('INVALID_TARGET_SCOPE');
  if (includeLabels && !ownedTarget) fail('LABELS_REQUIRE_OWNED_TARGET');
  const records = validateBundle(bundle, {nativeVerifiedIds});
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
        verifyCaptureSummary(capture.content, r);
        const verifiedScreenshots = verifyScreenshots(folder, r.result.screenshots);
        const first = controls.length;
        controls.push(...sanitizeOutline(capture.content.outline, r.id, includeLabels));
        observations.push({kind:'capture-artifact', evidenceId:r.id, sha256:capture.sha256, observedControls:controls.length - first, verifiedScreenshots, artifactAttestedByCraftEvidence:false, effectAuthorized:false});
        const a11y = a11ySummary(capture.content,r.id);
        if (a11y) observations.push(a11y);
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
        if (!obj(artifact.content) || artifact.content.url !== r.result.url || !Array.isArray(artifact.content.states) ||
            canonicalJson(artifact.content.states) !== canonicalJson(states) ||
            (r.params?.width != null && artifact.content.width !== r.params.width)) fail('STATES_ARTIFACT_INVALID');
        const verifiedScreenshots = verifyScreenshots(folder, states);
        observations.push({kind:'state-artifact',evidenceId:r.id,sha256:artifact.sha256,verifiedScreenshots, artifactAttestedByCraftEvidence:false,effectAuthorized:false});
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
  const unknownIds = [...new Set(bundle.unknowns.map(x => x.id))].sort();
  return {
    schema:ATLAS_SCHEMA,
    target:{siteId,origin,ownedTarget},
    source:{producer:'craft-anything',format:BUNDLE_FORMAT,version:1,bundleDigest:bundleDigest&&DIGEST_RE.test(bundleDigest)?bundleDigest:null,verifiedRecords:records.length,scopedBrowserRecords:browser.length,nativeVerified:nativeVerifiedIds instanceof Set},
    evidence:knownEvidence,
    observations,
    controls,
    unknownIds,
    authority:{executionAuthorized:false,activationPermitted:false,sendPermission:false},
    caveats:['JavaScript source hash is only locally checked unless nativeVerified is true; neither hash establishes authorization','Capture artifacts carry independent digests, not Craft evidence attestation','No runtime interaction binding is promoted','Absence without complete scoped coverage is unknown']
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
