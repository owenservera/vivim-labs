#!/usr/bin/env node
// Lane D Wave 1 Round 2 — inert inspection of an *existing* Lane E JSONL +
// HQ-generated markdown directory. Never runs the indexer, Rust or a wiki writer.
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {readWikiPages} from '../wiki-oracle.mjs';
import {validateEGraph} from '../contracts/validate-inputs.mjs';
import {traceCompiled} from '../lineage/traverse.mjs';
import {checkLinks} from '../quality/links.mjs';
import {inspectPrivacy} from '../quality/privacy.mjs';
import {checkPageManifest} from '../quality/page-manifest.mjs';
export function inspect({exchangeText,pages,pageManifest=null}={}){
 const view=validateEGraph(exchangeText);
 const lineage=traceCompiled(view), links=checkLinks(pages), privacy=inspectPrivacy(pages);
 const pageClaims=checkPageManifest({pages,manifest:pageManifest,exchangeDigest:view.manifest.graph_sha256});
 return {schema:'vivim.wave1.lane-d.read-only-observation/1',verdict:'BLOCKED_NEEDS_NATIVE_RUST_HQ_WIKI_F_WITNESS',readOnly:true,independentlyVerified:false,exchangeDigest:view.manifest.graph_sha256,provenance:view.diagnostics,graph:{nodes:view.nodes.size,edges:view.edges.length},lineage,links,privacy,pageClaims};
}
export function main(args=process.argv.slice(2),emit=process.stdout){
 if(args.length<4||args.length>6||args[0]!=='--exchange'||args[2]!=='--wiki-dir'||(args.length===6&&(args[4]!=='--page-manifest')))throw Error('usage: node bin/inspect-readonly.mjs --exchange DEV/exchange.jsonl --wiki-dir DEV/wiki [--page-manifest writer-manifest.json]');
 const exchangeText=readFileSync(resolve(args[1]),'utf8');
 const pages=readWikiPages(resolve(args[3]));
 const pageManifest=args.length===6?JSON.parse(readFileSync(resolve(args[5]),'utf8')):null;
 const result=inspect({exchangeText,pages,pageManifest});
 emit.write(JSON.stringify(result,null,2)+'\n');return 2;
}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href){
 try{process.exitCode=main();}catch(e){process.stderr.write('LANE_D_BLOCKED: '+e.message+'\n');process.exitCode=2;}
}
