/** Loopback-only, read-only visual lab server. Deliberately no Host dispatcher route. */
import { createServer } from 'node:http';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { resolve, join, extname } from 'node:path';
import { collectInventory, getHostPlanner, getContacts, paths } from './bridge.mjs';
import { visualCompile } from './compiler.mjs';
import { searchContext } from './context.mjs';
const publicRoot=resolve(fileURLToPath(new URL('../public/',import.meta.url)));
const port=Number(process.env.PORT||4387);
const types={'.html':'text/html; charset=utf-8','.css':'text/css; charset=utf-8','.js':'text/javascript; charset=utf-8','.mjs':'text/javascript; charset=utf-8','.json':'application/json'};
const json=(res,code,data)=>{res.writeHead(code,{'Content-Type':'application/json; charset=utf-8','Cache-Control':'no-store','X-Content-Type-Options':'nosniff','Referrer-Policy':'no-referrer','X-Frame-Options':'DENY'});res.end(JSON.stringify(data));};
function localhostOnly(req){const h=String(req.headers.host||'');return /^(?:127\.0\.0\.1|localhost)(?::\d+)?$/.test(h);}
async function body(req){let b='';for await(const c of req){b+=c;if(b.length>32768)throw Error('Request too large');}return JSON.parse(b||'{}');}
export function createLabServer(){return createServer(async(req,res)=>{
  if(!localhostOnly(req)){json(res,403,{error:'loopback_host_only'});return;}
  try{
    const url=new URL(req.url||'/',`http://${req.headers.host}`);
    if(req.method==='GET'&&url.pathname==='/api/inventory'){return json(res,200,await collectInventory());}
    if(req.method==='GET'&&url.pathname==='/api/contacts'){return json(res,200,getContacts((url.searchParams.get('q')||'').slice(0,90)));}
    if(req.method==='GET'&&url.pathname==='/api/health'){
      const inv=await collectInventory(),host=await getHostPlanner();
      return json(res,200,{schema:'vivim.labs.health/1',readOnly:true,hostPlanner:!!host.planner,hostPlannerReason:host.reason,
        hqInventory:!!inv.hqSummary,registries:inv.counts,sources:inv.sources,gaps:inv.gaps,paths});
    }
    if(req.method==='POST'&&url.pathname==='/api/context'){
      const origin=req.headers.origin;
      if(origin && origin!==`http://${req.headers.host}`)return json(res,403,{error:'origin_denied'});
      if(!String(req.headers['content-type']||'').startsWith('application/json'))return json(res,415,{error:'json_only'});
      const data=await body(req);
      if(typeof data.text!=='string'||data.text.length>1000)return json(res,400,{error:'text_required_max_1000'});
      if(data.scope!==undefined && !['auto','repos-hq','vivim-web-harness','tierflow','vivim-labs'].includes(data.scope))
        return json(res,400,{error:'invalid_scope'});
      if(data.includePersonal!==undefined && typeof data.includePersonal!=='boolean')return json(res,400,{error:'invalid_personal_toggle'});
      return json(res,200,await searchContext({text:data.text,scope:data.scope||'auto',includePersonal:data.includePersonal===true}));
    }
    if(req.method==='POST'&&url.pathname==='/api/interpret'){
      const origin=req.headers.origin;
      if(origin && origin!==`http://${req.headers.host}`)return json(res,403,{error:'origin_denied'});
      if(!String(req.headers['content-type']||'').startsWith('application/json'))return json(res,415,{error:'json_only'});
      const data=await body(req);
      if(typeof data.text!=='string'||data.text.length>1000)return json(res,400,{error:'text_required_max_1000'});
      const inv=await collectInventory();const {planner,reason}=await getHostPlanner();
      let actual=null,plannerError=reason;
      if(planner&&data.text.trim())try{actual=planner.planText(data.text,{source:'text'});}catch(e){plannerError=String(e.code||e.message);}
      const view=visualCompile(data.text,inv,actual);
      if(plannerError)view.gaps.push({code:'HOST_PLANNER_GAP',message:plannerError});
      return json(res,200,{...view,registryRevision:inv.revision,bridge:{host:!!planner,hq:!!inv.hqSummary}});
    }
    if(req.method==='GET'){
      const path=url.pathname==='/'?'/index.html':url.pathname;
      if(!['/index.html','/app.css','/app.js','/semantic-lens.mjs'].includes(path))return json(res,404,{error:'not_found'});
      const abs=path==='/semantic-lens.mjs'?fileURLToPath(new URL('./semantic-lens.mjs',import.meta.url)):join(publicRoot,path.slice(1));
      const bytes=readFileSync(abs);res.writeHead(200,{'Content-Type':types[extname(abs)]||'application/octet-stream','Cache-Control':'no-store','X-Content-Type-Options':'nosniff','Referrer-Policy':'no-referrer','X-Frame-Options':'DENY','Content-Security-Policy':"default-src 'self'; style-src 'self'; script-src 'self'; connect-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none'"});res.end(bytes);return;
    }
    return json(res,405,{error:'method_not_allowed'});
  }catch(e){return json(res,400,{error:'request_failed',detail:String(e.message||e)});}
});}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const server=createLabServer();server.listen(port,'127.0.0.1',()=>process.stdout.write(`VIVIM NLP visual command UI lab: http://127.0.0.1:${port}\nRead-only compiler. No Host actions can execute.\n`));
}