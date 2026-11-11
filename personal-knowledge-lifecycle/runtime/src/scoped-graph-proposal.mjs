// Pure adapter payload proposal to existing HQKG scoped_graph.rs/GraphChangeSet.
// Host MUST authenticate provenance and apply through the actual Rust bridge.
// This module does not apply a graph or write another graph database.
import {createHash} from 'node:crypto';
const hash=v=>createHash('sha256').update(v).digest('hex');
const fail=code=>Object.assign(new Error(code),{code});
const idOk=(s,max)=>typeof s==='string'&&Buffer.byteLength(s)>0&&Buffer.byteLength(s)<=max&&!/[\x00-\x1f]/.test(s);
const digestOk=s=>typeof s==='string'&&/^[a-f0-9]{64}$/.test(s);
export function personalPrefix(owner){
  if(!idOk(owner,80))throw fail('GRAPH_OWNER_INVALID');
  const prefix='personal:'+Buffer.byteLength(owner)+':'+owner+'/';
  if(Buffer.byteLength(prefix)>180)throw fail('GRAPH_OWNER_INVALID');
  return prefix;
}
const localId=(kind,source,native)=>kind+'-'+hash(source+'\0'+native).slice(0,40);
export function buildPersonalGraphProposal({owner,sourceId,provenanceRevision,provenanceDigest,expectedGraphEpoch,conversations,batchKey}={}){
  const prefix=personalPrefix(owner);
  if(!idOk(sourceId,256)||!digestOk(provenanceDigest)||!Number.isSafeInteger(provenanceRevision)||provenanceRevision<1||!Number.isSafeInteger(expectedGraphEpoch)||expectedGraphEpoch<0||!idOk(batchKey,256))throw fail('GRAPH_PROVENANCE_INVALID');
  if(!Array.isArray(conversations)||conversations.length>128)throw fail('GRAPH_BATCH_INVALID');
  const nodes=[],edges=[],ids=new Set();
  const stamp=()=>({
    'hqkg.contract':'hqkg.scoped-record/1','hqkg.namespace':'personal:'+owner,
    'hqkg.provenance.source':sourceId,'hqkg.provenance.revision':String(provenanceRevision),
    'hqkg.provenance.digest':provenanceDigest
  });
  for(const conversation of conversations){
    if(!idOk(conversation?.nativeId,256)||!digestOk(conversation?.contentDigest)||!Array.isArray(conversation?.messages))throw fail('GRAPH_CONVERSATION_INVALID');
    const convId=prefix+localId('thread',sourceId,conversation.nativeId);
    if(ids.has(convId))throw fail('GRAPH_DUPLICATE_ID');ids.add(convId);
    nodes.push({id:convId,kind:'document',title:(conversation.title||'Conversation').slice(0,1024),version:0,sources:[sourceId],properties:{...stamp(),
      'vivim.object.kind':'conversation','vivim.native.digest':conversation.contentDigest,
      'vivim.issuedAt':conversation.createdAt??'', 'vivim.source.account':conversation.accountId??''}});
    const msgIds=new Map();
    for(const message of conversation.messages){
      if(!idOk(message.nativeId,256)||!idOk(message.nodeId,256)||!digestOk(message.contentDigest))throw fail('GRAPH_MESSAGE_INVALID');
      if(msgIds.has(message.nodeId))throw fail('GRAPH_DUPLICATE_ID');
      const msgId=prefix+localId('message',sourceId,conversation.nativeId+'\0'+message.nodeId);
      msgIds.set(message.nodeId,msgId);
      if(ids.has(msgId))throw fail('GRAPH_DUPLICATE_ID');ids.add(msgId);
      nodes.push({id:msgId,kind:'document',title:'ChatGPT '+(message.role||'unknown'),version:0,sources:[sourceId],properties:{...stamp(),
        'vivim.object.kind':'message','vivim.native.digest':message.contentDigest,
        'vivim.message.role':message.role??'unknown','vivim.message.omittedMedia':String(message.omittedMedia===true),
        'vivim.issuedAt':message.createdAt??'', 'vivim.parent.nodeId':message.parentId??''}});
      edges.push({id:prefix+localId('edge',sourceId,convId+'\0'+msgId),from:convId,to:msgId,relation:'contains',weight:1,properties:stamp()});
    }
    for(const message of conversation.messages){
      const to=msgIds.get(message.nodeId),from=msgIds.get(message.parentId);
      if(from&&to){edges.push({id:prefix+localId('edge',sourceId,from+'\0'+to),from,to,relation:'precedes',weight:1,properties:stamp()});}
    }
  }
  return {
    contract:'vivim.personal-graph-proposal/1',targetRust:'hqkg.scoped-record/1',
    namespace:{domain:'personal',owner},expected:{namespace:{domain:'personal',owner},graphEpoch:expectedGraphEpoch},
    changes:{changeSetId:prefix+localId('change',sourceId,batchKey),source:sourceId,
      ops:[...nodes.map(node=>({op:'PutNode',node})),...edges.map(edge=>({op:'PutEdge',edge}))]},
    counts:{nodes:nodes.length,edges:edges.length},
    advisory:'Host must authenticate source, revision and graphEpoch and translate this proposal to typed Rust values before apply_scoped.'
  };
}
