#!/usr/bin/env node
import { readFile,writeFile } from 'node:fs/promises';
import { dirname,resolve,basename } from 'node:path';
import { fileURLToPath } from 'node:url';
import { chooseRoles,renderTemplate } from './model-router.mjs';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const x=process.argv.slice(2), opts={};
for(let i=0;i<x.length;i+=2){if(!['--catalog','--provider-id','--out'].includes(x[i])||!x[i+1]){console.error('Usage: --catalog .local-models.json --provider-id openrouter --out .local-closed-loop.workflow.ts');process.exit(2);}opts[x[i].slice(2)]=x[i+1];}
if(!opts.catalog||!opts['provider-id']||!opts.out){console.error('Missing required flag');process.exit(2);}
const checkedPath=p=>{const v=resolve(root,p);if(v===root||!(v.startsWith(root+'/')||v.startsWith(root+'\\'))) throw new Error('Path outside kit');return v;};
try {
 const catalog=JSON.parse(await readFile(checkedPath(opts.catalog),'utf8'));
 if(catalog.source!=='https://openrouter.ai/api/v1/models'||!Array.isArray(catalog.data)) throw new Error('Not an OpenRouter catalog snapshot');
 if(!Number.isFinite(Date.parse(catalog.checkedAt))||Date.now()-Date.parse(catalog.checkedAt)>86400000 || Date.parse(catalog.checkedAt)>Date.now()+60000) throw new Error('Catalog older than 24h or future dated');
 const roles=chooseRoles(catalog.data);
 const source=await readFile(resolve(root,'workflows/closed-loop-development.workflow.ts.txt'),'utf8');
 const script=renderTemplate(source,roles,opts['provider-id']);
 await writeFile(checkedPath(opts.out),script,{flag:'w'});
 console.log(JSON.stringify({status:'GENERATED_NOT_ZCODE_VALIDATED',file:basename(opts.out),roles,zcodeProviderId:opts['provider-id'],catalogAt:catalog.checkedAt},null,2));
} catch(e){console.error('REFUSED: '+e.message);process.exitCode=3;}