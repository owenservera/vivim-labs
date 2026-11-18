# V-visual — pure pre-G realtime semantic feedback slice

**PREPARED_NOT_PROMOTED**. Branch `work/p5-preg-visual-language-r001`. All files remain inside this directory; no donor or product source files are modified.

`PROTOTYPE/projection.mjs` implements read-only **multi-producer** revision convergence and three display densities. `PROTOTYPE/fixtures.mjs` contains a synthetic multi-clause, quote/negation/ambiguity case. `PROTOTYPE/projection.test.mjs` contains portable Node tests; run with:

```bash
node --test .integration/proposal5/phase2/pre-g/V-visual/PROTOTYPE/projection.test.mjs
```

Only `ingestPatch` accepts producer evidence; it refuses changed input and source/catalog/policy/graph revisions, old producer revisions, invalid spans and attempted permission/authorization inflation. `renderDensity` never changes candidate semantics: Quiet/Assisted/VIEW ALL only filter paint keys; accessible text covers all source candidates. `proposeSelection` and `proposeUndo` yield **reversible draft proposals**, never effects or implicit submit. They have local (not cross-device durable) CAS guards. IME/caret belongs to the surface.

See `VISUAL-TRUTH-MATRIX.md` for R005 source eligibility, R006/R007 design-only distinctions; `NATIVE-UI-BOUNDARY.md` for actual Rust R001 versus proposed rich interface; `OWNER-HANDOFF.md` for post-G ports and owners; `RUN-LEDGER.json` for executed vs UNRUN acceptance.
