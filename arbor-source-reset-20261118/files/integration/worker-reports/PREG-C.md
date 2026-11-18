# ARBOR Proposal 5 — PRE-G FINAL FREEZE — LANE C

**Verdict:** FREEZE_READY **for serial source integration only**; **NOT** build, security, provider, or device accepted.  
**Owner:** C — Studio and signed-in browser Harness  
**Repository:** `owenservera/ARBOR`  
**Frozen worker branch:** `work/p5-w01-c-studio`  
**Final frozen HEAD:** the commit adding **this** `PREG-C.md` file (self-referential SHA intentionally resolved by GitHub commit result; parent / pre-freeze HEAD `be2567f0b0d0e4d1defc3740888712548c2955a0`). Read `refs/heads/work/p5-w01-c-studio` to obtain exact final tip; do not use this report's parent SHA as tip.  
**Original common base:** `dd245d78ff3c110ba8630cad557ea0479c51a635`; the worker was not cut from an approved G0 integrate branch.  
**Donor repo/ref:** `owenservera/vivim-web-harness@main`, frozen SHA `cae97afe848d79957c9d6c698a90996a758bfd56`.  
**Physical source import terminal SHA:** `4bca551d58a1972cc02a8b20b772e1e2417b874d`.  
**Canonical migration report:** [W01-C.md](W01-C.md) (commit `be2567f0b0d0e4d1defc3740888712548c2955a0`).  
**Exhaustive path/blob manifest:** [W01-C-SOURCE-BLOBS.tsv](W01-C-SOURCE-BLOBS.tsv) (commit `d151a3e217e2abca173a1db1f42cc83d92730bb9`).

## All observed lane C commits, including the first focused follow-up period

| Commit SHA | Kind | Evidence |
|---|---|---|
| `52ece6d020e452145cca4b98d52234525c40ff6f` | SOURCE_IMPORTED | First 15 `harness/` files |
| `aacbf98eee0e06171f6a4fad0141399f45ae65f4` | SOURCE_IMPORTED | 113 further harness files and tests |
| `7b9305c8dc1a43835c6ac41197b5913657d8db7e` | SOURCE_IMPORTED | Entire checked-in `studio/src/`: 269 exact source blobs |
| `4bca551d58a1972cc02a8b20b772e1e2417b874d` | SOURCE_IMPORTED | Studio CLI/scripts/asset/fixture files: 104 |
| `d151a3e217e2abca173a1db1f42cc83d92730bb9` | REPORT_ONLY | 1,090-path complete donor/worker source inventory |
| `be2567f0b0d0e4d1defc3740888712548c2955a0` | REPORT_ONLY | W01-C report with module, test and cross-owner ledger |
| **this file's commit** | REPORT_ONLY | Final freeze receipt; exact SHA is branch tip |

The earlier first focused follow-up did **not** produce additional committed source changes beyond the four physical imports; no hidden migration or executed tests are implied. All subsequent changes on this branch are evidence-only. No code patch was made to imported Studio or harness.

## Full source truth

- Source tree examined via GitHub recursive Git tree of the frozen donor (not a 300-file capped compare): **1,090 donor blobs** under `studio/` and `harness/`.
- **501 source-preserving imported blobs**, all 501 have exactly the same Git blob ID as donor; **0 content-modified imported blobs**.
- **589 omitted donor blobs**, every one recorded with original path, blob ID and reason in `W01-C-SOURCE-BLOBS.tsv`. Most material omissions: **305 Studio tests**, 87 Android-only files, 176 Studio/Harness documentation files, 11 Studio agent configs and 10 other documentation/metadata/marketing files. Omitted Studio tests are **not** claimed migrated, verified or retired.
- Physical source owners: `harness/src/` (59), `harness/test/` (51), `studio/src/` (269), `studio/bin/` (39), `studio/scripts/` (41), plus 42 manifests, assets, profile fixture descriptors, and ancillary support files; precise path counts and blobs appear in W01-C.
- Checked-in `studio/profiles/*.json` are selector descriptors, not personal Chrome profiles. No donor runtime credentials, signed-in browser user-data directory, session logs/captures, local personal DB or generated output was imported into the worker code tree. Static inspection is **not** a full secret scan or release audit.
- A prior exploratory staging operation encountered a potential token-pattern match in uncommitted `studio/test/unit/stream-guard.test.mjs`. Treat as an **unresolved manual fixture-review caution**, not proof of a credential leak and not a test failure. The file was not imported.

## Source capabilities and authority

Preserved source includes Chrome dedicated `--user-data-dir`, loopback CDP and browser ownership; exact provider conversation targeting; Studio turn journal, typed-prompt ledger, driver lease, provider preflight, no blind resends after uncertain effect; browser L1/L2/L3 capture, network/DOM/stream and evidence adapters; harness journal/sendOnce and provider descriptors. This is **SOURCE_IMPORTED**, not a live-runtime guarantee.

ChatGPT and Claude: measured selectors and provider thread patterns in donor source. Gemini: prior donor observations document warm-tab interaction, cold-tab composer failure, and a Studio drive thread-mapping gap. None of ChatGPT/Claude/Gemini was tested in an ARBOR signed-in browser by this lane. Other seeded provider descriptors do not authorize newly supported products. No provider API, Playwright/Puppeteer, replacement browser engine, or parallel effect authority added.

**Authority ownership:** B Host is sole authorized external effect/World/journal/receipt owner, including admission and risk semantics. C owns signed-in browser mechanism and source-preserving local browser/turn-idempotency fencing, **not** a new Host claim authority. F owns common interop/schema evidence; A native authentication; E mobile/TierFlow; G cross-lane linker/root manifests, packaging and build acceptance. The imported Studio/harness copies must not automatically create a second privileged writer.

## Tests actually executed (versus UNRUN)

| Verification | Result | Exit code |
|---|---|---|
| Donor and ARBOR Git recursive tree enumeration | EXECUTED: 1,090 donor; 501 landed | Not applicable (GitHub API) |
| Per-path source Git blob identity comparison | EXECUTED: 501 identical; zero changed | Not applicable (GitHub API) |
| `node --test` (Studio or harness) | **UNRUN** | None |
| `bun test`, Node module import/CLI smoke, static syntax check | **UNRUN** | None |
| Install/link/build combined Rust+Bun+Node+Studio | **UNRUN** | None |
| Signed-in Chrome/CDP and ChatGPT/Claude/Gemini thread read/send/stream | **UNRUN** | None |
| Adversarial uncertain-send/recovery, Host effect parity | **UNRUN** | None |
| Windows installer; real phone/PWA; owner credential/security acceptance | **UNRUN** | None |
| Donor-absent standalone ARBOR test | **UNRUN** | None |

Do not translate recursive Git reads, static source descriptions, or presence of test files into an executable test PASS. Do not invent OS, runtime, browser, observed exit code, or provider credentials.

## Required G handoff and G/W2 classifications

| ID | Owner(s) | Classification and requirement |
|---|---|---|
| C-G1-01 | G/B/C | **SOURCE_IMPORTED / G1 linkage pending**: Host imports `studio/src/config.mjs`, `studio/src/workspace-manifest.mjs`, `studio/bin/studio.mjs`, `studio/src/studio/bridge-commands.mjs`, `bootstrap-owner-adapter.mjs`, `relay-journal-correlation.mjs`; verify closure inside one ARBOR tree and without donor processes |
| C-G1-02 | G | **BUILD_UNRUN**: root Bun/Node workspace, dependency lock, Node >=22 Studio/Harness with possible Node24 combined requirement, Windows executable inclusion; C did not edit root files |
| C-G1-03 | G/F/C | **TEST_GAP**: 305 donor `studio/test/` blobs are omitted. Choose required tests explicitly, inspect possible fixture token false-positive, stage only safe fixtures and run source suite; harness's 51 existing test files are imported but UNRUN |
| C-G1-04 | G/B/F | **SINGLE_EFFECT_AUTHORITY**: Host claim/approval/receipt always wins, Studio journal and harness sendOnce remain local replay fences; never interpret UNKNOWN as safe retry, no duplicated writer |
| C-W2-01 | A/B/G | **P0 S1 SECURITY_UNFIXED / BUILD_UNRUN**: authenticated native child identity, no owner bearer handed to unverified localhost, patched Tauri scopes. Studio CDP owner proof is not this security fix |
| C-W2-02 | B/E/F/G | **P0 S2 DEVICE_UNRUN**: browser PWA signer/Host enrollment contract and bearer replay review are separate owners; don't assume physical PWA works |
| C-W2-03 | C/B/F/G | **PROVIDER_UNRUN**: exact-thread ChatGPT and Claude smoke, Gemini warm/cold-tab and drive-path proof, streaming/capture attribution and uncertain-send adversarial tests with authorized owner browser |
| C-W2-04 | G | **NO_DONOR_BUILD_UNRUN**: packaging and service launch after unavailable donor source tree; no runtime path to donor is allowed |
| C-W2-05 | F/G | **CONTRACT_TEST_UNRUN**: align source identities, risk, receipt/effect status and evidence with B-owned Host and F interop; avoid protocol/catalog forks |

**Explicit classifications:** `SOURCE_IMPORTED=501`; `SOURCE_PATCHED=0` (only reports added); `BUILD_UNRUN=true`; `DEVICE_UNRUN=true`; `LIVE_PROVIDER_ACCEPTANCE_UNRUN=true`; `SECURITY_ACCEPTANCE_UNRUN=true`. Initial donor imports are source-only readiness, not beta GO.

## Freeze instruction

Lane C has completed the final required source evidence and intentionally **stops writing after this commit**. Master G/referee should record the branch HEAD exactly, import it serially with other frozen lanes, retain this inventory/report verbatim, and mark outstanding runtime/security/device proof gates as UNRUN until executed. **FREEZE_READY** means branch/report frozen for integration, not implementation release ready.
