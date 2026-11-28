# Host-programmable demand generation

Base: f30cec7b334dc8d7256a33c923f690eb0142f40c. Scope: R06 autonomous demand creation and native-first routing.
The host configures a versioned exact goal/family policy, an explicit profile allowlist, input/output/reasoning allocation ceilings, optional task deadline, frontier permission, extra quota pools, minimum quality and actual assembled-input byte ceiling. The controller generates at most the configured number of fresh demands per cycle for ready tasks without a native class. Native work remains preferred. Adapter and context compatibility is checked before queuing. Empty compatible profile sets never become unrestricted. Generated demands freeze their profile set; new configuration governs new demands only. Policy edits and generation are suspended during comparative routing trials. No provider dispatch is added.

CLI: kernel <store> routing-demand-configure <policy-file> <request-id>; routing-demand-plan; routing-demand-generate <limit> <request-id>. These are host-only mutation paths. Token allocation ceilings are not measured usage; real adapters must enforce ceilings at dispatch.

Remaining: live adapter receipts, dynamic demand refresh when a new model is admitted, and native qualification. No compile, tests or provider calls.
