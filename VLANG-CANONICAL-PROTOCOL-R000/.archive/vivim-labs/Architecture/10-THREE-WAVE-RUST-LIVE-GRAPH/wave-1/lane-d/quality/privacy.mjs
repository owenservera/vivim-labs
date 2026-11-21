// Minimal deterministic *known-pattern* leakage detector; no claim of complete DLP.
const suspects=[/-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----/i,/(?:api[_-]?key|private[_-]?key|password|authorization|bearer\s+)\s*[:=]\s*\S+/i,/(?:\b(?:storage-state\.json|session\.json|login data|cookies)\b|(?:^|[\/\s])\.env(?:\.|\b))/i,/[A-Z]:\\Users\\/i,/(?:^|\/)Users\/[^\/\s]+\//i];
export function inspectPrivacy(pages){
 if(!(pages instanceof Map))throw Error('pages_map_required');
 const defects=[];
 for(const [path,content] of pages){
  if(typeof path!=='string'||typeof content!=='string')throw Error('invalid_page');
  const hit=suspects.findIndex(r=>r.test(path)||r.test(content));
  if(hit>=0)defects.push({code:'sensitive_pattern_or_dev_path',page:path,patternIndex:hit});
 }
 return {schema:'vivim.wave1.lane-d.privacy/1',scanned:pages.size,defects,completeDlp:false,productSafe: false};
}
