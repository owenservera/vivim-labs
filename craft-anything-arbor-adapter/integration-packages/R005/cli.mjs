// Local-only; no browser, provider APIs, network or runtime activation.
import {readFileSync,writeFileSync,statSync} from 'node:fs';
import {compileProposal,makeReviewTemplate,diffProposals} from './compiler.mjs';
import {buildStudioIndex} from './studio-index.mjs';
function read(p){if(!p||!statSync(p).isFile()||statSync(p).size>32*1024*1024)throw Error('INPUT_INVALID');const b=readFileSync(p);if(b.length>32*1024*1024)throw Error('INPUT_TOO_LARGE');return JSON.parse(b);}
try{
 const [command,...args]=process.argv.slice(2);
 let output,path;
 if(command==='template'&&args.length===2){output=makeReviewTemplate(read(args[0]));path=args[1];}
 else if(command==='compile'&&[2,3].includes(args.length)){output=compileProposal(read(args[0]),args.length===3?read(args[1]):null);path=args.at(-1);}
 else if(command==='diff'&&args.length===3){output=diffProposals(read(args[0]),read(args[1]));path=args[2];}
 else if(command==='index'&&args.length===2){output=buildStudioIndex(read(args[0]));path=args[1];}
 else throw Error('USAGE: template ATLAS OUT | compile ATLAS [REVIEW] OUT | diff BEFORE AFTER OUT | index ENTRIES OUT');
 writeFileSync(path,JSON.stringify(output,null,2)+'\n',{flag:'wx',mode:0o600});
 console.log(JSON.stringify({status:'written',schema:output.schema}));
}catch(e){console.error(JSON.stringify({status:'failed',code:e.code??'R005_FAILED'}));process.exitCode=1;}
