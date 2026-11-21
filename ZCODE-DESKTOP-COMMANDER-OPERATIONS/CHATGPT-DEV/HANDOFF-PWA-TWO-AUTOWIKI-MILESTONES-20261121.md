# ChatGPT Dev → ZCode: first-contact PWA two-milestone release handoff

**Date:** 2026-11-21  
**Canonical product:** `owenservera/ARBOR`, direct `main`  
**Release source commit (verified HEAD):** `7ae24ff78a797581b510fe7a78105bbe301c0de4`  
**State:** SOURCE PUSHED; CLOUD SOURCE-SLICE CHECKS PASS; FULL REPO TESTS UNRUN; LIVE/PHONE ACCEPTANCE UNRUN. **Ready for SHA-pinned release validation by ZCode; NOT declared deployed or phone-verified.**

## Only two product outcomes in scope

1. **Human input → AutoWiki:** On the existing Direct first-contact PWA URL (`?first=1`), type into the existing `#text` textarea. AutoWiki automatically appears above the composer after the 180 ms debounce, reads only the admitted Host Wiki through existing Rust/WASM source-checked search, and never sends.
2. **Host-approved ChatGPT/agent input → same AutoWiki:** An explicit desktop-owner-approved Host Control plan produces a one-use draft delivery. The existing PWA textarea shows progressive input frames, which trigger that same Wiki listener; the Host later verifies source observation/ACK. Human input or revoked/stale fences win. No invented send or fake agent authority.

## Source reconciliation: REUSE and REPAIR, not NEW

The live implementation was **already in ARBOR**: `apps/mobile-relay/app.js`, `first-contact-composer.ts`, `agent-composer-adapter.ts`, Host `control/agent-composer-bridge.mjs`, Direct `composer/{observe,next,ack}` endpoints, `core/programmability/src/wiki_read.rs`, and the incumbent Rust/WASM bridge.

**Critical repair discovered in this Dev session:** `mountUniversalDock()` moves `#chat-composer` to `#app`, then `mountFirstWiki()` attempted `#v-talk .chat.insertBefore(firstWikiPanel, #chat-composer)`. Browser `insertBefore` throws `NotFoundError` because the composer is no longer in that parent, preventing AutoWiki mount. Updated to append `#first-wiki` at the end of the `#v-talk .chat` flex viewport, immediately above the root dock. No second composer or Wiki implementation introduced.

**ARBOR commit chain, in order:**
- `e1f3bd15e70fd985024081a8f03dcfc619545f4c` — fix mounting blocker in `apps/mobile-relay/app.js`.
- `fea0696230f021abc5b13f76e4e2bca23dac7e08` — dynamic DOM regression test in `apps/mobile-relay/test/first-customer.test.mjs`.
- `7ae24ff78a797581b510fe7a78105bbe301c0de4` — explicit regression assertion for original NotFoundError; current source pin.
- Prior integrated agent Unicode safety guard at `e2ef16ec221fb752fd50a1e5808d7e9a7a8e3e42` is an ancestor and preserved.

**Planning package checked:** Labs `DESIGN/AUTOWIKI-FOUNDATION-R001/R003-EXECUTION-HANDOFF.md` contains 37 authored Rust tests and staged modules for later context/language/HQKG work, explicitly outside the already-built human/agent typing path, and remains UNRUN. **Not merged**, because that would expand the owner's two-milestone task and introduce unverified graph/context dependencies. GPUI preparation remains paused.

## Cloud verification actually completed

Using the exact current GitHub `app.js` function slices in an isolated V8 mock-DOM harness:
- `mountFirstWiki`: PASS — Wiki attaches inside the visible Talk flex viewport, original reference-parent operation reproduces `NotFoundError`, moved composer remains at root, both normal `input` events reach the same listener.
- `scheduleFirstWiki`: PASS — human and agent input each create one 180 ms refresh, latest field text is searched, no Send.
- `renderFirstWiki`: PASS — source-backed row renders, authorized excerpt expands, old grant loses expansion rights.
- GitHub `main` commit and changed source/test blob SHAs fetched again: PASS.
- Full repository `node --test`, Rust Cargo, WASM package, Host runtime and actual phone: **UNRUN** (cloud container cannot clone GitHub; Rust/Cargo unavailable). **The V8 source-slice harness is not a real browser, HTTP Host, or physical phone test.**

## ZCode release-validation boundary — no source coding

1. Read the current live Host/release state. Preserve the user's dirty local checkout, running listener, existing URL and rollback option. **Do not overwrite local work, force-reset, or start a source-edit coding session.** Make a separate immutable SHA-pinned release working directory for the target commit or confirm newer HEAD contains these three commits.
2. Install/build strictly from product source. Use the real Bun executable/frozen lockfile as required. Ensure the Host can find its exact native Rust binary package location, and the PWA Rust/WASM asset has been built from this source. An isolated build is not a successful deployment.
3. Run source checks from the clean release directory; report command, exit code and counts:
   ```sh
   node --check apps/mobile-relay/app.js
   node --check apps/mobile-relay/sw.js
   node --experimental-strip-types --test apps/mobile-relay/test/first-customer.test.mjs
   node --experimental-strip-types --test apps/mobile-relay/test/agent-composer-browser.test.mjs
   node --experimental-strip-types --test apps/vivim-host/test/agent-composer-bridge.test.mjs
   node --experimental-strip-types --test apps/vivim-host/test/agent-composer-http-roundtrip.test.mjs
   cargo test -p vivim-programmability-core agent_composer
   node tools/packaging/build-product-wasm.mjs
   ```
   Validate actual scripts and native executable resolver before choosing command order; do not bypass a failed test.
4. Before switching the live service, capture prior source SHA, listener PID, Host authorization/Origin behavior, target URL and rollback details. Release new source only after build/tests pass; re-read new listener PID/served version and check protected same-origin read and composer routes. A denied unauthenticated request is not automatic failure; interpret the exact expected route policy.
5. On the **same existing first-contact URL**, perform physical-phone acceptance separately:
   - Human types a nonempty term: AutoWiki appears, updates as text changes, shows real Host Wiki matches or a truthful no-matches state. No Enter, Send, provider call or device scope leakage.
   - Desktop owner authorizes one Host `pwa.composer.draft.set` action: watch progressively rendered text in the *same* textarea and the *same* AutoWiki panel; confirm one source observation/ACK, grant and revision. No Send.
   - Negative: human editing mid-agent draft stops agent frames and prevents ACK; old revision or revoked grant cannot repaint private Wiki; IME composition does not permit agent edit.
   - Preserve screenshot/readback, exact response status, source revision/correlation ID **without** copying raw private draft or secrets into logs.
6. Report **SOURCE / TEST / PACKAGE / SERVE / HTTPS / PHONE-HUMAN / PHONE-AGENT** independently with commit SHA and proof. If any failure requires source changes, stop release and return exact reproducible finding to **ChatGPT Dev for a cloud fix and new GitHub commit**. ZCode must not write/refactor/commit product source.

**Exit criterion:** both real-phone typing milestones PASS on the stable URL and approved Host ACK evidence agrees; otherwise mark the named gate BLOCKED/UNRUN and hand source problems back to ChatGPT Dev.
