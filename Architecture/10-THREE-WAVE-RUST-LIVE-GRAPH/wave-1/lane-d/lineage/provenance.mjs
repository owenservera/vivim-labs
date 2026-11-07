import {safePath} from '../contracts/validate-inputs.mjs';
import {createHash} from 'node:crypto';
const COMMIT=/^[a-f0-9]{40}$/; const CONTENT=/^[a-f0-9]{64}$/;
export function sourceProvenance(node,roots=[]) {
 const m=node?.metadata??{};
 const defects=[];
 if(!safePath(m.source_path)) defects.push('source_path_untrusted');
 if(!COMMIT.test(m.source_commit??'')) defects.push('git_commit_unpinned');
 if(!CONTENT.test(m.content_sha256??'')) defects.push('content_bytes_unpinned');
 if(!Number.isSafeInteger(Number(m.source_line))||Number(m.source_line)<1) defects.push('source_line_unpinned');
 const rootId='hqg1:'+createHash('sha256').update('node\0root:'+m.source_root).digest('hex');
 const root=roots.find(r=>r.id===rootId);
 if(!root) defects.push('root_unknown');
 else {
   if(root.metadata?.commit_sha!==m.source_commit) defects.push('root_head_stale');
   if(!safePath(root.metadata?.path)||!(m.source_path===root.metadata.path||m.source_path?.startsWith(root.metadata.path+'/'))) defects.push('path_outside_root');
 }
 return {anchorPinned:!defects.length,bytesIndependentlyVerified:false,sourceCommit:m.source_commit??null,contentSha256:m.content_sha256??null,root:m.source_root??null,defects};
}
export function edgeState(edge){
  const m=edge?.metadata??{};
  const supported=['static','observed','declared','verified','source-scoped'];
  const claim=supported.includes(m.evidence_class)?m.evidence_class:'unknown';
  return {claim,evidence:typeof m.evidence==='string'?m.evidence:null,sourceAssertionOnly:true,independentlyVerified:false};
}
