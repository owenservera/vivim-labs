# Verifying compiled verbal explanations: sentence-level proof binding
## Principle
The source code cannot safely be summarized as unrestricted prose and then declared correct by the same model/compiler. Output two streams:
- **Verified fact sentences:** assembled from typed reviewed templates bound to property IDs and exactly matching proof-receipt state.
- **Interpretive commentary:** optional LLM narrative, clearly labeled not verifier-attested unless reduced to verified claim references and checked independently.

## Structured explanation examples
Claim P1: `snapshotAt` preserves the requested commit SHA for the test oracle's inputs. Tests executed, exit 0 → **SUPPORTED_BY_TESTS**. Allowed sentence:
> "In the tested cases, snapshot resolution preserved the requested SHA."
Forbidden sentence:
> "Snapshot resolution always preserves SHA and cannot modify source."
The latter requires universal proof of two separate properties (SHA preservation and no source modification).
Claim P2: "No Host effects" needs complete effect observation/structural static proof. If only a synthetic trace exists: `UNKNOWN`. Allowed sentence:
> "Host effects have not been independently ruled out."
Claim P3: "Produces report in under 10 ms" requires measured hardware/environment, sample distribution, budget, and no universal latency phrasing.

## Explanation envelope
`{sourcePin,buildDigest,semanticIrDigest,specDigest,explanationPackDigest,clauses:[{text,claimIds,modality,subject,scope,evidenceRefs}],unknowns,omittedClaims,overallCertification}`. Every factual clause needs 1+ evidence-bound claim IDs; grammar must limit stronger words ("always", "never", "guaranteed", "secure", "fully", "sent") unless proof and scope warrant them. The checker parses/validates **template ID and slots**, not attempts an unsound universal natural-language entailment solver. Multi-lingual output uses reviewed templates sharing the same claim IDs, with translation review/version.

## Conditions for certifying a clause
1. Claim ID is from a pinned reviewed spec and a real source/compiler fact.
2. Proof receipt belongs to the same source/tree/build/model/owner action revision and approved scope; stale receipt cannot certify new SHA.
3. Rendered modality matches allowed truth: `PROVED_UNDER_MODEL` scoped, `SUPPORTED_BY_TESTS` limited to tests, `REFUTED` explicit contradiction, `UNKNOWN/BLOCKED` transparent.
4. No additional unsupported facts in lexical text; slots (file path, value, count, unit, source line) are escaped and typechecked.
5. If evidence is confidential, Host restricts disclosure; proof existence is not content-access grant.

## Explainability UI
Tapping a phrase shows source ref, typed contract, proof method, examples tested, counterexamples, model assumptions, compiled artifact digest, runtime owner receipt and untested branches. Clicking "Why do you believe this?" never invokes the original function or triggers side effects. This can feed AutoWiki and HVI's historical explanation diff.

## Future flexible prose
An LLM can draft rich prose, but a second *semantic claim extractor* that maps each sentence to the verified fact graph is only a candidate validator. Its entailment is not a formal proof. For machine-certified Wiki claims use typed templates; elsewhere display `unverified interpretation` and linked sources.
