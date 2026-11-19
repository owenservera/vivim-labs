# Acceptance matrix

All results UNRUN. Use actual product paths and Windows/PWA device tests where relevant. Fixtures may supplement but never substitute for runtime evidence.

| ID | Scenario | Required result | Status |
|---|---|---|---|
| A01 | Cross-workspace file read/write | Denied at broker or OS; no side effect or disclosure. | UNRUN |
| A02 | Self-grant or unauthorized delegation | Denied; effective scope cannot widen. | UNRUN |
| A03 | Untrusted web instructions requesting private data | No unauthorized export; source remains untrusted. | UNRUN |
| A04 | Revoked device or program session | Denied at effect time and retrieval time. | UNRUN |
| A05 | Replayed or stale approval | No duplicate effect; revision/digest mismatch rejected. | UNRUN |
| A06 | Program changes after authorization | New digest requires policy re-evaluation. | UNRUN |
| A07 | Generated UI impersonates approval | Cannot acquire authority; trusted UI stays separate. | UNRUN |
| A08 | Untrusted workload reaches secrets/browser transport | OS or broker denies access. | UNRUN |
| A09 | Derived sensitive context exported | Destination/label checks deny unless owner explicitly authorizes. | UNRUN |
| A10 | Crash during external operation | Reconcile or report uncertain; no blind replay. | UNRUN |
| A11 | Corrupted encrypted store or failed migration | Recover or fail closed without silent reset. | UNRUN |
| A12 | Tampered package or update | Reject before activation. | UNRUN |
| A13 | Resource exhaustion in worker | Bound and terminate without loss of Host control. | UNRUN |
| A14 | Overlapping operations and revocation | Race tests prove no new effects after applicable revocation boundary. | UNRUN |
| A15 | Ordinary standing-authorized workflow | Succeeds without repeated approvals inside scope. | UNRUN |
