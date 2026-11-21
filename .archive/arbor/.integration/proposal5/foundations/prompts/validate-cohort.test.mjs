import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { validatePromptCohort } from './validate-cohort.mjs';

const root=dirname(fileURLToPath(import.meta.url));
const load=()=>({
 cohort:JSON.parse(readFileSync(join(root,'LANE-REGISTRY.v1.json'),'utf8')),
 waves:JSON.parse(readFileSync(join(root,'../WAVE-MANIFEST.v1.json'),'utf8'))
});
const mutate=fn=>{const input=load();fn(input);return validatePromptCohort(input)};
const has=(x,code)=>x.errors.some(e=>e.code===code);
test('5 prompts cover 32 PG tasks and have no execution authority',()=>{
 const x=validatePromptCohort(load());
 assert.equal(x.status,'STRUCTURALLY_READY_PROMPTS_NOT_EXECUTED');
 assert.equal(x.effectPerformed,false);
 assert.deepEqual(x.counts,{lanes:5,parallel:4,referees:1,coveredPreGTasks:32});
});
test('invalid input fails closed',()=>assert.ok(has(validatePromptCohort(),'BAD_INPUT')));
test('APK-first mode is refused',()=>assert.ok(has(mutate(x=>x.cohort.mobilePolicy='APK_FIRST'),'MOBILE_SCOPE')));
test('fake G completion is refused',()=>assert.ok(has(mutate(x=>x.cohort.GStatusAtAuthoring='GO'),'G_TRUTH_OVERCLAIM')));
test('fake executed-worker status is refused',()=>assert.ok(has(mutate(x=>x.cohort.status='WORKERS_PASSED'),'UNSAFE_PROGRESS_CLAIM')));
test('referee cannot merge',()=>assert.ok(has(mutate(x=>x.cohort.lanes[4].canMerge=true),'UNAUTHORIZED_WORKER_POWER')));
test('worker cannot write product code',()=>assert.ok(has(mutate(x=>x.cohort.lanes[0].writesToProduct=true),'UNAUTHORIZED_WORKER_POWER')));
test('worker cannot block original G',()=>assert.ok(has(mutate(x=>x.cohort.lanes[1].canBlockG=true),'UNAUTHORIZED_WORKER_POWER')));
test('collision in worker branch is rejected',()=>assert.ok(has(mutate(x=>x.cohort.lanes[1].workerBranch=x.cohort.lanes[0].workerBranch),'BRANCH_COLLISION')));
test('collision in worker path is rejected',()=>assert.ok(has(mutate(x=>x.cohort.lanes[1].exclusiveWritePrefix=x.cohort.lanes[0].exclusiveWritePrefix),'WRITE_COLLISION')));
test('any write path outside proof prefix is rejected',()=>assert.ok(has(mutate(x=>x.cohort.lanes[0].exclusiveWritePrefix='apps/vivim-host/'),'WRITE_PREFIX_MISMATCH')));
test('missing source SHA refused',()=>assert.ok(has(mutate(x=>x.cohort.frozen.A.sha='placeholder'),'INVALID_SOURCE_SNAPSHOT')));
test('unknown donor source key refused',()=>assert.ok(has(mutate(x=>x.cohort.lanes[3].inputs.push('UNREVIEWED')),'UNKNOWN_INPUT_REF')));
test('missing atomic task refused',()=>assert.ok(has(mutate(x=>x.cohort.lanes[0].taskIds.pop()),'PRE_G_ORDER')));
test('referee cannot launch as parallel worker',()=>assert.ok(has(mutate(x=>x.cohort.lanes[4].runOrder=1),'REFEREE_ORDER')));
test('lost original G base is rejected',()=>assert.ok(has(mutate(x=>x.cohort.programBaseRef='main'),'G_BASE_COLLISION')));
