# FA-PG03 — PWA-first mobile, Host trust, Rock physical storage

**Disposition: PREPARED_NOT_PROMOTED.** Isolated source/negative-test preparation ONLY. No product security patch, G merge, native APK, storage deletion, real user archive, or Android device acceptance.

## Immutable source baseline / ownership
- ARBOR foundation input: work/p5-foundational-architecture-prep-r001 at 80c29fbc0d45afbce0fadde5388a8f912cdd2762. New branch: work/p5-fa-pg03-pwa-storage-r001, created from this exact commit.
- G integration ref integrate/proposal5-beta-r001: **ABSENT** at this run; original G.md sole merge/integration authority. No W1-G acceptance inferred.
- Frozen W1 SHAs (verified by GitHub ref fetch, corresponding PREG reports all declare FREEZE_READY for source only): A dda546a11294920d6521a0f64ab7ee62ec993c70; B 78a5105a9c1f1880d658f350e130b1e2632d3532; C f950b91685927f37b48d020433aa8d2d3c271419; D ad55fe28c2c331841675f308f9199ee4c2add86a; E 1a5cf26ce14ebce4e31d9a4373060ace61852645; F be458f3c4cdb7eb38d47b1de2a00d441b7cdc704.
- HQ Rock: owenservera/repos-hq@0a3c11a6df72434b46931f90b25bf1f93da09f92; Labs private knowledge: owenservera/vivim-labs@022608f5797e7b4a14ccc4c393bd3c5a4704a4be. P2-K 4db9e0c2068be36ab42663a76186422c8f9813a1; P2-T c807010e52b4a9f82ea9578a979befd38f74cb31.
- Relevant exact blob IDs, paths, readiness classification and missing edges are machine-readable in SOURCE-DELTA.json. Donor code is NOT merged in this branch.

## Confirmed current source truth
1. E browser signer uses IndexedDB and nonextractable P-256 WebCrypto, signs canonical enrollment and *session creation* transcripts. A valid browser key is not a grant. B enrollment issues pending grants, has explicit PC-only approve/revoke and stores 4-hour sessions. B direct-phone authorizes most privileged operations with x-vivim-grant-id plus x-vivim-device-session; **the requests themselves have no independently verified P-256 proof, method/path/body nonce or signer binding in this source**. Stolen active session bearer is a P0 risk, even though Host plan claim-once mitigates repeated execution of an *identical* claimed plan.
2. B direct-phone's explicit static map contains device-enrollment.mjs, sw.js and manifest.webmanifest but **omits browser-device-key.mjs** imported by E device-enrollment. E SW tries to precache that file; it cannot fix B's 404. Host validates remote loopback, exact host, x-forwarded-proto=https and tailscale-user-login. These upstream assertions need trust-of-proxy and origin adversarial tests, not assumptions.
3. B source has /mobile/ and /mobile/api/..., exact enrolment/pairing and Host-owned plan/execute/receipt. E SW only handles same-origin GET non-API shell requests and never queues effect writes. Control client displays CONTROL_UNCERTAIN rather than automatic replay.
4. A Tauri remote capability permits http://127.0.0.1:*; launcher polls fixed 43187 and sends durable owner bearer to /api/host/control/ui-session without verified child process identity. A tauri.conf.json mandates resources/vivim.apk although only an APK-manifest placeholder was seen in A resources. No signed APK is assumed. Its build hook points at tools/packaging/prepare-host-runtime.mjs, a path not found under the frozen A branch's tools/packaging directory check; verify actual post-G source closure rather than inventing an asset.
5. D .hq/index.db is described as rebuildable but contains notes and receipts in the SQLite schema; blanket file eviction is **unsafe without row ownership/recovery proof**. D kernel/store is durable append-only revision/tombstone/receipt WAL data; its backup/restore uses VACUUM INTO with temporary disk demand. D wiki generates Markdown with maxFilePages=5000 in refresh and stale page pruning. .hq/kg-journal.hqj is not automatically disposable.
6. Rock storage-policy.mjs is a pure calculator with **effectPerformed=false**. Profiles have managed caps survival 96 MiB, lean 256 MiB, balanced 768 MiB, full 4096 MiB; independently, OS free reserve is max(512 MiB, 2% volume), capped at 2 GiB unless owner increases it. Its proposed evictions require rebuildable and sourceAvailable, not pinned/leased/readOnly, and never select Rock, source vault or authoritative rows. **Neither measured bytes nor operational compaction is implemented by this lane.**
7. P2-K already prepared consent/ZIP normalization/checkpoints/erasure and 26 negatives, so PG03 does not create another importer or DB. The Labs import/graph binding and cross-store effect/receipt consistency still require accepted post-G owner wiring.

## Deliverables
- SOURCE-DELTA.json exact imported/source-only/missing/deferred statuses.
- PWA-AUTH-AND-PACKAGING-MATRIX.md route/proof/asset/IPC/APK matrix.
- STORAGE-CLASSIFICATION.json physical byte inventory taxonomy; every actual byte measurement null pending host filesystem read.
- PROOF-PLAN.md commands and source-specific seams; NEGATIVE-CASES.json and synthetic-contracts.test.mjs are **fixture-policy checks, not actual Host/phone tests**.
- NEGATIVE-ACCEPTANCE.md security and storage adversarial expected outcomes.
- OWNER-HANDOFF.md maximum five post-G owner tasks, security first.
- RUN-LEDGER.json exact source refs and actual tests versus UNRUN.

**Changed paths:** exactly this proof subtree. No in-lane data deletion, Host/PWA/Tauri patch, real archive import, runtime command execution or G interference.
