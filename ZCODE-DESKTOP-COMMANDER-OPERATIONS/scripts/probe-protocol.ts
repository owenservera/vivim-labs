/** Read-only ZCode native protocol probe. No model calls, session writes or raw payload logging. */
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
const node = String.raw`C:\Program Files\nodejs\node.exe`;
const entry = String.raw`C:\Program Files\ZCode\resources\glm\zcode.cjs`;
const repo = String.raw`C:\0-BlackBoxProject-0\ARBOR`;
const workspace = {workspacePath:repo,workspaceKey:repo};
if (!existsSync(node) || !existsSync(entry)) {
  console.log("PROBE_UNAVAILABLE: installed Node/ZCode entrypoint missing");
  process.exit(3);
}
const child = spawn(node, [entry, "app-server"], { cwd: repo, stdio: ["pipe","pipe","pipe"], windowsHide: true });
const pending = new Set(["sessions","plugins","workflows"]);
let outputBuffer = "";
let notifications = 0;
let stderrBytes = 0;
let errors = 0;
let sent = false;
let gotSessions = false;
let finished = false;
const request = (id: string, method: string, params: object) =>
  child.stdin.write(JSON.stringify({id,method,params})+"\n");
const stop = (status: string, code: number) => {
  if (finished) return;
  finished = true;
  clearTimeout(requestTimer);
  clearTimeout(timeoutTimer);
  console.log("PROBE_FINAL="+status+" failed_requests="+errors+" notifications="+notifications+" stderr_bytes="+stderrBytes);
  child.stdin.end();
  child.kill();
  process.exitCode = code;
};
child.on("error", e=>stop("PROCESS_ERROR_"+e.name,2));
child.on("exit", c=>{ if (!finished) stop("PROCESS_EXIT_"+c,2); });
child.stderr.on("data", (chunk: Buffer)=>{stderrBytes+=chunk.length;});
child.stdout.on("data", (chunk: Buffer)=>{
  outputBuffer+=chunk.toString("utf8");
  while(outputBuffer.includes("\n")){
    const idx=outputBuffer.indexOf("\n");
    const line=outputBuffer.slice(0,idx).trim();
    outputBuffer=outputBuffer.slice(idx+1);
    if(!line) continue;
    let frame:any;
    try {frame=JSON.parse(line);} catch{errors++;console.log("INVALID_NDJSON_FRAME");continue;}
    if(typeof frame.id === "string" && pending.has(frame.id)){
      pending.delete(frame.id);
      if(frame.error){
        errors++;
        console.log("QUERY_"+frame.id+"=FAIL code="+frame.error.code+" reason="+String(frame.error.message).slice(0,100));
      }else{
        const value=frame.result;
        const items=frame.id==="sessions"?value?.sessions:frame.id==="plugins"?value?.plugins:frame.id==="workflows"?value?.workflows:value?.childSessionIds;
        if(frame.id==="sessions")gotSessions=Array.isArray(items);
        console.log("QUERY_"+frame.id+"=PASS count="+(Array.isArray(items)?items.length:"unknown")+
          " result_keys="+(value && typeof value === "object"?Object.keys(value).slice(0,5).join(","):"none"));
        if(frame.id==="sessions" && Array.isArray(items) && items.length){
          const item=items[0];
          console.log("SESSION_SHAPE_KEYS="+Object.keys(item).slice(0,16).join(","));
          if(typeof item.sessionId === "string"){
            pending.add("subagents");
            request("subagents","session/subagents",{sessionId:item.sessionId,endedLimit:5});
            console.log("PROBE_SENT_SUBAGENTS=READ_ONLY");
          }
        }
      }
      if(pending.size===0)stop("RESPONDED",errors===0&&gotSessions?0:1);
    }else{
      notifications++;
      if(notifications<=4) console.log("NOTIFICATION="+String(frame.method??"other").slice(0,75));
    }
  }
});
const requestTimer=setTimeout(()=>{
  if(finished)return;
  sent=true;
  request("sessions","session/list",{limit:5});
  request("plugins","plugins/list",{workspace});
  request("workflows","workflows/list",{workspace});
  console.log("PROBE_SENT=3_READ_ONLY_NATIVE_PROTOCOL_REQUESTS");
},2200);
const timeoutTimer=setTimeout(()=>stop(sent?"TIMEOUT_AFTER_SEND":"TIMEOUT_BEFORE_SEND",4),35000);
