import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {createHash} from 'node:crypto';
import {createNativeResourcePort,validateResourcePayload,NATIVE_RESOURCE_ACTION} from '../control/native-resource-port.mjs';
const digest=x=>createHash('sha256').update(x).digest('hex');
const valid={schema:'vivim.windows.resource-probe/1',status:'ok',memoryTotalBytes:100,memoryAvailableBytes:75,cpuBusyBasisPoints:2500,cpuScope:'calling-processor-group',sampleMs:120};
test('closed result schema and bounded values',()=>{
 assert.equal(validateResourcePayload(valid).status,'ok');
 assert.equal(validateResourcePayload({...valid,cpuBusyBasisPoints:null}).status,'partial');
 assert.equal(validateResourcePayload({schema:valid.schema,status:'unsupported'}).status,'unsupported');
 for(const v of [{...valid,memoryAvailableBytes:101},{...valid,cpuBusyBasisPoints:10001},{...valid,unexpected:'private'},{...valid,sampleMs:5001}])assert.throws(()=>validateResourcePayload(v));
});
test('explicit opt-in, exact native invocation, revoked/tampered config refusal',t=>{
 const d=mkdtempSync(join(tmpdir(),'vivim-resource-'));t.after(()=>rmSync(d,{recursive:true,force:true}));
 const roots=join(d,'roots.json'),helper=join(d,'vivim-workstation-probe.exe'),now=100000;
 const b='fixture-only; never executed';writeFileSync(helper,b);
 const env={VIVIM_WORKSTATION_ROOTS:roots,VIVIM_WORKSTATION_PROBE_EXE:helper,VIVIM_WORKSTATION_PROBE_SHA256:digest(b),PRIVATE_SECRET:'never-forward'};
 const save=v=>{const s=JSON.stringify(v);writeFileSync(roots,s);env.VIVIM_WORKSTATION_ROOTS_SHA256=digest(s)};
 let calls=0;
 const port=createNativeResourcePort({env,now:()=>now,run:(p,args,o)=>{calls++;assert.equal(p,helper);assert.deepEqual(args,['--native-resources']);assert.equal(o.shell,false);assert.equal(o.env.PRIVATE_SECRET,undefined);return {status:0,stdout:JSON.stringify(valid),stderr:''}}});
 save({schema:'vivim.workstation-roots/1',roots:[]});assert.throws(()=>port.read({}));assert.equal(calls,0);
 save({schema:'vivim.workstation-roots/1',roots:[],nativeResources:{expiresAt:now+10000}});
 assert.equal(port.read({}).actionId,NATIVE_RESOURCE_ACTION);assert.equal(calls,1);
 assert.throws(()=>port.read({extra:true}));writeFileSync(roots,'changed');assert.throws(()=>port.read({}));assert.equal(calls,1);
 save({schema:'vivim.workstation-roots/1',roots:[],nativeResources:{expiresAt:now}});assert.throws(()=>port.read({}));assert.equal(calls,1);
});
