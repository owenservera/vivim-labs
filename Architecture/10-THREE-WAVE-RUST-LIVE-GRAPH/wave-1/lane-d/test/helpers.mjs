// Explicitly synthetic fixtures for E's published source-owned HQKGX1 shape.
import {createHash} from 'node:crypto';
import {parseExchange} from '../wiki-oracle.mjs';
const sha=x=>createHash('sha256').update(x).digest('hex');
export const id=x=>'hqg1:'+sha('node\0'+x);
const edgeId=(from,rel,to)=>'hqg1:'+sha('edge\0'+from+'\0'+rel+'\0'+to);
export const COMMIT='a'.repeat(40),CONTENT='b'.repeat(64);
export function node(logical,kind,metadata){return {type:'node',id:id(logical),kind,title:logical,metadata};}
export function edge(from,relation,to,metadata={evidence_class:'declared',evidence:'synthetic:fixture'}){return {type:'edge',id:edgeId(from,relation,to),from:id(from),to:id(to),relation,weight:1,metadata};}
export function graphFixture({badIdentity=false,missingKb=false,stale=false,noCompiler=false}={}){
 const root='root:hq-root',a='a:source',program='compiled:a:source:wiki-view@0.1',s='b:meaning',cmd='c:handler',kb='b:decision';
 const nodes=[node('project:vivim','project',{intent_id:'vivim.graph-intent@1',scope_revision:1}),node(root,'source-root',{repo:'owenservera/repos-hq',path:'hq',commit_sha:COMMIT}),node(a,'file',{source_id:a,source_lane:'A',source_root:'hq-root',source_path:'hq/lib/example.mjs',source_commit:stale?'c'.repeat(40):COMMIT,content_sha256:CONTENT,source_line:7})];
 if(!noCompiler)nodes.push(node(program,'compiled-wiki-program',{source_path:'hq/lib/example.mjs',content_sha256:CONTENT,source_line:7,status:'inert',compiler_version:'0.1'}));
 nodes.push(node(s,'semantic_id',{source_id:s,source_lane:'B',source_root:'hq-root',source_path:'hq/lib/example.mjs',source_commit:COMMIT,content_sha256:CONTENT,source_line:7,status:'current',semantic_id:'vivim.graph.inspect@1',description:'Inspect graph',authority:'hq-sqlite'}));
 nodes.push(node(cmd,'action',{source_id:cmd,source_lane:'C',source_root:'hq-root',source_path:'hq/lib/example.mjs',source_commit:COMMIT,content_sha256:CONTENT,source_line:7,status:'wired',semantic_id:badIdentity?'vivim.graph.delete@1':'vivim.graph.inspect@1',syntax:'hq graph inspect',handler:'hq/bin/hq.mjs',risk:'low'}));
 if(!missingKb)nodes.push(node(kb,'kb_document',{source_id:kb,source_lane:'B',source_root:'hq-root',source_path:'hq/lib/example.mjs',source_commit:COMMIT,content_sha256:CONTENT,source_line:7,status:'accepted',description:'KB decision source-linked, unverified in this synthetic test'}));
 const edges=[edge('project:vivim','INDEXES_ROOT',root,{intent_id:'vivim.graph-intent@1'}),edge(root,'CONTAINS_SOURCE',a,{evidence_class:'source-scoped'}),edge(root,'CONTAINS_SOURCE',s,{evidence_class:'source-scoped'}),edge(root,'CONTAINS_SOURCE',cmd,{evidence_class:'source-scoped'})];
 if(!missingKb)edges.push(edge(root,'CONTAINS_SOURCE',kb,{evidence_class:'source-scoped'}),edge(kb,'INFORMS',s,{evidence_class:'verified',evidence:'synthetic:kbcrosswalk'}));
 if(!noCompiler)edges.push(edge(a,'COMPILES_WIKI_PROGRAM',program,{evidence_class:'declared',source_line:7}),edge(program,'DECLARES_INTENT_FOR',s,{evidence_class:'declared',action_id:'vivim.graph.inspect@1'}),edge(s,'HAS_REGISTERED_HANDLER',cmd,{evidence_class:'declared',action_id:'vivim.graph.inspect@1'}));
 return {nodes,edges};
}
export function encodeExchange(rows){const body=[...rows.nodes,...rows.edges].map(x=>JSON.stringify(x)).join('\n')+'\n';return JSON.stringify({type:'manifest',schema:'repos-hq.hqkg-exchange/1',source:'test:synthetic',authority:'data-only; no executable code or provider actions',graph_sha256:sha(body),node_count:rows.nodes.length,edge_count:rows.edges.length})+'\n'+body;}
export const graph=(opts)=>parseExchange(encodeExchange(graphFixture(opts)));
