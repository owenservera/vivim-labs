# Later: Rust intelligence engine inventory (NOT STARTED)

Map the **existing** Rust core's intelligence-engine concept before proposing upgrades. This note is a parking place, not a second engine registry or an implementation order.

Future narrow investigation:
- Locate the real Rust core and active engine/plugin interfaces in Repos HQ/TierFlow/related repositories; pin source SHAs and distinguish compiled/wired engines from prototypes.
- Map each engine to inputs/outputs, caller, owner, algorithm, replaceability, test harness, resource costs and permission boundaries.
- Identify upgrades possible **within current interfaces**, choose one isolated differential benchmark, and test only in Labs before any owner-approved promotion.
- Link reusable findings to the five tracks, especially ranking/retrieval/reconciliation; avoid parallel engine framework or new plugin ABI.

No runtime, repo ownership, graph-core or Rust source changes authorized. Revisit after PULSE-1 begins and Wave 4–6 integration is stable.
