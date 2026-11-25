// Materialize only into the isolated scratch test directory. Never modify donor repos.
import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
const d=process.argv[2];if(!d)throw Error('scratch dir required');
const p=JSON.parse(readFileSync(join(d,process.argv[3]||'R038-OVERLAY.json'),'utf8'));
for(const f of p.files){
 const file=join(d,f.name);
 const actual=execFileSync('git',['-C',d,'hash-object',f.name],{encoding:'utf8'}).trim();
 if(actual!==f.blob)throw Error('SOURCE_DRIFT '+f.name);
 let s=readFileSync(file,'utf8');
 for(const [from,to] of f.edits){if(s.split(from).length!==2)throw Error('NON_UNIQUE '+f.name);s=s.replace(from,to);}
 writeFileSync(file,s,'utf8');
}
console.log('EXACT_OWNED_OVERLAY_OK');
