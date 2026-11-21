import {test} from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {buildAtlas,evidenceDigest,validateBundle} from '../src/adapter.mjs';
import {checkNativeReceipt} from '../src/native-verifier.mjs';
const origin='https://test.example';
const sha='0'.repeat(64);
const make=(result={url:origin+'/'})=>{
 const r={subject:{kind:'website'},operation:'web_site_capture',params:{},result,
 confidence:'observed',authority:'our-run',environment:{clean_room:true,engine:'cdp',host:'test'},
 coverage:{status:'unknown'},limitations:[],locations:[{kind:'url',value:origin+'/'}],links:[]};
 r.id=evidenceDigest(r);return r;
};
const bundle=records=>({format:'craft-anything-evidence-bundle',version:1,records,unknowns:[]});
test('native-receipt protocol binds whole bundle and all IDs; never grants execution',()=>{
 const rec=make();const receipt={schema:'vivim.craft-native-verification/1',bundleSha256:sha,
  verifiedRecordIds:[rec.id],totalRecords:1,uniqueRecords:1,unknowns:0,
  nativeVerifier:'craft-anything-evidence::Evidence::verify',executionAuthorized:false};
 assert.deepEqual([...checkNativeReceipt(receipt,{bundleSha256:sha,records:[rec]})],[rec.id]);
 assert.throws(()=>checkNativeReceipt({...receipt,bundleSha256:'a'.repeat(64)},{bundleSha256:sha,records:[rec]}),/NATIVE_RECEIPT_INVALID/);
 assert.throws(()=>checkNativeReceipt({...receipt,verifiedRecordIds:[]},{bundleSha256:sha,records:[rec]}),/NATIVE_RECEIPT_INVALID|NATIVE_RECEIPT_RECORD_MISMATCH/);
 assert.throws(()=>checkNativeReceipt({...receipt,executionAuthorized:true},{bundleSha256:sha,records:[rec]}),/NATIVE_RECEIPT_INVALID/);
});
test('Craft unknown IDs are constrained and private questions never leave the atlas',()=>{
 const r=make(),b=bundle([r]);
 b.unknowns=[{id:'unk_0000000000000000',revision:1,question:'secret personal question',domain:'ui',status:'open',severity:'medium'}];
 const a=buildAtlas({bundle:b,siteId:'test',origin});
 assert.deepEqual(a.unknownIds,['unk_0000000000000000']);
 assert.equal(JSON.stringify(a).includes('secret personal question'),false);
 b.unknowns[0].id='private-secret-text';
 assert.throws(()=>validateBundle(b),/INVALID_CRAFT_UNKNOWN/);
});
test('hash parity remains explicitly unverified absent real Rust execution',()=>{
 const r=make();const a=buildAtlas({bundle:bundle([r]),siteId:'test',origin});
 assert.equal(a.source.nativeVerified,false);
 assert.equal(a.authority.sendPermission,false);
});
