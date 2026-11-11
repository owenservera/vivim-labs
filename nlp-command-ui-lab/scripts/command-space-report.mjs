#!/usr/bin/env node
/** Read-only source census: run in a checkout that contains HQ, Host and TierFlow.
 * Default output is public-safe aggregate metadata. --full prints command descriptors to stdout only.
 */
import { collectInventory } from '../src/bridge.mjs';
const inv=await collectInventory();
const counts={};
for(const a of inv.actions){
 const ns=a.namespace||'unknown';
 counts[ns]??={total:0,readiness:{},languageMetadata:0,typedArguments:0,icons:0,liveVerifiedClaim:0};
 const r=counts[ns];r.total++;
 r.readiness[a.readiness]=(r.readiness[a.readiness]||0)+1;
 if(a.language)r.languageMetadata++;
 if(Object.keys(a.args||{}).length)r.typedArguments++;
 if(a.visual?.icon)r.icons++;
 if(a.liveVerified)r.liveVerifiedClaim++;
}
const files=inv.sources.map(s=>({authority:s.authority,role:s.role,present:s.present,digest:s.digest||null}));
const out={schema:'vivim.labs.command-space-census/1',revision:inv.revision,generatedAt:inv.generatedAt,
 interpretation:'Source-observed commands, not runtime availability, actor permissions, production effect receipts, or proof of platform completeness.',
 registryCounts:counts,sourceEvidence:files,hqInventorySummary:inv.hqSummary,
 coverage:{areas:['Host typed action catalog','HQ action registry','HQ CLI switch roots','TierFlow UiCommandKind'],
  notYetIndexed:['Host/Studio browser tools','Host provider actions outside control catalog','HQ MCP and skills','TierFlow programs','mobile consumer command adapters','historical VIVIM patterns','runtime permissions and receipts'],
  completeness:'unknown'},
 gaps:inv.gaps.map(x=>({code:x.code,message:x.message}))};
if(process.argv.includes('--full'))out.commands=inv.actions.map(x=>({namespace:x.namespace,id:x.id,title:x.title,category:x.category,readiness:x.readiness,risk:x.risk,
  visual:x.visual,examples:x.examples,typedArgumentNames:Object.keys(x.args||{})}));
process.stdout.write(JSON.stringify(out,null,2)+'\n');