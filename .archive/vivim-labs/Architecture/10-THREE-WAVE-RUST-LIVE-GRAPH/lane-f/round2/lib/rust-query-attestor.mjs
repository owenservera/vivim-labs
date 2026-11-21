export const QUERY_SCHEMA='vivim.wave1.query/1';
const CMDS=['where','what','talks-to','gap'];
export function compareQueries(exchange,queries=[],{receipt=null}={}){
 const findings=[],nodes=new Map(exchange?.nodes?.map(n=>[n.id,n])??[]),edges=new Map(exchange?.edges?.map(e=>[e.id,e])??[]),seen=new Set();
 if(!exchange?.manifest?.graph_sha256)findings.push({code:'EXCHANGE_REQUIRED'});
 for(const q of queries){
  if(q?.schema!==QUERY_SCHEMA||!CMDS.includes(q.query)||typeof q.exactKey!=='string'||!Array.isArray(q.matches)||q.matches.length<1||q.matches.length>32||!Number.isInteger(q.journalEpoch)){
   findings.push({code:'QUERY_SHAPE',command:q?.query??null});continue;
  }
  seen.add(q.query);for(const match of q.matches){
   const n=nodes.get(match.id);
   if(!n){findings.push({code:'QUERY_NODE_NOT_IN_EXCHANGE',id:match.id});continue}
   const m=n.metadata??{};
   if(match.sourceId!==m.source_id||match.sourcePath!==(m.source_path??null)||match.sourceCommit!==(m.source_commit??null))findings.push({code:'QUERY_SOURCE_PROVENANCE_DRIFT',id:match.id});
   const links=q.query==='what'?match.links:q.query==='talks-to'?match.communication:q.query==='gap'?match.evidence:[];
   if(!Array.isArray(links)){if(q.query!=='where')findings.push({code:'QUERY_LINKS_NOT_ARRAY',id:match.id});continue}
   for(const l of links){const e=edges.get(l.edgeId);
    if(!e||l.relation!==e.relation||l.evidenceClass!==(e.metadata?.evidence_class??null))findings.push({code:'QUERY_EDGE_DIFFERENCE',id:l.edgeId});
    if(e&&l.other?.id!== (e.from===match.id?e.to:e.from))findings.push({code:'QUERY_EDGE_DIRECTION_DIFFERENCE',id:l.edgeId});
   }
  }
 }
 for(const c of CMDS)if(!seen.has(c))findings.push({code:'QUERY_MISSING',command:c});
 if(receipt?.evidenceClass!=='observed-native'||receipt?.exchangeDigest!==exchange?.manifest?.graph_sha256||!receipt?.journalSha256)findings.push({code:'RUST_RECEIPT_NOT_NATIVE_VERIFIED'});
 return {status:findings.length?'BLOCKED':'QUERY_SHAPES_AGREE_INDEPENDENT_REPLAY_STILL_REQUIRED',findings,queryCount:queries.length};
}
