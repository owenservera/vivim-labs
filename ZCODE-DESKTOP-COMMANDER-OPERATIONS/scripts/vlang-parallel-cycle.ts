// VIVIM Labs VLANG parallel regression launcher, Bun/TypeScript.
// No Host calls, browser calls, provider sends, source writes, retries, or test fixtures.
// It creates only local evidence metadata. ZCode model scouts are a separate gated DWF.
import { spawn } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";

type Lane = { id: string; files: string[] };
type Result = { lane: string; status: "PASS"|"FAIL"|"BLOCKED"; exitCode: number|null;
  tests: number|null; pass: number|null; fail: number|null; skipped: number|null;
  durationMs: number; error?: string };
const root=resolve(process.env.VIVIM_ARBOR_ROOT ?? "C:/0-BlackBoxProject-0/ARBOR");
const evidence=resolve(import.meta.dirname,"../local-logs/vlang-iterations");
const force=process.argv.includes("--force");
const runOnce=process.argv.includes("--once") || !process.argv.includes("--watch");
const pollArg=process.argv.find(x=>x.startsWith("--poll-minutes="));
const maxArg=process.argv.find(x=>x.startsWith("--max-cycles="));
const intervalMs=Math.max(5,Math.min(120,Number(pollArg?.split("=")[1]??30)))*60000;
const maxCycles=Math.max(1,Math.min(12,Number(maxArg?.split("=")[1]??8)));
const lanes:Lane[]=[
 {id:"Z-VLANG", files:[
  "apps/vivim-host/test/vlang-command-compilation.test.mjs",
  "apps/vivim-host/test/control-intent-planner.test.mjs",
  "apps/vivim-host/test/command-intent-native.test.mjs"]},
 {id:"Z-SAFETY", files:[
  "apps/vivim-host/test/command-intent-v01.test.mjs",
  "apps/vivim-host/test/command-context-grounding.test.mjs",
  "apps/vivim-host/test/command-program.test.mjs"]},
 {id:"Z-AUTOWIKI", files:[
  "apps/mobile-relay/test/wiki-live-context.test.mjs",
  "apps/vivim-host/test/wiki-revision-projection.test.mjs"]},
 {id:"Z-VISUAL", files:[
  "apps/mobile-relay/test/composer-assist.test.mjs",
  "apps/mobile-relay/test/first-customer.test.mjs",
  "apps/vivim-host/test/chat-command-surface.test.mjs"]}
];
async function command(exe:string,args:string[],timeoutMs=65000):Promise<{exitCode:number|null;stdout:string;stderr:string;durationMs:number}>{
 const start=Date.now();
 return await new Promise((done)=>{
  const p=spawn(exe,args,{cwd:root,shell:false,windowsHide:true,env:{...process.env,NO_COLOR:"1"}});
  let stdout="",stderr="",completed=false;
  const stop=(code:number|null, error="")=>{
   if(completed)return;completed=true;clearTimeout(timer);
   done({exitCode:code,stdout:stdout.slice(-4800),stderr:(stderr+" "+error).slice(-2200),durationMs:Date.now()-start});
  };
  const timer=setTimeout(()=>{try{p.kill()}catch{};stop(null,"TIMEOUT")},timeoutMs);
  p.stdout?.on("data",c=>{stdout+=String(c);if(stdout.length>150000)stdout=stdout.slice(-8000)});
  p.stderr?.on("data",c=>{stderr+=String(c);if(stderr.length>100000)stderr=stderr.slice(-8000)});
  p.on("error",e=>stop(null,e.code??"PROCESS_ERROR"));
  p.on("close",code=>stop(code));
 });
}
function metric(output:string,key:string):number|null{
 const match= new RegExp("(?:^|\\n)\\s*(?:#|ℹ)\\s*"+key+"\\s+(\\d+)","m").exec(output);
 return match?Number(match[1]):null;
}
async function auditLane(lane:Lane):Promise<Result>{
 for(const file of lane.files)if(!existsSync(join(root,file)))
  return {lane:lane.id,status:"BLOCKED",exitCode:null,tests:null,pass:null,fail:null,skipped:null,durationMs:0,error:"MISSING_TEST "+file};
 // The 160-test safety lane takes ~44s alone on Windows and can exceed 65s
 // under four-lane contention. Still fail closed if the bounded 120s expires.
 const timeoutMs=lane.id==="Z-SAFETY"?120000:65000;
 const outcome=await command("node",["--test",...lane.files],timeoutMs);
 const output=outcome.stdout+"\n"+outcome.stderr;
 const tests=metric(output,"tests"),pass=metric(output,"pass"),fail=metric(output,"fail"),skipped=metric(output,"skipped");
 const ok=outcome.exitCode===0&&tests!==null&&tests>0&&pass!==null&&pass>0&&fail===0;
 return {lane:lane.id,status:ok?"PASS":"FAIL",exitCode:outcome.exitCode,
  tests,pass,fail,skipped,durationMs:outcome.durationMs,...(ok?{}:{error:output.slice(-400)})};
}
async function gitValue(args:string[]):Promise<string|null>{
 const r=await command("git",args,12000);return r.exitCode===0?r.stdout.trim():null;
}
function record(name:string, report:unknown){
 mkdirSync(evidence,{recursive:true});
 const body=JSON.stringify(report,null,2)+"\n";
 writeFileSync(join(evidence,name),body,{encoding:"utf8"});
}
async function oneCycle(){
 const sha=await gitValue(["rev-parse","HEAD"]);
 const upstream=await gitValue(["rev-parse","refs/remotes/origin/main"]);
 const dirty=await gitValue(["status","--porcelain","--untracked-files=no"]);
 const runId="vlang-"+new Date().toISOString().replace(/[:.]/g,"-");
 if(!sha||!/^[a-f0-9]{40}$/.test(sha)||sha!==upstream||dirty===null||dirty!==""){
  const report={schema:"vivim.vlang-scout-cycle/1",runId,state:"BLOCKED_SOURCE",sha,upstream,dirty:dirty===null?"unknown":Boolean(dirty),
   note:"No source/tests run on unpinned or dirty ARBOR main"};
  record("latest.json",report);console.log(JSON.stringify(report));return 2;
 }
 const prior=join(evidence,"latest.json");
 if(!force&&existsSync(prior)){
  try {const old=JSON.parse(readFileSync(prior,"utf8"));
   if(old.sha===sha&&old.state==="PASS"){console.log("UNCHANGED_VERIFIED "+sha);return 0;}
  }catch{}
 }
 console.log("PINNED_SHA="+sha+" PARALLEL_LANES="+lanes.length);
 const results=await Promise.all(lanes.map(auditLane));
 const state=results.every(x=>x.status==="PASS")?"PASS": "FAIL";
 const report={schema:"vivim.vlang-scout-cycle/1",runId,sha,state,lanes:results,
  hostEffects:0,providerSends:0,sourceWrites:0,
  unrun:["Live GPUI physical input","Android PWA cross-device parity","ZCode model scouts unless separately launched and verified"],
  next:"Submit one source-pinned reproducible defect to ChatGPT primary coder; never auto-patch parser or Host effects"};
 record(runId+".json",report);record("latest.json",report);
 for(const r of results)console.log(r.lane+" "+r.status+" "+r.pass+"/"+r.tests+" fail="+r.fail+" exit="+r.exitCode);
 console.log("VERDICT="+state+" RECEIPT="+join(evidence,runId+".json"));
 return state==="PASS"?0:3;
}
const nap=(ms:number)=>new Promise<void>(r=>setTimeout(r,ms));
let outcome=0;
for(let i=0;i<(runOnce?1:maxCycles);i++){
 outcome=await oneCycle();
 if(outcome!==0||runOnce||i===maxCycles-1)break;
 await nap(intervalMs);
}
process.exitCode=outcome;
