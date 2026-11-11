// T=0 fast lane: approved ChatGPT personalization/bootstrapping snapshot ->
// Host-owned candidate evidence. Never calls a browser or a model.
import {createHash} from 'node:crypto';
import {createHostLedgerStore} from './host-ledger-port.mjs';
const hash=s=>createHash('sha256').update(s).digest('hex');
const fail=code=>Object.assign(new Error(code),{code});
const valid=s=>typeof s==='string'&&s.length>0&&s.length<=256&&!/[\x00-\x1f]/.test(s);
export const FASTLANE_KINDS=Object.freeze({
 custom_instructions:{operation:'read_profile',lane:'profile',authority:'owner-authored'},
 memory_summary:{operation:'read_profile',lane:'profile',authority:'model-summarized'},
 project_instructions:{operation:'read_profile',lane:'profile',authority:'project-scoped'},
 bootstrap_f02:{operation:'run_bootstrap',lane:'bootstrap',authority:'model-derived'},
 bootstrap_f01:{operation:'run_bootstrap',lane:'bootstrap',authority:'model-derived'}
});
export function normalizeFastLaneSnapshot({accountId,kind,text,observedAt=new Date().toISOString(),
  capture={method:'owner-submitted',approved:false}}={}){
 if(!valid(accountId)||!FASTLANE_KINDS[kind]||typeof text!=='string'||!text.trim()||Buffer.byteLength(text)>256*1024)throw fail('SNAPSHOT_INPUT_INVALID');
 if(!Number.isFinite(Date.parse(observedAt)))throw fail('SNAPSHOT_TIME_INVALID');
 if(!['owner-submitted','studio-verified'].includes(capture?.method)||capture?.approved!==true)throw fail('SNAPSHOT_APPROVAL_REQUIRED');
 if(capture.method==='studio-verified'&&(!valid(capture.effectRef)||capture.verified!==true))throw fail('SNAPSHOT_RECEIPT_REQUIRED');
 if(kind.startsWith('bootstrap_')&&!(capture.method==='studio-verified'&&capture.verified===true))throw fail('SNAPSHOT_BOOTSTRAP_RECEIPT_REQUIRED');
 const digest=hash(text);
 const sourceId='chatgpt-profile:'+hash(accountId).slice(0,24)+':'+kind;
 const id='snapshot-'+hash(sourceId+'\0'+digest).slice(0,32);
 return {schema:'vivim.fastlane-snapshot/1',id,kind,accountId,sourceId,observedAt,
   authority:FASTLANE_KINDS[kind].authority,state:'candidate',contentDigest:digest,
   bytes:Buffer.byteLength(text),text,
   provenance:{method:capture.method,effectRef:capture.effectRef??null,verified:capture.verified===true}};
}
export function createHostFastLaneIntake({ledger,host,coordinator,worldId,accountId}={}){
 if(!ledger||!valid(worldId)||!valid(accountId)||!host?.authorizeSource||!host?.inspectEvidenceBatch||!host?.commitEvidenceCandidates||!coordinator?.recordBatch)throw fail('FASTLANE_PORTS_REQUIRED');
 const store=createHostLedgerStore({ledger});
 return Object.freeze({
  async ingest({kind,text,observedAt,capture}){
   const unit=normalizeFastLaneSnapshot({accountId,kind,text,observedAt,capture});
   const {operation,lane}=FASTLANE_KINDS[kind];
   const authorize=async()=>{
     const grant=await host.authorizeSource({worldId,accountId,sourceId:unit.sourceId,operation});
     if(grant?.allowed!==true||!valid(grant.grantId))throw fail('FASTLANE_GRANT_REQUIRED');
   };
   await authorize();
   const key='fastlane.'+hash(worldId+'\0'+accountId+'\0'+unit.id);
   const batchKey='fl-'+hash(unit.sourceId+'\0'+unit.contentDigest);
   const digest=hash(JSON.stringify({id:unit.id,kind:unit.kind,contentDigest:unit.contentDigest}));
   const mutate=fn=>store.transact(key,s=>fn(s?structuredClone(s):null));
   let state=mutate(s=>{
     if(!s)return {schema:'vivim.fastlane-job/1',worldId,accountId,sourceId:unit.sourceId,
       digest,batchKey,status:'pending'};
     if(s.worldId!==worldId||s.accountId!==accountId||s.digest!==digest||s.batchKey!==batchKey)throw fail('FASTLANE_CHECKPOINT_CONFLICT');
     return s;
   });
   if(state.status==='complete')return {...state,duplicateRun:true};
   await authorize();
   const observed=await host.inspectEvidenceBatch({key:batchKey,worldId,accountId});
   if(!observed||!['absent','committed','uncertain'].includes(observed.status))throw fail('FASTLANE_INSPECTION_REQUIRED');
   if(observed.status==='uncertain')throw fail('FASTLANE_UNCERTAIN');
   let receipt=observed.receipt;
   if(observed.status==='absent'){
     await authorize();
     receipt=await host.commitEvidenceCandidates({key:batchKey,digest,worldId,accountId,sourceId:unit.sourceId,
       unit,classification:'candidate-only'});
   }
   if(!receipt||receipt.key!==batchKey||receipt.digest!==digest||receipt.status!=='committed'||receipt.verified!==true)throw fail('FASTLANE_RECEIPT_REQUIRED');
   await authorize();
   coordinator.recordBatch({lane,sourceId:unit.sourceId,batchId:batchKey,sourceRevision:unit.contentDigest,
     items:[{nativeId:unit.id,kind:'profile_snapshot',contentDigest:unit.contentDigest}],acceptedByHost:true});
   state=mutate(s=>{if(s.status!=='pending')throw fail('FASTLANE_CHECKPOINT_CONFLICT');s.status='complete';s.receiptRef=batchKey;return s;});
   return {...state,duplicateRun:false};
  }
 });
}
