import {createHash} from 'node:crypto';
import {lstatSync,readFileSync,realpathSync} from 'node:fs';
import {dirname,isAbsolute,relative,resolve,sep} from 'node:path';

export const sha256 = x => createHash('sha256').update(x).digest('hex');
export const HEX40 = /^[0-9a-f]{40}$/;
export const HEX64 = /^[0-9a-f]{64}$/;
export const EVIDENCE_SCHEMA = 'vivim.wave1.lane-f.proof/2';
export const CLASSES = new Set(['synthetic','static-source','observed-native','observed-device','owner-declared','unknown']);

export function assertSafePath(workspace, file,{mustExist=true}={}) {
  const root=realpathSync(workspace),p=resolve(root,file),rel=relative(root,p);
  if (!p.startsWith(root+sep) || rel.startsWith('..')||isAbsolute(rel)) throw Error('PATH_ESCAPE');
  // Refuse symlinks for every existing path component rather than trusting lexical containment.
  let step=p;
  while(step!==root){
    try { if(lstatSync(step).isSymbolicLink())throw Error('SYMLINK_REFUSED'); }
    catch(e){if(e.message==='SYMLINK_REFUSED')throw e;if(e.code!=='ENOENT'||mustExist)throw e;}
    step=dirname(step);
  }
  return p;
}
export function readPinned(workspace,path,{expectedSha,maxBytes=8*1024*1024}={}){
  const p=assertSafePath(workspace,path),b=readFileSync(p);
  if(b.length>maxBytes)throw Error('ARTIFACT_TOO_LARGE');
  const digest=sha256(b);
  if(expectedSha && (!HEX64.test(expectedSha)||expectedSha!==digest))throw Error('ARTIFACT_DIGEST_MISMATCH');
  return {path,sha256:digest,bytes:b.length,content:b.toString('utf8')};
}
export function validateProof(p,{requireNative=false}={}){
  const faults=[];
  if(!p||p.schema!==EVIDENCE_SCHEMA)faults.push('PROOF_SCHEMA');
  if(!CLASSES.has(p?.evidenceClass))faults.push('PROOF_CLASS');
  for(const key of ['id','producer','commandId','runtime','platform'])if(typeof p?.[key]!=='string'||!p[key].trim())faults.push('MISSING_'+key.toUpperCase());
  if(!HEX64.test(p?.artifactSha256??''))faults.push('PROOF_ARTIFACT_SHA');
  if(!HEX40.test(p?.sourceCommit??''))faults.push('PROOF_SOURCE_COMMIT');
  if(!Number.isInteger(p?.exitCode)||p.exitCode<0||p.exitCode>255)faults.push('PROOF_EXIT_CODE');
  if(requireNative && !['observed-native','observed-device'].includes(p?.evidenceClass))faults.push('NOT_NATIVE_PROOF');
  if(['observed-native','observed-device'].includes(p?.evidenceClass)&&p?.synthetic===true)faults.push('SYNTHETIC_MASQUERADE');
  if(p?.verifiedBy===p?.producer && requireNative)faults.push('SELF_CERTIFICATION');
  return {ok:faults.length===0,faults};
}
export function attestBundle(proofs,requiredIds=[]){
  const findings=[],ids=new Set();
  for(const p of Array.isArray(proofs)?proofs:[]){
    const check=validateProof(p,{requireNative:requiredIds.includes(p?.id)});
    if(ids.has(p.id))findings.push({id:p.id,kind:'DUPLICATE_PROOF'});ids.add(p.id);
    for(const f of check.faults)findings.push({id:p?.id??'unknown',kind:f});
  }
  for(const id of requiredIds)if(!ids.has(id))findings.push({id,kind:'MISSING_NATIVE_PROOF'});
  return {status:findings.length?'BLOCKED':'PROOFS_WELL_FORMED_NOT_INDEPENDENTLY_RERUN',findings};
}
