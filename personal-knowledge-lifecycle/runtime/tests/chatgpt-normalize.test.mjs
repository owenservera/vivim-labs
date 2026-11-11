import test from 'node:test';
import assert from 'node:assert/strict';
import {normalizeChatGptConversations,batchMetadata} from '../src/chatgpt-normalize.mjs';
const sample={id:'thread-a',title:'Research',create_time:1730000000,current_node:'n2',mapping:{
  n1:{parent:null,children:['n2'],message:{id:'m1',author:{role:'user'},content:{content_type:'text',parts:['Hello']},create_time:1730000000}},
  n2:{parent:'n1',children:[],message:{id:'m2',author:{role:'assistant'},content:{content_type:'multimodal_text',parts:['Answer',{asset_pointer:'image'}]}}}
}};
test('preserve branching, metadata, text and omissions',()=>{
  const [c]=normalizeChatGptConversations([sample],{accountId:'qualified-a'});
  assert.equal(c.messages.length,2);
  assert.equal(c.messages[1].parentId,'n1');
  assert.equal(c.messages[1].omittedMedia,true);
  assert.equal(c.omissions,1);
  assert.equal(batchMetadata([c])[0].kind,'conversation');
});
test('reject malformed or unbounded input',()=>{
  assert.throws(()=>normalizeChatGptConversations([sample],{}),{code:'ACCOUNT_REQUIRED'});
  assert.throws(()=>normalizeChatGptConversations([sample],{accountId:'a',limit:0}),{code:'CONVERSATION_LIMIT'});
  assert.throws(()=>normalizeChatGptConversations([{id:'a'}],{accountId:'a'}),{code:'MAPPING_REQUIRED'});
  assert.throws(()=>normalizeChatGptConversations([sample],{accountId:'a',maxTextBytes:1}),{code:'MESSAGE_SIZE_LIMIT'});
});
