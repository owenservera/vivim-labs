#!/usr/bin/env node
// Lane D: read-only acceptance oracle over Lane E's HQKGX1 source-owned exchange
// and the existing HQ buildWiki() Markdown output. Not a wiki renderer or writer.
import { createHash } from 'node:crypto';
import { existsSync, lstatSync, readFileSync, readdirSync, realpathSync, writeFileSync } from 'node:fs';
import { dirname, extname, isAbsolute, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

export const REPORT_SCHEMA = 'vivim.wave1.wiki-quality/1';
const EXCHANGE_SCHEMA = 'repos-hq.hqkg-exchange/1';
const HEX_ID = /^hqg1:[a-f0-9]{64}$/;
const SHA = /^(?:[a-f0-9]{40}|[a-f0-9]{64})$/;
const MAX_INPUT = 16 * 1024 * 1024;
const SECRET = /(?:^|\/)(?:\.env[^/]*|cookies|login data|session\.json|storage-state\.json|[^/]+\.(?:pem|p12|keystore))(?:$|\/)/i;
const DOMAIN = { source: 'code', file: 'code', symbol: 'code', code: 'code', kb: 'knowledge', decision: 'knowledge', knowledge: 'knowledge', semantic: 'semantic', identity: 'semantic', action: 'command', command: 'command', cli: 'command' };
const SAFE_LINK = /\]\(([^)]+)\)/g;
const cmp = (a, b) => String(a).localeCompare(String(b), 'en');
const digest = x => createHash('sha256').update(x).digest('hex');
function assert(c, reason) { if (!c) throw new Error(reason); }
function object(v) { return v !== null && typeof v === 'object' && !Array.isArray(v); }
function safeMeta(m) {
  assert(m === undefined || object(m), 'invalid_metadata');
  if (!m) return {};
  assert(Object.keys(m).length <= 256, 'metadata_limit');
  for (const [k, v] of Object.entries(m)) {
    assert(k.length > 0 && k.length <= 256 && !/[\x00-\x1f]/.test(k) && !k.startsWith('hqkg.') && !['__proto__','constructor','prototype'].includes(k), 'invalid_metadata_key');
    assert(!/(password|credential|token|cookie|secret|api_key|private_key|authorization)/i.test(k), 'secret_metadata_key');
    assert(['string','number','boolean'].includes(typeof v) && String(v).length <= 1024 && (typeof v !== 'number' || Number.isFinite(v)), 'invalid_metadata_value');
  }
  return m;
}
/** Validate the exact HQKGX1 transport rather than trusting an unpinned graph-shaped JSON blob. */
export function parseExchange(text) {
  assert(typeof text === 'string' && Buffer.byteLength(text) <= MAX_INPUT && text.endsWith('\n'), 'invalid_exchange_size_or_newline');
  const newline = text.indexOf('\n');
  assert(newline > 0, 'missing_manifest');
  const manifest = JSON.parse(text.slice(0, newline));
  assert(object(manifest) && manifest.type === 'manifest' && manifest.schema === EXCHANGE_SCHEMA, 'wrong_exchange_schema');
  assert(typeof manifest.source === 'string' && manifest.source.length > 0 && manifest.source.length <= 256, 'invalid_source');
  assert(manifest.authority === 'data-only; no executable code or provider actions', 'invalid_authority');
  const body = text.slice(newline + 1);
  assert(manifest.graph_sha256 === digest(body), 'exchange_digest_mismatch');
  assert(Number.isSafeInteger(manifest.node_count) && manifest.node_count >= 0 && Number.isSafeInteger(manifest.edge_count) && manifest.edge_count >= 0, 'invalid_counts');
  const nodes = new Map(); const edges = []; let seenEdge = false; const edgeIds = new Set();
  for (const line of body.split('\n').slice(0, -1)) {
    assert(line && !line.endsWith('\r'), 'invalid_record_newline');
    const r = JSON.parse(line);
    assert(object(r) && HEX_ID.test(r.id), 'invalid_record_id');
    safeMeta(r.metadata);
    if (r.type === 'node') {
      assert(!seenEdge && typeof r.kind === 'string' && r.kind.length > 0 && typeof r.title === 'string', 'invalid_node');
      assert(!nodes.has(r.id), 'duplicate_node');
      nodes.set(r.id, r);
    } else if (r.type === 'edge') {
      seenEdge = true;
      assert(HEX_ID.test(r.from) && HEX_ID.test(r.to) && typeof r.relation === 'string' && r.relation.length > 0 && Number.isSafeInteger(r.weight) && r.weight > 0, 'invalid_edge');
      assert(!edgeIds.has(r.id), 'duplicate_edge');
      edgeIds.add(r.id); edges.push(r);
    } else throw new Error('invalid_record_type');
  }
  assert(nodes.size === manifest.node_count && edges.length === manifest.edge_count, 'exchange_count_mismatch');
  for (const e of edges) assert(nodes.has(e.from) && nodes.has(e.to), 'orphan_edge');
  return { manifest, nodes: [...nodes.values()].sort((a,b)=>cmp(a.id,b.id)), edges: edges.sort((a,b)=>cmp(a.id,b.id)) };
}

/** Inspect existing HQ wiki output; no writes, symlink traversal or external links. */
export function readWikiPages(root, { maxPages = 5000, maxPageBytes = 256 * 1024 } = {}) {
  const base = realpathSync(root);
  assert(lstatSync(base).isDirectory(), 'wiki_root_not_directory');
  const pages = new Map();
  function walk(path) {
    for (const name of readdirSync(path).sort(cmp)) {
      const child = join(path, name);
      const stat = lstatSync(child);
      assert(!stat.isSymbolicLink(), 'wiki_symlink_refused');
      const rel = relative(base, child).split(sep).join('/');
      if (SECRET.test(rel)) continue;
      if (stat.isDirectory()) { walk(child); continue; }
      if (!stat.isFile() || extname(name).toLowerCase() !== '.md') continue;
      assert(pages.size < maxPages && stat.size <= maxPageBytes, 'wiki_budget_exceeded');
      pages.set(rel, readFileSync(child, 'utf8'));
    }
  }
  walk(base);
  return pages;
}
function classify(node) { const m = node.metadata || {}; return DOMAIN[m.domain] || DOMAIN[node.kind] || 'unknown'; }
function datum(node, key) { return typeof node.metadata?.[key] === 'string' ? node.metadata[key].trim() : ''; }
function sourceSafe(node) {
  const p = datum(node,'source_path');
  const repo = datum(node,'source_repo');
  const sha = datum(node,'source_sha');
  return Boolean(repo && p && !SECRET.test(p) && !p.startsWith('/') && !p.split('/').some(s => s === '..' || s === '.') && SHA.test(sha));
}
function wikiLinkGaps(pages) {
  const errors = [];
  for (const [name, content] of pages) {
    for (const match of content.matchAll(SAFE_LINK)) {
      const href = match[1].trim().split(/\s+['"]/)[0];
      if (/^(?:[a-z][a-z0-9+.-]*:|#|\/\/)/i.test(href)) continue;
      let path;
      try { path = decodeURIComponent(href.split('#')[0].split('?')[0]); } catch { errors.push({page:name,href,reason:'invalid_percent_encoding'}); continue; }
      if (!path) continue;
      const dest = resolve('/', dirname(name), path).slice(1).replace(/\\/g,'/');
      if (!dest.endsWith('.md')) continue;
      if (!pages.has(dest)) errors.push({page:name,href,reason:'broken_internal_link'});
    }
  }
  return errors.sort((a,b)=>cmp(a.page,b.page)||cmp(a.href,b.href));
}
function hasEvidence(edge) {
  return ['verified','observed','source_pinned'].includes(edge.metadata?.link_status) &&
    Boolean(edge.metadata?.evidence_ref);
}
function sharePage(nodes, pages) {
  const terms = [
    datum(nodes[0],'source_path'), datum(nodes[0],'source_sha'),
    datum(nodes[1],'semantic_id') || nodes[1].id,
    datum(nodes[2],'action_id') || nodes[2].id,
    datum(nodes[2],'syntax'),
  ];
  if (terms.some(t => !t)) return null;
  for (const [path, text] of pages) if (terms.every(term => text.includes(term))) return path;
  return null;
}
/** A/B/C is the only input authority; wiki is assessed as its *derived* representation. */
export function assessWiki({ exchange, pages, rustRuntimeEvidence = null } = {}) {
  assert(exchange?.manifest?.schema === EXCHANGE_SCHEMA && Array.isArray(exchange.nodes) && Array.isArray(exchange.edges), 'validated_exchange_required');
  assert(pages instanceof Map, 'wiki_pages_required');
  const byId = new Map(exchange.nodes.map(n => [n.id,n]));
  const classes = Object.fromEntries(['code','knowledge','semantic','command','unknown'].map(k => [k,exchange.nodes.filter(n=>classify(n)===k).length]));
  const defects = [];
  function defect(code, id, detail) { defects.push({code, id, detail}); }
  for (const n of exchange.nodes) {
    const d = classify(n);
    if (d === 'unknown') defect('unknown_graph_domain',n.id,n.kind);
    if (d === 'code' && !sourceSafe(n)) defect('missing_source_provenance',n.id,n.title);
    if (['semantic','knowledge'].includes(d) && !datum(n,'description')) defect('missing_semantic_description',n.id,n.title);
    if (d === 'semantic' && !datum(n,'semantic_id')) defect('unbound_semantic_identity',n.id,n.title);
    if (['semantic','knowledge'].includes(d) && !['current','accepted','proposed','deprecated'].includes(datum(n,'status'))) defect('unknown_knowledge_status',n.id,datum(n,'status')||'unknown');
    if (d === 'code' && sourceSafe(n) && ![...pages.values()].some(text=>text.includes(datum(n,'source_path')))) defect('missing_wiki_source_reference',n.id,datum(n,'source_path'));
    if (d === 'command') {
      if (!datum(n,'syntax')) defect('missing_command_syntax',n.id,n.title);
      if (datum(n,'status') !== 'wired' || !datum(n,'handler')) defect('unwired_or_unverified_command',n.id,datum(n,'status') || 'unknown');
    }
  }
  const adj = new Map();
  for (const e of exchange.edges) {
    if (!adj.has(e.from)) adj.set(e.from, []);
    adj.get(e.from).push({to:e.to, edge:e});
    if (!hasEvidence(e)) defect('unverified_relationship',e.id,e.relation);
  }
  const linkedIds = new Set(exchange.edges.flatMap(e=>[e.from,e.to]));
  for (const n of exchange.nodes) {
    if (classify(n)==='knowledge' && !linkedIds.has(n.id)) defect('orphan_knowledge_record',n.id,n.title);
    if (classify(n)==='command' && !linkedIds.has(n.id)) defect('orphan_command_record',n.id,n.title);
  }
  const chains = []; const seen = new Set();
  // Verified directional code -> semantic -> command links. KB sidecars count separately.
  for (const code of exchange.nodes.filter(n=>classify(n)==='code')) {
    for (const a of adj.get(code.id)||[]) {
      const mid = byId.get(a.to);
      if (classify(mid)==='semantic') {
        for (const b of adj.get(mid.id)||[]) {
          const cmd=byId.get(b.to); if(classify(cmd)!=='command') continue;
          const identity = `${code.id}|${mid.id}|${cmd.id}`;
          if(seen.has(identity)) continue; seen.add(identity);
          const sourceVerified=sourceSafe(code);
          const semanticVerified=Boolean(datum(mid,'semantic_id')&&datum(mid,'description')&&['current','accepted'].includes(datum(mid,'status')));
          const commandVerified=datum(cmd,'status')==='wired'&&Boolean(datum(cmd,'handler')&&datum(cmd,'syntax'));
          const joinsVerified=hasEvidence(a.edge)&&hasEvidence(b.edge);
          const page=sharePage([code,mid,cmd],pages);
          const verified=sourceVerified&&semanticVerified&&commandVerified&&joinsVerified&&Boolean(page);
          if(!page) defect('missing_cross_domain_wiki_page',cmd.id,identity);
          chains.push({code:code.id,semantic:mid.id,command:cmd.id,page,sourceVerified,semanticVerified,commandVerified,joinsVerified,verified});
        }
      }
    }
  }
  if (!chains.length) defect('no_code_semantic_command_chain','wave1','No explicit graph edges joining A to B to C');
  const brokenLinks = wikiLinkGaps(pages);
  for(const link of brokenLinks) defect('broken_wiki_link',link.page,link.href);
  const totals = { graphNodes: exchange.nodes.length, graphEdges: exchange.edges.length, wikiPages: pages.size, potentialChains: chains.length, verifiedWikiChains: chains.filter(c=>c.verified).length, defects: defects.length, domains:classes };
  // This is a *sample quality* score, not a claim that all source roots were scanned.
  // Every dimension exports its numerator and denominator; absent domains score zero.
  const d = (name, numerator, denominator) => ({name,numerator,denominator,percent:denominator ? Math.round(numerator*100/denominator) : 0});
  const codeNodes = exchange.nodes.filter(n=>classify(n)==='code');
  const meaningNodes = exchange.nodes.filter(n=>['semantic','knowledge'].includes(classify(n)));
  const commands = exchange.nodes.filter(n=>classify(n)==='command');
  const kbNodes = exchange.nodes.filter(n=>classify(n)==='knowledge');
  const dimensions = [
    d('code_source_provenance',codeNodes.filter(sourceSafe).length,codeNodes.length),
    d('semantic_meaning',meaningNodes.filter(n=>Boolean(datum(n,'description'))).length,meaningNodes.length),
    d('wired_registered_commands',commands.filter(n=>datum(n,'status')==='wired'&&datum(n,'handler')&&datum(n,'syntax')).length,commands.length),
    d('graph_to_wiki_trace',totals.verifiedWikiChains,chains.length),
    d('linked_knowledge_records',kbNodes.filter(n=>linkedIds.has(n.id)).length,kbNodes.length),
  ];
  const quality = {score:Math.round(dimensions.reduce((a,x)=>a+x.percent,0)/dimensions.length),dimensions,scope:'observed graph sample only; NOT source-root coverage'};
  const hasRuntime = rustRuntimeEvidence?.observed === true && rustRuntimeEvidence?.exchangeDigest === exchange.manifest.graph_sha256 && typeof rustRuntimeEvidence?.journalSha === 'string' && SHA.test(rustRuntimeEvidence.journalSha);
  // Runtime receipts supplied to this tool are *claims*. Independent F/integration rerun still required.
  const verdict = totals.verifiedWikiChains > 0 && hasRuntime ? 'READY_FOR_INDEPENDENT_VERIFICATION' : 'BLOCKED';
  return { schema:REPORT_SCHEMA, verdict, source:exchange.manifest.source, exchangeDigest:exchange.manifest.graph_sha256, rustEvidenceSupplied:hasRuntime, rustEvidenceIndependentlyVerified:false, totals, quality, chains:chains.sort((a,b)=>cmp(a.code,b.code)||cmp(a.semantic,b.semantic)), defects:defects.sort((a,b)=>cmp(a.code,b.code)||cmp(a.id,b.id)) };
}

function parseArgs(argv) {
  const o={};
  for (let i=0;i<argv.length;i++) {
    const k=argv[i];assert(['--exchange','--wiki-dir','--report','--rust-evidence'].includes(k)&&argv[i+1]&&!argv[i+1].startsWith('--'),`invalid_argument_${k}`);
    o[k.slice(2)]=argv[++i];
  }
  assert(o.exchange&&o['wiki-dir'], 'usage: node wiki-oracle.mjs --exchange <HQKGX1.jsonl> --wiki-dir <HQ-generated-wiki-dir> [--report <json>] [--rust-evidence <json>]');
  return o;
}
export function main(argv=process.argv.slice(2)) {
  const args=parseArgs(argv);
  const exchange=parseExchange(readFileSync(args.exchange,'utf8'));
  const pages=readWikiPages(args['wiki-dir']);
  const evidence=args['rust-evidence']?JSON.parse(readFileSync(args['rust-evidence'],'utf8')):null;
  const result=assessWiki({exchange,pages,rustRuntimeEvidence:evidence});
  const json=JSON.stringify(result,null,2)+'\n';
  if(args.report) { assert(!existsSync(args.report),'report_exists_refuse_overwrite');writeFileSync(args.report,json,{flag:'wx'}); }
  else process.stdout.write(json);
  return result.verdict==='BLOCKED'?2:0;
}
if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  try {process.exitCode=main();} catch(e) {console.error(`lane-d: ${e.message}`);process.exitCode=2;}
}