// Pure parser for USER-SELECTED official ChatGPT conversations.json content.
// Not a ZIP extractor, browser scraper, provider client or private store.
import {createHash} from 'node:crypto';
const hash=v=>createHash('sha256').update(v).digest('hex');
const error=(code)=>Object.assign(new Error(code),{code});
const str=(value,max)=>typeof value==='string'?value.slice(0,max):'';
const timestamp=v=>typeof v==='number'&&Number.isFinite(v)?new Date(v*1000).toISOString():null;
export const CONTENT_REVISION_SCHEMA='vivim.personal-content-revision/2';
const descriptor=value=>{
  if(value===null||typeof value!=='object')return value;
  if(Array.isArray(value))return value.map(descriptor);
  return Object.fromEntries(Object.keys(value).filter(k=>!['data','payload','base64','bytes'].includes(k)).sort().map(k=>[k,descriptor(value[k])]));
};
function textParts(content){
  if(!content||typeof content!=='object')return {text:'',omitted:true,parts:[],descriptors:[]};
  const parts=Array.isArray(content.parts)?content.parts:[];
  let omitted=false;const values=[],descriptors=[],orderedParts=[];
  for(const part of parts){
    if(typeof part==='string'){values.push(part);orderedParts.push(['text',part]);}
    else if(part&&typeof part==='object'&&typeof part.text==='string'){
      values.push(part.text);orderedParts.push(['text',part.text]);
    } else {
      omitted=true;const safe=descriptor(part);descriptors.push(safe);orderedParts.push(['omitted',safe]);
    } // Never retain media payload bytes.
  }
  if(!parts.length)omitted=true;
  return {text:values.join('\n'),omitted,parts:orderedParts,descriptors};
}
export function normalizeChatGptConversations(input,{accountId,limit=1000,maxTextBytes=1_000_000}={}){
  if(typeof accountId!=='string'||!accountId||accountId.length>256)throw error('ACCOUNT_REQUIRED');
  if(!Array.isArray(input)||!Number.isSafeInteger(limit)||limit<1||limit>10000||input.length>limit)throw error('CONVERSATION_LIMIT');
  const conversations=[];
  for(const conversation of input){
    if(!conversation||typeof conversation!=='object'||typeof conversation.id!=='string'||!conversation.id||conversation.id.length>256)throw error('INVALID_CONVERSATION');
    const mapping=conversation.mapping;
    if(!mapping||typeof mapping!=='object'||Array.isArray(mapping))throw error('MAPPING_REQUIRED');
    const messages=[];
    if(Object.keys(mapping).length>200000)throw error('NODE_LIMIT');
    for(const [nodeId,node] of Object.entries(mapping)){
      if(!node||typeof node!=='object'||!node.message)continue;
      const message=node.message;
      const id=typeof message.id==='string'&&message.id?message.id:nodeId;
      if(id.length>256||nodeId.length>256)throw error('INVALID_NODE');
      const {text,omitted,parts,descriptors}=textParts(message.content);
      if(Buffer.byteLength(text,'utf8')>maxTextBytes)throw error('MESSAGE_SIZE_LIMIT');
      const role=str(message.author?.role,32)||'unknown';
      const record={nativeId:id,nodeId,parentId:typeof node.parent==='string'?node.parent:null,
        children:Array.isArray(node.children)?node.children.filter(v=>typeof v==='string').sort():[],
        role,createdAt:timestamp(message.create_time),updatedAt:timestamp(message.update_time),
        contentType:str(message.content?.content_type,64),text,omittedMedia:omitted,
        omissionDescriptors:descriptors,contentParts:parts,
        contentDigest:hash(JSON.stringify([CONTENT_REVISION_SCHEMA,id,nodeId,role,message.content?.content_type??null,parts]))};
      messages.push(record);
    }
    messages.sort((a,b)=>a.nodeId.localeCompare(b.nodeId));
    const title=str(conversation.title,1024),createdAt=timestamp(conversation.create_time),updatedAt=timestamp(conversation.update_time);
    const currentNode=typeof conversation.current_node==='string'?conversation.current_node:null;
    const contentDigest=hash(JSON.stringify([CONTENT_REVISION_SCHEMA,title,currentNode,createdAt,updatedAt,
      messages.map(m=>[m.nativeId,m.nodeId,m.parentId,m.children,m.role,m.createdAt,m.updatedAt,m.contentType,m.contentParts,m.omittedMedia,m.omissionDescriptors])]));
    conversations.push({
      source:'chatgpt-export',accountId,nativeId:conversation.id,title,createdAt,updatedAt,currentNode,
      messages,revisionSchema:CONTENT_REVISION_SCHEMA,contentDigest,
      omissions:messages.filter(m=>m.omittedMedia).length
    });
  }
  return conversations;
}
// Return only bounded metadata intended for coordinator.recordBatch after Host evidence acceptance.
export function batchMetadata(conversations){
  return conversations.map(c=>({nativeId:c.nativeId,kind:'conversation',contentDigest:c.contentDigest}));
}
