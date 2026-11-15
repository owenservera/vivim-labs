// OpenRouter public metadata policy; zero paid inference by construction.
export const PRIORITIES = Object.freeze({
  planner: ['nvidia/nemotron-3-ultra:free', 'baidu/ernie-5.5-flash-preview:free'],
  coder: ['particle-ai/particle-1:free', 'cohere/north-mini-code:free'],
  reviewer: ['baidu/ernie-5.5-flash-preview:free', 'nvidia/nemotron-3-ultra:free'],
  triage: ['nvidia/nemotron-3.5-lightning:free', 'cohere/north-mini-code:free'],
});
export function freeTextModel(model) {
  if (!model || typeof model.id !== 'string' || !model.id.endsWith(':free')) return false;
  const p = model.pricing || {};
  if (p.prompt == null || p.completion == null || Number(p.prompt) !== 0 || Number(p.completion) !== 0) return false;
  // Conservative when explicit input/output data says not text-capable.
  const input = model.architecture?.input_modalities;
  const output = model.architecture?.output_modalities;
  if (Array.isArray(input) && !input.includes('text')) return false;
  if (Array.isArray(output) && !output.includes('text')) return false;
  if (Array.isArray(model.supported_parameters) && !model.supported_parameters.includes('tools')) return false;
  return true;
}
export function chooseRoles(catalog) {
  if (!Array.isArray(catalog)) throw new TypeError('Catalog must be an array');
  const entries = new Map(catalog.filter(freeTextModel).map(m => [m.id,m]));
  const result = {};
  for (const [role, order] of Object.entries(PRIORITIES)) {
    const chosen = order.find(id => entries.has(id));
    if (!chosen) throw Object.assign(new Error(`No verified free model for ${role}; refusing paid or unknown fallback`), { code:'NO_ELIGIBLE_FREE_MODEL', role });
    result[role] = chosen;
  }
  return result;
}
export function validateProviderId(s) {
  if (typeof s !== 'string' || !/^[A-Za-z0-9][A-Za-z0-9_-]{0,62}$/.test(s)) throw new Error('Invalid provider id');
  return s;
}
export function renderTemplate(template, roles, providerId) {
  validateProviderId(providerId);
  const tokens = {__MODEL_PLANNER__:roles.planner, __MODEL_CODER__:roles.coder,
    __MODEL_REVIEWER__:roles.reviewer, __MODEL_TRIAGE__:roles.triage};
  let out=template;
  for (const [placeholder,id] of Object.entries(tokens)) {
    if (!/^[a-z0-9][a-z0-9./_-]*:free$/.test(id)) throw new Error('Untrusted model ID for '+placeholder);
    if (!out.includes(placeholder)) throw new Error('Missing template placeholder '+placeholder);
    out=out.replaceAll(placeholder,providerId+'/'+id);
  }
  if (/__MODEL_[A-Z_]+__/.test(out)) throw new Error('Unresolved model token');
  return out;
}