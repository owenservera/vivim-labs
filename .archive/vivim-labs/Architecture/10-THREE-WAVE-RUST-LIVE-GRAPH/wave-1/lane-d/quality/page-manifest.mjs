// D-only inspector for a *proposed* HQ-owned page manifest. Never creates wiki pages.
// The writer receipt is still a claim until F independently replays its exact source/graph.
import {createHash} from 'node:crypto';
const sha=s=>createHash('sha256').update(s,'utf8').digest('hex');
const HASH=/^[a-f0-9]{64}$/; const GRAPH=/^hqg1:[a-f0-9]{64}$/;
function safe(p){return typeof p==='string'&&p.endsWith('.md')&&p.length<=260&&!p.startsWith('/')&&!p.includes('\\')&&!p.includes(':')&&!p.split('/').some(x=>x===''||x==='.'||x==='..');}
export function checkPageManifest({pages,manifest,exchangeDigest}={}){
 if(!(pages instanceof Map))throw Error('pages_map_required');
 if(!manifest||manifest.schema!=='vivim.wave1.hq-page-manifest/PROPOSED') return {status:'BLOCKED_WRITER_MANIFEST_ABSENT',valid:0,claimed:0,verified:0,defects:[{code:'missing_approved_writer_manifest'}]};
 const defects=[];
 if(!HASH.test(exchangeDigest??'')||exchangeDigest!==manifest.graphDigest)defects.push({code:'graph_digest_mismatch'});
 if(!Array.isArray(manifest.pages)||manifest.pages.length>10000)throw Error('invalid_page_manifest');
 const seen=new Set();let valid=0;
 for(const row of manifest.pages){
   if(!safe(row?.path)||seen.has(row.path)){defects.push({code:'invalid_or_duplicate_page_path',path:row?.path??null});continue;}seen.add(row.path);
   if(!Array.isArray(row.graphIds)||row.graphIds.length===0||row.graphIds.some(id=>!GRAPH.test(id))){defects.push({code:'invalid_page_entity_link',path:row.path});continue;}
   if(!HASH.test(row.contentSha256??'')){defects.push({code:'missing_page_hash',path:row.path});continue;}
   const text=pages.get(row.path);
   if(typeof text!=='string'){defects.push({code:'page_not_generated',path:row.path});continue;}
   if(sha(text)!==row.contentSha256){defects.push({code:'page_content_digest_mismatch',path:row.path});continue;}
   valid++;
 }
 // Never call it verified: hash agreement is self-consistency, not a native HQ writer or F witness.
 return {schema:'vivim.wave1.lane-d.page-manifest-check/1',status:defects.length?'BLOCKED_MANIFEST_MISMATCH':'CLAIMED_MANIFEST_CONSISTENT_NOT_INDEPENDENT',valid,claimed:manifest.pages.length,verified:0,writerIndependentlyVerified:false,defects};
}
