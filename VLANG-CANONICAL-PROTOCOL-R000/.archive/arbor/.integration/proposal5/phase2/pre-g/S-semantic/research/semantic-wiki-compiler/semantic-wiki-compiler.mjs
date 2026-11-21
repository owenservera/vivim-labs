/**
 * SWC-001 source-comment → Wiki Semantic IR research compiler.
 *
 * Not a command parser, Host executor, permission grant, new wiki generator
 * or source registry. Reuses the existing HQ annotation v2 parser and accepts
 * only owner-pinned Host catalog projections. Research output remains INERT.
 */
import { parseAnnotationV2 } from '../../hq/lib/annotation-v2.mjs';

export const SEMANTIC_WIKI_IR = 'vivim.semantic-wiki-ir/0.1';
const VALUE_TAGS = new Set(['type','capability','effect','evidence','contract','emits','consumes','action','test']);
const ID = /^[a-z0-9][a-z0-9._-]{0,127}$/;
const HASH = /^[a-f0-9]{64}$/i;
const SAFE_PATH = /^[A-Za-z0-9._/\\-]{1,450}$/;
const SENSITIVE = /(?:^|[\\/])(?:\.env[^\\/]*|[^\\/]*\.pem|[^\\/]*\.p12|[^\\/]*\.keystore|cookies|login data|session\.json|storage-state\.json)(?:$|[\\/])/i;
const compare = (a,b)=> a < b ? -1 : a > b ? 1 : 0;
const bounded = (a,n)=>Array.isArray(a)&&a.length<=n;
const plain = v=>v!==null&&typeof v==='object'&&!Array.isArray(v);
function error(code, sourceId, detail='') { return {code,sourceId,detail:String(detail).slice(0,200)}; }
function safePath(s) {
  return typeof s==='string' && SAFE_PATH.test(s) &&
  !s.split(/[\\/]/).some(x=>x==='..'||x==='.') && !SENSITIVE.test(s) && !s.startsWith('/') && !/^[a-z]:/i.test(s);
}
function validSource(x) {
  return plain(x)&&typeof x.sourceId==='string'&&ID.test(x.sourceId)&&
    typeof x.repoRef==='string'&&ID.test(x.repoRef)&&safePath(x.path)&&
    typeof x.contentHash==='string'&&HASH.test(x.contentHash)&&
    typeof x.sourceText==='string'&&x.sourceText.length<=500000&&
    typeof x.symbolId==='string'&&ID.test(x.symbolId);
}
function validBinding(x) {
  return plain(x) && typeof x.semanticAction==='string' &&
    /^[a-z0-9][a-z0-9._-]*@\d+(?:\.\d+)*$/.test(x.semanticAction) &&
    typeof x.hostActionId==='string'&&ID.test(x.hostActionId) &&
    typeof x.owner==='string'&&ID.test(x.owner);
}
function errorInParsed(diags) { return diags.some(d=>d.severity==='error'); }

/**
 * @param {{
 *  sources: Array<{sourceId:string,repoRef:string,path:string,contentHash:string,sourceText:string,symbolId:string}>,
 *  registry: {lookup(id:string):{id?:string,kind?:string,status?:string}|null},
 *  hostCatalog: {version:number,sourceDigest:string,actions:Array<object>},
 *  bindings?: Array<{semanticAction:string,hostActionId:string,owner:string}>
 * }} input
 *
 * No raw source bodies are returned, only normalized annotations and source refs.
 * Source digests are caller-provided and MUST be verified by the trusted HQ walker
 * before this output can be used as a real build artifact.
 */
export function compileSemanticWiki(input={}) {
  if(!plain(input)||!bounded(input.sources,128)||!bounded(input.bindings??[],256)||
     !plain(input.hostCatalog)||!bounded(input.hostCatalog.actions,512)||
     !Number.isSafeInteger(input.hostCatalog.version) || input.hostCatalog.version<1 ||
     typeof input.hostCatalog.sourceDigest!=='string'||!HASH.test(input.hostCatalog.sourceDigest)||
     !plain(input.registry)||typeof input.registry.lookup!=='function') {
    throw new TypeError('SWC_INVALID_INPUT');
  }
  const diagnostics=[],sources=[],nodes=[],edges=[],actions=[],rejections=[];
  const seenSource=new Set(),actionRegistry=new Map(),bindingRegistry=new Map();
  for(const a of input.hostCatalog.actions) {
    if(!plain(a)||typeof a.id!=='string'||!ID.test(a.id)||actionRegistry.has(a.id))
      throw new TypeError('SWC_INVALID_ACTION_CATALOG');
    actionRegistry.set(a.id,a);
  }
  for(const b of input.bindings??[]) {
    if(!validBinding(b))throw new TypeError('SWC_INVALID_EXPLICIT_BINDING');
    if(bindingRegistry.has(b.semanticAction)){
      diagnostics.push(error('AMBIGUOUS_OWNER_BINDING',b.semanticAction));
      bindingRegistry.set(b.semanticAction,null); // Fail closed on all duplicates
    } else bindingRegistry.set(b.semanticAction,b);
  }
  for(const src of [...input.sources].sort((a,b)=>compare(a?.sourceId??'',b?.sourceId??''))) {
    if(!validSource(src)||seenSource.has(src.sourceId)) {
      diagnostics.push(error('INVALID_OR_DUPLICATE_SOURCE',String(src?.sourceId??'')));
      continue;
    }
    seenSource.add(src.sourceId);
    const records=parseAnnotationV2(src.sourceText,{
      projectId:src.repoRef,path:src.path,contentHash:src.contentHash,
      symbolId:src.symbolId,registry:input.registry
    });
    for(const diag of records.diagnostics) diagnostics.push({
      code:'ANNOTATION_'+diag.code,sourceId:src.sourceId,line:diag.line,severity:diag.severity
    });
    sources.push({sourceId:src.sourceId,repoRef:src.repoRef,path:src.path,
      contentHash:src.contentHash,symbolId:src.symbolId,
      status:errorInParsed(records.diagnostics)?'rejected':'parsed',
      annotationCount:records.records.length});
    if(errorInParsed(records.diagnostics)) {
      rejections.push({sourceId:src.sourceId,reason:'annotation_errors'});
      continue; // no dangerous half-compiled annotations
    }
    const refs=new Map();
    for(const rec of records.records) {
      if(!VALUE_TAGS.has(rec.kind))continue;
      const nodeId=src.sourceId+':'+rec.line+':'+rec.kind;
      if(nodes.some(n=>n.id===nodeId)) {
        diagnostics.push(error('DUPLICATE_TYPED_ANNOTATION',src.sourceId,rec.kind));
        continue;
      }
      const node={id:nodeId,kind:rec.kind,ref:rec.value,
        source:{repoRef:src.repoRef,path:src.path,line:rec.line,
        contentHash:src.contentHash,symbolId:src.symbolId}};
      nodes.push(node);
      if(!refs.has(rec.kind))refs.set(rec.kind,[]);
      refs.get(rec.kind).push(node);
    }
    const act=refs.get('action')??[];
    // One symbol must have one explicit action identity in this v2 research
    // compiler. Multi-action meaning needs an approved typed composition frame;
    // otherwise all adjacent effects/contracts could be wrongly attached.
    if(act.length>1) {
      diagnostics.push(error('AMBIGUOUS_ACTION_SYMBOL',src.sourceId,src.symbolId));
      rejections.push({sourceId:src.sourceId,reason:'ambiguous_action_symbol'});
      continue;
    }
    for(const actionNode of act) {
      for(const kind of ['capability','contract','effect','evidence','emits','consumes','type','test']) {
        for(const target of refs.get(kind)??[]) {
          edges.push({id:actionNode.id+'>'+target.id,from:actionNode.id,to:target.id,
            kind:kind==='capability'?'supports_capability':kind==='contract'?'subject_to_contract':
            kind==='effect'?'declares_effect':kind==='evidence'||kind==='test'?'has_source_evidence':'associated_source_semantics'});
        }
      }
      const binding=bindingRegistry.get(actionNode.ref);
      if(!binding) {
        rejections.push({sourceId:src.sourceId,semanticAction:actionNode.ref,
          reason:bindingRegistry.has(actionNode.ref)?'ambiguous_owner_binding':'no_owner_binding'});
        continue;
      }
      const host=actionRegistry.get(binding.hostActionId);
      if(!host||host.owner!==binding.owner) {
        rejections.push({sourceId:src.sourceId,semanticAction:actionNode.ref,
          reason:'host_catalog_owner_mismatch'});
        continue;
      }
      actions.push({
        semanticAction:actionNode.ref,hostActionId:host.id,catalogVersion:input.hostCatalog.version,
        catalogDigest:input.hostCatalog.sourceDigest,
        title:typeof host.title==='string'?host.title.slice(0,160):host.id,
        readiness:typeof host.readiness==='string'?host.readiness:'unknown',
        risk:typeof host.risk==='string'?host.risk:'unknown',
        parameterNames:plain(host.args)?Object.keys(host.args).sort(compare):[],
        sourceNodeId:actionNode.id,
        sourceRefs:[actionNode.id,...edges.filter(e=>e.from===actionNode.id).map(e=>e.to)],
        status:'source_crosswalk_only',
        authority:'none',sourceDigestVerified:false,runtimeVerified:false,
        executable:false,executionPerformed:false
      });
    }
  }
  sources.sort((a,b)=>compare(a.sourceId,b.sourceId));
  nodes.sort((a,b)=>compare(a.id,b.id));
  edges.sort((a,b)=>compare(a.id,b.id));
  actions.sort((a,b)=>compare(a.semanticAction,b.semanticAction)||compare(a.sourceNodeId,b.sourceNodeId));
  diagnostics.sort((a,b)=>compare(a.sourceId,b.sourceId)||compare(a.code,b.code));
  rejections.sort((a,b)=>compare(a.sourceId,b.sourceId)||compare(a.reason,b.reason));
  return {
    schema:SEMANTIC_WIKI_IR,version:1,
    origin:'hq_annotation_v2_owner_crosswalk',
    catalog:{version:input.hostCatalog.version,digest:input.hostCatalog.sourceDigest},
    sources,nodes,edges,actions,diagnostics,rejections,
    coverage:{inputs:input.sources.length,parsedSources:sources.filter(s=>s.status==='parsed').length,
      rejectedSources:sources.filter(s=>s.status==='rejected').length,
      actionCrosswalks:actions.length,missingOrInvalidBindings:rejections.length,
      interpreterScope:'source_comment_only',runtimeAcceptance:'not_evaluated'},
    outputRole:'read_only_semantic_wiki_projection',
    effectPerformed:false
  };
}
