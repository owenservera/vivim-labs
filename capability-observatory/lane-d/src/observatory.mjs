import { createHash } from 'node:crypto';

export const SCHEMA = 'vivim.catalogue-export/1';
export const SNAPSHOT_SCHEMA = 'vivim.labs.observatory-snapshot/1';
const STATUSES = new Set(['active','historical','missing','unlocated','stale','blocked','conflicting','unknown']);
const FINDINGS = new Set(['duplicate_exact','duplicate_semantic','authority_conflict','code_without_tracker','tracker_without_code','research_unused','missing_reference','stale_abandoned','orphan_other']);
const safeText = value => typeof value === 'string' ? value.slice(0,240) : '';
const uniq = a => [...new Set(a)].sort();
const sha = value => createHash('sha256').update(JSON.stringify(value)).digest('hex');
const isObj = value => value !== null && typeof value === 'object' && !Array.isArray(value);
const error = message => { throw new Error('HQ_EXPORT_INVALID: ' + message); };
const revisionOk = value => typeof value === 'string' && /^[0-9a-f]{40}$/i.test(value);

// A narrow, fail-closed adapter: HQ produces identities and evidence; Labs never resolves or merges them.
export function validateHqExport(envelope) {
  if (!isObj(envelope) || envelope.schema !== SCHEMA) error('unsupported/missing schema');
  if (envelope.producer !== 'repos-hq') error('producer must be repos-hq');
  if (!isObj(envelope.revision) || !revisionOk(envelope.revision.head)) error('missing pinned HQ head');
  for (const k of ['sources','entities','links','findings','tombstones','conflicts']) {
    if (!Array.isArray(envelope[k])) error(k + ' must be an array');
    if (envelope[k].length > 200000) error(k + ' exceeds bounded import');
  }
  const entityIds = new Set();
  for (const e of envelope.entities) {
    if (!isObj(e) || typeof e.id !== 'string' || !e.id || e.id.length > 256) error('invalid entity id');
    if (entityIds.has(e.id)) error('duplicate entity id ' + e.id);
    entityIds.add(e.id);
  }
  for (const t of envelope.tombstones) if (typeof t.entity_id !== 'string') error('invalid tombstone');
  for (const f of envelope.findings) {
    if (!FINDINGS.has(f.kind) || !Array.isArray(f.entity_ids) || !f.id) error('invalid finding');
    if (f.entity_ids.some(id => typeof id !== 'string')) error('invalid finding entity id');
  }
  return envelope;
}

const evidencePresent = row => Array.isArray(row.evidence_refs) && row.evidence_refs.length > 0;
export function certifiedProduction(row) {
  return !!row && ['deployed','active_verified','active'].includes(row.state) &&
    revisionOk(row.build_sha) && typeof row.observed_at === 'string' && !Number.isNaN(Date.parse(row.observed_at)) &&
    evidencePresent(row) && !!row.owner_approval_ref && !!row.activation_receipt_ref && !!row.device_or_client_ref &&
    row.receipt_verification === 'verified' && row.owner_approval_verification === 'verified';
}

const validId = v => typeof v === 'string' && v.length > 0 && v.length <= 256;
export function buildSnapshot({capabilities=[], dev=[], production=[], sourceCatalog=[], gates={}, hq=null, observedAt='2026-11-07T00:00:00Z'}={}) {
  if (![capabilities, dev, production, sourceCatalog].every(Array.isArray)) throw new Error('Invalid Labs ledgers');
  if (hq != null) validateHqExport(hq);
  const devById = new Map(dev.map(r=>[r.capability_id,r]));
  const certified = production.filter(certifiedProduction);
  const prodIds = new Set(certified.map(r=>r.capability_id));
  const capIds = new Set();
  const caps = capabilities.map(c=>{
    if (!validId(c.capability_id) || capIds.has(c.capability_id)) throw new Error('Duplicate/invalid seed ID: '+ c.capability_id);
    capIds.add(c.capability_id);
    const d = devById.get(c.capability_id);
    return {id:c.capability_id, label:safeText(c.name), domain:safeText(c.domain), kind:safeText(c.kind), owner:safeText(c.owner_repo), surfaces:uniq(c.surfaces||[]), wave_lanes:uniq(c.wave_lanes||[]), baseline:safeText(c.baseline_evidence), maturity:d?.stage||'unknown', dev_status:d?.status||'unknown', risk:d?.status==='blocked'?'blocked':'unknown', target:safeText(c.target), live:prodIds.has(c.capability_id), provenance:uniq(c.source_refs||[]).slice(0,32), source_sha:revisionOk(d?.source_sha)?d.source_sha:null, blockers:(d?.blockers||[]).map(safeText).slice(0,12), hq_links:[]};
  });
  let visibleHq=0, hiddenHq=0, unmapped=0, invalidLinks=0;
  const deleted = new Set(hq?.tombstones.map(t=>t.entity_id) || []);
  const hqIds = new Set(hq?.entities.map(e=>e.id) || []);
  const visibility = new Map();
  for (const entity of (hq?.entities||[])) {
    if (deleted.has(entity.id)) continue;
    const hidden = entity.privacy !== 'public'; // unknown is private by default
    visibility.set(entity.id,!hidden);
    if (hidden) {hiddenHq++;continue;}
    visibleHq++;
    const links=uniq((entity.capability_ids||[]).filter(validId));
    let matched=false;
    for (const id of links){const cap=caps.find(c=>c.id===id);if(cap){cap.hq_links.push(entity.id);matched=true;}}
    if (!matched) unmapped++;
  }
  const findings = (hq?.findings||[]).filter(f=>f.entity_ids.every(id=>visibility.get(id)===true)&& f.entity_ids.every(id=>!deleted.has(id))).map(f=>({
    id:safeText(f.id),kind:f.kind, status:STATUSES.has(f.status)?f.status:'unknown',confidence:Number.isFinite(f.confidence)?Math.max(0,Math.min(f.confidence,1)):null, entities:uniq(f.entity_ids), reason:safeText(f.reason), provenance:uniq((f.provenance||[]).filter(validId)).slice(0,16)
  })).sort((a,b)=>a.kind.localeCompare(b.kind)||a.id.localeCompare(b.id));
  for(const edge of hq?.links||[]) if(!hqIds.has(edge.from)||!hqIds.has(edge.to)) invalidLinks++;
  const allSources = (hq?.sources||sourceCatalog.map(s=>({source_id:s.id,repository:s.repository,revision:s.seed_sha,status:s.intake_state==='unlocated'?'unlocated':'unknown',measured_rows:s.measured_items,path:s.paths?.[0]})));
  const hiddenSources = hq ? allSources.filter(s=>s.privacy!=='public').length : 0;
  const sources = allSources.filter(s=>!hq||s.privacy==='public').map(s=>({id:safeText(s.source_id),repository:safeText(s.repository),path:safeText(s.path),revision:revisionOk(s.revision)?s.revision:null,status:STATUSES.has(s.status)?s.status:'unknown',age_days:typeof s.updated_at==='string' && !Number.isNaN(Date.parse(s.updated_at))?Math.max(0,Math.floor((Date.parse(observedAt)-Date.parse(s.updated_at))/86400000)):null,measured_rows:Number.isInteger(s.measured_rows)&&s.measured_rows>=0?s.measured_rows:null})).sort((a,b)=>a.id.localeCompare(b.id));
  const conflicts = (hq?.conflicts||[]).filter(c=>(c.entity_ids||[]).every(id=>visibility.get(id)===true)).map(c=>({id:safeText(c.id),status:'conflicting',reason:safeText(c.reason),entity_ids:uniq(c.entity_ids||[])})).sort((a,b)=>a.id.localeCompare(b.id));
  const findingCounts={};for(const f of findings)findingCounts[f.kind]=(findingCounts[f.kind]||0)+1;
  const summary={seed_capabilities:caps.length,dev_rows:dev.length,certified_production:certified.length,uncertified_production:production.length-certified.length,hq_export:hq?'accepted':'missing',hq_entities_public:visibleHq,hq_entities_restricted:hiddenHq,hq_sources_restricted:hiddenSources,unmapped_hq_public:hq?unmapped:null,duplicate_clusters:hq?(findingCounts.duplicate_exact||0)+(findingCounts.duplicate_semantic||0):null,orphan_findings:hq?findings.filter(f=>f.kind.includes('without_')||f.kind==='research_unused'||f.kind==='stale_abandoned'||f.kind==='orphan_other').length:null,conflict_findings:hq?conflicts.length+(findingCounts.authority_conflict||0):null,invalid_hq_links:hq?invalidLinks:null,unlocated_sources:sources.filter(s=>s.status==='unlocated').length,measured_source_rows:sources.filter(s=>s.measured_rows!==null).reduce((a,s)=>a+s.measured_rows,0),unmeasured_sources:sources.filter(s=>s.measured_rows===null).length};
  const safeGates={gate5:gates.gate5==='GO'?'GO':gates.gate5==='BLOCKED'?'BLOCKED':'UNKNOWN',gate6:gates.gate6==='GO'?'GO':gates.gate6==='BLOCKED'?'BLOCKED':'UNKNOWN',source_sha:revisionOk(gates.source_sha)?gates.source_sha:null,evidence_ref:safeText(gates.evidence_ref),fixture_lanes:Number.isInteger(gates.fixture_lanes)?gates.fixture_lanes:null,device_acceptance:'UNVERIFIED',host_acceptance:'UNVERIFIED'};
  const canonical = {schema:SNAPSHOT_SCHEMA,hq_revision:hq?.revision.head||null,labs_seed_revision:null,summary,gates:safeGates,capabilities:caps.sort((a,b)=>a.id.localeCompare(b.id)),sources,findings,conflicts,deleted_entities:hq?deleted.size:null};
  return { ...canonical, observed_at:observedAt, snapshot_id:'sha256:'+sha(canonical)};
}
