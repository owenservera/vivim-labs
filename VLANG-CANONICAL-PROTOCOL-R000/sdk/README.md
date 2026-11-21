# VLANG SDK — research scaffold R0

Status: **SDK scaffold, not an ARBOR-installed package**. No application runtime is changed.

## Surface

One zero-dependency Node ESM facade, `@vivim/vlang-sdk-research`, with three operations:

- `inspectSource({ pin, source, registry })`: hashes supplied source bytes, delegates to real HQ `parseAnnotationV2`, returns bounded declared/unverified annotations.
- `getVerifiedContext({ task, pin, maxTokens })`: delegates to HQ context composition and to a separate adapter that revalidates actual source bytes and exact packet references.
- `compareSourcePins(previous, current)`: conservatively identifies changes and refuses inferred move/rename identity.

The required adapters are `parseAnnotations(source, options)`, `composeContext({task,pin,maxTokens})`, and `verifyContext({packet,pin})`. All are injected: this SDK does not read the filesystem, run Git, write the database, open a network connection, call a model or grant approval.

### Installation / exploratory test

From this `sdk` directory run `node --test test/*.test.mjs`. Node 22+ is sufficient for the scaffold; ARBOR's production HQ currently requires Node >=24.

```js
import { createVlangSdk, sourceSha256 } from './src/index.mjs';
// In ARBOR, pass a real adapter around HQ's existing source scanner,
// semantic registry and context composer/verifier. Never use a stub in production.
const sdk = createVlangSdk({ parseAnnotations, composeContext, verifyContext });
const pin = { projectId: 'my-project', path: 'src/main.ts', sourceSha256: sourceSha256(sourceText) };
const inspection = await sdk.inspectSource({ pin, source: sourceText, registry });
// Declared comments are NOT runtime proof and do NOT authorize effects.
```

### Boundary and support matrix

| Input | R0 stance |
| --- | --- |
| `.js/.mjs/.cjs/.ts/.mts/.cts` | comment scanner inherited from HQ; not syntax-aware symbol binding |
| `.rs` | **file header only**; never claim inline symbol attribution |
| `.jsx/.tsx` | unsupported; fail closed |
| Python and other languages | unsupported |
| `@hq:` v1 | v1 stays owned by HQ protocol linter |
| `@hq:` v2 | supported through injected HQ v2 parser |
| `@hq:` proposed v3 and `@v.` | research-only; no automatic translation |
| tool/Host effects | unavailable; Host remains sole approval/receipt/writer |

Never use `sourceSha256(sourceText)` as proof that the text is current on disk. The caller must verify the live workspace through HQ's `verifyContext` and/or source verification before planning or publishing. The adapter normalizes its real verdict into `{ ok: true, sourceSha256: pin.sourceSha256 }` only after all source references validate. This bridge does not itself implement the verification.

### Next integration

1. Bind this scaffold to actual ARBOR `hq/lib/annotation-v2.mjs`, `hq/lib/semantic-registry.mjs` and `hq/lib/context.mjs`, without copying the parser or graph.
2. Build AST-aware source binding for inline Rust/TSX via an incremental language adapter (tree-sitter/rust-analyzer/tsc), never by regex scope guessing.
3. Add owner-reviewed evidence and claim verification, followed by passive HQKG/AutoWiki projections.
4. Only then consider a separate Rust implementation of the pure contract types, reusing the existing core/HQKG runtime.

Research source: `../.archive/` is historical and immutable; decisions: `../results/ROUND-1-SDK-ARCHITECTURE.md`.
