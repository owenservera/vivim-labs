# Chat atoms — proposed versioned contract and evidence model

**Status: PROPOSED, not implemented.** Current note units are not chat atoms; current Studio journal and Host live projection contain metadata/digests and are not raw transcript body stores.

## 1. Canonical addressability

A conversation/message belongs to a scoped source authority; a chat **atom** is an addressable contiguous text segment with stable content-derived identity and explicit occurrence/revision binding. Never use only content hash as message identity: two different speakers or threads may have identical text.

Candidate v1 (rename/reconcile after source inspection):

```json
{
  "schema": "vivim.chat-atom/1",
  "atomId": "ca-sha256-prefix",
  "unitDigest": "64-hex",
  "source": {
    "authority": "studio_capture | approved_import | local_world",
    "principalRef": "local-principal",
    "provider": "chatgpt",
    "threadRef": "source-scoped-thread-id",
    "messageRef": "source-scoped-message-id",
    "sourceRevision": "opaque-monotonic-revision",
    "sourceContentSha256": "64-hex",
    "journalEvidenceRef": null
  },
  "location": {
    "segmentIndex": 0,
    "startUtf8Byte": 0,
    "endUtf8Byte": 128,
    "startUtf16": 0,
    "endUtf16": 128
  },
  "role": "user | assistant | tool | system | unknown",
  "trust": "observed | imported_unverified | owner_confirmed",
  "sensitivity": "private",
  "visibility": "local_only",
  "createdAt": "source-observed-time",
  "contentRef": "local-private-store-ref",
  "contentStoredInPublicManifest": false
}
```

Contract details: original bytes and Unicode normalization policy must be fixed; hash stored bytes (do not silently normalize); use SHA-256 full digest for verification; truncated display ID needs collision resolution. source+revision+segment occurrence disambiguates repeated identical text. Start/end range semantics must be monotonic, within source bounds, and validated against digest. UTF-8 byte offsets are canonical for integrity; UTF-16 offsets enable browser display/highlight. No message body should be committed in these docs/feeds.

## 2. Source verification rules

- Capture must refer to exact provider account/profile/thread and message ID with an independently verifiable local source revision.
- If source journal only contains contentDigest/chars and no body, it can verify an already-observed matching reply but cannot supply missing text.
- Incoming provider text is **untrusted data** for instruction purposes, even after checksum validation; checksum proves bytes, not truth.
- Disallow cross-user/thread dedup that leaks the fact of a private matching message. Hashes/embeddings can be sensitive; keep private indexes on-device and scoped.
- Unknown/uncertain send or reply state must not be promoted to delivered. Rendering/ACK needs separate verification.
- On edit/regeneration/import, immutable historical atom revisions remain addressable, but current selection only uses explicitly current or requested past version.
- On delete/forget, cascade tombstones/caches and derived search/vector indexes by authoritative deletion scope; retain minimal permissible audit metadata only.

## 3. Segmenting and indexing

Start with deterministic block/paragraph/code-boundary segmentation from existing capture AST or canonical plain text. Respect source formatting and code blocks; avoid a generative splitter for identity. Permit optional smaller retrieval spans with stable anchors. Dedupe repeated segments only within permitted scope; never alter conversation role/source attribution. Tokenization/ranking/index quality is independent of atom identity.

Consider separate optional local FTS5 retrieval and optional tiny embedding model; neither is required for a correct deterministic source-backed first release. Context Lake Git events remain separate from personal chat index.

## 4. DCB integration

A chat-atom source adapter receives principal, exact conversation/thread ranges, allowed refs, source revisions, purpose and destination disclosure. It returns only source-authorized candidates with classification conversation, a digest, bounded span text read locally, evidence status and explicit sensitivity. DCB composer enforces existing candidate scope/sensitivity/destination rules. A manifest lists chosen span refs, source revs, hashes and omitted reasons; preview shows exact user-visible content *before* external send. A relay metadata feed never includes the body or secret-bearing atom identifiers.

## 5. Minimal acceptance tests

1. Same exact source bytes/segmentation/revision give byte-identical refs; role/thread differences remain distinguishable.
2. Insert neighboring paragraph: unchanged unit digest persists; occurrence and source range are corrected without false citation.
3. Duplicate paragraph twice: distinct occurrence refs, no collision; stable address policy documented.
4. Edit/regenerate: old revision still verifiable as historical; current selection does not mix revisions.
5. Source journal digest mismatch: fail closed; no independent send evidence manufactured.
6. Cross-principal, cross-thread, secret and denied-provider disclosure: zero selected content.
7. Unicode emoji/combining marks/CRLF/code blocks: correct byte+UTF-16 ranges, copy-back digest matches.
8. Prompt injection inside a chat atom cannot change instructions, invoke tools or authorize effects.
9. Export/import rebinds source-scoped IDs under explicit mapping, preserves digest and provenance; delete invalidates derived indexes.
10. DCB budget, omissions and context digest reproducible for identical pinned inputs.

**Do not write this format into live state until source ownership, privacy and migration are reviewed.**
