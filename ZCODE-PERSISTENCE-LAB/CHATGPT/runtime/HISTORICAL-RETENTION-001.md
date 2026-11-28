# Historical retention and bounded receipt observation

Baseline 287ff49f369338a20e80e0d4b5b9f6e6636e2d8b. E33 and E41 source hardening.

- Checkpoint retirement stores and rereads a checksummed archive envelope containing the original head and command record (plus any legacy snapshot) before removing the active files. Existing archive retries verify any surviving active components; uncommitted orphan preparations without a head are not retired as history. Archive directories are revision-bucketed. This retains commands and evidence provenance, not every old indexed world. Records already removed by earlier versions cannot be reconstructed here.
- Explicit named pins preserve a verified committed index root and original head anchor through garbage collection. A pinned lookup checks the active or archived head and reads only the requested index page. Pin creation is immutable; no destructive unpin/expiry policy was chosen. Historical command lookup validates the individual head/record; it does not certify the entire archive chain. External anchors remain required against deliberate truncation or replacement.
- Receipt status performs indexed point observation, returning only digests and revision, never stored response or renewed send authority. Actual mutations still use the complete state transition.
- Checkpoint-only automatic maintenance avoids repeatedly pruning per transaction; explicit collect retries. Archive errors after head commit cannot reverse the committed operation. Explicit collect surfaces errors. Archive and pins deliberately accumulate; future storage-budget policy must not silently destroy evidence.
- Fix signed-ingress requirement for AttestTracePattern, matching prior AttestCandidate behavior. World-level fixture calls are not authenticated durable ingress.
- CLI: history-pin NAME; history-pins; history-lookup NAME TABLE KEY; history-record REVISION; receipt-status REQUEST_ID; collect.
- Authored checkpoint retention fixture; no compilation or execution. Windows directory rename durability and archival fault injection still need qualification.
