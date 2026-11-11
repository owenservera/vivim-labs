const $=id=>document.getElementById(id);
const state={inventory:null,preview:null,view:'compose',selected:0,seq:0,abort:null,filters:{symbols:true,reading:true,tokens:true,effects:true},history:[],context:null,contextSeq:0,contextAbort:null};
const escape=s=>String(s??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const stateClass=s=>['candidate','ready','ok','visual_candidate'].includes(s)?'ok':/refus|unavail|invalid|unknown/.test(s)?'bad':'warn';
const iconOf=k=>({email:'✉',contact:'♙',contacts:'♙',control:'⌘',system:'◉',devices:'▣',hq:'◇','hq-cli':'⌥','tierflow-ui':'▧',ui:'▧',browser:'◎',file:'▤'}[k]||'◈');
function showView(name){state.view=name;document.querySelectorAll('.view').forEach(el=>el.classList.toggle('visible',el.id===name+'-view'));document.querySelectorAll('.nav[data-view]').forEach(el=>el.classList.toggle('active',el.dataset.view===name));}
document.querySelectorAll('[data-view]').forEach(b=>b.addEventListener('click',()=>showView(b.dataset.view)));
document.querySelectorAll('[data-fb]').forEach(b=>b.addEventListener('click',()=>{const id=b.dataset.fb;state.filters[id]=!state.filters[id];b.classList.toggle('active',state.filters[id]);drawPreview();}));
function sourceStatus(inventory){const host=inventory.sources.find(s=>s.authority==='host')?.present;
  const hq=!!inventory.hqSummary;
  $('dot-host').className='dot '+(host?'good':'warn');$('host-status').textContent=host?'Host catalog: connected':'Host catalog: not mounted';
  $('dot-hq').className='dot '+(hq?'good':'warn');$('hq-status').textContent=hq?'HQ inventory: connected':'HQ inventory: not mounted';
  $('nav-count').textContent=inventory.counts.all;$('nav-gaps').textContent=inventory.gaps.length;
  $('footer-source').textContent=`${inventory.counts.host} Host · ${inventory.counts.hq} HQ · ${inventory.counts.cli} CLI · ${inventory.counts.tierflow||0} TierFlow UI records`;
  $('rev').textContent='Registry '+inventory.revision.slice(0,8);
  $('source-list').innerHTML=inventory.sources.map(s=>`<div class="source"><b>${s.present?'●':'○'} ${escape(s.authority.toUpperCase())} · ${s.present?'Detected':'Unavailable'}</b><small>${escape(s.role)}<br>${escape(s.path)}</small></div>`).join('');
  $('gap-list').innerHTML=inventory.gaps.map(g=>`<article class="gap"><b>${escape(g.code)}</b><p>${escape(g.message)}</p></article>`).join('');
  drawCatalog();
}
async function refresh(){
  $('host-status').textContent='Reading sources…';$('hq-status').textContent='Reading sources…';
  try{const response=await fetch('/api/inventory',{cache:'no-store'});if(!response.ok)throw Error(`HTTP ${response.status}`);state.inventory=await response.json();sourceStatus(state.inventory);if($('prompt').value.trim()){await interpret();requestContext();}}
  catch(e){$('host-status').textContent='Registry fetch failed';$('hq-status').textContent=String(e.message);}
}
$('refresh').addEventListener('click',refresh);
function drawCatalog(){const inv=state.inventory;if(!inv)return;const q=$('search').value.toLowerCase().trim(),ns=$('namespace').value;
 const data=inv.actions.filter(a=>(ns==='all'||a.namespace===ns)&&(!q||(a.title+' '+a.id+' '+a.category).toLowerCase().includes(q))).slice(0,300);
 $('list-count').textContent=`${data.length} shown / ${inv.actions.length}`;
 $('catalog').innerHTML=data.length?data.map(a=>`<div class="catalog-item"><span class="catalog-ico">${iconOf(a.category||a.namespace)}</span><div><b>${escape(a.title)}</b><small><code>${escape(a.namespace)} / ${escape(a.id)}</code> · ${escape(a.category)} · ${escape(a.evidence)}</small></div><span class="pill ${a.liveVerified?'ok':a.namespace==='host'?'warn':'neutral'}">${escape(a.readiness)}</span></div>`).join(''):'<div class="placeholder">No matching source records. Missing source data is not proof of zero commands.</div>';
}
$('search').addEventListener('input',drawCatalog);$('namespace').addEventListener('change',drawCatalog);
function drawMenu(p){const e=$('menu');const menu=p?.menu;if(!menu||menu.kind!=='contacts'){e.classList.add('hidden');return;}
  const items=menu.items||[];state.selected=Math.max(0,Math.min(state.selected,items.length-1));
  e.innerHTML=`<div class="menu-head"><span>♙ ${escape(menu.title||'Contacts')}</span><small>${menu.origin==='fictional_demo_fixture'?'Fictional sample contacts':menu.origin==='local_unverified_read_only'?'Local, unverified contacts':'No contact source'}</small></div>`+
    (items.length?items.map((c,i)=>`<button class="menu-item ${i===state.selected?'selected':''}" role="option" aria-selected="${i===state.selected}" data-index="${i}"><span class="avatar">♙</span><span><b>${escape(c.label)}</b><small>${escape(c.detail)}</small></span></button>`).join(''):'<div class="menu-empty">No matching contacts. A trusted Host contact provider is not connected.</div>')+
    '<div class="menu-foot">↑↓ move · Tab select · Esc close · This lab never sends email</div>';
  e.classList.remove('hidden');e.querySelectorAll('[data-index]').forEach(x=>x.addEventListener('click',()=>applySuggestion(Number(x.dataset.index))));
}
function applySuggestion(index){const p=state.preview;const item=p?.menu?.items?.[index];if(!item)return;const prompt=$('prompt'),{start,end}=p.menu.replace;
 prompt.value=prompt.value.slice(0,start)+item.insert+prompt.value.slice(end);const caret=start+item.insert.length;prompt.focus();prompt.setSelectionRange(caret,caret);state.selected=0;interpret();scheduleContext();}
function drawPreview(){const p=state.preview;if(!p)return;const {symbols,reading,tokens,effects}=state.filters;
 $('symbols').innerHTML=symbols&&p.symbols?.length?(p.symbols.map((s,i)=>`${i?'<span class="arrow">→</span>':''}<div class="symbol ${s.registered?'':'unreg'}"><i>${escape(s.icon)}</i>${escape(s.label)}</div>`).join('')+`<div class="canonical">${escape(p.canonical||'')}</div>`):'<span class="empty">No registered symbols resolved yet.</span>';
 $('tokens').innerHTML=tokens&&p.tokens?.length?p.tokens.map(t=>`<span class="token ${escape(t.role||'text')}" title="${escape(t.note||t.role||'Unclassified')}">${escape(t.text)}<small>${escape(t.role||t.kind||'text')}</small></span>`).join(''):'<span class="empty">No token feedback yet.</span>';
 $('outcome').innerHTML=effects?`<span class="pill ${stateClass(p.outcome)}">${escape(p.outcome.replaceAll('_',' '))}</span> ${p.actionId?`<span class="pill">${escape(p.actionId)}</span>`:''}`:'';
 $('slots').innerHTML=effects?(p.slots||[]).map(s=>`<div class="slot"><small>${escape(s.icon||'@')} ${escape(s.key)} · ${escape(s.state)}</small><strong>${escape(s.label||'')}</strong></div>`).join(''):'';
 $('details').innerHTML=effects?`<b>Readiness:</b> ${escape(p.readiness)} · <b>Risk:</b> ${escape(p.risk)} · <b>Source:</b> ${escape(p.provenance||p.namespace||'unknown')}<br><b>Execution performed:</b> NO ${p.gaps?.length?`<div style="margin-top:8px;color:#d6b38b">${p.gaps.map(g=>escape(g.message)).join(' · ')}</div>`:''}`:'';
 $('longform').textContent=reading?p.reading:'Longform reading hidden. Toggle in Feedback Experiments.';
 $('under-composer').style.opacity=reading?'1':'.4';$('compile-status').textContent=p.outcome.replaceAll('_',' ')+' · visual only';drawMenu(p);
}
async function interpret(){const seq=++state.seq;const value=$('prompt').value;$('chars').textContent=value.length+' / 1000';
  if(state.abort)state.abort.abort();const controller=new AbortController();state.abort=controller;
  if(!value.trim()){state.preview=null;$('menu').classList.add('hidden');$('longform').textContent='Type a command to see its full interpretation in plain English.';$('symbols').textContent='Action and entity symbols will appear here.';$('tokens').textContent='Waiting for source text.';$('outcome').innerHTML='<span class="pill">No interpretation yet</span>';$('slots').textContent='';$('details').textContent='The visual compiler never executes actions.';$('compile-status').textContent='Waiting for input';return;}
  $('compile-status').textContent='Interpreting…';
  try{const response=await fetch('/api/interpret',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({text:value}),signal:controller.signal});if(!response.ok)throw Error('Interpreter returned '+response.status);const p=await response.json();if(seq!==state.seq)return;state.preview=p;state.selected=0;drawPreview();}
  catch(e){if(e.name==='AbortError'||seq!==state.seq)return;$('compile-status').textContent='Interpreter unavailable';$('longform').textContent=String(e.message);$('menu').classList.add('hidden');}
}
let debounce;
$('prompt').addEventListener('input',()=>{clearTimeout(debounce);debounce=setTimeout(interpret,70);scheduleContext();});
$('prompt').addEventListener('keydown',e=>{
 const menu=state.preview?.menu?.items||[];
 if(e.key==='Escape'){$('menu').classList.add('hidden');return;}
 if(e.key==='ArrowDown'&&menu.length&&!$('menu').classList.contains('hidden')){e.preventDefault();state.selected=Math.min(menu.length-1,state.selected+1);drawMenu(state.preview);return;}
 if(e.key==='ArrowUp'&&menu.length&&!$('menu').classList.contains('hidden')){e.preventDefault();state.selected=Math.max(0,state.selected-1);drawMenu(state.preview);return;}
 if(e.key==='Tab'&&menu.length&&!$('menu').classList.contains('hidden')){e.preventDefault();applySuggestion(state.selected);return;}
 if(e.key==='Enter'&&!e.shiftKey){e.preventDefault();recordCompile();}
});
async function recordCompile(){await interpret();const p=state.preview;if(!p||!p.text.trim())return;const bubble=document.createElement('div');bubble.className='message';
 bubble.textContent=p.text;const meta=document.createElement('small');meta.textContent=`${p.outcome.replaceAll('_',' ')} · ${p.actionId||'no registered action'} · Not executed`;bubble.append(meta);
 $('transcript').append(bubble);$('transcript').scrollTop=$('transcript').scrollHeight;}
$('compile').addEventListener('click',recordCompile);
document.querySelectorAll('[data-example]').forEach(b=>b.addEventListener('click',()=>{$('prompt').value=b.dataset.example;state.selected=0;interpret();scheduleContext();$('prompt').focus();}));
refresh();


// Round 2: a separate, abortable HQ knowledge lens. It never invokes any writer or executor.
const SOURCE_REPOS={
 'repos-hq':'owenservera/repos-hq',
 'vivim-web-harness':'owenservera/vivim-web-harness',
 'tierflow':'owenservera/tierflow',
 'vivim-labs':'owenservera/vivim-labs'
};
function sourceHref(item){
 const raw=String(item.project||'').toLowerCase();
 const repo=SOURCE_REPOS[raw];
 const path=String(item.path||'').replaceAll('\\','/');
 if(!repo||!path||path.startsWith('/')||path.split('/').some(p=>p==='..')||!(/^[a-zA-Z0-9_.\-/]+$/.test(path)))return null;
 const line=item.line&&item.line>0?'#L'+item.line:'';
 return 'https://github.com/'+repo+'/blob/main/'+path.split('/').map(encodeURIComponent).join('/')+line;
}
function emptyContext(status='Waiting for your thought'){
 $('wiki-status').textContent=status;
 $('wiki-status').className='pill neutral';
 $('wiki-revision').textContent='HQ index —';
 $('knowledge-focus').textContent='⌕ Start typing a project, feature, file, or goal.';
 $('knowledge-page').innerHTML='<h3>Living working page</h3><p>Grounded source evidence will appear as your thought develops.</p>';
 $('knowledge-count').textContent='0 sources';
 $('knowledge-results').innerHTML='<p class="placeholder">No indexed evidence displayed.</p>';
}
function showContext(c){
 state.context=c;
 const f=c.focus||{};
 $('wiki-status').textContent=c.status==='results'?'HQ evidence live':c.status.replaceAll('_',' ');
 $('wiki-status').className='pill '+(c.status==='results'?'ok':c.status==='unavailable'?'bad':'warn');
 $('wiki-revision').textContent=c.revision?'Index '+escape(c.revision):'Index unavailable';
 $('knowledge-focus').textContent=f.query?'⌕ '+f.query+(f.scope?'  ·  '+f.scope:''):'⌕ Waiting for a topic';
 const pg=c.page;
 if(pg){
   $('knowledge-page').innerHTML=`<h3>${escape(pg.title)}</h3><small>${escape(pg.subtitle)}</small><p>${escape(pg.overview)}</p>${pg.project?`<p>Project: ${escape(pg.project.name)} · ${escape(pg.project.head?.slice(0,10)||'revision unknown')} · indexed ${escape(pg.project.indexedAt||'unknown')}</p>`:''}<small>${escape(pg.caveat)}</small>`;
 } else $('knowledge-page').innerHTML=`<h3>${c.status==='unavailable'?'Knowledge source unavailable':'Topic not yet resolved'}</h3><p>${escape(c.reason||'Enter more topic-specific words to query the HQ index.')}</p>`;
 const items=c.items||[];
 $('knowledge-count').textContent=items.length+' sources';
 $('knowledge-results').innerHTML=items.length?items.map(x=>{
    const href=sourceHref(x);
    return `<article class="knowledge-card"><div class="knowledge-kicker">${escape(x.kind)} · ${escape(x.project||'HQ')} · ${escape(x.source)}</div><b>${escape(x.title)}</b><p>${escape(x.snippet||'Indexed source: no preview available')}</p>${x.path?`<div class="source-path">${escape(x.path)}${x.line?':'+x.line:''}</div>`:''}${href?`<a href="${escape(href)}" target="_blank" rel="noopener noreferrer">Inspect source ↗</a>`:''}</article>`;
 }).join(''):'<p class="placeholder">'+escape(c.status==='unavailable'?'Connect the indexed HQ checkout. No placeholder articles are invented.':'No matches in the current index. Unknown does not mean absent.')+'</p>';
}
async function requestContext(){
 const seq=++state.contextSeq;
 if(state.contextAbort)state.contextAbort.abort();
 const text=$('prompt').value;
 if(!text.trim()){state.context=null;emptyContext();return;}
 const abort=new AbortController();state.contextAbort=abort;
 $('wiki-status').textContent='Searching HQ…';
 try{
   const response=await fetch('/api/context',{method:'POST',headers:{'Content-Type':'application/json'},
     body:JSON.stringify({text,scope:$('project-scope').value,includePersonal:$('personal-sources').checked}),signal:abort.signal,cache:'no-store'});
   if(!response.ok)throw Error('HQ context HTTP '+response.status);
   const data=await response.json();
   if(seq!==state.contextSeq)return;
   showContext(data);
 }catch(e){if(e.name==='AbortError'||seq!==state.contextSeq)return;showContext({status:'unavailable',reason:String(e.message),items:[],focus:{query:text}});}
}
let contextDebounce;
function scheduleContext(){clearTimeout(contextDebounce);contextDebounce=setTimeout(requestContext,260);}
$('project-scope').addEventListener('change',requestContext);
$('personal-sources').addEventListener('change',requestContext);
$('clear-knowledge').addEventListener('click',()=>{clearTimeout(contextDebounce);state.contextSeq++;state.contextAbort?.abort();state.context=null;emptyContext('Cleared');});
for(const [id,key] of [['theme-style','theme'],['type-style','type'],['outline-style','outline']]){
  $(id).addEventListener('change',()=>{document.body.dataset[key]=$(id).value;});
}