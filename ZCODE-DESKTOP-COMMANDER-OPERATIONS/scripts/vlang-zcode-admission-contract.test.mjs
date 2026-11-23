// Z03: no-effect contract tests for the existing manual pilot. No ZCode launch.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const ps=readFileSync(new URL('./vlang-zcode-pilot.ps1',import.meta.url),'utf8');

test('all launches demand an absolute owner-supplied receipt path',()=>{
 assert.match(ps,/\[string\]\$AdmissionReceipt=''/);
 assert.match(ps,/MODEL_ADMISSION_RECEIPT_REQUIRED/);
 assert.match(ps,/IsPathRooted\(\$ReceiptPath\)/);
});
test('pilot invokes existing ARBOR read-only verifier, not a second router',()=>{
 assert.match(ps,/scripts\\zcode-model-session-admission\.mjs/);
 assert.match(ps,/& node \$gate --verify \$ReceiptPath \$SelectedSha/);
 assert.match(ps,/MODEL_ADMISSION_UNVERIFIED/);
});
test('direct child checks clean exact source before invoking ZCode',()=>{
 const child=ps.slice(ps.indexOf("if($Action -eq 'child'){"),ps.indexOf("$latest="));
 assert.match(child,/CHILD_SOURCE_SHA_OR_TREE_MISMATCH/);
 assert.match(child,/--untracked-files=all/);
 assert.ok(child.indexOf('Assert-ModelAdmission')<child.indexOf('& zcode '));
});
test('manual launch admits model before state marker or process creation',()=>{
 const launch=ps.slice(ps.indexOf("if(-not (Test-Path $latest))"));
 assert.ok(launch.indexOf('Assert-ModelAdmission')<launch.indexOf('MODEL_PILOT_ALREADY_ATTEMPTED'));
 assert.ok(launch.indexOf('Assert-ModelAdmission')<launch.indexOf('Start-Process'));
 assert.match(launch,/'-AdmissionReceipt'/);
});
test('status remains read-only and never invokes a model',()=>{
 const status=ps.slice(ps.indexOf("if($Action -eq 'status'){"),ps.indexOf("if(-not (Test-Path $latest))"));
 assert.doesNotMatch(status,/Assert-ModelAdmission|Start-Process|& zcode/);
});
