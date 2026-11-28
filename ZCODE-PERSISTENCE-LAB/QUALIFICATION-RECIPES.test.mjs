import assert from 'node:assert/strict';
import fs from 'node:fs';
const read=n=>JSON.parse(fs.readFileSync(new URL('./'+n,import.meta.url),'utf8'));
const q=read('QUALIFICATION-RECIPES-R015.json'),r=read('EXPERIMENT-READINESS-R014.json'),c=read('CAMPAIGN-R010.json');
assert.equal(q.status,'DESIGNED_NOT_LIVE_QUALIFIED');
assert.deepEqual(q.selected,Object.keys(c.lanes).filter(k=>c.lanes[k].selected));
assert.deepEqual(q.reserved,Object.keys(c.lanes).filter(k=>!c.lanes[k].selected));
assert.deepEqual(q.recipes.map(x=>x.gate).sort(),r.readiness_gates.filter(x=>x.status==='OPEN').map(x=>x.id).sort());
const sources=new Set(q.sources.map(s=>s.id)),recipes=new Map(q.recipes.map(x=>[x.id,x]));
assert.equal(recipes.size,6);
for(const x of q.recipes){
 assert.ok(x.name&&x.pass&&x.fail&&x.receipt.length&&x.sources.length);
 assert.ok(x.sources.every(s=>sources.has(s)));
 assert.ok(x.dependencies.every(d=>recipes.has(d)||r.readiness_gates.some(g=>g.id===d&&g.status==='PASS')));
 assert.ok(x.status!=='PASS');
}
const visited=new Set(),active=new Set();
function visit(k){if(visited.has(k)||!recipes.has(k))return;assert.ok(!active.has(k),'recipe dependency cycle');active.add(k);for(const d of recipes.get(k).dependencies)visit(d);active.delete(k);visited.add(k);}
for(const k of recipes.keys())visit(k);
for(const [n,provider] of [['CODEX-CLI-RECIPE.json','codex'],['ANTIGRAVITY-CLI-RECIPE.json','agy']]){
 const a=read(n);assert.equal(a.qualification.launch_enabled,false);assert.equal(a.qualification.manifest,'QUALIFICATION-RECIPES-R015.json');assert.deepEqual(a.qualification.required,q.recipes.map(x=>x.id));assert.equal(a.completion_charter,'SYSTEM-GOALS-R013.json');assert.deepEqual(a.assignments,Object.keys(c.lanes).filter(k=>c.lanes[k].provider===provider).map(k=>'EXPERIMENT-'+k));
}
console.log('R015: all six open readiness gates covered; source bindings, dependency graph, lane assignments and disabled launch verified.');
