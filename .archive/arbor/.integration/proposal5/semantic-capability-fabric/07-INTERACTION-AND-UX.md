# One semantic reality, multiple surfaces
The application can feel immediately self-understanding **only where there is trusted typed state and authorized source linkage**. Don't pretend arbitrary external websites have known internals.

## 1. Developer at file
At top of a file show a compact **Capability DNA chip**: `HQ · owner HQ · 3 features · 2 contracts · 1 gap`. Tapping reveals purpose, source code spans, imported dependencies, tests declared/executed, last change, feature versions, impact and reviewer decisions. Never synthesize a pass result from a tag.

## 2. Developer at pull request
Beside file diff show **Semantic Delta**:
- Membership changed `vivim.hvi.compare-revisions@1` retained; function moved / source SHA changes.
- Contract changed and 7 dependent units need recompilation.
- 4 declared tests exist, 3 executed here, 1 UNRUN.
- Host readiness remains `SOURCE_WIRED_UNVERIFIED`.
- Any observed scope/authority conflict is visually prominent and blocks automated promotion.

## 3. Person opening VIVIM
Trusted screen state supplies `uiRevision,activeComponent,workspace,featureRefs`; the compiled wiki supplies matching feature/contract/provenance; Host grants and owner data supply current allowed read. Show concise `What this screen does`, `What is connected`, `What can I ask`, `What changed`, `What isn't ready`. For unknown mapping show `Not mapped/insufficient evidence`, not guessed internals.

## 4. Inspector: architecture time machine
Select `ARBOR@SHA-A`, `ARBOR@SHA-B`, `ARBOR@SHA-C`, and optional `repos-hq@SHA`. A table of feature passports highlights source lineages, semantic contract deltas, exact tests and migration decisions. Click a feature to navigate source→symbol→contract→test→Host owner→UI projection. View `same feature, three versions` without moving branches.

## 5. AutoWiki
The same compiled source ABI feeds the planned read-only **screen mirror** (R008) via current trusted screen context; flipping back preserves cursor, selection and original UI state. AutoWiki does not store 100 permanent copies for 100 screen changes; it projects ephemeral pages and cites known sources, with partial/blocked badges when required. Personal DCB/memory requires separate Host owner permission and never joins public HQ by default.

## 6. Agent mission cockpit
An agent receives a lightweight `MissionContextPack {goal, sourcePins, relevantFeatureIds, ownerContracts, invariants, affectedUnits, testsNeeded, warnings, evidenceDigest, authorityStatus}`. When agent proposes a change, compiler shows exactly what downstream features may break. Model suggestions do not bypass real Host approval/receipt.

## Real-time design considerations
Background index event after source save, immutable source fingerprint after Git commit; UI subscribes to **existing** TierFlow projection/update channel, not a new display publisher. Debounce/IME and stale-source revision guards. Show meaningful statuses; always distinguish source changes from runtime observations. PWA uses same read-only domain and original Host authorization, never independent phone graph.
