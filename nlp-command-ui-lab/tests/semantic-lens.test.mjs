import test from 'node:test';
import assert from 'node:assert/strict';
import {
 scanLexical,detectNegation,projectHints,commandQuery,commandMenu,resolveTypedReference,
 semanticSummary,meaningDelta,replaceSuggestion
} from '../src/semantic-lens.mjs';
import {visualCompile} from '../src/compiler.mjs';
import {affordanceFor,decorateActions,sigilFor} from '../src/affordances.mjs';

const A=[
 {namespace:'host',id:'vivim.status.show',title:'Show status',category:'system',readiness:'source_wired',risk:'R0',examples:['show status'],args:{}},
 {namespace:'hq',id:'knowledge.search',title:'Search knowledge',category:'wiki',readiness:'UNTESTED',risk:'unknown',examples:[],args:{}},
 {namespace:'tierflow-ui',id:'ui.draft.edit',title:'Edit UI draft',category:'ui',readiness:'source_declared',risk:'unknown',examples:[],args:{}}
];
const inventory={actions:decorateActions(A),revision:'test-inventory-v1'};

test('quoted instructions stay literals and never become negated commands',()=>{
 const tokens=scanLexical('"do not send this"');assert.equal(tokens.length,1);
 assert.equal(tokens[0].role,'literal');assert.equal(detectNegation('"do not send this"').negated,false);
 const p=visualCompile('"do not send this"',inventory);
 assert.equal(p.outcome,'source_unavailable');assert.equal(p.executionPerformed,false);
});
test('negation preserves full scope in contractions and do not pair',()=>{
 for(const text of ["Don't email Peter",'do not email Peter','Never send this','please don’t publish','email Peter never send']) {
   const negative=detectNegation(text);assert.equal(negative.negated,true,`should negate: ${text}`);
   const p=visualCompile(text,inventory);assert.equal(p.outcome,'blocked_negated');assert.equal(p.menu,null);
   assert.ok(p.gaps.some(g=>g.code==='NEGATION_SCOPE'));assert.equal(p.authorization,'not_assessed');
 }
});
test('a Host refusal cannot be overridden by the email visual prototype',()=>{
 const refused={outcome:'refused',refusal:{code:'DENIED',message:'Not permitted to send'},language:{feedback:{reading:'Send it immediately'}}};
 const p=visualCompile('email Peter',inventory,refused);
 assert.equal(p.outcome,'refused');assert.equal(p.reading,'Not permitted to send');assert.equal(p.menu,null);
 assert.equal(p.actionId,null);assert.equal(p.executionPerformed,false);
});
test('Host clarification suppresses suggestions until clarified',()=>{
 const p=visualCompile('email Peter',inventory,{outcome:'clarification_required',question:'Which account?'});
 assert.equal(p.outcome,'clarification_required');assert.equal(p.menu,null);assert.equal(p.reading,'Which account?');
});
test('explicit slash command suggestions use only the provided source inventory',()=>{
 const menu=commandMenu('/sta',inventory);
 assert.equal(menu.kind,'commands');assert.deepEqual(menu.items.map(x=>x.identity.id),['vivim.status.show']);
 assert.equal(menu.items[0].verified,false);assert.equal(menu.items[0].insert,'/vivim.status.show');
 assert.equal(commandMenu('please /sta',inventory),null);
 assert.equal(commandQuery('"/sta"'),null);
 assert.equal(commandMenu('/not-found',inventory).items.length,0);
});
test('exact slash references are read-only source discovery, not permissions',()=>{
 const p=visualCompile('/vivim.status.show',inventory);
 assert.equal(p.outcome,'discovery_only');assert.equal(p.namespace,'host');assert.equal(p.actionId,'vivim.status.show');
 assert.equal(p.executionPerformed,false);assert.equal(p.authorization,'not_assessed');
 assert.equal(p.intentStatus,'unresolved');assert.equal(resolveTypedReference('/vivim.status.show',inventory).id,'vivim.status.show');
});
test('cross-domain suggestions remain distinct namespaces',()=>{
 const menu=commandMenu('/ui',inventory);assert.deepEqual(menu.items.map(x=>x.identity.namespace),['tierflow-ui']);
 const p=visualCompile('/ui.draft.edit',inventory);
 assert.equal(p.namespace,'tierflow-ui');assert.equal(p.executionPerformed,false);
});
test('visual suggestion insertion is reversible and rejects mismatched source spans',()=>{
 const input='email Pet';const p=visualCompile(input,inventory);
 const selected=p.menu.items[0];const op=replaceSuggestion(input,p.menu,selected);
 assert.equal(op.text,'email Peter Miller');assert.equal(op.undo.text,input);
 assert.equal(op.text.length,op.caret);assert.equal(replaceSuggestion('email Pex',p.menu,selected),null);
 const slash=commandMenu('/sta',inventory);
 assert.equal(replaceSuggestion('/sta',slash,slash.items[0]).text,'/vivim.status.show');
 assert.equal(replaceSuggestion('bad/sta',slash,slash.items[0]),null);
});
test('semantic summary separates understanding from permission',()=>{
 const p=visualCompile('show status',inventory,{outcome:'ready',actionId:'vivim.status.show',language:{feedback:{reading:'Show status'}}});
 const summary=semanticSummary(p.text,p);assert.equal(summary.understood,'candidate');assert.equal(summary.executable,'not_assessed');
 assert.equal(summary.caveat.includes('not assessed'),true);
});
test('project hints do not impersonate grounded project IDs',()=>{
 const refs=projectHints('Update TierFlow and repos-hq');
 assert.deepEqual(refs.map(x=>x.name),['tierflow','repos-hq']);assert.ok(refs.every(x=>x.status==='lexical_hint'));
});
test('meaning delta shows meaningful changes without inventing a score',()=>{
 const a=visualCompile('show status',inventory);
 const b=visualCompile("don't show status",inventory);
 assert.deepEqual(meaningDelta(a,b),{kind:'safety',label:'Negation detected'});
 const c=visualCompile('/vivim.status.show',inventory);
 assert.equal(meaningDelta(visualCompile('unrelated',inventory),c).kind,'action');
 assert.equal(meaningDelta(c,c).kind,'stable');
});
test('per-command visual sigils are stable, unique within the inventory and non-authoritative',()=>{
 const decorated=decorateActions(A);
 assert.deepEqual(decorated.map(x=>x.visual.sigil),decorateActions(A).map(x=>x.visual.sigil));
 assert.equal(new Set(decorated.map(x=>x.visual.sigil)).size,3);
 assert.equal(affordanceFor(A[0]).semantics,'presentation_only');
 assert.notEqual(sigilFor(A[0]),sigilFor(A[1]));
});
test('Unicode and punctuation token spans maintain source integrity',()=>{
 const source='Email 😊 Peter → “mañana” #tag Δ /ui.draft.edit';
 const tokens=scanLexical(source);
 for(const t of tokens){assert.equal(source.slice(t.start,t.end),t.text);assert.ok(t.end<=source.length);}
 for(let i=1;i<tokens.length;i++)assert.ok(tokens[i-1].end<=tokens[i].start);
});
test('very long inputs are bounded and contain no execution effect',()=>{
 const p=visualCompile('email '+('a'.repeat(1400)),inventory);
 assert.equal(p.text.length,1000);assert.equal(p.executionPerformed,false);
 assert.equal(p.lens.schema,'vivim.labs.semantic-lens/1');
});