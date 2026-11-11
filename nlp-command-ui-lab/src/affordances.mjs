/** Pure presentation projection over discovered source-owned command descriptors.
 * This is NOT a new command registry, a permission mapping, or an executable instruction.
 * No source owner needs to accept a new icon to be represented in the UI lab.
 */
export const AFFORDANCE_SCHEMA='vivim.labs.visual-affordances/1';
const ICONS=Object.freeze({
 email:'✉',mail:'✉',contact:'♙',contacts:'♙',control:'⌘',system:'◉',devices:'▣',
 network:'◎',appearance:'◐',personalization:'✧',privacy:'◇',machine:'▤',browser:'◎',
 ui:'▧',file:'▤',chat:'☷',message:'▤',search:'⌕',status:'◉',automation:'⏱',
 'hq-cli':'⌥',wiki:'☷',capability:'◈',other:'◈'
});
/** Readable per-command identity mark. IDs remain authoritative; marks are generated UI hints. */
export function sigilFor(record) {
 const id=String(record?.namespace||'unknown')+':'+String(record?.id||'unknown');
 let hash=2166136261;
 for(let i=0;i<id.length;i++){hash^=id.charCodeAt(i);hash=Math.imul(hash,16777619);}
 const end=String(record?.id||'?').split('.').pop().replace(/[^a-z0-9]/gi,'').toUpperCase().slice(0,1)||'X';
 return end+(hash>>>0).toString(36).toUpperCase().padStart(7,'0').slice(-3);
}
const ROLES=Object.freeze({
 R0:'read',R1:'change',R2:'restricted',R3:'restricted',low:'read',medium:'change',high:'restricted',unknown:'unknown'
});

export function affordanceFor(record) {
 const category=String(record?.category||'other').toLowerCase();
 const namespace=String(record?.namespace||'unknown');
 const icon=ICONS[category]||ICONS[namespace]||'◈';
 const role=ROLES[record?.risk]||'unknown';
 const sourceStatus=String(record?.readiness||'unknown');
 const evidence=record?.liveVerified===true?'accepted_runtime_claim':sourceStatus==='source_wired'?'source_wired_only':'source_observation_only';
 const border=evidence==='accepted_runtime_claim'?'solid':evidence==='source_wired_only'?'dashed':'dotted';
 const tone=role==='restricted'?'caution':role==='change'?'amber':role==='read'?'jade':'neutral';
 return {schema:AFFORDANCE_SCHEMA,icon,sigil:sigilFor(record),role,tone,border,namespace,
   accessibleLabel:String(record?.title||record?.id||'Unknown command').slice(0,150),
   description:`${namespace} · ${record?.id||'unknown'} · ${sourceStatus}`,
   semantics:'presentation_only',evidence};
}

export function decorateActions(actions=[]) {
 const seen=new Set();
 return [...actions].map(a=>{
   const base=affordanceFor(a);
   // This is a within-snapshot uniqueness guard, not a universal semantic identity.
   let sigil=base.sigil,increment=2;
   while(seen.has(sigil))sigil=base.sigil+'-'+increment++;
   seen.add(sigil);
   return {...a,visual:{...base,sigil}};
 });
}