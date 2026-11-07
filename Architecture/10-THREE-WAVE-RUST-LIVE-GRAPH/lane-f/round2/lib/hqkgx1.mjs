import {sha256,HEX64} from './evidence.mjs';
export const SCHEMA='repos-hq.hqkg-exchange/1';
export const ID=/^hqg1:[0-9a-f]{64}$/;
export const nodeId = logical => 'hqg1:'+sha256('node\0'+logical);
export const edgeId = (from,relation,to) => 'hqg1:'+sha256('edge\0'+from+'\0'+relation+'\0'+to);
const SECRET=/(password|credential|token|cookie|secret|api.?key|private.?key|authorization)/i;
export function parseExchange(text,{maxBytes=8*1024*1024,maxNodes=100000,maxEdges=250000}={}){
 if(typeof text!=='string'||Buffer.byteLength(text)>maxBytes||!text.endsWith('\n'))throw Error('EXCHANGE_FRAMING');
 const cut=text.indexOf('\n');if(cut<=0)throw Error('EXCHANGE_MANIFEST');
 let head;try{head=JSON.parse(text.slice(0,cut))}catch{throw Error('EXCHANGE_MANIFEST_JSON')}
 const body=text.slice(cut+1);if(head.schema!==SCHEMA||head.type!=='manifest'||head.authority!=='data-only; no executable code or provider actions')throw Error('EXCHANGE_AUTHORITY');
 if(!HEX64.test(head.graph_sha256)||sha256(body)!==head.graph_sha256)throw Error('EXCHANGE_DIGEST');
 if(!Number.isInteger(head.node_count)||head.node_count<0||head.node_count>maxNodes||!Number.isInteger(head.edge_count)||head.edge_count<0||head.edge_count>maxEdges)throw Error('EXCHANGE_COUNTS');
 const nodes=[],edges=[],seen=new Set();let seenEdge=false;
 for(const line of body.trimEnd().split('\n')){
   if(!line)throw Error('EMPTY_RECORD');
   let rec;try{rec=JSON.parse(line)}catch{throw Error('EXCHANGE_JSON')}
   if(!ID.test(rec.id)||seen.has(rec.id))throw Error('EXCHANGE_ID_COLLISION');seen.add(rec.id);
   if(rec.type!=='node'&&rec.type!=='edge')throw Error('EXCHANGE_TYPE');
   if(rec.type==='node'&&seenEdge)throw Error('EXCHANGE_ORDER');
   if(rec.type==='edge')seenEdge=true;
   if(!rec.metadata||typeof rec.metadata!=='object'||Array.isArray(rec.metadata))throw Error('EXCHANGE_METADATA');
   for(const [key,value] of Object.entries(rec.metadata))if(SECRET.test(key)||key.startsWith('hqkg.')||!['string','number','boolean'].includes(typeof value)||String(value).length>1024)throw Error('EXCHANGE_SECRET_OR_NONSCALAR');
   if(rec.type==='node')nodes.push(rec);else edges.push(rec);
 }
 if(nodes.length!==head.node_count||edges.length!==head.edge_count)throw Error('EXCHANGE_CARDINALITY');
 const ids=new Set(nodes.map(n=>n.id));
 for(const e of edges)if(!ids.has(e.from)||!ids.has(e.to)||typeof e.relation!=='string')throw Error('EXCHANGE_DANGLING_EDGE');
 return {manifest:head,nodes,edges};
}
export function inspectProvenance(exchange){
 const findings=[];
 for(const n of exchange.nodes){
   const m=n.metadata;
   if(['A','B','C'].includes(m.source_lane)){
     if(!m.source_root||!m.source_path||!/^[a-f0-9]{40}$/.test(m.source_commit??''))findings.push({id:n.id,code:'SOURCE_PROVENANCE_UNVERIFIED'});
     if(m.content_sha256&&!HEX64.test(m.content_sha256))findings.push({id:n.id,code:'CONTENT_DIGEST_INVALID'});
   }
 }
 for(const e of exchange.edges){const m=e.metadata;if(!m.evidence_class&&!m.evidence)findings.push({id:e.id,code:'EDGE_EVIDENCE_UNKNOWN'});}
 return {status:findings.length?'BLOCKED':'STRUCTURE_ONLY_NOT_NATIVE_VERIFIED',findings};
}
export function checkCrosswalk(exchange,entries){
 const issues=[],known=new Set([...exchange.nodes,...exchange.edges].map(x=>x.id)),original=new Set(),bridged=new Set();
 for(const e of entries??[]){
  if(typeof e.id!=='string'||!ID.test(e.bridgeId)||!known.has(e.bridgeId)||original.has(e.id)||bridged.has(e.bridgeId))issues.push({id:e.id,kind:'CROSSWALK_MISMATCH'});
  original.add(e.id);bridged.add(e.bridgeId);
  if(!e.id.includes(' ')&&e.bridgeId!==nodeId(e.id))issues.push({id:e.id,kind:'NONCANONICAL_NODE_ID'});
 }
 return {status:issues.length?'BLOCKED':'STRUCTURE_ONLY_NOT_NATIVE_VERIFIED',issues};
}
