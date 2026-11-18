# OWNER-HANDOFF — minimum post-G integration, not worker implementation

**Disposition:** PREPARED_NOT_PROMOTED. No work outside V-visual or on frozen branches. Branch base `8b23c73bf450738a76b256bf0fa17ecbf86db591`; V's first committed prototype `929961891414d279d1fe2b931d9eddb0f4c2a51d`.

## Dependencies before starting

G must merge and verify A/B/D/E/F source topology and imports. Especially resolve B planner's missing `../../mobile-relay/control-language.mjs` path on its isolated branch. Consume the frozen E TierFlow implementation **first**, never a second composer. Use the exact pins in `SOURCE-PINS.json` to calculate deltas after G; re-read live SHAs, do not silently assume frozen branch identity still equals the landed source. The native Rust R001 `user_io.rs` is evidence of a read-only snapshot capability, not proof that it was included in ARBOR A or already wired.

## Four tasks total (post-G owners)

| Owner | Single minimal task | Authoritative ports to use | Measured closure |
| --- | --- | --- | --- |
| **A / Native** | Extend/reconcile existing `UserIoCore` with audience-filtered read-only multi-producer projection snapshot/delta, per-producer watermark and source-change invalidation; add CAS correction/undo without extra executor or graph | `core/programmability/src/user_io.rs:UserIoCore::apply/apply_json`, shared Rust graph source revision, **authenticated** desktop/mobile ingress; `PROTOTYPE/projection.mjs` as pure proof | Rust unit/property fixtures for simultaneous candidates, reversed completion order, stale/revoked policy, IME/UTF-16/UTF-8; no effect calls |
| **B / Host authority** | Supply **read-only** owner catalog, interpretation and effect-risk facts; never expose permission as UI-computed | `apps/vivim-host/control/intent-planner.mjs:createControlPlanner().interpretText/planText`, `action-catalog.v1.json`, `dispatcher.mjs` approval/claim/receipt **only outside** V | Refusal/clarification wins; planner candidate distinguishable from authorized effect; no provider action in tests |
| **E / TierFlow + mobile** | Bind the canonical mobile/desktop composer to native semantic read projections using existing E `command/ir.ts`, inert render/revisions and semantic projection; Quiet/Assisted/VIEW ALL are **masks** over one graph | `packages/tierflow/src/visual-runtime/command/ir.ts`, `render/wave3-semantics.ts`, `revisions/port.ts`, `apps/mobile-relay/generative-ui/semantic-projection.mjs`; no new publisher | Real Windows Chrome and phone acceptance: caret/IME, Tab, undo, screen-reader and touch; test source-visible bad permission/coverage flags |
| **F / Verification** | Converge V schema with F `SourceRef`/risk/immutability contracts, validate replay and no-cross-owner regressions, audit actual runtime producer coverage | `contracts/interop/v1/{types.ts,protocol.schema.json,validate.mjs}`, D `hq/lib/semantic-registry.mjs:{getSemanticId,listSemanticIds,resolveSemanticAlias,listSemanticLinks}`, `hq/lib/annotation-v2.mjs:parseAnnotationV2` | Schema validation, cross-product regression, no auto Submit/approve/publish, scoped source denial, verified-versus-UNRUN ledger |

## Explicit validation commands (not executed here)

```bash
node --test .integration/proposal5/phase2/pre-g/V-visual/PROTOTYPE/projection.test.mjs
# Owner-run after G, when landed and actual imports resolved:
node --test apps/vivim-host/test/*.test.mjs
node --test apps/mobile-relay/test/generative-semantic.test.mjs
# Actual Rust crate manifest and TierFlow test runner must be confirmed after G;
# do NOT infer commands or report passing until run.
```

## Stop / acceptance contract

Current V is bounded **data projection prototype** with 39/39 direct V8 synthetic checks executed, **not** a native Rust proof or device-tested visual editor. P2-T owns general test census; P2-C owns Rust/Chronos interop convergence. E owns styling/renderers. G owns merge/integration. No automatic Submit; no full platform source-census claim; no real personal conversation tests. No promotion until G and post-G A/B/E/F gate evidence.
