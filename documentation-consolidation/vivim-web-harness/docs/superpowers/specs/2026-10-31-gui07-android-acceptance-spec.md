# GUI07 — Real Android Bootstrap V1 → Build generative UI acceptance (spec)

**Status:** deferred — blocked on real paired Android device + verified signed-in web AI provider.  
**Owner:** unassigned. **Roadmap:** GUI07 (depends on GUI02, GUI05, GUI06, BF01, BF03, H09).  
**Purpose:** acceptance criteria executable on a real Android phone; code-side prerequisites (GUI01–GUI06) are implemented and unit-tested (105 tests pass, integration check 0 errors).

## Acceptance criteria (run on a real paired Android phone)

Each step must produce a timestamped receipt with request IDs; no step may be claimed on unit tests alone.

1. **Pairing.** Phone pairs with existing PC over Direct/Tailscale. `GET /mobile/api/relay?path=status` returns `paired: true`, `transport: 'direct'`, `cloudEnabled: false`. Receipt: pairing request ID.
2. **Provider verified.** At least one signed-in web AI provider shows verified-ready (read-only readiness manifest, not inferred from tab load). Receipt: provider readiness timestamp.
3. **BF00 immediate first paint.** Before any AI reply, the phone paints a safe provisional theme/blank Build surface (< 500 ms local paint). Local preset edits respond immediately (< 100 ms p95 tap feedback). Receipt: first-paint timestamp + preset.
4. **F02 + F01 through shared renderer.** Real ChatGPT F02 theme + grounded F01 card arrive via the existing Studio owner + send-once journal and render through the SAME GUI V1 renderer (Build surface), not a separate bootstrap renderer. Receipt: Studio thread/request ID, compiler validation, Host commit.
5. **D03–D07 optional view.** At least one representative deep-pass view proposal (card/collection/timeline/mode) renders as a reviewable optional component on the shared surface; D06 stays PC-private (absent from phone). Receipt: pass ID + view proposal ID.
6. **Continuation into Build.** User opens Build on the same surface/same revision/same component IDs — no conversion or data loss. Talk draft and protected Personal AI entrance preserved.
7. **Selected Build edit.** User selects/edits a component (tap) without AI. Tap produces a `vivim.ui-proposal/1` against the current `baseRevision` (compare-and-swap). Receipt: proposal digest + base revision.
8. **Host commit + Undo.** Owner approves the proposal; Host CAS-commits a new `vivim.ui-revision/1`; phone repaints to the new revision; Undo restores the previous revision (append-only history, never deletion). Receipts: Host commit, phone repaint, Undo.
9. **Reconnect / resync.** After phone/Host reconnect, stale revisions resync correctly or block honestly; no uncertain send is auto-replayed. Receipt: reconnect request ID.
10. **Talk regression.** Existing Talk draft and multiline draft are intact throughout; no Talk-draft clobber. Receipt: Talk draft checksum before/after.

## Negative fixtures (must fail closed)

- Malformed/truncated/prose/Markdown-fenced provider output → rejected, last-known-good UI intact.
- Unknown component / over-depth tree / cycles / duplicate IDs / invalid revision → rejected without writes.
- Stale `baseRevision` → refused, active revision unchanged.
- Uncertain browser send → `uncertain` status, no resend, undo unavailable.
- D06 private relationship → never appears in phone projection.

## Measurable targets (not claimed results)

- First local paint < 500 ms; local tap feedback < 100 ms p95; compiler validation < 50 ms for bounded payload on PC; Direct Host-approved revision → visible paint < 500 ms p95.
- AI reply latency measured separately.

## Why blocked

GUI07 requires a real paired Android phone with a verified signed-in web AI provider and a live Host/relay. No such device or provider session is available in this environment. All code-side prerequisites (GUI01 catalog/compiler/renderer, GUI02 adapters, GUI03 design-context composer, GUI04 Host CAS commit/Undo, GUI05 phone Build surface + apply-proposal command channel, GUI06 ui-revision status) are implemented and unit-tested. The roadmap item remains `queued` until real-device receipts exist.

## Related commits (code-side, already merged)

- `67deeef` feat(gui04): Host-owned `vivim.ui-revision/1` CAS commit for ui-proposal commands
- `f1c7d48` feat(gui05): phone Build surface renders committed UI revisions and taps commit via apply-proposal
- (this session) GUI03 design-context composer, undo-ui-revision command, ui-revision status tracking, Build tab wiring + polling

## Decision register

None — all implementation decisions were bounded by PRD 11 / GENERATIVE-UI-BOOTSTRAP-V1 and the existing seven bootstrap prompt schemas (unchanged).
