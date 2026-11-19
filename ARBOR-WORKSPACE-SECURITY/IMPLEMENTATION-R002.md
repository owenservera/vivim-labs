# R002 execution order and dependency design
Baseline e07e61385a53f30890b4a7f7441fe25d22a79b1c. Reconcile current main before touching product. No PRs/branches unless owner changes policy. No new router or competing journal. Apply minimum coherent patches to existing implementations, preserve user data. Design specs are proposed, not implementation evidence.
1. S20 runs alongside every lane; establish confirmed effect inventory and runtime boundary tests.
2. S01/S02/S06 define existing identity, roots and grant authority jointly. S03 reuses enrollment and browser keys. S04/S05 validate key lifecycle and migration. S07/S08 implement runtime enforcement at resource boundary.
3. S09/S10/S11 contain process, egress and browser; do not enable new untrusted executable workloads until their actual boundaries work.
4. S12/S13/S14/S15/S16 attach security context to existing semantic, compiler, UI and knowledge paths, without reinterpreting authority.
5. S17/S18/S19 integrate packages, receipts, recovery and release integrity; tests in S20 anchored to actual product paths.
First vertical slice: isolated research program reads allowed public data, creates typed summary, writes approved Wiki section and receipt; all out-of-scope reads, egress and self-grant attempts denied. Run negative and positive cases; prove data and resource boundaries on real Windows and PWA. No artificial organizational gates; actual security enforcement remains mandatory.
