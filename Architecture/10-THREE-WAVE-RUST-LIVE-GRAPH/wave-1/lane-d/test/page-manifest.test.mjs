import {test} from 'node:test';import assert from 'node:assert/strict';import {createHash} from 'node:crypto';
import {checkPageManifest} from '../quality/page-manifest.mjs';
const h=x=>createHash('sha256').update(x).digest('hex');
const p=new Map([['page.md','# Graph\n']]);const digest=h('graph'),id='hqg1:'+h('node');
const original=()=>({schema:'vivim.wave1.hq-page-manifest/PROPOSED',graphDigest:digest,pages:[{path:'page.md',graphIds:[id],contentSha256:h('# Graph\n')}]});
test('missing writer receipt blocks',()=>assert.equal(checkPageManifest({pages:p,exchangeDigest:digest}).status,'BLOCKED_WRITER_MANIFEST_ABSENT'));
test('a content-hash-consistent claimed page remains UNVERIFIED',()=>{
 const x=checkPageManifest({pages:p,exchangeDigest:digest,manifest:original()});assert.equal(x.valid,1);assert.equal(x.verified,0);assert.equal(x.writerIndependentlyVerified,false);
});
test('stale graph digest refuses',()=>{
 const x=checkPageManifest({pages:p,exchangeDigest:h('other'),manifest:original()});assert(x.defects.some(x=>x.code==='graph_digest_mismatch'));
});
test('page content tamper refuses',()=>{
 const x=checkPageManifest({pages:new Map([['page.md','# Forged\n']]),exchangeDigest:digest,manifest:original()});assert(x.defects.some(x=>x.code==='page_content_digest_mismatch'));
});
test('manifest wrong graph entity ID refuses',()=>{
 const m=original();m.pages[0].graphIds=['an-alias'];assert(checkPageManifest({pages:p,exchangeDigest:digest,manifest:m}).defects.some(x=>x.code==='invalid_page_entity_link'));
});
test('path traversal in page manifest refuses',()=>{
 const m=original();m.pages[0].path='../secret.md';assert(checkPageManifest({pages:p,exchangeDigest:digest,manifest:m}).defects.some(x=>x.code==='invalid_or_duplicate_page_path'));
});
