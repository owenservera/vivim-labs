import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const root = dirname(fileURLToPath(import.meta.url));
const doc = JSON.parse(readFileSync(join(root,"manifest.json"),"utf8")) as {
 schema:string; autostart:boolean; ownsScheduler:boolean; maxInitialAdmittedReadOnlyWorkers:number;
 lanes:Array<{id:string;folder:string;execution:string;workflowRefs:string[]}>
};
const problems:string[]=[];
if(doc.schema!=="vivim.five-lane-operation/1") problems.push("unexpected schema");
if(doc.autostart || doc.ownsScheduler) problems.push("unapproved autostart or scheduler ownership");
if(doc.maxInitialAdmittedReadOnlyWorkers>2) problems.push("model concurrency exceeds initial safe policy");
if(doc.lanes.length!==5 || new Set(doc.lanes.map(x=>x.id)).size!==5) problems.push("expected five unique lanes");
for(const f of ["README.md","AGENTS.md","DIRECTIVE.md","LANE-BOARD.md"]) if(!existsSync(join(root,f))) problems.push("missing "+f);
for(const lane of doc.lanes){
 if(!/^L0[1-5]$/.test(lane.id) || lane.execution!=="READY_NOT_LAUNCHED") problems.push("invalid lane state "+lane.id);
 for(const f of ["AGENTS.md","START-PROMPT.md","HANDOFF.md"]) if(!existsSync(join(root,lane.folder,f))) problems.push("missing "+lane.folder+"/"+f);
 if(lane.workflowRefs.length===0 || lane.workflowRefs.some(p=>!p.startsWith(".zcode/workflows/"))) problems.push("missing existing DWF references "+lane.id);
}
console.log(JSON.stringify({schema:doc.schema,lanes:doc.lanes.map(x=>x.id),filesChecked:4+3*doc.lanes.length+1,autostart:doc.autostart,issues:problems},null,2));
if(problems.length) process.exitCode=1;
