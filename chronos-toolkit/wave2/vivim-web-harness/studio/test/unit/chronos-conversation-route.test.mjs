import {test} from 'node:test';
import assert from 'node:assert/strict';
import {createConversationApi} from '../../src/studio/conversation-api.mjs';
const at=ms=>new Date(1700000000000+ms).toISOString();
function mockRes(req){
  const res={req,headers:{},status:null,body:Buffer.alloc(0),destroyed:false,writableEnded:false};
  res.setHeader=(name,value)=>{res.headers[name]=value;};
  res.writeHead=(status,headers)=>{res.status=status;Object.assign(res.headers,headers);};
  res.end=(body)=>{if(body)res.body=Buffer.from(body);res.writableEnded=true;};
  return res;
}

test('GET timeline reuses authenticated conversation API read path, never submits/sends',async()=>{
  let reads=0, effects=0;
  const runtime={async get(id){reads++;assert.equal(id,'conv-1');return {
    id,revision:2,createdAt:at(0),updatedAt:at(1000),state:'ready',
    turns:[{id:'turn-1',createdAt:at(10),updatedAt:at(900),phase:'ready',prompt:'PRIVATE'}]};},
    async submit(){effects++;throw new Error('MUST_NOT_SUBMIT');},
    async connect(){effects++;throw new Error('MUST_NOT_CONNECT');}};
  const api=createConversationApi({runtime});
  const req={method:'GET',headers:{}},res=mockRes(req);
  const handled=await api.handle(req,res,new URL('http://localhost/api/v1/conversations/conv-1/timeline'));
  assert.equal(handled,true);assert.equal(res.status,200);
  const body=JSON.parse(res.body.toString('utf8'));
  assert.equal(body.timeline.schema,'vivim.chronos.conversation-read/1');
  assert.equal(body.timeline.createdAtMs,1700000000000);
  assert.equal(body.timeline.turns.length,1);
  assert.equal(JSON.stringify(body).includes('PRIVATE'),false);
  assert.equal(reads,1);assert.equal(effects,0);
});
test('POST timeline is denied before any runtime operation',async()=>{
  let calls=0;
  const api=createConversationApi({runtime:{async get(){calls++;throw new Error('GET SHOULD NOT RUN');}}});
  const req={method:'POST',headers:{}},res=mockRes(req);
  await api.handle(req,res,new URL('http://localhost/api/v1/conversations/conv-1/timeline'));
  assert.equal(res.status,405);assert.equal(calls,0);
});
