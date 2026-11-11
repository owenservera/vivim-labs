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
const stableId=(kind,native)=>kind+'-'+hash(native).slice(0,40);
const HEX64=/^[a-f0-9]{64}$/;
const packetProperty=value=>typeof value==='string'?value:String(value);
// Exact apply packet for the existing hqkg-personal CLI (vivim.hqkg-personal/1):
// LOCAL unqualified node/edge ids (the CLI qualifies them into the pk-<scope>
// namespace), string-only app properties without reserved hqkg./personal. keys
// (scoped provenance and the commit marker are minted by the CLI itself).
export function buildHqkgPersonalPacket({scope,opId,payloadDigest,expectedEpoch,expectedBytes,conversations}={}){
  if(!HEX64.test(scope??'')||!HEX64.test(opId??'')||!HEX64.test(payloadDigest??''))throw fail('GRAPH_PACKET_IDENTITY_INVALID');
  if(expectedEpoch!==undefined&&(!Number.isSafeInteger(expectedEpoch)||expectedEpoch<0))throw fail('GRAPH_PACKET_IDENTITY_INVALID');
  if(expectedBytes!==undefined&&(!Number.isSafeInteger(expectedBytes)||expectedBytes<0))throw fail('GRAPH_PACKET_IDENTITY_INVALID');
  if(!Array.isArray(conversations)||conversations.length>128)throw fail('GRAPH_BATCH_INVALID');
  const nodes=[],edges=[],ids=new Set();
  for(const conversation of conversations){
    if(!idOk(conversation?.nativeId,256)||!digestOk(conversation?.contentDigest)||!Array.isArray(conversation?.messages))throw fail('GRAPH_CONVERSATION_INVALID');
    const convLocal=stableId('thread',conversation.nativeId);
    if(ids.has(convLocal))throw fail('GRAPH_DUPLICATE_ID');ids.add(convLocal);
    nodes.push({id:convLocal,kind:'document',title:packetProperty((conversation.title||'Conversation').slice(0,1024)),properties:{
      'vivim.object.kind':'conversation','vivim.native.digest':conversation.contentDigest,
      'vivim.revision.schema':CONTENT_REVISION_SCHEMA_SCHEMA(conversation),'vivim.issued.at':packetProperty(conversation.createdAt??''),
      'vivim.source.account':packetProperty(conversation.accountId??'')}});
    const msgLocals=new Map();
    for(const message of conversation.messages){
      if(!idOk(message.nativeId,256)||!idOk(message.nodeId,256)||!digestOk(message.contentDigest))throw fail('GRAPH_MESSAGE_INVALID');
      if(msgLocals.has(message.nodeId))throw fail('GRAPH_DUPLICATE_ID');
      const msgLocal=stableId('message',conversation.nativeId+'\0'+message.nativeId+'\0'+message.nodeId);
      msgLocals.set(message.nodeId,msgLocal);
      if(ids.has(msgLocal))throw fail('GRAPH_DUPLICATE_ID');ids.add(msgLocal);
      nodes.push({id:msgLocal,kind:'document',title:packetProperty('ChatGPT '+(message.role||'unknown')),properties:{
        'vivim.object.kind':'message','vivim.native.digest':message.contentDigest,
        'vivim.message.role':packetProperty(message.role??'unknown'),
        'vivim.message.omittedMedia':String(message.omittedMedia===true),
        'vivim.issued.at':packetProperty(message.createdAt??''),
        'vivim.parent.native':packetProperty(message.parentId??''),
        'vivim.parent.node':msgLocals.get(message.parentId)??''}});
      edges.push({id:'edge-'+hash(convLocal+'\0'+msgLocal).slice(0,40),from:convLocal,to:msgLocal,relation:'contains'});
    }
    for(const message of conversation.messages){
      const to=msgLocals.get(message.nodeId),from=msgLocals.get(message.parentId);
      if(from&&to)edges.push({id:'edge-'+hash(from+'\0'+to).slice(0,40),from,to,relation:'precedes'});
    }
  }
  if(nodes.length>20000||edges.length>50000)throw fail('GRAPH_CAPACITY_LIMIT');
  for(const id of [...ids,...edges.map(e=>e.id)])if(!id||Buffer.byteLength(id)>120||/[\x00-\x1f]/.test(id))throw fail('GRAPH_LOCAL_ID_INVALID');
  const packet={schema:'vivim.hqkg-personal/1',op:'apply',scope,opId,payloadDigest,expectedEpoch,
    ...(expectedBytes!==undefined?{expectedBytes}:{}),nodes,edges};
  if(Buffer.byteLength(JSON.stringify(packet))>1024*1024)throw fail('GRAPH_PROPOSAL_LIMIT');
  return packet;
}
const CONTENT_REVISION_SCHEMA_SCHEMA=conversation=>typeof conversation?.revisionSchema==='string'?conversation.revisionSchema:'vivim.personal-content-revision/2';
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
    const convId=prefix+stableId('thread',conversation.nativeId);
    if(ids.has(convId))throw fail('GRAPH_DUPLICATE_ID');ids.add(convId);
    nodes.push({id:convId,kind:'document',title:(conversation.title||'Conversation').slice(0,1024),version:0,sources:[sourceId],properties:{...stamp(),
      'vivim.object.kind':'conversation','vivim.native.digest':conversation.contentDigest,
      'vivim.issuedAt':conversation.createdAt??'', 'vivim.source.account':conversation.accountId??''}});
    const msgIds=new Map();
    for(const message of conversation.messages){
      if(!idOk(message.nativeId,256)||!idOk(message.nodeId,256)||!digestOk(message.contentDigest))throw fail('GRAPH_MESSAGE_INVALID');
      if(msgIds.has(message.nodeId))throw fail('GRAPH_DUPLICATE_ID');
      const msgId=prefix+stableId('message',conversation.nativeId+'\0'+message.nativeId+'\0'+message.nodeId);
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
