# ChatGPT Dev — readable local UI image verification protocol

Date: 2026-11-21 (Europe/Madrid)  
Owner: ChatGPT coding/acceptance lane  
Status: Screenshot readback **operational**; full end-to-end source-backed Wiki/agent-typing acceptance **not yet passed**.

## Objective

The ChatGPT coding agent must **see the actual pixels** of real local VIVIM PWA screenshots, not merely filenames, HTTP codes, accessibility-tree strings, or a test script's PASS. Each local UI acceptance exercise must be verifiable again in a separate ChatGPT session without confusing evidence collection with product implementation.

### Fixed boundaries

- ARBOR GitHub `main` is canonical product source. ChatGPT implements application changes in its cloud workspace, pushes via direct GitHub connection, and independently verifies the remote SHA.
- Desktop Commander is for read-only local observations and explicitly authorized runtime/UI acceptance; not a replacement product-source coder. No local source commits, destructive reset, or unrequested ZCode delegation.
- The PWA is an internal VIVIM interface. Do not test external AI-provider browsers. Chrome, WebView or another renderer may display this **internal** PWA, but is not a provider integration.
- Do not use Playwright, Puppeteer, CDP, browser-use or synthetic DOM-only render as an acceptance substitute. The 2026-11-21 exploratory CDP snapshots below were visually readable but **do not count as OS-level capture compliance**.
- Protect local screenshots; never publish screenshots containing sensitive text or credentials to GitHub.

## End-to-end evidence path

**1 — Local capture, bound to the correct window.** Launch/open the internal PWA at the stable URL and record the real process ID, HWND/title, page identity, viewport, and git release SHA. Assert foreground/window identity *before each screenshot*, verify that UI Automation finds the actual PWA composer, and abort on a mismatched window, occlusion, blank/black frame or wrong page. Prefer an OS-owned window capture method that cannot silently screenshot an overlapping ChatGPT tab. If CopyFromScreen is used, validate the foreground HWND and visually inspect the first frame before continuing.

**2 — Drive three real user-visible states.** Keep the same window/viewport: (a) empty composer before input, (b) partial text input with AutoWiki visible **without Send**, (c) longer completed text with AutoWiki updated. Generate real OS input in the intended field, with enough pause for Wiki debounce and repaint. Prefer accessibility/UIA readback of composer text and Wiki content as independent corroboration, not as a replacement for screenshots. If testing agent-initiated typing, use an owner-authorized Host command/effect receipt and label it separately from human-simulated keystrokes.

**3 — Save a local private evidence bundle.** Store three PNG files plus a small manifest containing `releaseSha`, `url`, `captureTime`, `windowPid`, `windowHandle`, `step`, `imagePath`, `width`, `height`, `sha256`, `typedText` (only for non-sensitive fixture input), `wikiState`, `wikiRevision`, `testMode`, `captureMethod`, `expectedWindowMatched`, `errors`. Preserve images unmodified. Count PASS only after actual image readback.

**4 — Bring image pixels into ChatGPT's visual context.** Remote Desktop Commander `read_file` returns image content in `content_items`; simply printing `JSON.stringify`, the path, or metadata does **not** present the image to the model. Discover the exact normalized tool and pass image blocks through the `image` helper in `functions.exec`:

```js
const deviceId = "<online device ID>";
const files = [
  "<absolute path to empty PNG>",
  "<absolute path to partial PNG>",
  "<absolute path to complete PNG>",
];
for (const path of files) {
  const r = await tools.mcp__Remote_Desktop_Commander__read_file({ path, deviceId });
  text("Visual evidence: " + path);
  const parts = r.content_items ?? [];
  for (const part of parts) {
    if (part.type === "image") image(part);
    else if (part.type === "text" && part.text) text(part.text.slice(0, 500));
  }
}
```

If a call returns `status: running` with a `call_id`, call `mcp__Remote_Desktop_Commander__get_tool_result` with that ID, **not** a duplicate read. If `content_items` has no real image part, report IMAGE_NOT_READ and do not infer screenshot content.

**5 — Inspect and independently adjudicate.** ChatGPT visually identifies the PWA (not ChatGPT or another foreground window), exact typed text, presence/state of Wiki, changes between states, readable error strings and whether real search results appear. Cross-check manifest hashes and Host diagnostic revisions where available. Explicitly distinguish (i) field/overlay functionality, (ii) Wiki content/data enrichment, (iii) agent-authorized typing, and (iv) real phone/touch proof.

**6 — Truthful release receipt.** Report `PASS/FAIL/PARTIAL/UNRUN` for all four areas, the exact GitHub SHA, unchanged PWA URL, capture mechanism, hashes, any failed runner code, and the next defect. No claim of a full e2e PASS when only screenshots or unit tests have passed.

## Verified test observation — 2026-11-21

- Canonical ARBOR GitHub `main`: `7ae24ff78a797581b510fe7a78105bbe301c0de4`. Local Host was listening on `127.0.0.1:43187`, PID 5608 at inspection. Internal `GET /mobile/index.html?first=1` returned HTTP 200.
- Local evidence root: `C:\\0-BlackBoxProject-0\\.arbor-pwa-releases\\7ae24ff78a79\\_screenshots`. This path is **on the authorized user's Windows computer**, not a ChatGPT sandbox path or GitHub attachment.
- Three exploratory browser-viewport PNGs were opened using Desktop Commander `read_file` and **each image part was forwarded to ChatGPT visual inspection**:
  - `01-before-typing.png` SHA256 `D893AD76099C9B5934E31715A44BA87E2510DBCF31D9A8863D060430F38CA253`: PWA input reads “Ask anything...” and Wiki not visible.
  - `02-typing-rust.png` SHA256 `6CC895EB9D64E065F8B86C590A1F3A0E0D6250BE346455CD044CCF7A71DF9A2D`: typed “Rust”; Wiki popup appears, “WIKI · 0 matches · revision 1”, message “No matching Host Wiki entries yet. ARBOR source-code indexing is the next connection.”
  - `03-typing-rust-core.png` SHA256 `6C4EC44E208D1DF21EA0986AAE3FC12D5852BECF0284667893B3F11A5A6BBB54`: typed “Rust core”; Wiki remains visible but still shows 0 matches at revision 1.
- These three viewport images are **visually valid and readable**, but were captured by a one-off local Chrome/CDP verification harness, not the desired OS-level capture method. That harness ended with `TypeError: Cannot read properties of undefined (reading 'composerConnected')`; therefore its own machine acceptance result is **FAIL**, not PASS. This runner error did not prevent the visual inspection of the saved PNG pixels.
- Earlier OS-level `1-empty.png` wrongly captured a **ChatGPT** window rather than the VIVIM PWA: **INVALID** evidence. Window/process identity and visual inspection must be hard gates in any replacement capture workflow.
- Visual field input and Wiki popup without Send: **PASS (browser-viewport visual inspection only)**. Data-backed Wiki results: **NOT YET WORKING** for these queries (0 matches, source index absent). Owner-authorized live agent typing: **UNRUN**. Real phone/Android UI verification: **UNRUN**. Compliant three-step OS screenshot capture: **UNRUN**.

## Next acceptance slice

Run a bounded OS-level capture using the foreground identity guard, consume **all three image blocks** via the direct readback pattern above, and compare them to this visually inspected baseline. Then separately connect the existing Host Wiki/HQKG source index on canonical cloud `main` and verify at least one real indexed result in the Wiki. Do not merge the image-reader proof with the source-indexing task; they are independently measurable.
