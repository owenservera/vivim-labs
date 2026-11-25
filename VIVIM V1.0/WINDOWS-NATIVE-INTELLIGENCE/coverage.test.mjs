import {readFileSync} from "node:fs";
import {projectCandidates,coverageSummary} from "./coverage.mjs";
function runChecks(projectCandidates,coverageSummary,manifest){
 const assert=(ok,label)=>{if(!ok)throw Error(label)};
 const rows=projectCandidates(manifest),n=manifest.descriptors.length;
 assert(rows.length===n,"count");
 assert(coverageSummary(rows).mcpMapped===n&&coverageSummary(rows).appMapped===n,"parity");
 assert(rows.every(r=>!r.executionAuthorized&&r.coverage==="proposed"),"no implied grants");
 const d=manifest.descriptors[0];
 const partial=projectCandidates(manifest,{[d.id]:{hostActionId:d.id,dispatcher:true,nativeAdapter:true,schemaMatch:true,mcpTool:d.mcp,sourceEvidence:true}});
 assert(partial[0].coverage==="proposed","missing GPUI");
 const complete=projectCandidates(manifest,{[d.id]:{hostActionId:d.id,dispatcher:true,nativeAdapter:true,schemaMatch:true,mcpTool:d.mcp,gpuiActionId:d.id,sourceEvidence:true}});
 assert(complete[0].coverage==="source_wiring_evidence_declared"&&!complete[0].executionAuthorized,"evidence not grant");
 for(const broken of [{...manifest,descriptors:[d,d]},{...manifest,descriptors:[{...d,surfaces:["mcp"]}]},{...manifest,descriptors:[{...d,requiresLiveGrant:false}]}]){
  let rejected=false;try{projectCandidates(broken)}catch{rejected=true}assert(rejected,"invalid manifest refused");
 }
 return {checks:8,...coverageSummary(rows)};
}

console.log(JSON.stringify(runChecks(projectCandidates,coverageSummary,JSON.parse(readFileSync(new URL("./CAPABILITY-CANDIDATES.json",import.meta.url),"utf8")))));
