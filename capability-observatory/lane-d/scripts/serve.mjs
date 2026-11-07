#!/usr/bin/env node
// Read-only loopback server; no HQ export or private .local files exposed.
import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const allowed=new Map([['/','dashboard.html'],['/dashboard.html','dashboard.html'],['/data/seed-snapshot.json','data/seed-snapshot.json']]);
const port=Number(process.env.VIVIM_OBSERVATORY_PORT||8764);
if(!Number.isInteger(port)||port<1024||port>65535)throw Error('Invalid port');
http.createServer((req,res)=>{
 const file=allowed.get(new URL(req.url,'http://localhost').pathname);
 res.setHeader('Cache-Control','no-store');res.setHeader('X-Content-Type-Options','nosniff');res.setHeader('Content-Security-Policy',"default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; base-uri 'none'; form-action 'none'");
 if(req.method!=='GET'||!file){res.writeHead(404);res.end('Not found');return;}
 res.setHeader('Content-Type',file.endsWith('.json')?'application/json; charset=utf-8':'text/html; charset=utf-8');
 fs.createReadStream(path.join(root,file)).on('error',()=>{if(!res.headersSent)res.writeHead(404);res.end('Not found')}).pipe(res);
}).listen(port,'127.0.0.1',()=>console.log(`VIVIM Observatory: http://127.0.0.1:${port}/`));
