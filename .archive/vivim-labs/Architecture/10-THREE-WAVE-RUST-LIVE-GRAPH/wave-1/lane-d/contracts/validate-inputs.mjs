// Lane D R2: fail-closed reader of Lane E's *actual* HQKGX1 projection.
// Does not lower A/B/C, write to HQ/SQLite, or grant an action.
import { parseExchange } from '../wiki-oracle.mjs';
import {createHash} from 'node:crypto';
const COMMIT=/^[a-f0-9]{40}$/; const HASH=/^[a-f0-9]{64}$/;
const ID=/^hqg1:[a-f0-9]{64}$/;
const FIELDS=['source_id','source_lane','source_root','source_path','source_commit','content_sha256','source_line','evidence_class','evidence'];
const KINDS=new Set(['project','source-root','compiled-wiki-program']);
const rootGraphId=id=>'hqg1:'+createHash('sha256').update('node\0root:'+id).digest('hex');
const CLASSES=new Set(['static','observed','declared','verified','source-scoped']);
export const EXPECTED_EXCHANGE='repos-hq.hqkg-exchange/1';
export function safePath(s) {
  return typeof s==='string' && s.length>0 && s.length<=240 && !s.includes('\\') && !s.includes(':') && !s.startsWith('/')
    && s.split('/').every(x=>x && x!=='.' && x!=='..' && !/^(?:\.env.*|cookies|login data|session\.json|storage-state\.json|release-profile\.json|.*\.(?:pem|p12|keystore))$/i.test(x));
}
const present=(x)=>typeof x==='string'&&x.trim().length>0;
const compare=(a,b)=>String(a)<String(b)?-1:String(a)>String(b)?1:0;
export function inspectProducerFeed(feed) {
  const accepted='vivim.wave1.source-records/1';
  const known={A:'vivim.wave1.code-observations/1',B:'vivim.wave1.kb-semantic/1',C:'vivim.wave1.command-candidates/1'};
  if (!feed || typeof feed!=='object' || typeof feed.schema!=='string') return {ok:false,status:'BLOCKED',reason:'missing_schema'};
  if (feed.schema===accepted && ['A','B','C'].includes(feed.lane) && Array.isArray(feed.nodes) && Array.isArray(feed.edges)) return {ok:true,status:'STRUCTURE_ONLY_NOT_VERIFIED'};
  const lane=Object.entries(known).find(([,schema])=>schema===feed.schema)?.[0];
  if (lane) return {ok:false,status:'BLOCKED_CONTRACT_CROSSWALK',lane,reason:'native_producer_needs_integration_owned_lowering',expected:accepted,got:feed.schema};
  return {ok:false,status:'BLOCKED',reason:'unsupported_feed_schema',got:feed.schema,expected:accepted};
}
// Returns precise observed coverage/refusals, not synthetic source authorization.
export function validateEGraph(input) {
  const graph=typeof input==='string'?parseExchange(input):input;
  if (!graph?.manifest || graph.manifest.schema!==EXPECTED_EXCHANGE || !Array.isArray(graph.nodes) || !Array.isArray(graph.edges)) throw Error('invalid_or_unverified_hqkg_exchange');
  if (!HASH.test(graph.manifest.graph_sha256)) throw Error('invalid_exchange_digest');
  const nodes=new Map(); const edges=[]; const diagnostics=[]; const roots=new Map();
  for(const n of graph.nodes) {
    if(!ID.test(n.id) || nodes.has(n.id) || typeof n.kind!=='string') throw Error('invalid_or_duplicate_graph_id');
    nodes.set(n.id,n);
    if (n.kind==='source-root') {
      if (!present(n.metadata?.repo) || !safePath(n.metadata?.path) || !COMMIT.test(n.metadata?.commit_sha)) diagnostics.push({code:'invalid_source_root',id:n.id});
      else roots.set(n.metadata.source_id??`root:${n.title}`,n);
    }
  }
  for(const n of graph.nodes) {
    if(KINDS.has(n.kind)) continue;
    const m=n.metadata??{};
    const missing=FIELDS.slice(0,5).filter(k=>!present(m[k]));
    if(missing.length) diagnostics.push({code:'missing_e_source_metadata',id:n.id,fields:missing});
    if(present(m.source_path) && !safePath(m.source_path)) diagnostics.push({code:'unsafe_source_path',id:n.id});
    if(present(m.source_commit) && !COMMIT.test(m.source_commit)) diagnostics.push({code:'wrong_git_commit_hash',id:n.id});
    if(m.content_sha256!==undefined && !HASH.test(m.content_sha256)) diagnostics.push({code:'invalid_content_hash',id:n.id});
    if(m.source_line!==undefined && (!Number.isInteger(Number(m.source_line)) || Number(m.source_line)<1)) diagnostics.push({code:'invalid_source_line',id:n.id});
    if(m.source_root && m.source_commit) {
      const root=nodes.get(rootGraphId(m.source_root));
      if(!root) diagnostics.push({code:'missing_declared_root',id:n.id});
      else if(root.metadata?.commit_sha!==m.source_commit || !(m.source_path===root.metadata.path || m.source_path?.startsWith(root.metadata.path+'/')))
        diagnostics.push({code:'root_revision_or_scope_mismatch',id:n.id});
    }
  }
  for(const e of graph.edges){
    if(!ID.test(e.id) || !nodes.has(e.from) || !nodes.has(e.to)) throw Error('unresolved_exchange_edge');
    const m=e.metadata??{};
    if(!CLASSES.has(m.evidence_class)) diagnostics.push({code:'missing_or_unknown_evidence_class',id:e.id,relation:e.relation});
    if(m.evidence_class!=='source-scoped' && !present(m.evidence)) diagnostics.push({code:'missing_edge_evidence',id:e.id});
    edges.push(e);
  }
  const byCode=Object.groupBy?Object.groupBy(diagnostics,d=>d.code):Object.fromEntries([...new Set(diagnostics.map(x=>x.code))].map(k=>[k,diagnostics.filter(x=>x.code===k)]));
  return {status:diagnostics.length?'BLOCKED_CONTRACT_OR_PROVENANCE':'STRUCTURE_VALID_NOT_RUNTIME_PROVEN',schema:EXPECTED_EXCHANGE,manifest:graph.manifest,nodes,edges,roots,diagnostics:diagnostics.sort((a,b)=>compare(a.code,b.code)||compare(a.id,b.id)),diagnosticCounts:Object.fromEntries(Object.entries(byCode).map(([k,v])=>[k,v.length]))};
}
