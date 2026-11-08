# Corpora / Knowledge — provenance, redaction and revocation fixtures

**Lane:** VIVIM Labs, Wave 3 booster **07** (provenance / redaction / tombstone corpus).
**Status:** characterisation corpus, complete and executable.
**TierFlow source pin:** `owenservera/tierflow`, branch `main`,
**commit `ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7`**, local checkout
`C:/0-BlackBoxProject-0/vivim-ui`. The pin is asserted at runtime against the
on-disk `.git` metadata.

**SYNTHETIC FIXTURES ONLY.** Every row in `fixtures.mjs` is invented here. No
live KB, no owner knowledge, no private corpus, no FTS index, no provider and
no network were contacted. Nothing in this corpus may be presented as live-KB
evidence.

## Files

| File | Role |
| --- | --- |
| `fixtures.mjs` | 9 synthetic knowledge rows, one per state, plus the trusted-context variants, the request-scope variants, and the expected behaviour of each state with a `file:line` citation for the expectation. |
| `provenance.test.mjs` | The Bun test that drives the REAL Lane F client (`knowledge/client.ts`) and the REAL Lane F projector (`knowledge/projection.ts`) against those fixtures. |
| `README.md` | This file. |

## Run

```sh
cd C:/0-BlackBoxProject-0/vivim-labs
bun test corpora/knowledge/provenance.test.mjs
```

Measured: **30 tests — 29 pass, 1 skipped (FAILING-BY-DESIGN), 0 fail, exit 0.**

Modules under test are imported directly from the pinned checkout; nothing is
copied, stubbed or re-implemented:

```
corpora/knowledge/provenance.test.mjs
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/knowledge/client.ts      (Lane F, read-only client)
  -> C:/0-BlackBoxProject-0/vivim-ui/src/visual-runtime/knowledge/projection.ts  (Lane F, read-only projector)
```

## State matrix — fixture → expected → observed → source

`expected` is what the pinned source says should happen; `observed` is what the
real module actually did at the pinned SHA. Where they differ, the difference
is called out.

| State | Fixture id | Expected | Observed | Source of the expectation |
|---|---|---|---|---|
| **fresh** | `kn_fresh_0001` | visible; page `ok` / `complete`; projection `ready`; 1 card; `provenanceLabel = "synthetic.corpus/1"`; `sourceVersion = "rev-3"`; body visible | **as expected** | `client.ts:153`, `client.ts:155-171`, `projection.ts:26-32` |
| **stale** (version behind: `source.nativeVersion = rev-2` vs `rev-3`) | `kn_stale_0002` | visible; page `partial` / `partial`; projection `partial`; `statusLabel = "Knowledge stale — confirm source before relying on it"`; body visible | **as expected** | `client.ts:222`, `projection.ts:29` |
| **redacted** (strict context) | `kn_redacted_0003` | 0 items — existence is itself private; no marker, no body | **as expected for the items**; but the empty result is reported `ok` / `complete`, not a denial (**observed, not sanctioned**) | `client.ts:151`, `client.ts:156-170` |
| **redacted** (permissive: `maySeeRedactedMarkers`) | `kn_redacted_0003` | 1 existence-only marker: `stableId "withheld"`, `label "Redacted"`, `summary ""`, `provenance []`, `evidenceRefs []`, `asOf ""`, `source` all `withheld`; the projector emits **0 cards** | **as expected** — but the projector drops the marker entirely, so the UI shows nothing and says "ready" (**observed**) | `client.ts:151`, `client.ts:156-170`, `projection.ts:26` |
| **unavailable** | `kn_unavail_0004` | visible; page `partial`; projection `partial`; `statusLabel = "Knowledge unavailable — …"`; body visible | **as expected** | `client.ts:222`, `projection.ts:29` |
| **tombstoned / deleted** | `kn_tombstone_0005` | 0 items; never resurrected; body never appears | **as expected** — the empty result is again reported `ok` / `complete` (**observed**) | `client.ts:152-153`, `client.ts:229-244` |
| **tombstone source event** | `kn_fresh_0001` | after `applyFixtureSourceEvent({type:'tombstone'})` the record is gone and the epoch advanced | **as expected** (epoch 1 → 2) | `client.ts:229-244` |
| **revoked scope** (record scope revoked) | `kn_revoked_0006` | 0 items; never resurrected; body never appears | **as expected** | `client.ts:152-153` |
| **missing provenance** | `kn_noprov_0007` | readable; `provenance []`; `evidenceRefs []`; page `ok`; projection `ready`; **1 card whose `provenanceLabel` is the empty string**; body visible | **as expected, and it is a defect** — the card is indistinguishable from a fully-provenanced card except for an empty label; nothing marks the missing provenance (**observed, not sanctioned**) | `projection.ts:30` — the source states no refusal or marker for this case |
| **unknown** | `kn_unknown_0008` | visible; page `partial`; projection `partial`; `statusLabel = "Knowledge unknown — …"` | **as expected** | `client.ts:222`, `projection.ts:29` |
| **row-level scope** (another device only) | `kn_devscope_0009` | 0 items; body never appears | **as expected** — but the exclusion is **silent**: the page reports `ok` / `complete` with zero items (**observed, not sanctioned**) | `client.ts:145` — the source states no denial for a row-level exclusion |

### Context and request scope — must DENY, not degrade

| State | Expected | Observed | Source |
|---|---|---|---|
| revoked session (`ctx.revoked`) | `denied` / `SCOPE_DENIED`, 0 items, no identifiers | **as expected** | `client.ts:192`, `client.ts:111-114` |
| expired session | `denied` / `SCOPE_DENIED` | **as expected** | `client.ts:138`, `:192` |
| expired grant | `denied` / `SCOPE_DENIED` | **as expected** | `client.ts:138`, `:192` |
| offline | `unavailable` / `OFFLINE`, 0 items (no cached replay) | **as expected** | `client.ts:195`, `:115-117` |
| no trusted context | `denied` / `NO_TRUSTED_CONTEXT`, **0 discovery scans** (denied before any search) | **as expected** | `client.ts:191` |
| foreign `personalWorldRef` | `denied` / `SCOPE_DENIED` | **as expected** | `client.ts:194`, `:188` |
| foreign `surface` | `denied` / `SCOPE_DENIED` | **as expected** | `client.ts:194` |
| `purpose` not allowed | `denied` / `SCOPE_DENIED` | **as expected** | `client.ts:194` |

**The "denies rather than degrades silently" requirement is asserted, not
asserted-in-prose.** For every context and request scope case the test asserts
`page.status !== 'ok'` and `page.completeness === 'unknown'`, so a client that
answered a scope violation with an empty `ok` page would fail the suite. The
denial also carries no owner, session, device, world, policy or record
identifier, no `asOf` and no `projectionRevision` — asserted field by field.

**Row-level scope exclusion is the exception, and it is silent.** A record that
is scoped to another device is simply absent, and the page reports
`ok` / `complete`. That is arguably correct (existence is private) but it is
indistinguishable in the payload from "no such record". It is recorded as an
observation, not as a defect claim.

## Leak-negative evidence (before / after)

The suite is required to fail if the projection leaks a redacted or tombstoned
body. That is proven by mutation, not claimed.

**Mechanism.** `provenance.test.mjs` loads its fixtures through
`VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE` (default: the pristine
`corpora/knowledge/fixtures.mjs`). The proof loads a **scratch copy** in
`C:/0-BlackBoxProject-0/_scratch_l3/knowledge-mutation/fixtures.mjs` in which
the redacted row's `availability` is flipped `'redacted' → 'present'` and the
tombstoned row's `'deleted' → 'present'`, so their bodies would become
visible. **The pristine fixture file is never modified** (verified: zero
`MUTATION` markers in `corpora/knowledge/fixtures.mjs` after the run).

**BEFORE — mutated run (the suite must go red):**

```
$ VIVIM_LABS_KNOWLEDGE_FIXTURE_MODULE=C:/0-BlackBoxProject-0/_scratch_l3/knowledge-mutation/fixtures.mjs \
    bun test corpora/knowledge/provenance.test.mjs
```

```
error: expect(received).not.toContain(expected)

Expected to not contain: "SECRET_BODY_REDACTED"
Received: "{\"schema\":\"vivim.knowledge-result/1\",\"status\":\"ok\",\"items\":[{\"stableId\":\"kn_redacted_0003\",
\"kind\":\"project_note\",\"projectRef\":\"project_one\",\"label\":\"SECRET_BODY_REDACTED\",
\"summary\":\"Synthetic summary of a redacted note.\",\"availability\":\"present\",\"visibility\":\"visible\",
\"claimStatus\":\"observed\",\"source\":{\"authority\":\"fixture.hq\",\"nativeId\":\"kn_redacted_0003\",
\"nativeVersion\":\"rev-3\",\"digest\":\"sha256:synthetic-kn_redacted_0003\"},
\"evidenceRefs\":[\"fixture-evidence:kn_redacted_0003\"],\"provenance\":[\"synthetic.corpus/1\"],
\"asOf\":\"2026-11-06T12:00:00.000Z\"}],\"completeness\":\"complete\",\"projectionRevision\":\"fixture-epoch-1\",
\"asOf\":\"2026-10-14T17:46:40.000Z\"}"

(fail) leak-negative: no redacted, tombstoned or revoked body reaches the page or the projection [2.64ms]
(fail) leak-negative: the permissive redacted marker carries no body in any field [1.91ms]
(fail) leak-negative: no hidden body appears in a multi-entity page either [3.49ms]

 23 pass
 1 skip
 6 fail
Ran 30 tests across 1 file. [116.00ms]
EXIT=1
```

**AFTER — restored run (the pristine fixtures):**

```
$ bun test corpora/knowledge/provenance.test.mjs

 29 pass
 1 skip
 0 fail
 235 expect() calls
Ran 30 tests across 1 file. [84.00ms]
EXIT=0
```

The leak-negative suite is therefore **not vacuous**: it detects the exact leak
it claims to detect, and the pristine corpus is green.

## The one FAILING-BY-DESIGN negative test

`FAILING-BY-DESIGN knowledge/missing-provenance` — *a card with no provenance
must be marked or refused, not shown as a normal card*.

It is registered as a skipped test whose body asserts the correct behaviour
(un-skipping it reproduces the defect as a red test), and it has a companion
`FAILING-BY-DESIGN PROOF` test that keeps the defect observable in a green
suite. The PROOF test asserts the defect **is** present at the pinned SHA, with
the detail:

> `DEFECT PRESENT: the missing provenance is not marked — the projection emitted
> a normal card with an empty provenanceLabel ({"stableId":"kn_noprov_0007",
> …,"statusLabel":"Observed (not an authorization)","provenanceLabel":"", …}) —
> source: knowledge/projection.ts:30`

## TierFlow baseline measured for comparison (read-only)

| Command | Exit | Result |
| --- | --- | --- |
| `bun test tests/` | **0** | 449 pass / 0 fail / 1120 expect() across 35 files |
| `bun test tests/visual-runtime/` | **0** | 204 pass / 0 fail across 18 files |
| `bun test tests/visual-runtime/integration-2/` | **0** | 44 pass / 0 fail across 3 files |

**No TierFlow test failed. Nothing in TierFlow was modified.**

## What this corpus does NOT prove

* **Not live-KB evidence.** Every fixture is invented. No Knowledge Base was
  contacted, queried, indexed or read. The client under test is Lane F's
  **synthetic in-memory fixture client** (`createFixtureKnowledgeClient`),
  which is an external transport double, not a KB adapter.
* **Not real FTS behaviour.** The client matches a needle with
  `String.includes` over label/summary/path (`client.ts:208-212`). No inverted
  index, tokenizer, ranking function, stemming, phrase handling or language
  behaviour was exercised. Retrieval quality, ranking and recall are
  **unproven**.
* **Not live KB identity.** Stable ids, native ids, digests, `asOf` and
  `projectionRevision` here are invented strings. Whether a real KB's identity,
  digest or revision scheme behaves the same way is **unproven**.
* **Not owner-scoped live KB.** Owner scoping was exercised only against a
  fixture broker returning a synthetic trusted context. The real trusted
  authentication/authorisation layer behind `TrustedKnowledgeBroker` was not
  exercised, so no claim is made about owner-scoped reads against a live KB.
* **Not a deletion-policy claim.** The corpus shows the *fixture* client does
  not resurrect deleted/revoked rows. Whether the live KB's own deletion and
  revocation policy agrees is **unproven** and is an owner decision.
* **Not environment verified and not production verified.** No device, Host,
  browser, Android, provider or release was exercised. Evidence level is
  **`unit verified` against a synthetic fixture corpus**, and at most
  **`specified`** for anything that depends on a real KB.
* **Not a defect verdict.** The two silent behaviours recorded above (a
  missing-provenance card with an empty label; a row-level scope exclusion
  reported as a complete page) are **measurements offered to the owning lane**,
  not findings of fault. TierFlow was not modified to "fix" them.
