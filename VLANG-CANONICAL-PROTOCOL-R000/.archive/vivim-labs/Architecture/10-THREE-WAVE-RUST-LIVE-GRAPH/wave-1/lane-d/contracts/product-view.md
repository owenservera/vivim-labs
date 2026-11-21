# Product-facing derived wiki projection — specification only

The dev source scanner has no product rights. A future user-scoped read-only consumer must obtain a privacy-filtered, explicitly approved projection from a source-native authority; it must not inherit the full developer filesystem, raw KB/session/note bodies, browser profile, agent secrets, synthetic fixtures or Rust journal diagnostics.

Only keys on a closed allowlist are proposed: `graphId`, `semanticId`, `safeTitle`, `safeSummary`, `status`, `effectClass`, `permissionOwner`, `sourceEvidenceSummary`, `unknowns`, `actionExecutableVerified` (must be false without independently observed receipt). There are no raw `source_path`, `abs_path`, userName, debug stack, private body, test harness command, `/mnt/data`, `C:\Users\`, passwords or approval grants. Unknown state is **not** a false boolean.

Host permission, consent, receipt and operating system/device effects remain Host-owned. TierFlow's UI document/revision/render writer remains its own. Product safety will be tested on actual approved projections only after native integration; deterministic fixture redaction is not production DLP certification.
