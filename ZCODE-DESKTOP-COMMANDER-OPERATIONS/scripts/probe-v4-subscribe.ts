/** Read-only V4 sessions-index subscription experiment; no mutation, model call or prompt. */
import { spawn } from "node:child_process";
const node=String.raw`C:\Program Files\nodejs\node.exe`;
const entry=String.raw`C:\Program Files\ZCode\resources\glm\zcode.cjs`;
const child=spawn(node,[entry,"app-server"],{cwd:String.raw`C:\0-BlackBoxProject-0\ARBOR`,windowsHide:true,stdio:["pipe","pipe","pipe"]});
let buffer="", frames=0, notifications=0, ack=false, errors=0, done=false;
const timers:Array<ReturnType<typeof setTimeout>>=[];
const send=(id:string,method:string,params:object)=>child.stdin.write(JSON.stringify({id,method,params})+"\n");
const finish=(tag:string,exitCode:number)=>{
 if(done)return; done=true;for(const t of timers)clearTimeout(t);
 console.log("V4_PROBE_FINAL="+tag+" ack="+ack+" frames="+frames+" notifications="+notifications+" errors="+errors);
 child.stdin.end();child.kill();process.exitCode=exitCode;
};
child.on("error",()=>finish("SPAWN_ERROR",2));
child.on("exit",code=>{if(!done)finish("CHILD_EXIT_"+code,2);});
child.stderr.on("data",()=>{});
child.stdout.on("data",chunk=>{
 buffer+=chunk.toString();
 while(buffer.includes("\n")){
  const at=buffer.indexOf("\n"),line=buffer.slice(0,at).trim();buffer=buffer.slice(at+1);
  if(!line)continue;
  let obj:any;try{obj=JSON.parse(line)}catch{errors++;continue}
  if(obj.method==="v4/conversation/frame"){frames++;if(frames===1)console.log("V4_FRAME_RECEIVED=YES");continue}
  if(obj.method){notifications++;continue}
  if(obj.id==="list"){
   if(obj.error){errors++;console.log("V4_LIST=ERROR");finish("FAILED_LIST",1);return}
   const s=obj.result?.sessions?.[0];const w=s?.workspace;
   const workspaceId=w?.workspaceIdentity??w?.workspaceKey??w?.workspacePath;
   console.log("V4_WORKSPACE_KEYS="+(w&&typeof w==="object"?Object.keys(w).join(","):"none"));
   if(typeof workspaceId!=="string"||!workspaceId){finish("NO_WORKSPACE_ID",3);return}
   send("sub","v4/conversation/subscribe",{topic:"sessions-index/"+workspaceId,visibility:"background",connectionId:"dc-probe-"+process.pid,clientMode:"desktop-continuous"});
   console.log("V4_SUBSCRIBE_SENT=READ_ONLY");
  }else if(obj.id==="sub"){
   if(obj.error){errors++;console.log("V4_SUBSCRIBE_ERROR_CODE="+obj.error.code+" MESSAGE="+String(obj.error.message).slice(0,100));finish("SUBSCRIPTION_REJECTED",1);return}
   ack=!!obj.result?.ack;
   console.log("V4_ACK="+(ack?"PASS":"MISSING")+" mode="+String(obj.result?.ack?.mode??""));
   timers.push(setTimeout(()=>finish("RESPONDED",ack?0:1),2000));
  }
 }
});
timers.push(setTimeout(()=>send("list","session/list",{limit:1}),2000));
timers.push(setTimeout(()=>finish("TIMEOUT",4),32000));
