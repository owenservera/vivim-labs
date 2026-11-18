# PG02 negative acceptance matrix

**Observed Node.js v22.16.0:** node --test test/semantic-visual-crosswalk.test.mjs; EXIT 0; 20 pass, 0 fail, 0 skip, 0 cancel. Test and module Git blob identity matches locally executed bytes. One earlier synthetic risk fixture error produced 19/20 and was corrected before the final pass; no production bug inferred.

| Adversarial case | Required behavior | PG02 fixture |
| --- | --- | --- |
| Unknown semantic action or duplicate binding | explicit omission, no handler inferred | PASS |
| Wrong Host owner, risk, or catalog version | omit/refuse, no catalog escalation | PASS |
| Stale input/source/catalog/policy/graph/world | refuse or omit patch | PASS |
| Foreign actor or producer scope | no visible candidate | PASS |
| Denied producer with private semantic ID | show coverage, hide node identity | PASS |
| skipped_budget, failed, unavailable states | surface coverage and no permission | PASS for skipped_budget/failed |
| Negated don't send | never allow provider send (test ID synthetic only) | PASS |
| Untrusted/revoked/stale ScreenContext | no items emitted | PASS |
| Wiki source stale while intent seems valid | report wiki stale independently | PASS |
| Split astral UTF-16 emoji span | refuse; preserve text/caret | PASS |
| IME composing | refuse derived selection | PASS |
| Two-Tab source revision outdated/unregistered | no unsupported suggestion or made-up action | PASS |
| All candidate pathways | effectPerformed=false, executable=false, no receipt | PASS |

UNRUN integration/security acceptance: actual trusted actor injection; source file content/digest owner authentication; unsupported @hq:semantic/3 parser rejection end-to-end; real D project reader/KB or grants; B exact original-text plan/approval/receipt; unknown/stale action under current live Host World; C Chrome send once and crash replay; A Cargo/N0 trust and cancellation; E real PWA/IME/caret/accessibility; W real revocation/flip back; F Ajv schemas; Rock disk and WAL/SHM/recovery; all G combined work. Source-only blob equality does NOT demonstrate any of these.
