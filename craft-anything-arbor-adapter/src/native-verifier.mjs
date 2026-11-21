// Invokes the operator-supplied Rust helper from the Craft Anything evidence crate.
// No native verification is claimed unless that executable actually runs and
// confirms this exact bundle hash and every record identity.
import {spawnSync} from 'node:child_process';
import {resolve} from 'node:path';

export const NATIVE_RECEIPT_SCHEMA = 'vivim.craft-native-verification/1';
const ID_RE=/^ev_[0-9a-f]{64}$/;
const SHA=/^[0-9a-f]{64}$/;
function fail(code) { const e=new Error(code);e.code=code;throw e; }
export function checkNativeReceipt(receipt, {bundleSha256, records}) {
  if(!receipt || receipt.schema!==NATIVE_RECEIPT_SCHEMA || receipt.nativeVerifier!=='craft-anything-evidence::Evidence::verify' ||
      receipt.bundleSha256!==bundleSha256 || receipt.executionAuthorized!==false ||
      !SHA.test(receipt.bundleSha256??'') || !Array.isArray(receipt.verifiedRecordIds) ||
      !Number.isSafeInteger(receipt.totalRecords) || receipt.totalRecords!==records.length ||
      receipt.verifiedRecordIds.length>2500 || receipt.uniqueRecords!==receipt.verifiedRecordIds.length ||
      !receipt.verifiedRecordIds.every(x=>typeof x==='string'&&ID_RE.test(x))) fail('NATIVE_RECEIPT_INVALID');
  const nativeIds=[...new Set(receipt.verifiedRecordIds)].sort();
  const expected=[...new Set(records.map(r=>r?.id))].sort();
  if(nativeIds.length!==receipt.verifiedRecordIds.length || JSON.stringify(nativeIds)!==JSON.stringify(expected))fail('NATIVE_RECEIPT_RECORD_MISMATCH');
  return new Set(nativeIds);
}
export function runNativeVerification({binary,bundlePath,bundleSha256,records}) {
  if(typeof binary!=='string'||!binary.trim()||typeof bundlePath!=='string'||!bundlePath.trim())fail('NATIVE_VERIFIER_PATH_REQUIRED');
  const r=spawnSync(resolve(binary),[resolve(bundlePath)],{encoding:'utf8',windowsHide:true,timeout:120_000,maxBuffer:3*1024*1024,env:process.env});
  if(r.error||r.status!==0||r.signal)fail('NATIVE_VERIFIER_FAILED');
  let receipt;
  try {receipt=JSON.parse(r.stdout);}catch{fail('NATIVE_VERIFIER_OUTPUT_INVALID');}
  const ids=checkNativeReceipt(receipt,{bundleSha256,records});
  return {ids,receipt};
}
