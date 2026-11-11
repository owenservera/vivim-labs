// Integration ports. Bind to production owners; no direct browser/CDP or graph writes.
import { createT0Coordinator } from './t0-core.mjs';
export const HOST_PORTS = Object.freeze({
  onboarding:'apps/vivim-host/onboarding/service.mjs',
  scheduler:'apps/vivim-host/bootstrap/scheduler.mjs',
  studioOwner:'studio/src/studio/bootstrap-owner-adapter.mjs',
  chatIndex:'studio/src/capture/chatgpt-chat-index-reader.mjs',
  world:'studio/src/studio/world-service.mjs',
  rust:'repos-hq/research/rust-knowledge-graph/src/scoped_graph.rs'
});
// Production must adapt the existing Host ledger with an atomic read-modify-write
// transaction. In-memory implementation is ONLY a synthetic test fixture.
export function createMemoryTestStore(){
  const values=new Map();
  return {
    read:key=>values.has(key)?structuredClone(values.get(key)):null,
    transact(key,update){const value=update(this.read(key));values.set(key,structuredClone(value));return structuredClone(value);}
  };
}
export function createLabsFixture({worldId='personal-test',accountId='chatgpt-test',now=()=>Date.UTC(2026,10,11,12)}={}){
  return createT0Coordinator({store:createMemoryTestStore(),worldId,accountId,now});
}
// Real Host integration should provide:
// 1. An existing, durable transaction-backed store port.
// 2. A fresh qualified ChatGPT account binding before T=0.
// 3. Explicit owner grant approval for every operation.
// 4. Studio-owned effect dispatch with exact action ID and digest.
// 5. Verified receipt reconciliation, never blind retry.
// 6. Host evidence commit before recordBatch(acceptedByHost=true).
// 7. Rust scoped-graph change sets applied by Host with revision CAS.
// 8. TierFlow read projections; NO standalone Labs UI authority.
