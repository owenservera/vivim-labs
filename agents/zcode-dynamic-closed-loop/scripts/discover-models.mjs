#!/usr/bin/env node
// Fetch read-only public model catalog. Does not call a generation endpoint or access API keys.
import { writeFile, mkdir } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { chooseRoles, freeTextModel } from './model-router.mjs';
const here=dirname(fileURLToPath(import.meta.url));
const root=resolve(here,'..');
const args=process.argv.slice(2);
const usage='node scripts/discover-models.mjs [--out .local-models.json]';
if(args.includes('--help')) { console.log(usage); process.exit(0); }
if(args.length && (args.length!==2 || args[0]!=='--out')) { console.error(usage); process.exit(2); }
const target=args.length?resolve(root,args[1]):null;
if(target && !(target===root || target.startsWith(root+'/') || target.startsWith(root+'\\'))) { console.error('Output must stay inside package'); process.exit(2); }
try {
 const r=await fetch('https://openrouter.ai/api/v1/models',{headers:{Accept:'application/json'},signal:AbortSignal.timeout(20000)});
 if(!r.ok) throw new Error(`Catalog HTTP ${r.status}`);
 const body=await r.json();
 if(!Array.isArray(body.data)) throw new Error('Malformed catalog');
 const free=body.data.filter(freeTextModel);
 const result={source:'https://openrouter.ai/api/v1/models',checkedAt:new Date().toISOString(),data:free};
 const selected=chooseRoles(free);
 if(target){await mkdir(dirname(target),{recursive:true});await writeFile(target,JSON.stringify(result,null,2)+'\n',{flag:'w'});}
 console.log(JSON.stringify({checkedAt:result.checkedAt,total:body.data.length,strictFreeText:free.length,roles:selected,output:target},null,2));
} catch(e) { console.error('CATALOG_UNAVAILABLE / NO_PAID_FALLBACK: '+e.message); process.exitCode=3; }