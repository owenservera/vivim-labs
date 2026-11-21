const SECRET_PATH=/(^|\/)(\.env[^/]*|cookies|login data|session\.json|storage-state\.json|release-profile\.json|[^/]+\.(pem|p12|keystore))(\/|$)/i;
const SENSITIVE_KEY=/(password|credential|api.?key|private.?key|authorization|cookie|session.?data|access.?token)/i;
const DEV_MARKERS=[/\.hq\/wave1-index-runs/i,/\/fixtures\//i,/\/research\/rust-knowledge-graph\//i,/dev-only/i];
export function inspectProfileArtifacts(artifacts,{profile='PRODUCT'}={}){
 const findings=[];
 for(const a of artifacts??[]){
  const path=String(a.path??'').replaceAll('\\','/');
  if(SECRET_PATH.test(path))findings.push({id:a.id,code:'SECRET_PATH'});
  if(profile==='PRODUCT'&&DEV_MARKERS.some(re=>re.test('/'+path)))findings.push({id:a.id,code:'DEV_ARTIFACT_IN_PRODUCT'});
  if(a.containsPersonalData===true&&!a.consentSourceIdentity) findings.push({id:a.id,code:'PERSONAL_SOURCE_IDENTITY_UNVERIFIED'});
  if(a.effectPermissionGranted===true||a.graphMayExecute===true)findings.push({id:a.id,code:'GRAPH_AUTHORITY_ESCALATION'});
  for(const key of Object.keys(a.metadata??{}))if(SENSITIVE_KEY.test(key))findings.push({id:a.id,code:'SENSITIVE_METADATA_KEY'});
  if(a.privateKB===true&&profile==='PRODUCT'&&a.ownerScopeVerified!==true)findings.push({id:a.id,code:'PRIVATE_KB_RELEASE_SCOPE_MISSING'});
 }
 return {status:findings.length?'BLOCKED':'STATIC_PROFILE_SHAPE_NOT_DEVICE_VERIFIED',findings};
}
