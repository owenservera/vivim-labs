# SCF H — Host read-only native feature
**Repository:** owenservera/ARBOR. **Reserved worker branch:** `work/p5-scf-h-host-r001`. **Exclusive writes:** `apps/vivim-host/**`, `.integration/proposal5/semantic-capability-fabric/proofs/H/**`.
## Shared invariant and required reading
Before work inspect the current exact SHA and full source of ARBOR `main`, the SCF draft PR #13 branch, accepted HVI source branches, and the **governing HVI policy** `.integration/proposal5/hq-version-intelligence/10-NATIVE-TOOLING-ONLY-SCOPE.md`. Read **every** SCF `implementation-ready/` root document, all `recipes/`, `contracts/`, `fixtures/`, `tests/`, `work-orders.json`, parent `semantic-capability-fabric/README.md` and SV-R012 `verifier/README.md` plus original `verifier/contracts/*.v0.schema.json`.
HVI is **native-tooling-only** in this wave. Real VIVIM/ARBOR/Labs migration comparisons are forbidden during installation or tests; use only synthetic invented Git-source fixtures and normal production source-module imports for build/compatibility. No provider APIs, Playwright, Puppeteer, personal corpora, browser sends, direct source effects or new canonical graph/wiki/registry.
Pin source exact SHA; do not silently use moving HEAD, do not force-push or merge without G gate. Report source pin/branch/changed files/test commands+exits vs UNRUN, actual blockers and gate status. A source-only import is not runtime verification.

## Prerequisite
G1 accepted merged main and HVI native agent-discoverable source tool installed/tested with synthetic Git sources. Fork exact accepted main SHA. If blocked, source-only proposal and stop.
## Do exactly this
Use accepted Host capability activation, command-intent, command-worker and auth/grant checks; attach SCF read-only capability via canonical registry instead of adding second dispatcher. For every pin and paged read verify actor/session, live source approval, revocation, scope, exact source SHA and resource budget. Reuse HQ service. No new effect writer, provider send, or personal World access. If PWA privileged replay/native loopback security remains blocked, remote SCF stays disabled.
Tests: allow/deny/revoke-after-compile, synthetic 3 pins, long report pagination, cached sensitive data denial, data redaction. Commit H scope and proofs, PREPARED_NOT_PROMOTED.
