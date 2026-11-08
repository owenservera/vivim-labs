#!/usr/bin/env node
/** Read-only bounded Chronos projector for an existing supervisor ledger. */
import {readFileSync,statSync} from 'node:fs';
import {resolve} from 'node:path';
import {projectSupervisorChronos} from '../lib/chronos/supervisor-timeline.mjs';
const args=process.argv.slice(2), idx=args.indexOf('--file');
if(idx<0||!args[idx+1]||args.length!==2){
  console.error('usage: node hq/bin/chronos-inspect.mjs --file <existing-supervisor-events.jsonl>');
  process.exit(2);
}
try{
  const path=resolve(args[idx+1]);
  if(statSync(path).size>4*1024*1024)throw new Error('CHRONOS_LEDGER_SIZE_LIMIT');
  const lines=readFileSync(path,'utf8').split('\n');
  if(lines.at(-1)==='')lines.pop();
  if(lines.length>10000)throw new Error('CHRONOS_LEDGER_EVENT_LIMIT');
  const events=lines.map(line=>JSON.parse(line));
  console.log(JSON.stringify(projectSupervisorChronos(events,{asOfMs:Date.now()})));
}catch(e){console.error(`chronos-inspect: ${e.code??e.message}`);process.exit(2);}
