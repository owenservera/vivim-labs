import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join, dirname } from "node:path";
import { checkCoverage, expandIssueIds, parseIssueRows, validate } from "./validate.ts";
const root=dirname(fileURLToPath(import.meta.url));
const doc=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));
const open=doc.lanes.flatMap(l=>l.issueIds.flatMap(expandIssueIds));
const historicalDone=["S01","G01","Z01","Z05","Z06","R02","R05"];
const issue= [...open.map(x=>"- [ ] **"+x+"** — open"),...historicalDone.map(x=>"- [x] **"+x+"** — done")].join("\n");
test("R002 exact ownership: all 64 open master IDs assigned once",()=>{
  const report=checkCoverage(doc,issue);
  assert.equal(report.open,64);
  assert.equal(report.checked,7);
  assert.equal(report.verifiedAgainstIssue,true);
  assert.deepEqual(report.errors,[]);
});
test("all launch assets exist; no automatic local model or source writer",()=>{
  const report=validate(doc,issue);
  assert.deepEqual(report.issues,[]);
  assert.equal(report.coverage.verifiedAgainstIssue,true);
  assert.equal(report.filesChecked,42);
});
test("duplicate lane owner fails explicitly",()=>{
  const clone=structuredClone(doc);
  clone.lanes[1].issueIds.push("A01");
  assert.ok(checkCoverage(clone,issue).errors.some(x=>x.startsWith("DUPLICATE_PRIMARY:A01:")));
});
test("omitted open item fails explicitly",()=>{
  const clone=structuredClone(doc);
  clone.lanes[0].issueIds=["A02-A16","C01-C03"];
  assert.ok(checkCoverage(clone,issue).errors.includes("UNASSIGNED_OPEN:A01"));
});
test("historically closed ID is rejected if re-queued",()=>{
  const clone=structuredClone(doc);
  clone.lanes[0].issueIds.push("S01");
  assert.ok(checkCoverage(clone,issue).errors.includes("ALREADY_CHECKED:S01"));
});
test("invalid cross-prefix or reversed ranges are rejected",()=>{
  assert.throws(()=>expandIssueIds("A08-B10"),/CROSS_PREFIX_RANGE/);
  assert.throws(()=>expandIssueIds("A09-A01"),/INVALID_RANGE_BOUNDS/);
  assert.throws(()=>expandIssueIds("A1"),/INVALID_RANGE/);
});
test("unreadable or duplicate canonical issue blocks acceptance",()=>{
  assert.ok(checkCoverage(doc,"not the #33 body").errors.includes("EMPTY_OR_UNPARSEABLE_ISSUE_BODY"));
  const doubled=issue+"\n- [ ] **A01** — duplicate";
  assert.ok(checkCoverage(doc,doubled).errors.includes("DUPLICATE_CANONICAL:A01"));
});
test("without live issue body, report coverage as unverified, not passed",()=>{
  const report=checkCoverage(doc);
  assert.equal(report.verifiedAgainstIssue,false);
  assert.equal(report.open,64);
});
test("canonical parser preserves checkboxes and IDs",()=>{
  assert.deepEqual(parseIssueRows("- [ ] **A01** — x\n- [x] **R05** — y"),[
    {id:"A01",checked:false},{id:"R05",checked:true}
  ]);
});
