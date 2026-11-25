import {readFileSync} from 'node:fs';
import {compile,normalize} from './capability-compiler.mjs';
function checks(compile,normalize,m){
 const ok=(x,l)=>{if(!x)throw Error(l);return l;};const out=[];
 const rows=compile(m),get=id=>rows.find(r=>r.id===id);
 out.push(ok(rows.length===33&&new Set(rows.map(r=>r.id)).size===33,"all 33 unique"));
 out.push(ok(rows.every(r=>r.host.readiness==="adapter_pending"&&!r.mcp.registered&&!r.app.enabled&&!r.authority.executionAuthorized),"all candidate projections disabled"));
 out.push(ok(rows.every(r=>Object.values(r.host.args).every(a=>["string","enum"].includes(a.type))),"all Host schema fields supported"));
 out.push(ok(rows.every(r=>r.app.actionId===r.id&&JSON.stringify(r.app.args)===JSON.stringify(r.host.args)),"app and Host parity"));
 const read=get("windows.filesystem.changes.read"),set=get("windows.filesystem.changes.configure"),start=get("windows.agent.session.start"),drive=get("windows.system.volume.capacity");
 const a=normalize(read,{scope_id:"owned",cursor:"0",limit:20},"mcp"),b=normalize(read,{scope_id:"owned",cursor:"0",limit:"20"},"app");
 out.push(ok(JSON.stringify(a)===JSON.stringify(b),"integer adapter converges on Host representation"));
 const c=normalize(set,{scope_id:"owned",enabled:true,request_id:"a".repeat(32)},"mcp");
 out.push(ok(c.args.enabled==="enabled"&&c.transportRequestId==="a".repeat(32),"boolean conversion and separate receipt key"));
 out.push(ok(!Object.hasOwn(start.host.args,"request_id")&&Object.keys(start.mcp.inputSchema.properties).filter(x=>x==="request_id").length===1,"no duplicate agent start idempotency key"));
 out.push(ok(normalize(drive,{drive:"c:"},"mcp").args.drive==="C:","canonical drive"));
 function reject(fn,l){let refused=false;try{fn()}catch{refused=true;}out.push(ok(refused,l));}
 reject(()=>normalize(read,{scope_id:"owned",cursor:"0",limit:201},"mcp"),"limit overflow refused");
 reject(()=>normalize(read,{scope_id:"owned",cursor:"0",limit:"020"},"app"),"ambiguous numeric text refused");
 reject(()=>normalize(read,{scope_id:"owned",cursor:"0",limit:20,extra:true},"mcp"),"extra argument refused");
 reject(()=>normalize(set,{scope_id:"owned",enabled:true},"mcp"),"missing effect receipt key refused");
 reject(()=>normalize(set,{scope_id:"owned",enabled:"true",request_id:"a".repeat(32)},"mcp"),"ambiguous boolean refused");
 reject(()=>normalize(set,{scope_id:"owned",enabled:true,request_id:"x"},"mcp"),"invalid receipt key refused");
 reject(()=>normalize(drive,{drive:"C:/"},"mcp"),"path in drive argument refused");
 reject(()=>normalize(read,{scope_id:"a\nb",cursor:"0",limit:1},"mcp"),"control characters refused");
 reject(()=>compile({...m,descriptors:[m.descriptors[0],m.descriptors[0]]}),"duplicate capability refused");
 reject(()=>compile({...m,descriptors:[{...m.descriptors[0],requiresLiveGrant:false}]}),"missing grant requirement refused");
 reject(()=>compile({...m,descriptors:[{...m.descriptors[0],args:{constructor:{type:"string",min:1,max:4}}}]}),"prototype-like argument refused");
 return out;
}

const manifest=JSON.parse(readFileSync(new URL('./CAPABILITY-CANDIDATES.json',import.meta.url),'utf8'));
console.log(JSON.stringify({checks:checks(compile,normalize,manifest)}));
