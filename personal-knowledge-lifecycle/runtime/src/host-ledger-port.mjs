// Binding to EXISTING Host bootstrap/ledger.mjs meta+transaction API.
// This creates no new database. Node 22's Host ledger must remain the single writer.
export function createHostLedgerStore({ledger}={}){
  if(typeof ledger?.transaction!=='function'||typeof ledger?.meta?.get!=='function'||typeof ledger?.meta?.set!=='function')throw new TypeError('Existing Host ledger required');
  return Object.freeze({
    read(key){const raw=ledger.meta.get(key);return raw===null||raw===undefined?null:JSON.parse(raw);},
    transact(key,update){return ledger.transaction(()=>{
      const current=ledger.meta.get(key);
      const next=update(current===null||current===undefined?null:JSON.parse(current));
      if(next===undefined)throw new Error('Host store callback must return new state');
      ledger.meta.set(key,JSON.stringify(next));
      return structuredClone(next);
    });}
  });
}
