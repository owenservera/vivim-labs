import test from 'node:test';import assert from 'node:assert/strict';import {Readable} from 'node:stream';
import {parseJsonObjectArray} from '../src/json-array-stream.mjs';
const parse=async(s,opts)=>{const out=[];for await(const x of parseJsonObjectArray(Readable.from([...Buffer.from(s)].map(b=>Buffer.from([b]))),opts))out.push(x);return out;};
test('streamed UTF-8, escaped quotes and nested structures',async()=>{
 const data=[{x:'ñ😀',nested:{a:[1,'}']},quote:'hi\\"'}, {second:true}];
 assert.deepEqual(await parse(JSON.stringify(data)),data);
});
test('rejects trailing comma, missing close, array items and trailing payload',async()=>{
 for(const bad of ['[{},]','[{"a":1}','[[1]]','[{}]oops'])await assert.rejects(parse(bad));
});
test('strict object count and size bounds',async()=>{
 await assert.rejects(parse('[{},{}]',{maxObjects:1}),{code:'JSON_OBJECT_COUNT_LIMIT'});
 await assert.rejects(parse('[{"long":"'+ 'x'.repeat(200) +'"}]',{maxObjectBytes:128}),{code:'JSON_OBJECT_LIMIT'});
});
