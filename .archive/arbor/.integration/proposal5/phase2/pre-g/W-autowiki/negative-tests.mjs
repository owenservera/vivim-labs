import {createAutoWikiMirror,autoWikiDelta,SCREEN_SCHEMA,PAGE_SCHEMA,DELTA_SCHEMA} from './mirror.mjs';
// Fully synthetic; no provider sessions, source files, persistent stores or side effects.
const check=(value,message)=>{if(!value)throw new Error(message)};
const eq=(a,b,message)=>check(JSON.stringify(a)===JSON.stringify(b),message);
const baseScreen=()=>({
 schema:SCREEN_SCHEMA,contextId:'ctx.1',revision:1,observedAt:'2026-11-12T05:00:00Z',
 source:{kind:'native',surfaceRef:'tierflow.synthetic',viewRef:'view.1',viewRevision:'v1',
  bindingState:'not-required',contentCoverage:'typed-native-state'},
 focus:{title:'Synthetic project',objectRefs:['kb.1'],projectCandidates:[
  {ref:'project.1',state:'trusted',evidence:'native-state'}],
  topicTerms:['synthetic'],activity:'read',draft:{state:'none'}},
 scope:{authority:'host-trusted-scope',ownerRef:'owner.1',worldRef:'world.1',
  purpose:'autowiki-read',policyRevision:'policy.1'},
 coverage:{status:'complete',detail:'native typed',unavailableSources:[],olderHistory:'known-complete'}
});
const baseScope=()=>({authorized:true,revoked:false,ownerRef:'owner.1',worldRef:'world.1',
 policyRevision:'policy.1',purpose:'autowiki-read',sessionRef:'session.1',
 deviceRef:'device.1',allowedAudiences:['owner','development']});
const record=(extra={})=>({id:'hq.kb.1',kind:'decision',title:'Synthetic choice',
 summary:'Synthetic information',authority:'hq-development',ownerRef:'owner.1',worldRef:'world.1',
 projectRef:'project.1',audience:'development',
 source:{nativeRef:'hq.kb.1',revision:'git.1',digest:'digest.1',
  repoRef:'repos-hq',path:'docs/synthetic.md',line:3},
 status:'accepted-decision',rankReasons:['accepted-decision'],
 sourceTimes:{occurredAt:'2026-11-12T04:00:00Z',indexedAt:null,verifiedAt:null},...extra});
const ready=(records=[record()])=>({status:'ready',readerId:'hq',contextRevision:1,
 records,sourceRevisions:[{sourceId:'repos-hq',revision:'git.1',freshness:'current'}]});
const make=(readScope=async()=>baseScope(),readSources=async()=>ready())=>
 createAutoWikiMirror({readScope,readSources,now:()=> '2026-11-12T05:00:01Z'});
export async function runNegativeTests() {
 const rows=[];
 async function test(name,fn){await fn();rows.push({name,exitCode:0});}
 await test('source-linked canonical HQ IDs',async()=>{
  const page=await make().open(baseScreen());
  check(page.schema===PAGE_SCHEMA&&page.status==='ready','page ready');
  check(page.items[0].id==='hq.kb.1'&&page.items[0].source.nativeRef==='hq.kb.1','HQ ID preserved');
  check(page.readOnly&&page.effectPerformed===false,'effect-inert');
 });
 await test('canonical HQ path-bearing document IDs preserved',async()=>{
  const sourceId='doc:#4:docs/feature.md';
  const page=await make(undefined,async()=>ready([record({id:sourceId,source:{nativeRef:sourceId,revision:'git.1',digest:'digest.1'}})])).open(baseScreen());
  check(page.items[0]?.id===sourceId&&page.items[0]?.source.nativeRef===sourceId,'HQ path ID rewritten or dropped');
 });
 await test('stale screen result invalidated',async()=>{
  let release;const pending=new Promise(resolve=>release=resolve);
  const mirror=make(undefined,()=>pending);const promise=mirror.open(baseScreen());
  await Promise.resolve();mirror.close();release(ready());
  const page=await promise;check(page.status==='stale'&&page.items.length===0,'stale did not clear');
 });
 await test('permission revocation between source read and response',async()=>{
  let scope=baseScope();const mirror=make(async()=>scope,async()=>{
   scope={...scope,revoked:true};return ready();});
  const page=await mirror.open(baseScreen());
  check(page.status==='denied'&&page.items.length===0,'revocation leaked source');
 });
 await test('partial chat history never claims completeness',async()=>{
  const screen=baseScreen();screen.source={kind:'bound-web',surfaceRef:'chatgpt.synthetic',
   viewRef:'view.1',bindingState:'bound',contentCoverage:'currently-rendered-only',
   conversationRef:'chatgpt-c:00000000-0000-0000-0000-000000000001'};
  screen.coverage={status:'partial',detail:'visible only',unavailableSources:[],
   olderHistory:'not-observed'};
  const page=await make().open(screen);
  check(page.status==='partial'&&page.coverage.state==='partial','chat history incorrectly complete');
 });
 await test('wrong source audience omitted',async()=>{
  const page=await make(undefined,async()=>ready([record({audience:'third-party'})])).open(baseScreen());
  check(page.items.length===0&&page.omissions[0]?.reason==='scope-filtered','audience leakage');
 });
 await test('HQ unavailable never invents knowledge',async()=>{
  const page=await make(undefined,async()=>{throw new Error('offline')}).open(baseScreen());
  check(page.status==='unavailable'&&page.items.length===0,'unavailable HQ did not fail closed');
 });
 await test('ambiguous project not automatically guessed',async()=>{
  const screen=baseScreen();screen.focus.projectCandidates.push(
   {ref:'project.2',state:'candidate',evidence:'visible-topic'});
  const page=await make().open(screen);
  check(page.focus.projectRef===null&&page.items.length===0,'ambiguous project chosen');
  check(page.omissions.some(x=>x.reason==='unresolved-identity'),'ambiguity not visible');
 });
 await test('unsubmitted draft remains provisional',async()=>{
  const screen=baseScreen();screen.focus.draft={state:'present-unsubmitted',digest:'draft.digest.1'};
  const page=await make().open(screen);
  const draft=page.items.find(x=>x.kind==='draft_intent');
  check(!!draft&&draft.status==='proposal'&&draft.summary.includes('not an accepted decision'),'draft promoted');
 });
 await test('flip back does not mutate editor state',async()=>{
  const screen=baseScreen();screen.focus.draft={state:'present-unsubmitted',digest:'draft.digest.1'};
  const editor={selection:[3,7],draft:'private text kept only by editor',history:['a'],cursor:7};
  const before=structuredClone(editor);const beforeScreen=structuredClone(screen);
  const m=make();await m.open(screen);m.close();
  eq(editor,before,'editor changed during flip');eq(screen,beforeScreen,'screen changed during flip');
  check(m.current()===null,'closed mirror still displayed');
 });
 await test('out-of-order async responses cannot overwrite newer screen',async()=>{
  let release,entered;const pending=new Promise(resolve=>release=resolve);
  const began=new Promise(resolve=>entered=resolve);let calls=0;
  const mirror=make(undefined,()=>{
    if(++calls===1){entered();return pending;}
    return Promise.resolve({...ready(),contextRevision:2});
  });
  const s1=baseScreen(),s2=baseScreen();s2.revision=2;
  const p1=mirror.open(s1);
  await began; // test-controlled barrier: first source read is actually in flight
  const p2=mirror.open(s2);
  const newer=await p2;
  release(ready());const older=await p1;
  check(newer.status==='ready'&&older.status==='stale','out of order accepted');
  check(mirror.current().contextRevision===2,'older response overwrote current');
 });
 await test('client-claimed Host scope cannot mint grants',async()=>{
  const screen=baseScreen();screen.scope.ownerRef='owner.spoof';
  const page=await make().open(screen);
  check(page.status==='denied'&&page.items.length===0,'spoofed owner accepted');
 });
 await test('source response revision mismatch is stale',async()=>{
  const page=await make(undefined,async()=>({...ready(),contextRevision:99})).open(baseScreen());
  check(page.status==='stale'&&page.items.length===0,'old source result accepted');
 });
 await test('revocation delta clears source IDs',async()=>{
  const before=await make().open(baseScreen());
  const after=await make(async()=>({...baseScope(),revoked:true})).open(baseScreen());
  const d=autoWikiDelta(before,after);
  check(d.schema===DELTA_SCHEMA&&d.kind==='revoked'&&d.removedItemIds.includes('hq.kb.1'),'delta missing revoke');
 });
 await test('invalid screen rejected before source read',async()=>{
  let reads=0;const s=baseScreen();delete s.scope.policyRevision;
  try{await make(async()=>{reads++;return baseScope()}).open(s);throw new Error('accepted invalid screen');}
  catch(e){check(e.message==='INVALID_SCREEN_CONTEXT','wrong rejection');}
  check(reads===0,'invalid screen reached authorized read');
 });
 return {suite:'W-autowiki-negative-synthetic',passed:rows.length,failed:0,cases:rows};
}