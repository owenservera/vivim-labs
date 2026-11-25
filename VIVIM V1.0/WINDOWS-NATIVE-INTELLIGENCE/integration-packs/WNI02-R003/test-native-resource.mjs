// Run actual compiled scratch fixture through the R003 Host adapter.
// Requires explicit new fixture directory and locally built native probe.
// Never uses live Host root configuration, owner tokens, or production binary.
import {readFileSync,writeFileSync,realpathSync,existsSync,statSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join,basename,relative,isAbsolute,sep} from 'node:path';
import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
const [overlayPath,fixturePath,exePath,...extra]=process.argv.slice(2);
if(!overlayPath||!fixturePath||!exePath||extra.length)throw Error('usage: node test-native-resource.mjs <overlay> <scratch-fixture-dir> <scratch-probe-exe>');
const overlay=realpathSync(overlayPath),fixture=realpathSync(fixturePath),exe=realpathSync(exePath);
if(!statSync(fixture).isDirectory()||!/^vivim-workstation-probe(?:\.exe)?$/i.test(basename(exe)))throw Error('Invalid fixture');
const cfg=join(fixture,'roots-fixture.json');if(existsSync(cfg))throw Error('fixture config exists');
const {createNativeResourcePort}=await import(pathToFileURL(join(overlay,'apps/vivim-host/control/native-resource-port.mjs')).href);
const sha=p=>createHash('sha256').update(readFileSync(p)).digest('hex');
writeFileSync(cfg,JSON.stringify({schema:'vivim.workstation-roots/1',roots:[],nativeResources:{expiresAt:Date.now()+60000}}),{flag:'wx'});
const port=createNativeResourcePort({env:{SystemRoot:process.env.SystemRoot,VIVIM_WORKSTATION_ROOTS:cfg,VIVIM_WORKSTATION_ROOTS_SHA256:sha(cfg),VIVIM_WORKSTATION_PROBE_EXE:exe,VIVIM_WORKSTATION_PROBE_SHA256:sha(exe)}});
const r=port.read({});
assert.equal(r.actionId,'windows.system.resources.read');
assert.ok(['ok','partial'].includes(r.status));
assert.ok(r.data.memoryTotalBytes>0&&r.data.memoryAvailableBytes<=r.data.memoryTotalBytes);
console.log(JSON.stringify({actualNativeAdapter:true,status:r.status,aggregateSchemaValid:true,cpuScope:r.data.cpuScope,sampleMs:r.data.sampleMs,rawMemoryValuesPublished:false}));
