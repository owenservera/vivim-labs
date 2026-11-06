> MIGRATION NOTE: Historical claim about an isolated reconstruction, not an executed clean-checkout test of the moved file. Native-device acceptance has not been demonstrated.

# 2026-11-06 primitive-requirements test evidence

Test mode: **isolated local reproduction of the research-only GitHub lab fixture**, not repository checkout/native Android test.
Executed with Node v22.16.0 in a Linux container. No device, provider or Host effects.

## Material
- GitHub research fixture: docs/MOBILE-EXTENSIONS/fixtures/primitive-requirements.v0.json (16 primitive requirements, 1 directed acyclic dependency graph).
- GitHub test: docs/MOBILE-EXTENSIONS/tests/primitive-graph.test.mjs.
- Locally reconstructed equivalent fixture and Node test under /mnt/data/mobile_ext_validation, without pulling an authenticated repository checkout.

## Command and actual result
Command: node --test /mnt/data/mobile_ext_validation/tests/primitive-graph.test.mjs
Exit code: 0
Results: 3 tests passed, 0 failed. Checked research-only/no-execution authority, unique IDs/explicit DAG dependencies, and passive extension absence of provider-send authority.

## Maturity and limitations
**Synthetic fixture-valid only.** The container test used a manually reconstructed counterpart of the GitHub files; it was NOT a byte-for-byte GitHub checkout/run. Repo-native test has NOT been executed. No APK has been compiled or installed; no Android/Windows/UI/provider/browser API integration is proven. This result cannot advance TierFlow Integration 3/4, Android readiness, or live send/approval status.

## Next evidence
- Run the GitHub test unchanged from a clean Repos HQ checkout after explicit environment setup.
- On real Android devices, record package/build hashes, browser versions, runtime permissions and synthetic DOM tests.
- Preserve denied/unknown/revoked/crash outcomes separately from happy path.
