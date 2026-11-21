// Read-only comparison of two normalized atlases. Scope and coverage are mandatory.
export const DIFF_SCHEMA = 'vivim.application-surface-diff/1';
const ATLAS_SCHEMA = 'vivim.application-surface-atlas/0.1';
const obj=v=>v!==null&&typeof v==='object'&&!Array.isArray(v);
function fail(code){const e=new Error(code);e.code=code;throw e;}
function summary(a) {
  const roles=new Map();
  for(const c of a.controls)roles.set(c.role,(roles.get(c.role)||0)+1);
  return Object.fromEntries([...roles.entries()].sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0));
}
const sorted = a=>[...new Set(a)].sort();
export function diffAtlases(before,after){
  for(const atlas of [before,after]){
    if(!obj(atlas)||atlas.schema!==ATLAS_SCHEMA || !obj(atlas.target) || !obj(atlas.source) ||
       !Array.isArray(atlas.evidence) || !Array.isArray(atlas.controls) || !Array.isArray(atlas.observations)||
       atlas.authority?.executionAuthorized!==false) fail('INVALID_ATLAS_FOR_DIFF');
  }
  if(before.target.siteId!==after.target.siteId || before.target.origin!==after.target.origin)fail('ATLAS_SCOPE_MISMATCH');
  const beforeRoles=summary(before),afterRoles=summary(after);
  const roleCounts=sorted([...Object.keys(beforeRoles),...Object.keys(afterRoles)]).map(role=>({role,before:beforeRoles[role]||0,after:afterRoles[role]||0,delta:(afterRoles[role]||0)-(beforeRoles[role]||0)}));
  const oldIds=sorted(before.evidence.map(e=>e.evidenceId)),newIds=sorted(after.evidence.map(e=>e.evidenceId));
  const a=before.observations.filter(o=>o.kind==='state-capture').reduce((n,o)=>n+(o.captured??0),0);
  const b=after.observations.filter(o=>o.kind==='state-capture').reduce((n,o)=>n+(o.captured??0),0);
  // Count delta is never interpreted as evidence of a removed application capability.
  return {schema:DIFF_SCHEMA,target:{siteId:before.target.siteId,origin:before.target.origin},
    snapshots:{before:before.source.bundleDigest??null,after:after.source.bundleDigest??null},
    evidence:{added:newIds.filter(id=>!oldIds.includes(id)),noLongerPresent:oldIds.filter(id=>!newIds.includes(id))},
    roleCounts,stateEntryCounts:{before:a,after:b,delta:b-a},
    status:'observation-difference-only',
    authority:{executionAuthorized:false,activationPermitted:false},
    caveats:['An observation count change does not prove that a feature appeared or disappeared','Evidence IDs may change when input or capture parameters change','Only overlapping bounded coverage can support deeper change claims']};
}
export function diffMarkdown(d){
  if(d?.schema!==DIFF_SCHEMA)fail('INVALID_DIFF');
  const lines=[`# Surface observation delta — ${d.target.siteId}`,'',
    `Added evidence identities: ${d.evidence.added.length}`,
    `Evidence identities not present in second atlas: ${d.evidence.noLongerPresent.length}`,
    `Captured state entries: ${d.stateEntryCounts.before} → ${d.stateEntryCounts.after}`,'',
    '| Role | Before | After | Δ |','|---|---:|---:|---:|'];
  for(const r of d.roleCounts)lines.push(`| ${r.role} | ${r.before} | ${r.after} | ${r.delta} |`);
  lines.push('','**Interpretation:** observations changed; feature removals, execution correctness and regression status remain unverified.');
  return lines.join('\n')+'\n';
}
