import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawn} from 'node:child_process';
import {pathToFileURL} from 'node:url';
const url=pathToFileURL(process.env.P05_JOURNAL).href;
const {createControlJournal}=await import(url);
function setup(t){const dataRoot=mkdtempSync(join(tmpdir(),'p05-journal-'));t.after(()=>rmSync(dataRoot,{recursive:true,force:true}));const j=createControlJournal({dataRoot});const planId='ctl-0000000000000001';const planDigest='a'.repeat(64);j.record({planId,planDigest,actionId:'test-only',risk:'R1',source:'desktop',origin:'typed',createdAt:new Date().toISOString()});return {j,dataRoot,planId,planDigest};}
const approval=planDigest=>({planDigest,channel:'desktop-owner-token',actor:'desktop-owner'});
test('first approval cannot bind another digest; distinct actor cannot replace it',t=>{const {j,planId,planDigest}=setup(t);assert.throws(()=>j.approve(planId,approval('b'.repeat(64))),/CONTROL_APPROVAL_CONFLICT/);assert.equal(j.get(planId).state,'planned');j.approve(planId,approval(planDigest));assert.throws(()=>j.approve(planId,{...approval(planDigest),actor:'foreign'}),/CONTROL_APPROVAL_CONFLICT/);});
test('two OS processes contend for one original journal claim; reopened uncertain record is not re-claimable',async t=>{const {j,dataRoot,planId,planDigest}=setup(t);j.approve(planId,approval(planDigest));const child=()=>new Promise((resolve,reject)=>{const p=spawn(process.execPath,['--input-type=module','-e','const {createControlJournal}=await import(process.argv[1]);console.log(createControlJournal({dataRoot:process.argv[2]}).claim(process.argv[3]));',url,dataRoot,planId],{stdio:['ignore','pipe','pipe']});let out='',err='';p.stdout.on('data',b=>out+=b);p.stderr.on('data',b=>err+=b);p.on('error',reject);p.on('close',code=>code===0?resolve(out.trim()):reject(Error(err)));});assert.deepEqual((await Promise.all([child(),child()])).sort(),['false','true']);const reopened=createControlJournal({dataRoot});assert.equal(reopened.get(planId).state,'uncertain');assert.equal(reopened.claim(planId),false);});
test('original journal receipt is immutable after claim; reopening reads the first receipt',t=>{const {j,dataRoot,planId,planDigest}=setup(t);j.approve(planId,approval(planDigest));assert.equal(j.claim(planId),true);const receipt={status:'applied',actionId:'test-only',executionPerformed:true};j.finish(planId,receipt);assert.throws(()=>j.finish(planId,{...receipt,status:'failed'}),/CONTROL_RECEIPT_EXISTS/);const reopened=createControlJournal({dataRoot});assert.deepEqual(reopened.get(planId).receipt,receipt);assert.equal(reopened.claim(planId),false);});
