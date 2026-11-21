#!/usr/bin/env node
import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { buildAtlas, coverageMarkdown, readJsonCapped } from './adapter.mjs';

function argumentsOf(argv) {
  if (argv[0] !== 'import') throw new Error('USAGE: node src/cli.mjs import --bundle PATH --site ID --origin URL --out NEW_DIR [--capture-dir PATH] [--states-dir PATH] [--owned-target --include-labels]');
  const o = {};
  for (let i=1;i<argv.length;i++) {
    const key=argv[i];
    if (key==='--owned-target' || key==='--include-labels') { if(o[key])throw new Error('DUPLICATE_OPTION');o[key]=true;continue; }
    if (!['--bundle','--site','--origin','--out','--capture-dir','--states-dir'].includes(key) || i+1>=argv.length || argv[i+1].startsWith('--') || o[key]) throw new Error('INVALID_OPTION: '+key);
    o[key]=argv[++i];
  }
  for (const k of ['--bundle','--site','--origin','--out']) if (!o[k]) throw new Error('MISSING_OPTION: '+k);
  return o;
}
try {
  const a=argumentsOf(process.argv.slice(2));
  const src=readJsonCapped(a['--bundle']);
  const atlas=buildAtlas({bundle:src.content,bundleDigest:src.sha256,siteId:a['--site'],origin:a['--origin'],captureDir:a['--capture-dir'],statesDir:a['--states-dir'],ownedTarget:!!a['--owned-target'],includeLabels:!!a['--include-labels']});
  const output=resolve(a['--out']);
  mkdirSync(output,{recursive:false});
  writeFileSync(resolve(output,'atlas.json'),JSON.stringify(atlas,null,2)+'\n',{flag:'wx',mode:0o600});
  writeFileSync(resolve(output,'coverage.md'),coverageMarkdown(atlas),{flag:'wx',mode:0o600});
  process.stdout.write(JSON.stringify({status:'pass',out:output,verified:atlas.source.verifiedRecords,browserRecords:atlas.source.scopedBrowserRecords,controls:atlas.controls.length})+'\n');
} catch(error) {
  process.stderr.write(JSON.stringify({status:'fail',code:error.code||'ADAPTER_FAILED',message:error.message})+'\n');
  process.exitCode=1;
}