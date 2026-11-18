# W2 H — Source-only Host adapter preparation

Frozen fork: bbe7e4896eea02bcdec6f8fedb66eab885250161. Owner-waived G1 source development, **not** accepted main.

Authored `apps/vivim-host/control/hvi-capability.mjs` and synthetic Node test suite. The read-only port reuses HQ `buildActions` and `route` and forwards to G1 `runHviTool`, requiring actorId, sessionId and caller-supplied live `verifyGrant` callback. Default `enabled=false`; discovery reports disabled. No HTTP route, grant database, executor, provider API or real repo comparison.

**Not yet wired:** canonical Host bootstrap action catalog/agent dispatch and independently attested source-reader/grant enforcement. The callback currently represents an injected test adapter, not trusted production authorization. This is a real implemented preparation module but **NOT** the final W2 H installation acceptance.

Tests: `node --test apps/vivim-host/test/hvi-capability.test.mjs`; exit **UNRUN**. No Node24 runner or checkout available here; do not claim runtime/device/security acceptance. W2 H status: `PREPARED_NOT_PROMOTED`.
