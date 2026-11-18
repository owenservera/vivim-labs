# F → Z1 / G1 integration-owner commands

**BASE_SHA:** `f9823a5131e73e7562db5a4ec14e1ec481557d69`. **F branch:** `work/p5-hvi-w1-f-verify-r001`. **No main merge performed.**

G1 should inspect exact F head, compare against BASE_SHA and verify **every changed file** is inside these three write roots. Then stage A, B, C, D, E, F serially under G integration branch; do not cherry-pick an F oracle as a production HVI implementation.

```sh
git fetch origin work/p5-hvi-w1-f-verify-r001
git diff --name-status f9823a5131e73e7562db5a4ec14e1ec481557d69 origin/work/p5-hvi-w1-f-verify-r001
# After authorized G1 serial staging of A-E and F:
node --version
node --test tests/hq-version-intelligence/*.test.mjs
node --test hq/test/catalogue-delta.test.mjs tests/hq-version-intelligence/*.test.mjs
```

**Combined G1 additions:** actual A-E source pin + N-way API tests with offline manifest replay; completeness, 361 individual path dispositions, source authority, source grant and malicious-path cases. Compare versioned wire JSON against schemas via a full Draft 2020-12 validator. Record exact checkout SHA, OS, tool versions, stdout, stderr, exit. If not run, report UNRUN. Host G2 must enforce mid-operation grant revocation independently; F schema and oracle are not a substitute.

**Blocker owners:** A source/symlink/pin drift; B identity receipt and collisions; C N-way/replay; D migration completeness; E export privacy; H Host grant World isolation; Z1 referee; G1 integration+Node24; G2 live read/revoke and UI; separate proposal G beta lock/device owners.

**Proposed disposition:** `PREPARED_NOT_PROMOTED`, independent worker source and evidence only. Do not issue `GO_FOR_W2` or `BETA_GO` here.
