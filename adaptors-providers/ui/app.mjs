// Labs R001: static, inert provider exploration. NEVER sends, logs in or requests user data.
// The only requests are two relative JSON files committed to the Labs repository.
const $ = id => document.getElementById(id);
const state = { catalog: [], mockThreads: [], providerId: 'messenger', threadId: 'synthetic-thread-a', query: '', family: '' };
const human = value => String(value ?? '').replace(/-/g, ' ');
function node(tag, className, text) {
  const n = document.createElement(tag);
  if (className) n.className = className;
  if (text !== undefined) n.textContent = String(text);
  return n;
}
function tags(holderId, values) {
  const target = $(holderId);
  target.replaceChildren(...values.map(v => node('span','tag',human(v))));
}
function error(text) { $('cards').replaceChildren(node('p','note',text)); }
function setNavigation(target) {
  ['sources','inbox','feeds','activity'].forEach(id => {
    const b = $('nav-' + id), current = id === target;
    b.classList.toggle('current', current);
    if (current) b.setAttribute('aria-current','page'); else b.removeAttribute('aria-current');
  });
  if (target === 'inbox') {
    state.providerId = 'messenger';
    $('search').value = '';
    $('family-filter').value = '';
    state.query = ''; state.family = '';
    render();
    $('messenger-preview').scrollIntoView({block:'nearest',behavior:'smooth'});
  } else if (target === 'feeds') {
    state.family = 'social'; $('family-filter').value = state.family;
    state.query = ''; $('search').value = ''; render();
  } else if (target === 'activity') {
    $('access-plan').textContent='Activity is a future Host-owned, receipt-backed projection. No live events are connected in R001.';
    $('access-plan').classList.remove('hidden');
    $('access-plan').scrollIntoView({block:'nearest',behavior:'smooth'});
  } else {
    state.family = ''; state.query = ''; $('family-filter').value = ''; $('search').value = ''; render();
  }
}
function drawCards() {
  const matches = state.catalog.filter(p =>
    (!state.family || p.family === state.family) &&
    (!state.query || (p.label + ' ' + p.id + ' ' + p.streams.join(' ')).toLowerCase().includes(state.query)));
  $('matching-count').textContent = matches.length + ' shown';
  const cards = matches.map(p => {
    const b = node('button','provider-card' + (p.id === state.providerId ? ' selected' : ''));
    b.type = 'button'; b.setAttribute('aria-pressed',p.id === state.providerId ? 'true':'false');
    const row = node('div','rowtop');
    row.append(node('span','glyph',p.label.slice(0,1)),node('span','ribbon',p.priority));
    b.append(row,node('strong','',p.label),node('span','meta',human(p.family).toUpperCase()),node('span','dim',p.streams.length + ' streams · not connected'));
    b.addEventListener('click', () => {state.providerId=p.id;render();});
    return b;
  });
  $('cards').replaceChildren(...(cards.length?cards:[node('p','note','No matching providers.')]));
}
function drawInspector() {
  const p = state.catalog.find(x => x.id === state.providerId) || state.catalog[0];
  if (!p) return;
  $('detail-title').textContent=p.label;
  $('detail-glyph').textContent=p.label.slice(0,1);
  $('detail-meta').textContent=human(p.family) + ' · ' + p.priority + ' · research candidate';
  $('detail-description').textContent = p.id==='messenger'
    ? 'Design target: read your Facebook Messenger threads in VIVIM, draft a reply, approve the exact recipient and message, then let the existing Host confirm the provider result. None of this is connected yet.'
    : 'A proposed source for owner-controlled import, read or interaction. Concrete account access, stream schemas, permissions and current feasibility are not verified.';
  tags('stream-tags',p.streams);
  tags('mode-tags',p.modes);
  $('consent-details').textContent = p.id==='messenger'
    ? 'Facebook web messages, account identity, individual thread grants and outbound sending must be checked independently. Messenger-only accounts may lack browser access.'
    : 'Discovery, import, live read and outbound actions need distinct Host permissions, where supported.';
  $('access-plan').classList.add('hidden');
  $('messenger-preview').classList.toggle('hidden',p.id!=='messenger');
  if (p.id==='messenger') drawDemo();
}
function drawDemo() {
  const selected=state.mockThreads.find(t=>t.id===state.threadId) || state.mockThreads[0];
  if (!selected) return;
  $('thread-list').replaceChildren(...state.mockThreads.map(t=>{
    const b=node('button','thread-button'+(t.id===selected.id?' active':''),t.label);
    b.type='button'; b.setAttribute('aria-pressed',t.id===selected.id?'true':'false');
    b.addEventListener('click',()=>{state.threadId=t.id;$('compose').value='';$('draft-preview').classList.add('hidden');drawDemo();});
    return b;
  }));
  $('thread-name').textContent=selected.label+' · fabricated';
  $('thread-messages').replaceChildren(...selected.messages.map(m=>node('div','bubble'+(m.author==='mine'?' mine':''),m.text)));
}
function render(){drawCards();drawInspector();}
async function init(){
  for(const id of ['sources','inbox','feeds','activity']) $('nav-'+id).addEventListener('click',()=>setNavigation(id));
  $('search').addEventListener('input',e=>{state.query=e.target.value.trim().toLowerCase();drawCards();});
  $('family-filter').addEventListener('change',e=>{state.family=e.target.value;drawCards();});
  $('review-access').addEventListener('click',()=>{$('access-plan').classList.toggle('hidden');});
  $('preview-draft').addEventListener('click',()=>{
    const selected=state.mockThreads.find(t=>t.id===state.threadId);
    const message=$('compose').value;
    const detail=$('draft-preview');
    detail.textContent=message.trim()
      ? 'LOCAL DRAFT PREVIEW — NOT SENT\nTo: '+(selected?.label||'demo thread')+'\nMessage: '+message+'\nStatus: NO HOST CONNECTION · SEND DISABLED'
      : 'Enter text to preview a synthetic local draft. No real provider message can be sent.';
    detail.classList.remove('hidden');
  });
  try {
    const [catalogResponse,mockResponse] = await Promise.all([
      fetch('../catalog/providers.json',{cache:'no-store'}),
      fetch('./mock-facebook-thread.json',{cache:'no-store'})
    ]);
    if(!catalogResponse.ok||!mockResponse.ok)throw new Error('Lab fixture files unavailable');
    const [data,mock]=await Promise.all([catalogResponse.json(),mockResponse.json()]);
    if(!Array.isArray(data.providers)||!Array.isArray(mock.threads))throw new Error('Invalid lab catalog');
    state.catalog=data.providers;state.mockThreads=mock.threads;
    $('provider-count').textContent=state.catalog.length;
    $('stream-count').textContent=new Set(state.catalog.flatMap(p=>p.streams)).size;
    for (const family of [...new Set(state.catalog.map(p=>p.family))].sort()) {
      const o=node('option','',human(family));o.value=family;$('family-filter').append(o);
    }
    render();
  } catch(err){error('The local catalog did not load. Serve the repository root over HTTP. '+String(err.message||err));}
}
init();
