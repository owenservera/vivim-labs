# ARBOR Acceleration R002 — W1 Lane A handoff

**Lane:** A — Host-owned journal authority and replay/CAS hardening  
**Assigned branch:** work/accel-r002-w1-a-host-authority  
**Base and initial main:** 2a42f1af025db8aacb965326b11c73da671ffb3f  
**Implementation source head (before report-only commit):** 64275bc8c72a3b478cccd1b1b9c93d1c6009d7c6  
**State:** SOURCE_COMMITTED_NOT_ACCEPTED. **Measured acceleration:** NOT_YET_MEASURED.

## ERA 1 — Source and ownership

Inspected parallel/README.md, parallel/A.md, the canonical roadmap and explanatory ACCELERATION-R002.md, both PDR handoffs, current Host server and product journal, both tests, browser-session, control dispatcher, HTTP error adapter and read-only Wiki/chat clients. The precreated branch initially matched the exact main baseline. No other lane, keeper file or main ref was written.

Four exclusive paths changed:

1. **apps/vivim-host/server.mjs:** Same-origin Origin and custom header plus an actor name no longer authorize Wiki/gate edits. Privileged product mutations require an actual existing Host owner bearer grant OR the existing short-lived authenticated browser control session with CSRF. Approval-claiming feedback rounds are fenced likewise; passive failure observations remain non-executing. Existing browser session cookie/logout scope moved to Path=/api to reach the product endpoint. Reuses bounded application/json reader (4096-byte input), rejects forged intent and server-owned metadata, and calls the existing pure control interpreter in-process. No new control-effect executor.
2. **apps/vivim-host/chat/product-journal.mjs:** Strict bounded per-kind typed envelopes/actor scope, no arbitrary file/path/code keys, 4KiB entry and 8MiB file limits, Wiki target and revision CAS, new random entry ID/monotonic sequence, duplicate/order/corrupt-middle refusal, committed-prefix torn-tail replay with append refusal, private dataRoot subdirectory, shared same-process per-path append queue, single append and fsync before acknowledgement. Historical legacy demo and gate observations replay without gaining approval authority.
3. **apps/vivim-host/test/product-journal.test.mjs:** Eight focused cases for replay, reinitialization, root isolation, simultaneous same-process instances, invalid envelope/path, stale revision, torn final line, corrupt middle/duplicate/order/UTF8, simulated write failure and legacy records.
4. **apps/vivim-host/test/product-routes.test.mjs:** Source assertions added for spoofed owner refusal, real owner bearer and browser session grant, bad CSRF/logout refusal, gate authority, simultaneous HTTP CAS and parity of /api/state, /api/wiki, /api/journal and /demo/journal.jsonl. These HTTP tests are UNRUN.

No second store, dispatcher, interpreter, publisher, graph, provider API, local user data read or phone permission introduced. The state/revision is still Host journal-derived.

## ERA 2 — Runtime tests, exact exits and honesty

**Executed only in an isolated temporary local mirror of exact committed journal and unit-test blobs** at /tmp/arbor-a, Linux, Node v22.16.0:

| Command | Actual exit | Result |
| --- | --- | --- |
| node --check apps/vivim-host/chat/product-journal.mjs | 0 | Syntax accepted |
| node --test apps/vivim-host/test/product-journal.test.mjs | 0 | 8 passed, 0 failed |
| git hash-object apps/vivim-host/chat/product-journal.mjs | 0 | 816f9b7bdbe0cb1ce47e1b4fe558fe7193ae6af0, matches committed blob |
| git hash-object apps/vivim-host/test/product-journal.test.mjs | 0 | af034f56fc140862cf37107a17c3896f9b33f477, matches committed blob |

**Required full-repo commands NOT executed:**

- node --test apps/vivim-host/test/product-journal.test.mjs apps/vivim-host/test/product-routes.test.mjs — **UNRUN**. Connected GitHub source could not be materialized as a full executable local checkout; no actual Host/server HTTP route process was run here.
- node --test apps/vivim-host/test/chat-box-baseline.test.mjs — **UNRUN**, read-only lane ownership.
- Node24, Windows, actual browser, paired PWA/physical device, separate-OS-process restart, multiple Host processes on one dataRoot, integration regression and independently verified CI — **UNRUN**.

Associated pull-request-triggered GitHub workflow query returned no runs for the source head, and commit-status API returned 403. Neither is a passing test. This is narrow exact-blob Node22 unit evidence, NOT production or owner acceptance. The write-failure test simulates a failed file open, not arbitrary disk crash atomicity.

## ERA 3 — Security blockers, cross-lane handoff and frontier

**Lane C blocker — deliberate fail-closed behavior:** the current read-only apps/vivim-host/public/pages/wiki.js still sends only actor=owner-browser and x-vivim-control=1. It does not provide owner session CSRF; saves now refuse with HTTP 401 until C integrates the existing owner-issued /api/host/control/ui-session → ui-exchange enrollment and forwards x-vivim-ui-csrf on same-origin Wiki edits. The control cookie is HttpOnly, SameSite=Strict, Path=/api. Never place the long-lived bearer in browser source or treat actor strings as privileges. Real browser enrollment, save, logout, stale-revision and rewalk evidence is required for acceptance.

**Security/evidence limitations:**

- Same-process instances are serialized; **inter-process single-writer safety is unproven**. File-size recheck is not an OS-level lease. Deployment must enforce one Host process per dataRoot or integrate a separately reviewed durable lease. Do not represent this as multi-process accepted.
- A torn final segment is preserved on disk but hidden from committed projection and further appends return 503 pending explicit evidence-preserving owner repair; no silent truncate or rewrite. A corrupt middle line fails closed. An append/fsync response failure can be **uncertain**; inspect the durable journal before retrying.
- Historical untrusted gate observations are readable, not retroactive owner receipts or independent acceptance. Older demo clients claiming positive verdicts or approveAll now need owner authority; passive negative feedback remains admissible.
- The new Path=/api cookie scope and Windows filesystem flags need full Host control regression + Node24/Windows tests. Real UI and CI are UNRUN. Do not mark accepted.

**Integration:** Review the four scoped diffs and this report, then combine with B's actual Node24 Host runner, C's browser owner-session CSRF surface, D's independent truth verifier and the main keeper owner gate. No merge or cherry-pick was made in Lane A. The canonical ready frontier remains A00/A01/A02 pending independent acceptance, not A03/A04/A06 effects. There is no observed subsequent-task speedup: **NOT_YET_MEASURED**.
