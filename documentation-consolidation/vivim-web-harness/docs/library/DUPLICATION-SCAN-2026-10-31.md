# Duplication-of-work scan — 2026-10-31

## Objective
Scan for `.og` files and score any duplication of work across the repo.

## 1. `.og` file scan
Result: **no `.og` or `.org` files exist** in the repo.
- `glob **/*.og` → 0 matches
- `glob **/*.org` → 0 matches

The literal `.og` scan found nothing to score, so duplication was scored against the **atomic capability registry** (`docs/capabilities/atomic-registry.json`, 336 items, sourceCommit `ac19e36...`) instead — that registry is the repo's declared inventory of work, each item linking to source files and verified symbols.

## 2. Registry entry duplication (direct duplicates)
| Check | Result |
|---|---|
| Duplicate IDs | 0 |
| Duplicate titles | 0 |
| Duplicate acceptance text | 2 groups |
| Duplicate spec file | 13 groups |

- **Duplicate acceptance** (likely copy/paste, not distinct work):
  - `UX-02, PERF-03` — both accept "Measured on the owner's Android"
  - `X-001..X-028` — all 28 share acceptance "Review exact source, choose canonical owner and measurable acceptance."
- **Duplicate spec** groups are PRD-family groupings (one spec file per capability family), e.g. CAP-01..CAP-23 all under `dev/prd/01-full-capability-space-on-phone.md`. These are legitimate family groupings, not duplication of work.

## 3. Cross-capability source/symbol sharing (real duplication-of-work signal)
A source file or verified symbol claimed by more than one capability ID is the strongest signal that the same work is being claimed twice.

- Source files shared by >1 capability: **44 files**
- Verified symbols shared by >1 capability: **88 symbols**
- Capabilities with at least one shared source/symbol: **301 / 336**
- Total shared-with incidences: **19,404**

### Top duplication-scored capabilities (sharedWith = other capability IDs claiming the same sources/symbols)
```
CAP-01 sharedWith=126  Shared catalogue builder used by cloud relay and Direct Host
CAP-02 sharedWith=126  Every Studio site adapter listed, not a fixed four
CAP-03 sharedWith=126  Per-model entries ...
CAP-04 sharedWith=126  executable flag per capability
CAP-05 sharedWith=126  Studio relay-chat adapter ...
CAP-06..CAP-15 sharedWith=126 (same 6 source files)
```

CAP-01 through CAP-15 all cite the same 6 source files (`harness/src/core/models.mjs`, `harness/src/interface/router.mjs`, `studio/bin/commands/relay.mjs`, `studio/bin/lib/relay-chat-studio.mjs`, `apps/mobile-relay/app.js`, `apps/vivim-host/direct-phone.mjs`). These are different capabilities built on shared infrastructure — shared foundation, not necessarily duplicated implementation.

### Symbol-level true-overlap examples (same symbol claimed by multiple distinct capabilities)
- `apps/vivim-host/direct-phone.mjs::createDirectPhoneHandler` → CAP-01, STATE-02, STATE-06, STATE-16, X-100
- `studio/bin/lib/relay-chat-studio.mjs::createStudioRelayChat` → CAP-01, CAP-02, CAP-05, CAP-22, SEND-01, SEND-03, SEND-05
- `harness/src/core/models.mjs::efforts` (and `effortRx`, `openEffortMenu`, `effortMenuExpression`, `resolveEffort`) → CAP-07, CAP-18
- `studio/src/capture/conversation-runtime.mjs::reply` → SEND-08 (x3), SEND-18 (x3)
- `studio/src/capture/conversation-runtime.mjs::boundedReplyWait` → SEND-08, SEND-09, SEND-18
- `studio/src/store/versioned.mjs::VersionedStore` → WORLD-01, OWN-01

## 4. Score summary
| Metric | Value |
|---|---|
| `.og` files found | 0 |
| Registry items scanned | 336 |
| Duplicate IDs | 0 |
| Duplicate titles | 0 |
| Duplicate acceptance groups | 2 |
| Source files shared by >1 capability | 44 |
| Verified symbols shared by >1 capability | 88 |
| Capabilities with any shared source/symbol | 301/336 |
| Total shared-with incidences | 19,404 |

## Conclusion
No `.og` files exist. Duplication of work is not present as duplicate registry IDs/titles, but **301 of 336 capabilities share at least one source file or verified symbol with another capability** — concentrated in the shared relay/host/catalogue infrastructure (CAP-01..CAP-15 family, 6 common files) and repeated symbol claims (88 symbols). The highest-value targets for deduplication review are the shared symbols claimed by 3+ distinct capabilities listed above.

Source: `docs/capabilities/atomic-registry.json` (sourceCommit ac19e368486cc945290b00823fca22e04c510256803b09d7950984914b12b0b8), scanned 2026-10-31.
