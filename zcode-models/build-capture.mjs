#!/usr/bin/env node
// build-capture.mjs — vivim-labs zcode-models capture builder (2026-11-20)
//
// Merges four independent sources into ONE zcode-models-format JSON:
//   1. ZCode provider_config.json providerRules (the "zcode models" shape)
//   2. provider-depot catalog.json (pricing + context + capabilities, source-cited)
//   3. Live probe results (capture-YYYYMMDD/manifest.json written by the probe agent)
//   4. ZCode ListModels reasoning levels (baked below — session ground truth 2026-11-20)
//
// Re-runnable: node build-capture.mjs [--probe <path to manifest.json>]
// Output: zcode-models-capture-<date>.json next to this script.
// Keys are ALWAYS redacted: any string field named like a credential becomes [REDACTED len=N].

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const cfgPath = 'C:/Users/VIVIM.inc/.zcode/v2/provider_config.json';
const depotPath = 'C:/Users/VIVIM.inc/.zcode/provider-depot/catalog.json';

const args = process.argv.slice(2);
const probeIdx = args.indexOf('--probe');
const probePath = probeIdx >= 0 ? args[probeIdx + 1] : null;

const TARGETS = ['new-provider', 'openrouter-2', 'minimax-m-plan', 'account:zai-start-plan'];
const LABELS = {
  'new-provider': 'NVIDIA NIM',
  'openrouter-2': 'OpenRouter 2',
  'minimax-m-plan': 'MiniMax M Plan (International)',
  'account:zai-start-plan': 'Zai Start Plan (coding plan gateway)',
};

// ListModels ground truth captured 2026-11-20 in-session (reasoning levels per model)
const REASONING = {
  'new-provider': {
    levels: ['disabled', 'enabled'], default: 'enabled',
    ids: ['meta/muse-glimmer-30b', 'mistralai/mistral-large-4-1t-a52b',
      'nvidia/nemotron-3-nano-omni-30b-a3b-reasoning', 'nvidia/nemotron-3-ultra-550b-a55b',
      'nvidia/nemotron-3.5-bolt-e4b', 'nvidia/nemotron-3.5-lightning-30b-a3b',
      'nvidia/nemotron-3.5-super-vl-120b-a12b', 'nvidia/riva-translate-4b-instruct-v2',
      'poolside/laguna-xs-2.1'],
  },
  'openrouter-2': {
    levels: ['disabled', 'enabled'], default: 'enabled',
    ids: ['baidu/ernie-5.5-flash-preview:free', 'nvidia/nemotron-3.5-super-vl:free',
      'nvidia/nemotron-3-ultra-550b-a55b:free', 'inclusionai/ling-3.2-flash:free'],
  },
  'minimax-m-plan': {
    perModel: {
      'MiniMax-M3': { levels: ['disabled', 'enabled'], default: 'enabled' },
      'MiniMax-M3.1-Flash': { levels: ['low', 'medium', 'high', 'xhigh', 'max'], default: 'max' },
    },
    otherLevels: ['disabled', 'enabled'],
    ids: ['MiniMax-M3', 'MiniMax-M2.7', 'MiniMax-M2.7-highspeed', 'MiniMax-M2.5', 'MiniMax-M2.5-highspeed', 'MiniMax-M3.1-Flash'],
    personal: ['MiniMax-M3', 'MiniMax-M3.1-Flash'],
  },
  'account:zai-start-plan': {
    levels: ['low', 'high', 'max'], default: 'max',
    ids: ['GLM-5.3-Flash', 'glm-5.3-flash', 'GLM-5.3'],
    personal: ['glm-5.3-flash', 'GLM-5.3'],
  },
};

function redact(o) {
  const r = JSON.parse(JSON.stringify(o));
  (function walk(n) {
    if (n && typeof n === 'object') {
      for (const k of Object.keys(n)) {
        const kk = k.replace(/[^a-z]/gi, '').toLowerCase();
        if ((kk.endsWith('key') || kk.endsWith('token') || kk === 'secret' || kk === 'password')
            && typeof n[k] === 'string' && n[k].length > 0) {
          n[k] = `[REDACTED len=${n[k].length}]`;
        } else walk(n[k]);
      }
    }
  })(r);
  return r;
}

function normId(id) {
  return (id || '').toLowerCase().replace(/-bf16|-nvfp4$/g, '').replace(/_/g, '-');
}

function pick(rows, id) {
  // exact → normalized → contains
  let hit = rows.find((r) => (r.id || '').toLowerCase() === id.toLowerCase());
  if (hit) return hit;
  const n = normId(id);
  hit = rows.find((r) => normId(r.id) === n);
  if (hit) return hit;
  hit = rows.find((r) => normId(r.id).includes(n) || n.includes(normId(r.id)));
  return hit || null;
}

function priceOf(row) {
  if (!row) return null;
  const out = {};
  if (typeof row.contextLength === 'number') out.contextLength = row.contextLength;
  if (typeof row.priceIn === 'number') out.priceInPerM = row.priceIn;
  if (typeof row.priceOut === 'number') out.priceOutPerM = row.priceOut;
  if (row.input) out.inputModalities = row.input;
  if (row.output) out.outputModalities = row.output;
  if (typeof row.free === 'boolean') out.free = row.free;
  const caps = [];
  if (row.tools) caps.push('tools');
  if (row.structured) caps.push('structured');
  if (caps.length) out.capabilities = caps;
  return Object.keys(out).length ? out : null;
}

const cfg = JSON.parse(fs.readFileSync(cfgPath, 'utf8'));
const depot = JSON.parse(fs.readFileSync(depotPath, 'utf8'));
const rules = (cfg.config?.providerConfigRules?.providerRules || []).filter((r) => TARGETS.includes(r.providerId));

const pr = depot.sources['hf-router']?.models || [];
const od = depot.sources['models-dev']?.models || [];
const ors = depot.sources['openrouter']?.models || [];
const nim = depot.sources['nvidia-nim']?.models || [];
const mmax = depot.sources['minimax']?.models || [];
const fetchedAt = depot.refreshedAt;

let probe = null;
if (probePath && fs.existsSync(probePath)) probe = JSON.parse(fs.readFileSync(probePath, 'utf8'));
else {
  const guess = path.join(here, `capture-${new Date().toISOString().slice(0, 10).replace(/-/g, '')}`, 'manifest.json');
  if (fs.existsSync(guess)) probe = JSON.parse(fs.readFileSync(guess, 'utf8'));
}

const capture = {
  schema: 'vivim.labs.zcode-models-capture/1.0',
  capturedAt: new Date().toISOString(),
  purpose: 'Provider status + budget assessment for the four ZCode providers, saved in ZCode providerRules shape for direct reuse (model pinning, routing, quota-aware scheduling).',
  howToLeverage: [
    'providers[].config mirrors ~/.zcode/v2/provider_config.json providerRules (keys redacted) — paste-ready for re-wiring',
    'providers[].models[].zcodeModelId pastes verbatim into CreateSession.model or workflow subagent_model',
    'budget.signals + budget.pricing carry per-model $/M with sourceUrl + fetchedAt provenance (provider-depot canon: a price without a source is not a price)',
    'status.liveProbe carries the HTTP-verified state of each provider at capture time',
  ],
  zcodeAuthority: depot._zcode_authority,
  depotRefreshedAt: depot.refreshedAt,
  staleDataWarnings: [
    '~/.zcode/v2/coding-plan-cache.json says builtin:zai-start-plan unavailable/coding_plan_not_entitled — its updatedAt is ~2026-06-22 (≈5 months stale); runtime proves the plan is LIVE (this session runs on account:zai-start-plan/GLM-5.3-Flash). Do not trust that cache.',
    '~/.zcode/v2/bots-model-cache.v2.json builtin:zai-start-plan entry says enabled:false / coding_plan_not_authenticated — same stale vintage. Recorded for provenance only.',
  ],
  providers: [],
};

for (const rule of rules) {
  const id = rule.providerId;
  const meta = REASONING[id] || { levels: [], ids: rule.config?.modelOrder || [] };
  const order = rule.config?.modelOrder || meta.ids;
  const personalSet = new Set([...(meta.personal || []), ...(rule.config?.personalModelIds || [])]);
  const ids = meta.ids;
  const levelsFor = (mid) => (meta.perModel && meta.perModel[mid]?.levels) || meta.levels || meta.otherLevels || [];
  const defaultFor = (mid) => (meta.perModel && meta.perModel[mid]?.default) || meta.default || null;

  const models = ids.map((mid) => {
    const m = {
      id: mid,
      zcodeModelId: `${id}/${mid}`,
      personal: personalSet.has(mid),
      reasoningLevels: levelsFor(mid),
      reasoningDefault: defaultFor(mid),
      inModelOrder: order.includes(mid),
    };
    // pricing pick: openrouter rows for :free, hf-router first, models-dev fallback
    if (id === 'openrouter-2') {
      const r = pick(ors, mid);
      Object.assign(m, priceOf(r) || {});
      m.priceSource = r ? `provider-depot source 'openrouter' (${depot.sources['openrouter'].sourceUrl}) fetchedAt ${fetchedAt}` : 'unmatched in depot catalog';
    } else if (id === 'new-provider') {
      let src = null;
      let r = pick(pr, mid);
      if (r) src = 'hf-router';
      if (!r) { r = pick(od, mid); if (r) src = 'models-dev'; }
      Object.assign(m, priceOf(r) || {});
      m.servedOnLiveNimCatalog = nim.some((n) => n.id === mid);
      m.priceSource = r ? `provider-depot source '${src}' fetchedAt ${fetchedAt}` : 'unmatched in depot catalog (no priced variant found)';
    } else if (id === 'minimax-m-plan') {
      let src = null;
      let r = pick(pr, `MiniMaxAI/${mid}`);
      if (r) src = 'hf-router';
      if (!r) { r = pick(od, `minimax/${mid.toLowerCase()}`); if (r) src = 'models-dev'; }
      Object.assign(m, priceOf(r) || {});
      m.servedOnLiveMiniMaxCatalog = mmax.some((n) => n.id === mid);
      m.priceSource = r ? `provider-depot source '${src}' fetchedAt ${fetchedAt}` : 'unmatched in depot catalog';
    } else if (id === 'account:zai-start-plan') {
      let src = null;
      let r = pick(pr, `zai-org/${mid}`);
      if (r) src = 'hf-router';
      if (!r) { r = pick(ors, `z-ai/${mid.toLowerCase()}`); if (r) src = 'openrouter'; }
      Object.assign(m, priceOf(r) || {});
      m.priceSource = r ? `provider-depot source '${src}' fetchedAt ${fetchedAt}` : 'unmatched in depot catalog';
    }
    return m;
  });

  const p = {
    providerId: id,
    providerName: rule.providerName || LABELS[id],
    templateId: rule.templateId || null,
    enabled: rule.enabled === undefined ? 'unset' : rule.enabled,
    config: redact(rule.config || {}),
    catalogOrderCount: order.length,
    catalogOrderModels: order.length > ids.length ? order : undefined,
    models,
  };
  if (probe) {
    const pp = (probe.providers || []).find((x) => x.id === id);
    p.status = pp?.liveStatus ?? 'not-probed';
    p.budget = pp?.budgetSignals ?? null;
    p.statusNotes = pp?.notes ?? null;
  } else {
    p.status = 'probe manifest not found at build time';
    p.budget = null;
  }
  capture.providers.push(p);
}

const outPath = path.join(here, `zcode-models-capture-${capture.capturedAt.slice(0, 10).replace(/-/g, '')}.json`);
fs.writeFileSync(outPath, JSON.stringify(capture, null, 1) + '\n');
console.log(`wrote ${outPath}`);
console.log(`providers: ${capture.providers.map((p) => `${p.providerId}(${p.models.length} models, status=${p.status})`).join(' | ')}`);
