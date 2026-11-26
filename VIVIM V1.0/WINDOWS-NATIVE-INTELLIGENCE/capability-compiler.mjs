// Labs-only compiler for candidate schemas; no execution or permission grants.
// Current incumbent Host planner accepts string and enum, not numeric args.
// All generated catalog rows remain adapter_pending; no live tool registration.
const fail=code=>{throw Error(code)};
const plain=x=>x!==null&&typeof x==="object"&&!Array.isArray(x)&&[Object.prototype,null].includes(Object.getPrototypeOf(x));
const exact=(x,ks)=>plain(x)&&Object.keys(x).length===ks.length&&ks.every(k=>Object.hasOwn(x,k));
const id=s=>typeof s==="string"&&/^[a-z][a-z0-9._-]{2,95}$/.test(s);
const arg=s=>typeof s==="string"&&/^[a-z][a-z0-9_]{0,63}$/.test(s)&&!["constructor","prototype","__proto__"].includes(s);
const hex=s=>typeof s==="string"&&/^[a-f0-9]{32}$/.test(s);
function field(key,f){
 if(!arg(key)||!plain(f)||!Number.isSafeInteger(f.min)||!Number.isSafeInteger(f.max)||f.min<1||f.max<f.min||f.max>8000)fail("INVALID_FIELD");
 if(key==="enabled")return {host:{type:"enum",values:["enabled","disabled"]},mcp:{type:"boolean"},kind:"boolean"};
 if(key==="drive")return {host:{type:"enum",values:Array.from({length:26},(_,i)=>String.fromCharCode(65+i)+":")},mcp:{type:"string",pattern:"^[A-Za-z]:$"},kind:"drive"};
 if(f.type==="integer")return {host:{type:"string",min:String(f.min).length,max:String(f.max).length},mcp:{type:"integer",minimum:f.min,maximum:f.max},kind:"integer",min:f.min,max:f.max};
 if(f.type!=="string")fail("INVALID_TYPE");
 return {host:{type:"string",min:f.min,max:f.max},mcp:{type:"string",minLength:f.min,maxLength:f.max},kind:"string",min:f.min,max:f.max};
}
export function compile(manifest){
 if(manifest?.schema!=="vivim.labs.windows-extraction-pack/1"||!Array.isArray(manifest.descriptors)||manifest.descriptors.length<1||manifest.descriptors.length>128)fail("INVALID_MANIFEST");
 const ids=new Set(),tools=new Set();
 return manifest.descriptors.map(d=>{
  if(!plain(d)||!id(d.id)||!d.id.startsWith("windows.")||d.semanticId!==d.id+"@1"||!/^vivim_windows_[a-z0-9_]+$/.test(d.mcp)||ids.has(d.id)||tools.has(d.mcp)||!["R0","R2"].includes(d.risk)||d.requiresLiveGrant!==true||d.readiness!=="proposed"||!plain(d.args)||!Array.isArray(d.surfaces)||!["mcp","text","desktop","mobile","visual"].every(s=>d.surfaces.includes(s)))fail("INVALID_DESCRIPTOR");
  ids.add(d.id);tools.add(d.mcp);
  const effect=d.risk==="R2",fields={};
  for(const [k,f]of Object.entries(d.args)){
   // Legacy R001 agent-start request_id duplicates transport idempotency.
   // R007 reserves it for transport only; no duplicate semantic Host arg.
   if(k==="request_id"){if(!effect||d.id!=="windows.agent.session.start")fail("RESERVED_FIELD");continue;}
   fields[k]=field(k,f);
  }
  const names=Object.keys(fields);
  const hostArgs=Object.fromEntries(names.map(k=>[k,fields[k].host]));
  const props=Object.fromEntries(names.map(k=>[k,fields[k].mcp]));
  if(effect)props.request_id={type:"string",pattern:"^[a-f0-9]{32}$"};
  const host={id:d.id,title:d.id.split(".").slice(1).join(" "),category:"workstation",owner:"host-control",readiness:"adapter_pending",risk:d.risk,target:"local-workstation",surfaces:["text","desktop","mobile","visual"],privacy:"authenticated-pc-owner-only",aiCandidate:false,args:hostArgs,examples:[`Inspect proposed capability ${d.id}`],reversible:false,subject:"windows-native",priority:"P0",precondition:"Candidate adapter pending; current Host grant and exact effect approval required."};
  return {id:d.id,semanticId:d.semanticId,package:d.package,fields,host,mcp:{name:d.mcp,inputSchema:{type:"object",properties:props,required:Object.keys(props),additionalProperties:false},registered:false},app:{actionId:d.id,args:hostArgs,enabled:false,desktopOwnerOnly:true,mobileAvailable:false},authority:{executionAuthorized:false,requiresLiveGrant:true,requiresEffectApproval:effect},result:d.result};
 });
}
function val(f,x,mode){
 if(f.kind==="boolean"){
  if(mode==="mcp"){if(typeof x!=="boolean")fail("INVALID_BOOLEAN");return x?"enabled":"disabled";}
  if(x!=="enabled"&&x!=="disabled")fail("INVALID_BOOLEAN");return x;
 }
 if(f.kind==="drive"){
  if(typeof x!=="string"||!(mode==="mcp"?/^[A-Za-z]:$/:/^[A-Z]:$/).test(x))fail("INVALID_DRIVE");return x.toUpperCase();
 }
 if(f.kind==="integer"){
  const n=mode==="mcp"?x:typeof x==="string"&&/^[1-9][0-9]*$/.test(x)?Number(x):NaN;
  if(!Number.isSafeInteger(n)||n<f.min||n>f.max)fail("INVALID_INTEGER");
  return String(n);
 }
 if(typeof x!=="string"||x.length<f.min||x.length>f.max||x!==x.trim()||/[\x00-\x1f\x7f]/.test(x))fail("INVALID_STRING");
 return x;
}
export function normalize(row,input,mode){
 if(mode!=="mcp"&&mode!=="app")fail("INVALID_SURFACE");
 const keys=Object.keys(row.fields),effect=row.host.risk==="R2",needsId=mode==="mcp"&&effect;
 if(!exact(input,needsId?[...keys,"request_id"]:keys))fail("INVALID_ARGUMENTS");
 if(needsId&&!hex(input.request_id))fail("INVALID_REQUEST_ID");
 return {actionId:row.id,args:Object.fromEntries(keys.map(k=>[k,val(row.fields[k],input[k],mode==="mcp"?"mcp":"host")])),...(needsId?{transportRequestId:input.request_id}:{}),executionAuthorized:false};
}
