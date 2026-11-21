#!/usr/bin/env node
// Development-only: never executes target code or loads a site profile as JavaScript.
import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { buildAtlas, coverageMarkdown, readJsonCapped } from './adapter.mjs';
import { reconcileAtlas, reconciliationMarkdown } from './reconcile.mjs';
import { diffAtlases, diffMarkdown } from './diff.mjs';
import { runNativeVerification } from './native-verifier.mjs';

const FLAGS=Object.freeze({
  import:new Set(['--bundle','--site','--origin','--out','--capture-dir','--states-dir','--owned-target','--include-labels','--native-bin']),
  reconcile:new Set(['--atlas','--baseline','--out']),
  diff:new Set(['--before','--after','--out']),
});
function argumentsOf(argv) {
  const command=argv[0];
  if(!Object.hasOwn(FLAGS,command))throw new Error('USAGE: import | reconcile | diff; run with --help for usage');
  const options={};
  for(let i=1;i<argv.length;i++) {
    const key=argv[i];
    if(!FLAGS[command].has(key)||options[key]!==undefined)throw new Error('INVALID_OPTION: '+key);
    if(key==='--owned-target'||key==='--include-labels'){options[key]=true;continue;}
    if(i+1>=argv.length||argv[i+1].startsWith('--'))throw new Error('MISSING_OPTION_VALUE: '+key);
    options[key]=argv[++i];
  }
  const required={import:['--bundle','--site','--origin','--out'],reconcile:['--atlas','--baseline','--out'],diff:['--before','--after','--out']};
  for(const key of required[command])if(!options[key])throw new Error('MISSING_OPTION: '+key);
  return {command,options};
}
function writeOut(dir,outputs) {
  const output=resolve(dir);
  mkdirSync(output,{recursive:false});
  for(const [name,content] of Object.entries(outputs))writeFileSync(resolve(output,name),content,{flag:'wx',mode:0o600});
  return output;
}
try {
  if(process.argv.includes('--help')){
    process.stdout.write('import --bundle PATH --site ID --origin URL --out NEW_DIR [--capture-dir PATH] [--states-dir PATH] [--owned-target --include-labels] [--native-bin CRAFT_RUST_EXE]\nreconcile --atlas PATH --baseline PATH --out NEW_DIR\ndiff --before PATH --after PATH --out NEW_DIR\n');
  } else {
    const {command,options:a}=argumentsOf(process.argv.slice(2));
    if(command==='import'){
      const src=readJsonCapped(a['--bundle']);
      const native=a['--native-bin']?runNativeVerification({binary:a['--native-bin'],bundlePath:a['--bundle'],bundleSha256:src.sha256,records:src.content.records??[]}):null;
      const atlas=buildAtlas({bundle:src.content,bundleDigest:src.sha256,siteId:a['--site'],origin:a['--origin'],captureDir:a['--capture-dir'],statesDir:a['--states-dir'],ownedTarget:!!a['--owned-target'],includeLabels:!!a['--include-labels'],nativeVerifiedIds:native?.ids??null});
      const out=writeOut(a['--out'],{'atlas.json':JSON.stringify(atlas,null,2)+'\n','coverage.md':coverageMarkdown(atlas)});
      process.stdout.write(JSON.stringify({status:'pass',command,out,verified:atlas.source.verifiedRecords,nativeVerified:atlas.source.nativeVerified,browserRecords:atlas.source.scopedBrowserRecords,controls:atlas.controls.length})+'\n');
    } else if(command==='reconcile') {
      const atlas=readJsonCapped(a['--atlas']).content;
      const baseline=readJsonCapped(a['--baseline']).content;
      const result=reconcileAtlas({atlas,baseline});
      const out=writeOut(a['--out'],{'reconciliation.json':JSON.stringify(result,null,2)+'\n','reconciliation.md':reconciliationMarkdown(result)});
      process.stdout.write(JSON.stringify({status:'pass',command,out,features:result.summary.configuredFeatures,divergences:result.summary.sourceDivergences})+'\n');
    }else {
      const before=readJsonCapped(a['--before']).content,after=readJsonCapped(a['--after']).content;
      const result=diffAtlases(before,after);
      const out=writeOut(a['--out'],{'diff.json':JSON.stringify(result,null,2)+'\n','diff.md':diffMarkdown(result)});
      process.stdout.write(JSON.stringify({status:'pass',command,out,changedRoles:result.roleCounts.filter(r=>r.delta!==0).length})+'\n');
    }
  }
} catch(error) {
  process.stderr.write(JSON.stringify({status:'fail',code:error.code||'ADAPTER_FAILED',message:error.message})+'\n');
  process.exitCode=1;
}
