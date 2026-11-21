// Source-to-source development intelligence only. A match never enables a provider action.
import { createHash } from 'node:crypto';

export const BASELINE_SCHEMA = 'vivim.application-profile-baseline/1';
export const RECONCILIATION_SCHEMA = 'vivim.application-surface-reconciliation/1';
const ATLAS_SCHEMA = 'vivim.application-surface-atlas/0.1';
const FEATURES_MAX = 200;
const OWNERS = new Set(['studio', 'harness']);
const STATES = new Set(['configured', 'source-measured', 'unverified']);
const SURFACE_RE = /^[a-z][a-z0-9-]{0,63}$/;
const FEATURE_RE = /^[a-z][a-z0-9.-]{1,96}$/;
const CONTROL_RE = /^[a-z\d -]{1,70}$/i;
const obj = x => x !== null && typeof x === 'object' && !Array.isArray(x);
const digest = x => createHash('sha256').update(JSON.stringify(x)).digest('hex');
function fail(code, message='') { const e = new Error(message?`${code}: ${message}`:code); e.code=code; throw e; }
function safeText(x, max=200) { return typeof x === 'string' && x.length>0 && x.length<=max && !/[\x00-\x1f\x7f]/.test(x); }
const compare = (a,b)=>a<b?-1:a>b?1:0;
const sortedUnique = xs => [...new Set(xs)].sort(compare);

export function validateProfileBaseline(baseline, atlas) {
  if (!obj(baseline) || baseline.schema !== BASELINE_SCHEMA || !obj(baseline.target) ||
      baseline.target.siteId !== atlas?.target?.siteId || baseline.target.origin !== atlas?.target?.origin ||
      !Array.isArray(baseline.profiles) || baseline.profiles.length<1 || baseline.profiles.length>2) fail('INVALID_OR_MISMATCHED_BASELINE');
  const owners=new Set();
  for(const profile of baseline.profiles) {
    if(!obj(profile) || !OWNERS.has(profile.owner) || owners.has(profile.owner) ||
        !safeText(profile.sourcePath,320) || profile.sourcePath.startsWith('/') ||
        profile.sourcePath.includes('..') || !Array.isArray(profile.features) ||
        profile.features.length>FEATURES_MAX) fail('INVALID_BASELINE_PROFILE');
    owners.add(profile.owner);
    const ids=new Set();
    for(const feature of profile.features) {
      if(!obj(feature) || !FEATURE_RE.test(feature.id??'') || !SURFACE_RE.test(feature.surface??'') ||
          !STATES.has(feature.sourceStatus) || ids.has(feature.id) ||
          !Array.isArray(feature.selectors) || feature.selectors.length>25 ||
          !feature.selectors.every(s=>safeText(s,320)) ||
          !Array.isArray(feature.matchTerms) || feature.matchTerms.length>12 ||
          !feature.matchTerms.every(s=>CONTROL_RE.test(s)) ||
          (feature.notes!=null && !safeText(feature.notes,300))) fail('INVALID_BASELINE_FEATURE');
      ids.add(feature.id);
    }
  }
  return true;
}
export function reconcileAtlas({atlas,baseline}={}) {
  if(!obj(atlas) || atlas.schema!==ATLAS_SCHEMA || !obj(atlas.source) || !Array.isArray(atlas.controls) ||
     atlas.authority?.executionAuthorized!==false || atlas.authority?.activationPermitted!==false) fail('INVALID_ATLAS');
  validateProfileBaseline(baseline,atlas);
  const features=[];
  for(const profile of baseline.profiles) for(const f of profile.features) {
    const controlRefs=atlas.controls.filter(c=>typeof c.label==='string' && c.label.length>0 && f.matchTerms.some(t=>c.label.toLowerCase()===t.toLowerCase()))
      .map(c=>({evidenceId:c.sourceEvidenceId, position:c.position, role:c.role}));
    const observed=controlRefs.length>0;
    features.push({owner:profile.owner,featureId:f.id,surface:f.surface,sourceStatus:f.sourceStatus,
      sourcePath:profile.sourcePath,configuredSelectors:f.selectors.length,
      observationStatus:observed?'label-correspondence-unverified':'not-observed-in-scoped-capture',
      evidenceRefs:controlRefs,
      effectAuthorized:false});
  }
  features.sort((a,b)=>compare(a.featureId,b.featureId)||compare(a.owner,b.owner));
  const byId = new Map();
  for(const p of baseline.profiles) for(const f of p.features){const old=byId.get(f.id)??[];old.push({owner:p.owner,feature:f});byId.set(f.id,old);}
  const divergences=[];
  for(const [id,entries] of byId){
    if(entries.length!==2)continue;
    const [a,b]=entries.sort((x,y)=>compare(x.owner,y.owner));
    // Compare sets rather than candidate order; different priorities are not necessarily incompatibilities.
    const sa=sortedUnique(a.feature.selectors),sb=sortedUnique(b.feature.selectors);
    if(JSON.stringify(sa)!==JSON.stringify(sb)) divergences.push({featureId:id,kind:'selector-configuration-drift',
      owners:[a.owner,b.owner],onlyInFirst:sa.filter(s=>!sb.includes(s)),onlyInSecond:sb.filter(s=>!sa.includes(s)),
      status:'source-discrepancy-not-runtime-defect'});
  }
  divergences.sort((a,b)=>compare(a.featureId,b.featureId));
  const terms=sortedUnique(baseline.profiles.flatMap(p=>p.features.flatMap(f=>f.matchTerms)).map(t=>t.toLowerCase()));
  const unmapped=atlas.controls.filter(c=>typeof c.label==='string' && c.label && !terms.includes(c.label.toLowerCase()))
    .slice(0,100).map(c=>({evidenceId:c.sourceEvidenceId,position:c.position,role:c.role,label:c.label,status:'unmapped-observation'}));
  return {
    schema:RECONCILIATION_SCHEMA,
    target:{siteId:atlas.target.siteId,origin:atlas.target.origin},
    source:{atlasBundleDigest:atlas.source.bundleDigest??null,baselineDigest:digest(baseline),baselinePaths:baseline.profiles.map(p=>p.sourcePath).sort()},
    features,divergences,unmapped,
    summary:{configuredFeatures:features.length,labelCorrespondences:features.filter(f=>f.observationStatus==='label-correspondence-unverified').length,
      sourceDivergences:divergences.length,unmappedNamedControls:unmapped.length},
    authority:{executionAuthorized:false,activationPermitted:false,sendPermission:false},
    caveats:['A selector present in source is not verified against the captured application','Label correspondence is a discovery hint, not a unique locator or runtime proof','No source claim is promoted into the Studio learned-adapter registry']
  };
}
export function reconciliationMarkdown(r) {
  if(r?.schema!==RECONCILIATION_SCHEMA) fail('INVALID_RECONCILIATION');
  const lines=[`# Studio / Harness source reconciliation — ${r.target.siteId}`,'',
    `Configured feature statements: ${r.summary.configuredFeatures}`,
    `Label correspondences (unverified): ${r.summary.labelCorrespondences}`,
    `Selector-configuration divergences: ${r.summary.sourceDivergences}`,
    `Unmapped named controls: ${r.summary.unmappedNamedControls}`,'',
    '## Source profile coverage','',
    '| Feature | Owner | Source status | Capture linkage |','|---|---|---|---|'];
  for(const f of r.features)lines.push(`| ${f.featureId} | ${f.owner} | ${f.sourceStatus} | ${f.observationStatus} |`);
  lines.push('','## Source divergences');
  if(!r.divergences.length) lines.push('None found across shared feature IDs.');
  for(const d of r.divergences){lines.push(`- ${d.featureId}: source selectors differ between ${d.owners.join(' / ')}; **not evidence of runtime failure**.`);
    for(const s of d.onlyInFirst)lines.push(`  - Only ${d.owners[0]}: \`${s.replaceAll('`','')}\``);
    for(const s of d.onlyInSecond)lines.push(`  - Only ${d.owners[1]}: \`${s.replaceAll('`','')}\``);
  }
  lines.push('','## Unmapped observed labels (first 100)');
  if(!r.unmapped.length)lines.push('None, or labels were not authorized in capture.');
  for(const u of r.unmapped)lines.push(`- ${u.role}: ${u.label.replaceAll('`','').replaceAll('|',' ')} — ${u.evidenceId}`);
  lines.push('','No action permitted; no live-run verification was performed by this comparator.');
  return lines.join('\n')+'\n';
}
