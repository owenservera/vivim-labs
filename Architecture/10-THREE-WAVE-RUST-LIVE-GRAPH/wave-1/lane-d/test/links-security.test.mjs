import {test} from 'node:test';import assert from 'node:assert/strict';
import {checkLinks} from '../quality/links.mjs';import {inspectPrivacy} from '../quality/privacy.mjs';
test('relative and anchored markdown paths resolve without web access',()=>{
 const v=checkLinks(new Map([['index.md','# Home\n[Graph](./sub/graph.md#symbols)'],['sub/graph.md','# Symbols\n'] ]));assert.equal(v.checked,1);assert.equal(v.resolved,1);
});
test('broken anchor found even when file exists',()=>{
 const v=checkLinks(new Map([['index.md','[Graph](./other.md#missing)'],['other.md','# Existing']]));assert.equal(v.defects[0].code,'missing_anchor');
});
test('broken link does not disappear from score',()=>{
 const v=checkLinks(new Map([['index.md','[missing](missing.md)']]));assert.equal(v.checked,1);assert.equal(v.resolved,0);assert.equal(v.defects[0].code,'broken_link');
});
test('link traversal beyond wiki root refused',()=>{
 const v=checkLinks(new Map([['index.md','[Oops](../../private.md)']]));assert.equal(v.defects[0].code,'path_escape');
});
test('casefold slug/path collisions produce explicit defect',()=>{
 const v=checkLinks(new Map([['A.md','# A'],['a.md','# B']]));assert(v.defects.some(x=>x.code==='casefold_page_collision'));
});
test('javascript protocol not silently ignored',()=>{
 const v=checkLinks(new Map([['index.md','[Click](javascript:alert)']]));assert(v.defects.some(x=>x.code==='unsafe_link_scheme'));
});
test('missing search.json declared unverified not missing Markdown page',()=>{
 const v=checkLinks(new Map([['index.md','[Search](./search.json)']]));assert(v.defects.some(x=>x.code==='non_markdown_target_unverified'));
});
test('malformed percent encoding reports defect',()=>{
 const v=checkLinks(new Map([['index.md','[A](./%GG.md)']]));assert(v.defects.some(x=>x.code==='invalid_href_encoding'));
});
test('known key and DEV filesystem leak shapes detected (not complete DLP)',()=>{
 const x=inspectPrivacy(new Map([['wiki.md','private_key=abcdefg'],['tools.md','C:\\Users\\Owner\\.hq']]));assert.equal(x.defects.length,2);assert.equal(x.completeDlp,false);assert.equal(x.productSafe,false);
});
test('unsafe page-relative traversal hard fails',()=>assert.throws(()=>checkLinks(new Map([['../bad.md','# bad']])),/unsafe_page_path/));
