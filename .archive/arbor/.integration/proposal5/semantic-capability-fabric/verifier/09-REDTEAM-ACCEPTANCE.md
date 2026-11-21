# SV negative acceptance: prove that false information is rejected
**All cases are proposed. Executed: NONE.** Every claim of PASS requires exact SHA, test command/runner, exit and receipt.

## Compiler truth (C01–C08)
C01 tag exists in code string/regex/template instead of actual comment → reject.
C02 comment attached to wrong function or stale line after refactor → mismatch/refute.
C03 same symbol name across files with different semantic ID or owner → refuse ambiguous resolution.
C04 compiler claims a nonexisting Host action/contract but registry says absent → blocked.
C05 unknown/deprecated ID silently promoted or mixed `hq.annotation/2`/v3 semantics → fail.
C06 file tags feature X but exports are only unrelated helpers → discrepancy, not auto-verified.
C07 compiler-generated IR changes under same Git/source+schema digest → corruption/fork.
C08 source mapped to Git commit A but blob read from B → blocked.

## Behavioral truth (B01–B12)
B01 function claims input SHA preserved but changed output SHA on adversarial input → counterexample refutes.
B02 code and test both contain same accidental bug; independent reference oracle catches mismatch.
B03 function claims no write, but effect trace includes Host write → refuted.
B04 function claims no write, trace lacks complete instrumentation → UNKNOWN, not proved.
B05 1,000 green tests for a universal "always" claim → only SUPPORTED_BY_TESTS.
B06 low-disk, timeout, cancellation, invalid Unicode, stale ref, thread interleaving negative failures reported with environment.
B07 Rust build binary hash differs from tested artifact hash → old receipts invalid for new deployment.
B08 CI stdout says PASS but runner exit !=0 or no attested runner/sha → invalid/blocked.
B09 mutation to reverse branch guard not killed by tests → coverage insufficiency, no universal proof.
B10 model checker passes finite state bound but production has unmodeled external dependency → PROVED_UNDER_MODEL only with caveat; runtime unknown.
B11 external provider send local ACK but no delivery ack → effect may remain uncertain.
B12 staging test only but claim "Windows/Android accepted" → refuted/misleading; device acceptance UNRUN.

## Explanation truth (E01–E08)
E01 claim tested 2 inputs; explanation says "always" → reject.
E02 phrase says "safe", without scoped safety contract/proof → reject.
E03 explanation contains fabricated source line or omitted contrary proof → reject.
E04 formatter uses older source sha after source change → stale, not verified.
E05 two claims with conflicting owner/catalog versions merged into sentence → reject.
E06 freeform LLM sentence not grounded in typed template/claim → UNVERIFIED_INTERPRETATION.
E07 translated phrase strengthens modality ("may"→"always") → reject translation.
E08 output text contains confidential source from revoked scope → redact/deny.

## Security and verifier self-test (S01–S10)
S01 hostile comment instructions to modify verifier or execute shell → data only.
S02 malicious Git path/symlink/huge inputs or nested predicate bomb → bounded error.
S03 agent alters spec and tests without trusted oracle review → cannot issue certified receipt.
S04 source/runner/proof receipt signature/digest mismatch → refuse.
S05 trusted checker version unknown/deprecated → block proof promotion.
S06 old Host grant or wrong actor/session/world revision used to validate source → deny.
S07 replayed old runtime receipt used for new binary → reject.
S08 negative proof claim "no effect" with missing effect producer → UNKNOWN.
S09 duplicate graph/writer/dispatcher introduced by verifier feature → architectural NO_GO.
S10 corrupted SQLite/cache or ENOSPC → keep last verified snapshot and Host originals; signal incomplete.

## Proof of falsification capability
Release test suite intentionally contains known-false implementations and misleading explanations. Verifier must detect them and preserve counterexamples. A successful verifier release requires not only green compliant cases but reproducibly **red intentionally wrong cases**. Run source mutation and checker mutation tests; establish measured detected/killed vs survived mutant counts. The verifier is not certified merely because its own unit tests pass.
