# S-semantic — isolated PRE-G SWC source-portability proof

This directory is **not** a production compiler, registry, permission system, new wiki writer or executor. All tracked files in this worker's commits live under `.integration/proposal5/phase2/pre-g/S-semantic/`. Branch: `work/p5-preg-semantic-wiki-r001`; base: `8b23c73bf450738a76b256bf0fa17ecbf86db591`.

## Layout

- `hq/lib/annotation-v2.mjs` — untouched donor parser, blob `7e94dfbd4fbca8ef0fdb9e0610053364ce2c38c8`.
- `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs` — untouched donor research compiler, blob `83c2eab7921c84dd55f0503518e7fc7e4a466657`.
- `research/semantic-wiki-compiler/test/semantic-wiki-compiler.test.mjs` — untouched donor original tests, blob `d92f10a21b12a0a5c145e4e20497edaa1fdb2c3f`.
- `semantic-ir.schema.json` — untouched donor schema, blob `a05a98cb354f73f78ddb1906590503e759ab699b`.
- `semantic-ir.json`, `fixtures/status.example.mjs` — exact inert example generated from actual HQ parser/compiler via isolated V8 source-smoke; not Node runtime acceptance.
- `inert-adapter.mjs` — separate negative-only precondition/refusal adapter using built-in Node crypto; does **not** parse action intent.
- `inert-adapter.test.mjs` — 16 Node 22 tests executed on byte-identical code locally.
- Mapping, crosswalk, baseline, owner tasks and ledger — review docs only.

## Run in a real merged/local Node checkout

```sh
cd .integration/proposal5/phase2/pre-g/S-semantic
node --test inert-adapter.test.mjs research/semantic-wiki-compiler/test/semantic-wiki-compiler.test.mjs
```

For source-reader-only smoke `semantic-ir.json` is inert and has no dynamic import, execution, receipt or authority. Do **not** run SWC over private corpora or point fixture bindings at live writes. No node_modules, lockfile or network requests are required.

**Important limitations:** adapter hash checking proves caller byte agreement only, not trusted repository provenance. Its unsupported-v3 regex is intentionally conservative and can reject a tag spelled inside a quoted string; it does not parse v3. Host source digest matches the inspected frozen B file, but the fixture uses a one-action projection, not a complete trusted Host catalog. The original SWC research suite was copied but **not run in Node in this session**; see `RUN-LEDGER.json`.
