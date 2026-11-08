import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,existsSync} from 'node:fs';
import {resolve,join,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
const here=dirname(fileURLToPath(import.meta.url));
const root=resolve(here,'..','..'); // SOFTWARE_HQ/scripts -> sibling repository root
const read=(repo,path)=>readFileSync(join(root,repo,path),'utf8');
const contract=JSON.parse(read('repos-hq','contracts/chronos/contract.v1.json'));

test('Chronos version/slot and operations match Rust + CLI source',()=>{
  const core=read('repos-hq','research/rust-knowledge-graph/src/chronos.rs');
  const cli=read('repos-hq','research/rust-knowledge-graph/src/bin/hqkg-time.rs');
  assert.equal(contract.rust.interface,'vivim.chronos/1');
  assert.ok(core.includes('pub const INTERFACE: &str = "vivim.chronos/1"'));
  assert.ok(core.includes('pub const SLOT: &str = "utility:chronos"'));
  for(const op of contract.operations){
    assert.ok(core.includes(`"${op}"`),`${op} missing from core`);
    assert.ok(cli.includes(`\\"${op}\\"`)||cli.includes(`"${op}"`),`${op} missing from cli`);
  }
});

test('all consumer adapters use canonical read-model versions and prohibit effects',()=>{
  const host=read('vivim-web-harness','studio/src/studio/chronos-conversation-projection.mjs');
  const tier=read('tierflow','src/visual-runtime/host/chronos-read-projection.ts');
  const supervisor=read('repos-hq','hq/lib/chronos/supervisor-timeline.mjs');
  assert.ok(host.includes(contract.readModels.conversation));
  assert.ok(supervisor.includes(contract.readModels.supervisor));
  assert.ok(tier.includes(contract.readModels.tierflow));
  for(const file of [host,tier,supervisor]) assert.ok(file.includes('effectAllowed:false')||file.includes('effectAllowed: false'));
});

test('integration plan never creates another engine scheduler or executable permission',()=>{
  const rust=read('repos-hq','research/rust-knowledge-graph/src/chronos.rs');
  assert.ok(rust.includes('impl ComputeEngine for ChronosCompute'));
  assert.equal(/\bpub struct Scheduler\b/.test(rust),false);
  assert.equal(/\bCommand::new\b|std::process::Command|std::net::|reqwest::/.test(rust),false);
  assert.equal(contract.nonAuthority.timeReadCanSend,false);
});
