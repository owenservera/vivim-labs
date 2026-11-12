/** Read-only bridges to the product authorities. No execution methods are imported or exposed. */
import { readFileSync, existsSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import { decorateActions } from './affordances.mjs';
const LAB_ROOT = resolve(fileURLToPath(new URL('..', import.meta.url)));
const DEFAULT_PARENT = resolve(LAB_ROOT, '../..');
const repoPath = (env, folder) => resolve(process.env[env] || join(process.env.VIVIM_ROOT || DEFAULT_PARENT, folder));
export const paths = Object.freeze({
  host: repoPath('VIVIM_HARNESS', 'vivim-web-harness'),
  hq: repoPath('VIVIM_HQ', 'repos-hq'),
  tierflow: repoPath('VIVIM_TIERFLOW', 'tierflow'),
});
const source = (path) => ({ path, present: existsSync(path) });
const hash = text => createHash('sha256').update(text).digest('hex');
function failure(code, message, path) { return { code, message, path }; }

/** Re-evaluates on each request: the lab does not maintain a shadow canonical registry. */
export async function collectInventory() {
  const out = {schema:'vivim.labs.command-inventory/1', generatedAt:new Date().toISOString(), actions:[], sources:[], gaps:[], hqSummary:null, hostVersion:null};
  const hostFile=join(paths.host,'apps/vivim-host/control/action-catalog.v1.json');
  out.sources.push({...source(hostFile),authority:'host',role:'user-facing typed actions'});
  if(existsSync(hostFile)) {
    try {
      const raw=readFileSync(hostFile,'utf8'); const catalog=JSON.parse(raw);
      if(catalog.schema!=='vivim.control-action-catalog/1'||!Array.isArray(catalog.actions)) throw Error('Unknown Host catalog schema');
      out.hostVersion=catalog.version;
      for(const a of catalog.actions) if(typeof a.id==='string') out.actions.push({
        id:a.id,namespace:'host', title:a.title||a.id,category:a.category||'other',
        readiness:a.readiness||'unknown',risk:a.risk||'unknown',
        examples:Array.isArray(a.examples)?a.examples:[],args:a.args||{},
        language:a.language||null,sourceFile:hostFile, canonical:true,liveVerified:false,declaredVerified:a.readiness==='verified',
        evidence:'Host catalog source record; not a runtime receipt'
      });
      out.sources[0].digest=hash(raw);
    }catch(e){out.gaps.push(failure('HOST_CATALOG_READ',String(e.message),hostFile));}
  } else out.gaps.push(failure('HOST_MISSING','Host catalog unavailable: configure VIVIM_HARNESS',hostFile));

  const hqAdapter=join(paths.hq,'hq/live-graph/wave1/lane-c/registry-adapter.mjs');
  out.sources.push({...source(hqAdapter),authority:'hq',role:'source-verified HQ inventory adapter'});
  if(existsSync(hqAdapter)) {
    try {
      const adapter=await import(pathToFileURL(hqAdapter).href);
      if(typeof adapter.inventory!=='function') throw Error('Missing inventory()');
      const snapshot=adapter.inventory();
      if(snapshot.schema!=='vivim.wave1.command-candidates/1'||!Array.isArray(snapshot.records)) throw Error('HQ inventory schema unsupported');
      out.hqSummary={...snapshot.summary,digest:snapshot.digest,sourceFiles:snapshot.sourceFiles.length};
      for(const r of snapshot.records) out.actions.push({
        id:r.actionId,namespace:'hq',title:r.purpose||r.actionId,category:r.domain,
        readiness:r.evidenceState?.handlerSource||'unknown',risk:r.effects?.risk||'unknown',
        examples:[],args:{},language:null,sourceFile:hqAdapter,canonical:true,liveVerified:false,
        evidence:'HQ source observation; ' + (r.evidenceState?.runtime||'UNTESTED')
      });
      for(const c of snapshot.cli||[]) out.actions.push({
        id:c.id,namespace:'hq-cli',title:c.syntax,category:'hq-cli',readiness:'source_observed',
        risk:'unknown',examples:[],args:{},language:null,sourceFile:hqAdapter,
        canonical:true,liveVerified:false,evidence:'HQ CLI switch source observed; not runtime-verified'
      });
      if(snapshot.summary.orphanHandlerKeys?.length) out.gaps.push(failure('HQ_ORPHAN_HANDLER_KEYS',`${snapshot.summary.orphanHandlerKeys.length} handler keys lack action identity`,hqAdapter));
    }catch(e){out.gaps.push(failure('HQ_ADAPTER_FAILURE',String(e.message),hqAdapter));}
  } else out.gaps.push(failure('HQ_MISSING','HQ inventory adapter missing: set VIVIM_HQ',hqAdapter));
  // TierFlow UI commands belong to its existing typed world protocol, never to Host effects.
  const uiTypes=join(paths.tierflow,'src/ui/types.ts');
  out.sources.push({...source(uiTypes),authority:'tierflow',role:'UI-core typed command vocabulary'});
  if(existsSync(uiTypes)) {
    try {
      const raw=readFileSync(uiTypes,'utf8');
      const block=raw.match(/export\s+type\s+UiCommandKind\s*=([\s\S]*?);/);
      if(!block)throw Error('UiCommandKind type not found');
      const ids=[...new Set([...block[1].matchAll(/["'](ui\.[a-z.]+)["']/g)].map(m=>m[1]))];
      if(ids.length<1||ids.length>60)throw Error('Unexpected UI command vocabulary size');
      out.sources[out.sources.length-1].digest=hash(raw);
      for(const id of ids)out.actions.push({id,namespace:'tierflow-ui',title:id.replace(/^ui\./,'UI · ').replaceAll('.',' / '),
        category:'ui',readiness:'source_declared',risk:'unknown',examples:[],args:{},language:null,
        sourceFile:uiTypes,canonical:true,liveVerified:false,evidence:'TierFlow UI type union only; no authenticated Host/TierFlow execution proof'});
    }catch(e){out.gaps.push(failure('TIERFLOW_UI_PARSE',String(e.message),uiTypes));}
  }else out.gaps.push(failure('TIERFLOW_MISSING','TierFlow typed UI source not mounted; set VIVIM_TIERFLOW',uiTypes));

  if(out.actions.length===0) out.gaps.push(failure('NO_LIVE_REGISTRIES','No local canonical inventories connected; prototype interactions remain exploratory',LAB_ROOT));
  if(out.hostVersion!==null && !out.actions.some(a=>a.category==='email'&&a.namespace==='host'))
    out.gaps.push(failure('EMAIL_VISUAL_GAP','No registered Host email action detected. Email menu is a visual prototype only.',hostFile));
  out.gaps.push(failure('HQ_CROSSWALK_GAP','HQ and Host identifiers are shown side-by-side; no approved equivalent-ID crosswalk was supplied.',hqAdapter));
  out.gaps.push(failure('AVAILABILITY_EVIDENCE_GAP','Source registrations do not prove actor grants, device availability or runtime receipts. Live readiness needs Host evidence.',hostFile));
  out.gaps.push(failure('PLATFORM_COMMAND_COVERAGE','Host catalog + HQ action/CLI inventory + TierFlow typed UI commands are source-observed only. MCP, skills, Studio, pack commands, legacy families and per-actor availability require a reviewed completeness crosswalk.',LAB_ROOT));
  out.gaps.push(failure('CONTACT_BINDING_GAP','Contacts require a scoped read-only Host contact discovery port; demo fixture never authorizes a send.',LAB_ROOT));
  out.actions=decorateActions(out.actions);
  out.counts={all:out.actions.length,host:out.actions.filter(x=>x.namespace==='host').length,hq:out.actions.filter(x=>x.namespace==='hq').length,cli:out.actions.filter(x=>x.namespace==='hq-cli').length,tierflow:out.actions.filter(x=>x.namespace==='tierflow-ui').length,liveVerified:out.actions.filter(x=>x.liveVerified).length};
  const sorted=out.actions.map(x=>[x.namespace,x.id,x.readiness]).sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));
  out.revision=hash(JSON.stringify([out.sources.map(x=>x.digest||x.present),out.hqSummary?.digest,sorted]));
  return out;
}
export async function getHostPlanner() {
  const path=join(paths.host,'apps/vivim-host/control/intent-planner.mjs');
  if(!existsSync(path))return {planner:null,reason:'Host intent planner not on local disk'};
  try {const api=await import(pathToFileURL(path).href); return {planner:api.createControlPlanner({catalog:api.loadControlCatalog()}),reason:null};}
  catch(e){return {planner:null,reason:String(e.message)};}
}
export function getContacts(query='') {
  const path=process.env.LAB_CONTACTS_PATH ? resolve(process.env.LAB_CONTACTS_PATH):join(LAB_ROOT,'fixtures/demo-contacts.json');
  let data;
  try {data=JSON.parse(readFileSync(path,'utf8'));}catch(e){return {origin:'unavailable',contacts:[],error:String(e.message)};}
  const raw=Array.isArray(data.contacts)?data.contacts:[];
  const q=String(query).trim().toLocaleLowerCase();
  const contacts=raw.filter(c=>typeof c.name==='string'&&typeof c.id==='string'&&(c.name.toLocaleLowerCase().includes(q)||String(c.email||'').toLocaleLowerCase().includes(q)))
    .slice(0,12).map(c=>({id:c.id,name:c.name,email:String(c.email||''),verified:false}));
  return {origin:process.env.LAB_CONTACTS_PATH?'local_unverified_read_only':'fictional_demo_fixture',contacts};
}