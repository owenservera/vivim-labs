export const SCHEMAS={A:'vivim.wave1.code-observations/1',B:'vivim.wave1.kb-semantic/1',C:'vivim.wave1.command-candidates/1',E:'vivim.wave1.source-records/1'};
export function auditCrosswalk({a,b,c,scope,eFeeds=[],compiled=[]}={}){
 const findings=[];const fail=(lane,code,detail)=>findings.push({lane,code,detail});
 for(const [key,record] of [['A',a],['B',b],['C',c]])if(record?.schema!==SCHEMAS[key])fail(key,'SOURCE_SCHEMA','Missing or stale schema');
 if(scope?.schema!=='vivim.wave1.project-scope/1'||!Array.isArray(scope.roots)||!scope.roots.length)fail('E','SCOPE_MISSING','One project / N root scope not established');
 const roots=new Map((scope?.roots??[]).map(x=>[x.id,x]));
 for(const root of a?.roots??[]){
  const match=roots.get(root.id);
  if(!match)fail('A','ROOT_NOT_MAPPED',root.id);
  else if(root.headSha!==match.commitSha)fail('A','ROOT_COMMIT_MISMATCH',root.id);
 }
 if(a?.project?.id&&scope?.projectId&&a.project.id!==scope.projectId)fail('A','PROJECT_ID_DRIFT',`${a.project.id} != ${scope.projectId}`);
 if((b?.project?.id??null)!==null&&typeof b.project.id==='number')fail('B','HQ_NUMERIC_PROJECT_ID_REQUIRES_CROSSWALK','Do not coerce to logical project ID');
 for(const r of b?.unknowns??[])fail('B','SOURCE_LINK_UNKNOWN',r.kind??'unknown');
 const semantics=new Set((b?.records??[]).filter(r=>r.type==='semantic_id').map(r=>r.id.replace(/^semantic:/,'')));
 for(const r of c?.records??[]){
  if(r.semanticId===null||r.semanticId===undefined)fail('C','SEMANTIC_BINDING_UNVERIFIED',r.id);
  else if(!semantics.has(r.semanticId))fail('C','SEMANTIC_ID_NOT_REGISTERED',r.id);
  if(r.evidenceState?.runtime!=='OBSERVED')fail('C','COMMAND_NOT_RUNTIME_TESTED',r.id);
 }
 const lanes=new Set();for(const feed of eFeeds){
  if(feed?.schema!==SCHEMAS.E||!['A','B','C'].includes(feed.lane)||lanes.has(feed.lane))fail('E','FEED_SCHEMA_OR_DUPLICATE',feed?.lane??'missing');
  lanes.add(feed?.lane);
  if(feed.projectId!==scope?.projectId||feed.intentId!==scope?.intentId||feed.scopeRevision!==scope?.revision)fail('E','FEED_SCOPE_DRIFT',feed.lane);
 }
 for(const name of ['A','B','C'])if(!lanes.has(name))fail('E','FEED_MISSING',name);
 if(!Array.isArray(compiled)||!compiled.some(x=>x.schema==='hq.comment-program/0.1'&&x.ok===true&&x.programs?.length))fail('E','REAL_COMPILER_BINDING_UNVERIFIED','Compiler subgate cannot pass');
 return {status:findings.length?'BLOCKED':'SOURCE_SHAPES_COMPATIBLE_NOT_NATIVE_VERIFIED',findings,counts:{aFiles:a?.files?.length??0,bRecords:b?.records?.length??0,cRecords:c?.records?.length??0,feeds:lanes.size}};
}
export function projectDCompatibility(exchange){
 const findings=[];const dKinds=new Set(['source','file','symbol','code','kb','decision','knowledge','semantic','identity','action','command','cli']);for(const n of exchange?.nodes??[]){const m=n.metadata??{};
  if(['A','B','C'].includes(m.source_lane)&&!dKinds.has(n.kind)&&!dKinds.has(m.domain))findings.push({id:n.id,code:'D_DOMAIN_KIND_UNKNOWN',kind:n.kind});
  if(m.source_lane&&m.source_path&&m.source_commit&&(!m.source_repo||!m.source_sha))findings.push({id:n.id,code:'D_SOURCE_FIELD_MISMATCH',from:['source_path','source_commit','content_sha256'],to:['source_repo','source_sha']});
 }
 for(const e of exchange?.edges??[])if(e.metadata?.evidence_class&&!e.metadata?.link_status)findings.push({id:e.id,code:'D_EDGE_FIELD_MISMATCH'});
 return {status:findings.length?'BLOCKED':'FIELDS_COMPATIBLE_NOT_SEMANTICALLY_VERIFIED',findings};
}
