import { test } from 'node:test';
import assert from 'node:assert/strict';
import {freeTextModel,chooseRoles,renderTemplate,validateProviderId,PRIORITIES} from '../scripts/model-router.mjs';
const free=id=>({id,pricing:{prompt:'0',completion:'0'},architecture:{input_modalities:['text'],output_modalities:['text']},supported_parameters:['tools']});
const data=Object.values(PRIORITIES).flat().filter((x,i,a)=>a.indexOf(x)===i).map(free);
test('selects distinct deterministic free models by role priority',()=>{
 const r=chooseRoles(data);assert.deepEqual(r,{planner:'nvidia/nemotron-3-ultra:free',coder:'particle-ai/particle-1:free',reviewer:'baidu/ernie-5.5-flash-preview:free',triage:'nvidia/nemotron-3.5-lightning:free'});
});
test('rejects paid alternate with same base ID',()=>assert.equal(freeTextModel({...free('baidu/ernie-5.5-flash-preview'),pricing:{prompt:'0',completion:'0'}}),false));
test('rejects nonzero completion despite zero prompt',()=>assert.equal(freeTextModel({...free('x/y:free'),pricing:{prompt:'0',completion:'0.1'}}),false));
test('rejects missing price and unknown tool support explicitly',()=>{
 assert.equal(freeTextModel({id:'x/y:free',pricing:{prompt:'0'}}),false);
 assert.equal(freeTextModel({...free('x/y:free'),supported_parameters:['temperature']}),false);
});
test('rejects multimodal-only non-text output',()=>assert.equal(freeTextModel({...free('x/y:free'),architecture:{input_modalities:['text'],output_modalities:['image']}}),false));
test('fails closed when no eligible model for a role',()=>assert.throws(()=>chooseRoles([free('baidu/ernie-5.5-flash-preview:free')]),{code:'NO_ELIGIBLE_FREE_MODEL'}));
test('rejects unsupported catalog type',()=>assert.throws(()=>chooseRoles(null),TypeError));
test('refuses provider identifier injection',()=>assert.throws(()=>validateProviderId('foo"; process.exit()'),/Invalid/));
test('strict render binds four compile-time names',()=>{
 const tpl='model("__MODEL_PLANNER__");model("__MODEL_CODER__");model("__MODEL_REVIEWER__");model("__MODEL_TRIAGE__");';
 const out=renderTemplate(tpl,chooseRoles(data),'openrouter');assert.match(out,/openrouter\/particle-ai\/particle-1:free/);
 assert.doesNotMatch(out,/__MODEL_/);
});
test('renderer refuses untrusted runtime arbitrary model',()=>assert.throws(()=>renderTemplate('model("__MODEL_PLANNER__")',{planner:'x/y" malicious:free'},'openrouter'),/Untrusted/));
test('renderer refuses missing required placeholders',()=>assert.throws(()=>renderTemplate('x',chooseRoles(data),'openrouter'),/Missing template/));