// Static-contract unit tests only; no live providers, Host effects or devices.
import test from 'node:test';
import assert from 'node:assert/strict';
import {admitReceipt} from './receipt-gate.ts';

const SRC='f5623943b746daac3d55a0ae4896430083a568ba';
const newer='dfb5480b17f5daa8861352e0cb81dd97e01e9f9d';
const hash='a'.repeat(64);
const base=()=>({
 schema:'vivim.acceptance-evidence/1',taskId:'Z01-VERIFIER',claim:'SOURCE_TESTED',
 source:{repository:'owenservera/ARBOR',sha:SRC},currentMainSha:newer,
 test:{state:'PASS',sourceSha:SRC,passed:215,failed:0,skipped:0,
   runId:'vlang-2026-11-23T19-24-07-866Z',
   command:'powershell source-pinned-native-cycle.ps1'}
});
test('actual recorded earlier SHA admits source tests but not current-head or devices',()=>{
 const got=admitReceipt(base());
 assert.equal(got.state,'EVIDENCE_ADMITTED');
 assert.equal(got.highest,'SOURCE_TESTED');
 assert.equal(got.currentHeadMatches,false);
 assert.equal(got.gates.PACKAGED,false);
 assert.equal(got.gates.GPUI_RENDERED,false);
 assert.equal(got.gates.PHONE_RENDERED,false);
});
test('historical passed test never justifies claiming packaged or phone-rendered',()=>{
 for(const claim of ['PACKAGED','LIVE_SERVED','GPUI_RENDERED','PHONE_RENDERED','CROSS_DEVICE_VERIFIED']){
  const got=admitReceipt({...base(),claim});
  assert.equal(got.accepted,false,claim);
  assert.equal(got.highest,'SOURCE_TESTED',claim);
 }
});
test('missing run ID, mismatched SHA, failures and skips cannot pass test gate',()=>{
 for(const changes of [
  {runId:null},{sourceSha:newer},{failed:1},{passed:0},{skipped:null},{state:'UNRUN'}
 ]){
  const v=base();v.test={...v.test,...changes};
  assert.equal(admitReceipt(v).gates.SOURCE_TESTED,false,JSON.stringify(changes));
 }
});
test('an unverified release manifest cannot be upgraded to a deployed receipt',()=>{
 const v=base();v.package={state:'PASS',sourceSha:SRC,lockVerified:true,
   manifestSha256:hash,nativeSha256:hash};
 assert.equal(admitReceipt({...v,claim:'PACKAGED'}).highest,'PACKAGED');
 assert.equal(admitReceipt({...v,claim:'LIVE_SERVED'}).accepted,false);
 assert.equal(admitReceipt({...v,package:{...v.package,sourceSha:newer},claim:'PACKAGED'}).accepted,false);
});
test('declared runtime status, simulated phone and unbound Host ACK are never physical proof',()=>{
 const v=base();
 v.package={state:'PASS',sourceSha:SRC,lockVerified:true,manifestSha256:hash,nativeSha256:hash};
 v.serve={state:'verified',sourceSha:SRC,manifestSha256:hash,
   runtimeId:'host-local-001',observedAt:'2026-11-23T19:24:00Z'};
 assert.equal(admitReceipt({...v,claim:'LIVE_SERVED'}).accepted,true);
 v.hostAck={confirmed:true,authority:'host',sourceSha:SRC,receiptId:'ctl-1234',
   correlationId:'corr-1234',epoch:'host.001',regionId:'phone.composer',
   sourceRevision:1,viewRevision:1};
 v.gpui={device:'windows-gpui',sourceSha:SRC,observedOnDevice:true,
   screenshotSha256:hash,hostReceiptId:'ctl-1234',correlationId:'corr-1234',
   epoch:'host.001',sourceRevision:1};
 v.phone={...v.gpui,device:'android-pwa',simulated:true};
 assert.equal(admitReceipt({...v,claim:'PHONE_RENDERED'}).accepted,false);
 assert.equal(admitReceipt({...v,claim:'GPUI_RENDERED'}).accepted,true);
 assert.equal(admitReceipt({...v,hostAck:{...v.hostAck,sourceSha:newer},claim:'HOST_ACK_VERIFIED'}).accepted,false);
});
test('malformed or arbitrary claims fail closed, not auto-completed',()=>{
 assert.equal(admitReceipt(null).accepted,false);
 assert.equal(admitReceipt({...base(),claim:'DONE'}).accepted,false);
 assert.equal(admitReceipt({...base(),source:{repository:'other/repo',sha:SRC}}).accepted,false);
});
