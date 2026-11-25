// Public-safe exact-source overlay replay; no filesystem/effects/authority.
export function replayPack(spec,baseFiles,additionFiles){
 const fail=code=>{throw Error(code)};
 const safe=p=>typeof p==='string'&&p.length>0&&p.length<=512&&!p.startsWith('/')&&!p.includes('\\')&&!p.includes(':')&&p.split('/').every(x=>x&&x!=='.'&&x!=='..');
 const sha=s=>typeof s==='string'&&/^[0-9a-f]{40}$/.test(s);
 if(spec?.schema!=='vivim.labs.exact-overlay/1'||!sha(spec.baseCommit)||!Array.isArray(spec.patches)||!Array.isArray(spec.additions)||spec.patches.length>256||spec.additions.length>256)fail('INVALID_SPEC');
 const paths=new Set(),out=[];
 function reserve(p){if(!safe(p)||paths.has(p.toLowerCase()))fail('PATH_CONFLICT');paths.add(p.toLowerCase());}
 for(const p of spec.patches){
  reserve(p.path);
  const f=Object.hasOwn(baseFiles,p.path)?baseFiles[p.path]:null;
  if(!sha(p.baseBlob)||!f||f.sha!==p.baseBlob||typeof f.content!=='string')fail('BASE_BLOB_DRIFT');
  if(!Array.isArray(p.edits)||!p.edits.length||p.edits.length>128)fail('INVALID_EDITS');
  let s=f.content;
  for(const e of p.edits){
   if(typeof e.from!=='string'||!e.from||typeof e.to!=='string'||s.split(e.from).length!==2)fail('ANCHOR_DRIFT');
   s=s.replace(e.from,()=>e.to);
  }
  out.push({path:p.path,content:s,kind:'replacement'});
 }
 for(const a of spec.additions){
  reserve(a.path);
  if(!safe(a.source)||Object.hasOwn(baseFiles,a.path)||!Object.hasOwn(additionFiles,a.source)||typeof additionFiles[a.source]!=='string')fail('ADD_COLLISION_OR_MISSING_SOURCE');
  out.push({path:a.path,content:additionFiles[a.source],kind:'addition'});
 }
 return out;
}
export function verifyResourceSlice(files){
 const map=new Map(files.map(x=>[x.path,x.content]));
 const get=p=>{if(!map.has(p))throw Error('MISSING_WIRING '+p);return map.get(p)};
 const action='windows.system.resources.read',tool='vivim_windows_system_resources_read';
 const catalog=JSON.parse(get('apps/vivim-host/control/action-catalog.v1.json'));
 const rows=catalog.actions.filter(a=>a.id===action);
 if(rows.length!==1||rows[0].risk!=='R0'||rows[0].readiness!=='source_wired'||Object.keys(rows[0].args).length!==0)throw Error('CATALOG_MISMATCH');
 const required=[
  ['native/host-supervisor/src/bin/vivim-workstation-probe.rs','--native-resources'],
  ['apps/vivim-host/control/native-resource-port.mjs',action],
  ['apps/vivim-host/control/dispatcher.mjs','[NATIVE_RESOURCE_ACTION]'],
  ['tools/mcp/vivim-arbor/server/workstation-tunnel.cjs',tool],
  ['tools/mcp/vivim-arbor/server/release-evidence.mjs','resourceEnabled'],
  ['tools/mcp/vivim-arbor/windows/Install-WorkstationProfile.ps1','EnableNativeResources'],
  ['tools/mcp/vivim-arbor/windows/Build-WorkstationRelease.ps1','native-resource-port.test.mjs'],
  ['native/gpui-desktop/src/control.rs',action],
  ['native/gpui-desktop/src/main.rs','control::read_native_resources()'],
  ['native/gpui-desktop/src/main.rs','dev-native-resources']
 ];
 for(const [p,s]of required)if(!get(p).includes(s))throw Error('WIRING_MARKER_MISSING '+p);
 return {actionId:action,mcpName:tool,markers:required.length,sourceProjectionOnly:true,liveAccepted:false};
}
