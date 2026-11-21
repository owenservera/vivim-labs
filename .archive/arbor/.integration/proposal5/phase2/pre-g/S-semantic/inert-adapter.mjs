// S-semantic local proof adapter. A REFUSAL gate, not a parser, planner, permission grant or executor.
import { createHash } from 'node:crypto';
const sha = s => createHash('sha256').update(s, 'utf8').digest('hex');
const id = /^[a-z0-9][a-z0-9._-]*@\d+(?:\.\d+)*$/;
const hex = /^[0-9a-f]{64}$/i;
const v3 = /@hq:(?:utterance|frame|slot|lexicon|visual|explain|semantics|semantic\/3)\b/;
const obj = v => v !== null && typeof v === 'object' && !Array.isArray(v);
function refused(code) { return {ok:false,code,effectPerformed:false,authority:'none'}; }
export function prepareInertCompilerInput({sources,hostCatalog,expectedCatalog,bindings=[],aliases=[]}={}) {
  if (!Array.isArray(sources) || !obj(hostCatalog) || !obj(expectedCatalog) || !Array.isArray(bindings) || !Array.isArray(aliases)) return refused('INVALID_INPUT');
  if (!Number.isInteger(expectedCatalog.version) || !hex.test(expectedCatalog.sourceDigest??'') || hostCatalog.version !== expectedCatalog.version || hostCatalog.sourceDigest !== expectedCatalog.sourceDigest) return refused('STALE_CATALOG');
  const sourceIds = new Set();
  for (const s of sources) {
    if (!obj(s) || typeof s.sourceText !== 'string' || typeof s.contentHash !== 'string' || !hex.test(s.contentHash) || typeof s.sourceId !== 'string' || sourceIds.has(s.sourceId)) return refused('INVALID_SOURCE');
    sourceIds.add(s.sourceId);
    if (sha(s.sourceText) !== s.contentHash.toLowerCase()) return refused('SOURCE_DIGEST_MISMATCH');
    // Intentionally conservative: an unsupported v3 string-literal mention can also refuse.
    // A true lexer is a future versioned owner-reviewed grammar change, not this gate.
    if (v3.test(s.sourceText)) return refused('UNSUPPORTED_V3_TAG');
  }
  const bindingIds = new Set();
  for (const b of bindings) {
    if (!obj(b) || !id.test(b.semanticAction??'') || !b.hostActionId || !b.owner || bindingIds.has(b.semanticAction)) return refused('DUPLICATE_OR_INVALID_BINDING');
    bindingIds.add(b.semanticAction);
    const match = hostCatalog.actions?.find(a => a.id === b.hostActionId);
    if (!match || match.owner !== b.owner) return refused('MISSING_OR_WRONG_HOST_ACTION');
  }
  const aliasOwners = new Map();
  for (const a of aliases) {
    if (!obj(a) || typeof a.alias !== 'string' || !id.test(a.semanticId??'')) return refused('INVALID_ALIAS');
    const key = a.alias.toLowerCase().normalize('NFKC').trim();
    if (!key || (aliasOwners.has(key) && aliasOwners.get(key)!==a.semanticId)) return refused('CONFLICTING_ALIAS');
    aliasOwners.set(key,a.semanticId);
  }
  return {ok:true,code:'SOURCE_CONSISTENCY_ONLY',input:{sources,hostCatalog,bindings},effectPerformed:false,authority:'none',sourceAuthenticityVerified:false,catalogAuthenticityVerified:false};
}
// Negative-only text boundary. Never maps natural language to an action or grants a Host intent.
export function guardHumanRequest({text,hostOutcome=null,meanings=[]}={}) {
  if(hostOutcome==='refused') return refused('HOST_REFUSED');
  if(hostOutcome==='clarification_required') return refused('HOST_CLARIFICATION');
  if(typeof text!=='string'||!text.trim()||text.length>1000) return refused('INVALID_TEXT');
  if(/\b(?:do\s+not|don['’]?t|never|not)\b/i.test(text)) return refused('NEGATED_SCOPE_UNRESOLVED');
  if(/^\s*["'“‘].*["'”’]\s*$/s.test(text)) return refused('QUOTED_ONLY_NO_COMMAND');
  if(!Array.isArray(meanings)||meanings.length!==1) return refused('AMBIGUOUS_OR_UNRESOLVED');
  return {ok:false,code:'REQUIRES_HOST_ORIGINAL_TEXT_REINTERPRETATION',effectPerformed:false,authority:'none'};
}
