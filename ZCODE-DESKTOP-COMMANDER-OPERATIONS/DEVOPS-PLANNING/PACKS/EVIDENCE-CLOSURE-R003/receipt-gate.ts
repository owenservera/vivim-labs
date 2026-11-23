// VF-001/VF-002: pure, offline, fail-closed evidence classification.
// This is a reviewer of receipts, NOT a Host authority, scheduler, device
// probe, deployment script, source writer, or evidence generator.
// Node 24: node --experimental-strip-types receipt-gate.ts <receipt.json>
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

export const GATES = Object.freeze([
  'SOURCE_PINNED','SOURCE_TESTED','PACKAGED','LIVE_SERVED',
  'HOST_ACK_VERIFIED','GPUI_RENDERED','PHONE_RENDERED','CROSS_DEVICE_VERIFIED'
] as const);
type Gate = typeof GATES[number];
type AnyMap = Record<string, any>;
const map=(x:unknown):AnyMap => x!==null&&typeof x==='object'&&!Array.isArray(x)?x as AnyMap:{};
const sha=(x:unknown):x is string=>typeof x==='string'&&/^[a-f0-9]{40}$/.test(x);
const digest=(x:unknown):x is string=>typeof x==='string'&&/^[a-fA-F0-9]{64}$/.test(x);
const label=(x:unknown):x is string=>typeof x==='string'&&/^[a-zA-Z0-9._:-]{4,160}$/.test(x);
const natural=(x:unknown)=>Number.isSafeInteger(x)&&Number(x)>=0;
const positive=(x:unknown)=>natural(x)&&Number(x)>0;
const evidence=(x:unknown)=>map(x).observedOnDevice===true&&map(x).simulated!==true;

export function admitReceipt(input:unknown) {
  const row=map(input),source=map(row.source), tests=map(row.test),
    pkg=map(row.package),serve=map(row.serve),host=map(row.hostAck),
    gpui=map(row.gpui),phone=map(row.phone),cross=map(row.crossDevice);
  const src=source.sha;
  const gates:Record<Gate,boolean> = {
    SOURCE_PINNED:row.schema==='vivim.acceptance-evidence/1'&&
      row.taskId!==undefined&&label(row.taskId)&&
      source.repository==='owenservera/ARBOR'&&sha(src),
    SOURCE_TESTED:false, PACKAGED:false, LIVE_SERVED:false,
    HOST_ACK_VERIFIED:false, GPUI_RENDERED:false, PHONE_RENDERED:false,
    CROSS_DEVICE_VERIFIED:false
  };
  gates.SOURCE_TESTED=gates.SOURCE_PINNED&&tests.state==='PASS'&&
    tests.sourceSha===src&&positive(tests.passed)&&tests.failed===0&&
    natural(tests.skipped)&&label(tests.runId)&&
    typeof tests.command==='string'&&tests.command.length>5&&tests.command.length<=400;
  gates.PACKAGED=gates.SOURCE_TESTED&&pkg.state==='PASS'&&
    pkg.sourceSha===src&&pkg.lockVerified===true&&
    digest(pkg.nativeSha256)&&digest(pkg.manifestSha256);
  gates.LIVE_SERVED=gates.PACKAGED&&serve.state==='verified'&&
    serve.sourceSha===src&&serve.manifestSha256===pkg.manifestSha256&&
    label(serve.runtimeId)&&typeof serve.observedAt==='string'&&
    Number.isFinite(Date.parse(serve.observedAt));
  gates.HOST_ACK_VERIFIED=gates.LIVE_SERVED&&host.confirmed===true&&
    host.authority==='host'&&host.sourceSha===src&&
    label(host.receiptId)&&label(host.correlationId)&&label(host.epoch)&&
    label(host.regionId)&&natural(host.sourceRevision)&&natural(host.viewRevision);
  const visible=(v:AnyMap,device:string)=>
    gates.HOST_ACK_VERIFIED&&evidence(v)&&v.device===device&&v.sourceSha===src&&
    v.hostReceiptId===host.receiptId&&v.correlationId===host.correlationId&&
    v.epoch===host.epoch&&v.sourceRevision===host.sourceRevision&&
    digest(v.screenshotSha256);
  gates.GPUI_RENDERED=visible(gpui,'windows-gpui');
  gates.PHONE_RENDERED=gates.GPUI_RENDERED&&visible(phone,'android-pwa');
  const legs=Array.isArray(cross.legs)?cross.legs:[];
  const validLeg=(leg:unknown,from:string,to:string)=>{
    const v=map(leg);
    return v.from===from&&v.to===to&&v.sourceSha===src&&
      v.acknowledged===true&&v.observedOnDevice===true&&v.simulated!==true&&
      label(v.epoch)&&label(v.hostReceiptId)&&label(v.ackId)&&
      natural(v.sourceRevision)&&digest(v.originScreenshotSha256)&&
      digest(v.destinationScreenshotSha256);
  };
  gates.CROSS_DEVICE_VERIFIED=gates.PHONE_RENDERED&&legs.length===2&&
    validLeg(legs[0],'windows-gpui','android-pwa')&&
    validLeg(legs[1],'android-pwa','windows-gpui')&&
    legs[0].ackId!==legs[1].ackId;
  const highest=[...GATES].reverse().find(k=>gates[k])??'NONE';
  const requested=typeof row.claim==='string'?row.claim:'NONE';
  const accepted=GATES.includes(requested as Gate)&&gates[requested as Gate]===true;
  const currentHeadMatches=sha(row.currentMainSha)?row.currentMainSha===src:null;
  return {
    schema:'vivim.acceptance-admission/1',
    taskId:typeof row.taskId==='string'?row.taskId:null,
    sourceSha:sha(src)?src:null,
    requested, highest, accepted, currentHeadMatches,
    gates, missing:GATES.filter(k=>!gates[k]),
    state:accepted?'EVIDENCE_ADMITTED':'CLAIM_NOT_PROVEN',
    caveat:'Validates structured receipt consistency only. Does not inspect screenshot pixels, authenticate a provider reply, or independently observe devices.'
  };
}
const invoked=process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url);
if(invoked){
  try{
    if(process.argv.length!==3)throw new Error('USAGE: receipt-gate.ts <receipt.json>');
    const raw=readFileSync(process.argv[2],'utf8');
    if(Buffer.byteLength(raw,'utf8')>128*1024)throw new Error('RECEIPT_TOO_LARGE');
    const result=admitReceipt(JSON.parse(raw));
    process.stdout.write(JSON.stringify(result,null,2)+'\n');
    if(!result.accepted)process.exitCode=3;
  }catch(error){
    process.stderr.write('INVALID_RECEIPT: '+String((error as Error).message).slice(0,160)+'\n');
    process.exitCode=2;
  }
}
