import {test} from 'node:test';import assert from 'node:assert/strict';
import {projectProductFixture} from '../quality/product.mjs';
test('unknown/dev-only fields are dropped',()=>{
 const r=projectProductFixture({graphId:'hqg1:abc',safeTitle:'Read-only',privateUserEmail:'someone@example.test',source_path:'C:\\Users\\owner\\.hq',handler:'rm -rf /'});
 assert(!('source_path'in r));assert(!('handler'in r));assert.equal(r.actionExecutableVerified,false);
});
test('dev path in safeSummary refuses instead of leaking',()=>assert.throws(()=>projectProductFixture({safeSummary:'C:\\Users\\owner\\.hq'}),/unsafe_product_field/));
test('product projection cannot inherit execution grant from input',()=>{
 const x=projectProductFixture({safeTitle:'Status',actionExecutableVerified:true,ownerAuthority:'Host'});assert.equal(x.actionExecutableVerified,false);assert.equal(x.ownerAuthority,'NO_GRANT');
});
test('secret field values in approved text refuse',()=>assert.throws(()=>projectProductFixture({safeTitle:'password: abc'}),/unsafe_product_field/));
test('malformed oversized unknown list refuses',()=>assert.throws(()=>projectProductFixture({unknowns:Array(51).fill('a')}),/unsafe_unknowns/));
