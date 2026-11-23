/** Read-only five-lane GitHub/ZCode observer. Observations are NOT live ChatGPT agent proof. */
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, openSync, closeSync, unlinkSync, statSync, readFileSync, writeFileSync, renameSync, appendFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const LANES = ["L01","L02","L03","L04","L05"];
const CLAIM_PATH = "ZCODE-DESKTOP-COMMANDER-OPERATIONS/TASK-CLAIMS.md";
type Claim = {task:string; owner:string; state:string};
type Receipt = {id:number; at:string; summary:string};
export function summarizeClaims(markdown:string):Record<string,Claim[]> {
  const out:Record<string,Claim[]> = Object.fromEntries(LANES.map(x=>[x,[]]));
  for(const line of markdown.split(/\r?\n/)) {
    if(!line.startsWith("| ") || /^(\| *[-:])/.test(line))continue;
    const f=line.split("|").slice(1,-1).map(s=>s.trim());
    if(f.length<4 || !/\bACTIVE\b|\bCLAIMED\b/.test(f[3]))continue;
    for(const lane of LANES)if(new RegExp("\\b"+lane+"\\b").test(f[1]+" "+f[2])) {
      out[lane].push({task:f[0].replace(/[*\x60]/g,"").slice(0,110),
        owner:f[1].replace(/[*\x60]/g,"").slice(0,110),
        state:f[3].replace(/[*\x60]/g,"").slice(0,120)});
    }
  }
  return out;
}
export function summarizeIssue(body:string):{total:number;closed:number;open:number} {
  let total=0,closed=0;
  for(const m of body.matchAll(/^- \[([ xX])\]\s+\*\*([A-Z]\d{2})\*\*/gm)){
    total++;if(m[1].toLowerCase()==="x")closed++;
  }
  return {total,closed,open:total-closed};
}
export function summarizeReceipts(comments:Array<{id?:number;created_at?:string;body?:string}>):Record<string,Receipt[]> {
  const out:Record<string,Receipt[]>=Object.fromEntries(LANES.map(x=>[x,[]]));
  for(const c of comments)for(const lane of LANES) {
    if(!new RegExp("\\b"+lane+"\\b").test(c.body||""))continue;
    const first=(c.body||"").split("\n").find(x=>x.trim())||"";
    out[lane].push({id:Number(c.id)||0,at:c.created_at||"",
      summary:first.replace(/[|\r]/g," ").slice(0,140)});
  }
  for(const lane of LANES)out[lane]=out[lane].slice(-3);
  return out;
}
function exec(program:string,args:string[],timeout=18000):string {
  return execFileSync(program,args,{encoding:"utf8",timeout,windowsHide:true,
    maxBuffer:4*1024*1024,stdio:["ignore","pipe","pipe"]}).trim();
}
function api(path:string):any{return JSON.parse(exec("gh",["api",path]));}
function readAudit(dir:string):object {
  const p=join(dir,"scripts","zcode-workflow-run-status.ts");
  if(!existsSync(p))return {state:"UNAVAILABLE"};
  try{
    const a=JSON.parse(exec(process.execPath,["--experimental-strip-types","--no-warnings",p]));
    return {state:"AUDITED",rows:a.totals?.total??null,
      recentProcessCandidates:a.counts?.recentProcessCandidates??null,
      orphaned:a.counts?.staleOrOrphaned??null};
  }catch{return {state:"UNVERIFIED"};}
}
function localHead(dir:string):object {
  try{
    const h=exec("git",["-C",dir,"rev-parse","HEAD"],8000);
    const dirty=exec("git",["-C",dir,"status","--porcelain","--untracked-files=no"],8000).length>0;
    return {head:h,dirty};
  }catch{return {head:null,dirty:null};}
}
function atom(path:string,v:unknown):void {
  const tmp=path+"."+process.pid+".tmp";
  writeFileSync(tmp,JSON.stringify(v,null,2)+"\n","utf8");
  renameSync(tmp,path);
}
function main():void {
  const ops=resolve(process.env.VIVIM_OPS_DIR||process.cwd());
  const out=resolve(process.env.VIVIM_WATCH_DIR||join(ops,".private","five-lane-watch"));
  mkdirSync(out,{recursive:true});
  const lock=join(out,"watch.lock");
  let fd:number|undefined;
  try{
    try{fd=openSync(lock,"wx");}
    catch{
      if(existsSync(lock)&&Date.now()-statSync(lock).mtimeMs>120000){
        try{unlinkSync(lock);}catch{}
        fd=openSync(lock,"wx");
      }else{console.log("WATCH_SKIPPED_ALREADY_RUNNING");return;}
    }
    const observedUtc=new Date().toISOString();
    const a=api("repos/owenservera/ARBOR/commits/main");
    const l=api("repos/owenservera/vivim-labs/commits/main");
    const issue=api("repos/owenservera/ARBOR/issues/33");
    const comments=api("repos/owenservera/ARBOR/issues/33/comments?per_page=100");
    const doc=api("repos/owenservera/vivim-labs/contents/"+CLAIM_PATH);
    const claims=Buffer.from(String(doc.content||"").replace(/\s/g,""),"base64").toString("utf8");
    const byLane=summarizeClaims(claims),receipts=summarizeReceipts(comments);
    const state={
      schema:"vivim.five-lane-readonly-watch/1",observedUtc,
      classification:"SOURCE_MONITOR_NOT_LIVE_AGENT_PROOF",
      source:{arborSha:a.sha,labsSha:l.sha,claimsBlobSha:doc.sha},
      master:{...summarizeIssue(issue.body||""),updatedAt:issue.updated_at},
      lanes:Object.fromEntries(LANES.map(x=>[x,{activeClaims:byLane[x],recentIssueReceipts:receipts[x]}])),
      zcode:readAudit(ops),
      localArbor:localHead(resolve(ops,"..","..","ARBOR")),
      commentsObserved:comments.length,
      note:"No remote writes, agent dispatch, model calls, provider sends or Host effects."
    };
    const fingerprint=createHash("sha256").update(JSON.stringify({
      source:state.source,master:state.master,lanes:state.lanes,zcode:state.zcode
    })).digest("hex");
    const latest=join(out,"latest.json");
    const prior=existsSync(latest)?JSON.parse(readFileSync(latest,"utf8")):null;
    atom(latest,{...state,fingerprint,changedSincePrevious:prior?.fingerprint!==fingerprint});
    if(prior?.fingerprint!==fingerprint)appendFileSync(join(out,"changes.jsonl"),
      JSON.stringify({observedUtc,fingerprint,source:state.source,master:state.master,
        laneClaims:Object.fromEntries(LANES.map(x=>[x,byLane[x].length])),
        lastCommentId:comments.at(-1)?.id||null,zcode:state.zcode})+"\n","utf8");
    console.log(JSON.stringify({status:"WATCH_PASS",observedUtc,arborSha:a.sha,labsSha:l.sha,
      open:state.master.open,changed:prior?.fingerprint!==fingerprint,
      activeClaims:Object.fromEntries(LANES.map(x=>[x,byLane[x].length])),zcode:state.zcode}));
  }catch(e:any){
    const error={schema:"vivim.five-lane-watch-error/1",observedUtc:new Date().toISOString(),
      status:"WATCH_FAILED",reason:String(e?.message||e).replace(/gho_[A-Za-z0-9_]+/g,"[redacted]").slice(0,320)};
    atom(join(out,"last-error.json"),error);
    console.error(JSON.stringify(error));process.exitCode=1;
  }finally{
    if(fd!==undefined){closeSync(fd);try{unlinkSync(lock);}catch{}}
  }
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))main();
