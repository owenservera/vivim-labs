import { test } from "node:test";
import assert from "node:assert/strict";
import { summarizeClaims, summarizeIssue, summarizeReceipts } from "./orchestrator-watch.ts";
test("canonical issue ledger counts only checked and open stable IDs",()=>{
 const issue=["- [x] **R05** — done","- [ ] **A01** — open","- [X] **S01** — done","- [ ] **O06** — open","- [ ] irrelevant"].join("\n");
 assert.deepEqual(summarizeIssue(issue),{total:4,closed:2,open:2});
});
test("active owned claim is reported, released and offered work is not",()=>{
 const body=["| Task | Owner | Scope | State |",
 "| --- | --- | --- | --- |",
 "| A16 | **ChatGPT L01** | Studio test | **ACTIVE / CLAIMED** |",
 "| U05 | **L03** | Wiki test | **CLAIMED** |",
 "| S11 | **L02** | PWA | RELEASED / PASS |",
 "| VF-001 | PLANNER OFFERED_TO:L05 | Docs | OFFERED_TO |",
 "| PY-001 | ChatGPT Dev | Work | ACTIVE |"].join("\n");
 const x=summarizeClaims(body);
 assert.equal(x.L01.length,1);assert.equal(x.L01[0].task,"A16");
 assert.equal(x.L03.length,1);
 assert.equal(x.L02.length,0);assert.equal(x.L05.length,0);
});
test("latest three public issue receipts per lane, not proof of live sessions",()=>{
 const comments=Array.from({length:5},(_,i)=>({id:10+i,created_at:"2026-11-23",body:"L04 U08 — source-tested "+i}));
 comments.push({id:15,body:"L01 A16 negative test"});
 const x=summarizeReceipts(comments);
 assert.deepEqual(x.L04.map(v=>v.id),[12,13,14]);
 assert.equal(x.L01[0].id,15);
 assert.equal(x.L02.length,0);
});
