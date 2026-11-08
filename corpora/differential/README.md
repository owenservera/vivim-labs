# Corpora / Differential — A→B and A→D compatibility lab

**Lane:** VIVIM Labs, Wave 3 booster **05** (differential A/B/D compatibility).
**Status:** characterisation corpus, complete and executable.
**Evidence level:** `real_module_integration` against a pinned TierFlow checkout.
**TierFlow source pin:** `owenservera/tierflow`, branch `main`,
**commit `ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7`**, local checkout
`C:/0-BlackBoxProject-0/vivim-ui`. The pin is asserted at runtime against the
on-disk `.git` metadata, so a moved checkout fails loudly instead of silently
describing a different source.

**Spec followed:** `VIVIM-VISUAL-RUNTIME/WAVE-2/INTEGRATION-2-CLOSURE-RESULT.md`
§6 — *"exact A/B and A/D incompatibilities (measured, not repaired)"*. Every
seam below is one that §6 already recorded as OPEN, or a loss that §6 did not
name and that this lab measured and reports as NEW.

## Files

| File | Role |
| --- | --- |
| `seams.mjs` | The seam catalogue: 19 entries, each with `a_source` and `b_or_d_source` as `file:line`, the exact mismatch, the failure mode and the lossiness verdict. |
| `differential.test.mjs` | The Bun test that drives the REAL Lane A → Lane B and Lane A → Lane D handoffs and asserts the characterised behaviour. |
| `README.md` | This file. |

## Run

```sh
cd C:/0-BlackBoxProject-0/vivim-labs
bun test corpora/differential/differential.test.mjs
```

Measured: **30 tests — 27 pass, 3 skipped (FAILING-BY-DESIGN), 0 fail, exit 0.**

Optional overrides (used by the knowledge leak proof, and available here):
`VIVIM_LABS_TIERFLOW_ROOT` points the suite at a different TierFlow checkout.

## How the real modules are reached

No TierFlow source is copied into this repository. The test resolves the pinned
checkout and imports the real modules directly:

```
corpora/differential/differential.test.mjs
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/document/validate.ts    (Lane A)
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/document/schema.ts       (Lane A)
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/components/catalog.ts    (Lane B)
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/render/dom.ts           (Lane D)
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/integration-2/action-adapter.mjs
```

**Direct cross-repository import worked** — no scratch harness, no copy, no
proxy was needed. Bun resolves the `.ts` entrypoints and their `.ts` internal
imports natively. The scratch directory `C:/0-BlackBoxProject-0/_scratch_l3`
was used only for throw-away probes and for the knowledge mutation proof.

## The seam map

`lossy` names the dimension that is lost: `canonical-version-ref`, `provenance`,
`content`, `semantic-role`, `semantic-focus-order`. `permission-added` would
mean the seam grants authority — **measured: none**.

| # | Seam | Mismatch (field / type) | Lossy | Mode | Code | Lane A source | Lane B/D source |
|---|---|---|---|---|---|---|---|
| 1 | A→B card description | `props.description` : string → key absent from closed prop list | no | refusal | `UNSAFE_PROP` | `document/validate.ts:72` | `components/catalog.ts:30` |
| 2 | A→B stack props | `props.direction`/`props.gap` : enum → absent from closed prop list | no | refusal | `UNSAFE_PROP` | `document/validate.ts:70` | `components/catalog.ts:29` |
| 3 | A→B grid props | `props.columns`/`props.gap` : number/enum → absent | no | refusal | `UNSAFE_PROP` | `document/validate.ts:71` | `components/catalog.ts:29` |
| 4 | A→B text prop | `props.value` (mandatory in A) vs `props.text` | no | refusal | `UNSAFE_PROP` | `document/validate.ts:80` | `components/catalog.ts:29` |
| 5 | A→B action ref | `{id,version}` object → inert `string[]` | no | refusal | `INVALID_COMPONENT` | `document/validate.ts:202` | `components/catalog.ts:129` |
| 6 | A→B focus order | `a11y.focusOrder` : number → key absent from B's a11y set | no | refusal | `INVALID_A11Y` | `document/validate.ts:181` | `components/accessibility.ts:33` |
| 7 | A→B role vocabulary | `a11y.role` : A enum → B per-type list | no | refusal | `INVALID_A11Y` | `document/validate.ts:182` | `components/accessibility.ts:24` |
| 8 | A→B layout vocabulary | `layout.gap` : `'sm'\|'md'\|'lg'` → `gapPx` : number | no | refusal | `INVALID_LAYOUT` | `document/validate.ts:176` | `components/layout.ts:29` |
| 9 | A→B node types | `collection`/`profile-card`/`project-card`/`timeline`/`chart` → not in B's catalogue | no | refusal | `UNREGISTERED_COMPONENT` | `document/schema.ts:12` | `components/catalog.ts:8` |
| 10 | A→B id alphabet | `node.id` `[A-Za-z][A-Za-z0-9._:-]{0,95}` → `[A-Za-z0-9_-]{1,64}` | no | refusal | `INVALID_COMPONENT` | `document/validate.ts:24` | `components/catalog.ts:27` |
| 11 | A→B binding count | `bindingRefs.length` 0..128 → 0..4 | no | refusal | `INVALID_COMPONENT` | `document/validate.ts:194` | `components/catalog.ts:130` |
| 12 | A→B binding payload | `knowledgeRef`/`provenanceRef`/`readScope`/`availability` → **no field in the B contract** | **yes — provenance** | silent-drop | — | `document/validate.ts:234` | `components/catalog.ts:18` |
| 13 | A→D action ref shape | `{id,version}` object → inert `string[]` | no | refusal (whole plan) | `INVALID_REFERENCE` | `document/validate.ts:202` | `render/dom.ts:164` |
| 14 | A→D canonical version ref | `{id:'fixture.preview',version:1}` → `'synthetic_ref_4_0'` alias; canonical pair survives only in the adapter's side table | **yes — canonical-version-ref** | silent-drop | — | `document/validate.ts:203` | `integration-2/action-adapter.mjs:19` |
| 15 | A→D binding provenance | `document.bindings` → never read by the renderer | **yes — provenance** | silent-drop | — | `document/validate.ts:234` | `render/dom.ts:28` |
| 16 | A→D card description | `props.description` allowed by the prop list, unread by the projector | **yes — content** | silent-drop | — | `document/validate.ts:72` | `render/dom.ts:122` |
| 17 | A→D focus order | authored `a11y.focusOrder` → Lane D's own traversal order | **yes — semantic-focus-order** | degrade | — | `document/schema.ts:63` | `render/dom.ts:216` |
| 18 | A→D role remap | `'group'` → `'region'` | **yes — semantic-role** | degrade | — | `document/validate.ts:182` | `render/dom.ts:209` |
| 19 | A→D permission | inert catalogue ref → `aria-disabled`/`tabindex`/`INERT_ACTION`, no dispatch path | **no — SAFE** | safe | — | `document/validate.ts:198` | `render/dom.ts:211` |

Totals: **19 seams — 10 refusal (fail-closed), 4 silent-drop, 2 degrade, 1 safe
(permission). 5 seams are lossy. 0 seams add permission.**

## Crosswalk mismatches found

**The A→B seam is a whole-vocabulary collision, not one property.** A Lane A
document cannot be rendered by Lane B at all: **every** node of the canonical
Lane A document is refused.

* `root` (stack, `props.direction`) → `UNSAFE_PROP`
* `panel` (card, `props.description`) → `UNSAFE_PROP` — the refusal §6 names
* `grid-1` (grid, `props.columns`) → `UNSAFE_PROP`
* `cell-a` (text, `props.value`) → `UNSAFE_PROP`
* `action-button` (button, object `actionRefs`) → `INVALID_COMPONENT`

Two node kinds can **never** satisfy Lane B, because Lane A makes the
conflicting field mandatory:

* **text** — Lane A requires `props.value`; Lane B only knows `props.text`.
* **button** — Lane A requires `a11y.focusOrder`; Lane B rejects the field.
  (Lane A also requires an object action ref; Lane B wants strings.)

The **only** Lane A-valid subset Lane B accepts is `stack`/`grid`/`card` nodes
with `props: {}` / `props: { title }` and a role Lane B knows — and even the
**document root** cannot pass, because Lane A forces the root to
`a11y.role = 'main'` (`validate.ts:211`) while Lane B's stack roles are
`{group, region}`.

**The A→D seam is a reference-shape collision, bridged but not closed.** The
raw Lane A document renders `unavailable` with issue `INVALID_REFERENCE`
(`dom.ts:164-165`), because Lane A's action references are `{id, version}`
objects and Lane D accepts only inert string lists (`dom.ts:107-109`). Through
the committed integrator adapter the document renders `partial` with
`SPATIAL_UNSUPPORTED` at 320 / 360 / 1280 — exactly as §6 records — and the
canonical version reference survives in the adapter's separate read-only table
as `{ id: 'fixture.preview', version: 1, inert: true }`.

## Lossy vs safe

**LOSSY (5 seams).** Three of these are new findings §6 does not name:

1. **canonical-version-ref** (#14) — the canonical `{id, version}` pair is *not*
   in the document Lane D renders. Only the synthetic alias is. The pair is
   recoverable only from the adapter's side table, which a consumer of the
   render plan alone does not have. §6 describes this arrangement; it does not
   call it a loss.
2. **provenance** (#12, #15) — the binding payload (`knowledgeRef`,
   `provenanceRef`, `readScope`, `availability`) never crosses into Lane B
   (no field for it) and is never read by Lane D (`RenderPlan` has no
   `bindings` field). At the D seam the loss is completely silent: the node
   renders `rendered` / `OK` with no marker.
3. **content** (#16), **semantic-role** (#18), **semantic-focus-order** (#17) —
   fidelity losses, not authority losses.

**SAFE (measured, no permission added).** Nothing at either seam grants,
implies or upgrades authority. Lane D renders the bound button with
`aria-disabled="true"`, `tabindex="0"` and
`aria-description="Read-only preview. No action can be performed here."`,
traces it as `INERT_ACTION`, and the plan contains no `execute`, `dispatch`,
handler, `href` or URL. The adapter adds no execute path and its version table
is explicitly `inert: true`. **This is the negative of the required check, and
it is asserted by a test, not claimed.**

## Characterisation vs FAILING-BY-DESIGN

Tests that pin current behaviour are labelled `characterisation: true` in a
comment and name the defect they pin.

Where the current behaviour loses information, the corpus also registers an
explicit **FAILING-BY-DESIGN** negative test. The harness works like this:

* the registered `test.skip` is the real negative test — its body asserts the
  **correct** behaviour, so un-skipping it reproduces the defect as a red test;
* the companion `FAILING-BY-DESIGN PROOF` test runs the same probe **now** and
  asserts that the defect **is present** at the pinned SHA. The suite stays
  green while the defect stays observable, instead of being hidden behind a
  skip.

Three negative tests are registered:

| Negative test | Correct behaviour it asserts | Currently fails because |
| --- | --- | --- |
| `A->B/binding-provenance` | Lane B exposes the provenance of the bindings it carries | `InspectedComponent` has only `bindingRefs: readonly string[]` (`catalog.ts:18`) |
| `A->D/canonical-version-ref` | the rendered document retains the canonical `{id, version}` | the renderer document holds only `synthetic_ref_4_0` (`action-adapter.mjs:19`) |
| `A->D/binding-provenance` | the render plan carries the binding provenance it renders | `RenderPlan` has no `bindings` field and `project()` reads only type/props/a11y/children (`dom.ts:28`, `dom.ts:186-237`) |

Each carries a comment pointing back at closure §6.

## TierFlow baseline measured for comparison (read-only)

| Command | Exit | Result |
| --- | --- | --- |
| `bun test tests/` | **0** | 449 pass / 0 fail / 1120 expect() across 35 files |
| `bun test tests/visual-runtime/` | **0** | 204 pass / 0 fail across 18 files |
| `bun test tests/visual-runtime/integration-2/` | **0** | 44 pass / 0 fail across 3 files |
| `bun test tests/visual-runtime/integration-2/real-a-h.test.ts` | **0** | 25 pass / 0 fail |
| `bun test tests/visual-runtime/integration-2/compose.test.ts` | **0** | 15 pass / 0 fail |

The 449 / 25 / 15 counts match the closure result §5 exactly, so the pinned
checkout is the one that closure measured. **No TierFlow test failed. Nothing
in TierFlow was modified.**

## What this corpus does NOT prove

* **Not a repair.** Nothing here fixes either seam. Both remain OPEN and owned
  by their lanes (§6: Lane B owns A→B; Lanes A and D own the A/D shape
  difference, the integrator owns the adapter).
* **Not device, Host, browser, Android or live-KB evidence.** No live Host, KB,
  Studio, Android, provider browser or device was contacted. No network call
  and no provider model call was made.
* **Not environment verified.** No Windows/Tauri, physical Android, signed-in
  provider or owner-scoped live KB was exercised. Evidence level stays
  `real_module_integration`.
* **Not production verified.** No owner-authorised release, no operational
  receipts.
* **Not a performance or scale claim.** One canonical document, one shape.
* **Not an approval of any behaviour.** A characterisation asserts *what the
  pinned source does*, never that it is *right*. Four of the five lossy seams
  are new observations and are offered as evidence for the owning lanes to
  decide on, not as verdicts.
* **The corpus cannot drift silently.** The SHA pin is asserted on every run,
  and the FAILING-BY-DESIGN proofs re-assert that the defects are still present.
  A TierFlow repair turns a PROOF test red until the corresponding negative
  test is un-skipped and passes — the corpus says so rather than going quiet.
