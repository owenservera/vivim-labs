# web-harness: one interface for many AI webapps

Goal: one local interface that uses the user's own signed-in ChatGPT, Claude, DeepSeek, Qwen, Z.ai (and Gemini) web sessions as LLM backends, and can be called as a subagent. Zero runtime dependencies (Node >= 22, global `WebSocket`, `node:test`).

## Hard rules (inherited from /AGENTS.md)
- The harness drives ONE Chrome profile on loopback port **9333**: by default VIVIM's signed-in profile `<CAPTURE_STUDIO_HOME or C:\VIVIM>\chrome-profile` (same path vivim-harness resolves), or `runtime/chrome-profile/` via `--profile lab`. It must refuse port 9222, any non-loopback host, and any other user-data-dir. Never attach to VIVIM's Chrome (38702/9222); refuse to launch while another Chrome or a VIVIM drive lease holds the profile.
- No cookies, tokens, storage or conversation exports are read, printed or stored. Journal stores digests and metadata only, never prompt or reply bodies.
- Every send needs a human approval record (see Approval). No automatic retry, no second send method, no draft clearing.

## Layout (lane ownership)
```
harness/
  CONTRACT.md                    (orchestrator)
  package.json                   (orchestrator)
  src/core/cdp.mjs               LANE A
  src/core/journal.mjs           LANE A
  src/core/send.mjs              LANE A
  src/core/reply.mjs             LANE A
  test/core/*.test.mjs           LANE A
  src/providers/<id>.mjs         LANE B  (chatgpt claude deepseek qwen zai gemini)
  src/providers/index.mjs        LANE B
  scripts/discover.mjs           LANE B
  test/providers/*.test.mjs      LANE B
  bin/wh.mjs                     LANE C
  bin/wh-mcp.mjs                 LANE C
  src/interface/*.mjs            LANE C  (server, approval, launch, router)
  test/interface/*.test.mjs      LANE C
```

## Provider profile (data only, default export of src/providers/<id>.mjs)
```js
{ id, label, hosts: [..], url,
  threadPattern: /regex with a named group (?<id>...) on pathname/,   // exact native thread route, or null if unknown
  composer: { selectors: [..] },
  send: { strategy: 'enter' | 'button', buttonSelectors: [..] },
  assistant: { containerSelectors: [..], streamingSelectors: [..] },
  preflight: { authWallSelectors: [..] },
  status: 'measured' | 'seeded-unverified' }
```
`src/providers/index.mjs` exports `listProviders()`, `getProvider(id)`, `providerForUrl(url)` and `validateProfile(p)` (throws coded errors).

## Page (CDP) interface, Lane A, consumed by send/reply and by everything else
```js
connect({ port, host = '127.0.0.1' }) -> Browser   // throws CDP_PORT_REFUSED for 9222 / non-loopback
Browser.targets() -> [{ id, url, title, type }]    // read-only
Browser.openPage(url) -> Page                      // new tab owned by harness
Browser.attach(targetId) -> Page                   // EXACT id only; never "first/active/newest tab"
Page.url() -> string ; Page.evaluate(expr) -> value ; Page.insertText(text) ; Page.pressEnter() ; Page.click(selector) ; Page.close()
```
All of Page must also be implementable by a plain in-memory fake (tests pass a fake Page; no real Chrome in tests).

## Journal (Lane A): `createJournal({ dir })`
Per-request receipt file `<dir>/<requestId>.json`, written BEFORE any effect, replaced atomically, created with exclusive create. Fields: `requestId, provider, threadKey, payloadDigest, state ('accepted'|'typing'|'dispatching'|'observed'|'uncertain'|'blocked'|'failed'), submitted ('no'|'yes'|'unknown'), boundary{userTurns,assistantTurns}, replyDigest, createdAt, updatedAt, reason`. API: `create(rec)`, `update(id, patch)`, `read(id)`, `list()`. Rule: a request whose `submitted` is `yes` or `unknown` is NEVER sent again; `create` with an existing id returns `{ created:false, existing }`.

## sendOnce (Lane A): `sendOnce({ page, provider, journal, requestId, threadUrl|'new', prompt, approval })`
1. Refuse (state `blocked`, nothing typed) unless `approval` is a valid record for this exact `requestId` + `payloadDigest` (+ thread) and unexpired.
2. Preflight read-only: auth wall absent; page URL matches provider hosts; if `threadUrl` given, page URL's thread id equals it exactly (else `blocked` BINDING_MISMATCH); composer found; **existing draft non-empty => blocked DRAFT_PRESENT, never cleared**; record turn baseline.
3. Journal `typing` -> `insertText`; verify composer text equals prompt exactly; mismatch => `blocked` (nothing submitted).
4. Journal `dispatching` (submitted:'unknown') -> exactly ONE submit by `send.strategy`; an error during dispatch is NOT proof of non-delivery.
5. Observe read-only (bounded waits): a NEW user turn appears with matching text and counts exceed baseline => `observed`, submitted:'yes'. Otherwise `uncertain`, submitted:'unknown'. Cleared composer alone is not evidence. Return `{ state, submitted, requestId, threadUrl }`; never retry.
`readReply({ page, provider, journal, requestId, timeoutMs })`: read-only; waits until streaming selectors are absent AND reply text is stable for N samples; returns `{ text, complete:boolean, digest }`; records replyDigest. `resume(requestId)` reads only; for uncertain requests it can read the bound thread and report, never send.

## Approval (Lane C): `createApprovals({ dir })`
`request({requestId, provider, threadUrl, payloadDigest, preview})` writes a pending file and prints a human-readable preview (provider, thread, first 200 chars, char count); `approve(requestId)` is only callable from the interactive CLI command `wh approve <id>` (never from the HTTP/MCP surface); approvals expire (default 10 min), are single-use, and bind requestId+payloadDigest+threadUrl.

## Interfaces (Lane C)
- `wh providers | status <provider> | ask <provider> [--thread URL|--new] --prompt-file F [--id ID] | approve <id> | reply <id> | list`
- `wh serve`: loopback-only (127.0.0.1) HTTP: `POST /v1/ask` creates the request + pending approval and returns `202 {requestId, status:'pending_approval'}`; `GET /v1/requests/:id` returns status/reply; `POST /v1/chat/completions` (OpenAI-compatible, non-streaming) which blocks until the request is approved + answered or times out. Approval is NEVER grantable over HTTP.
- `wh-mcp`: stdio MCP server exposing `web_ai_providers`, `web_ai_status`, `web_ai_ask` (creates pending request, returns requestId), `web_ai_result`. This is how opencode can call a signed-in web AI as a subagent.
- `scripts/launch-chrome` (in src/interface/launch.mjs): launches Chrome with `--remote-debugging-address=127.0.0.1 --remote-debugging-port=9333 --user-data-dir=<repo>/runtime/chrome-profile`, no other flags that copy a profile; the user signs in manually.

## Safety acceptance tests every lane must include (fake Page)
exact thread mismatch blocks; non-empty draft blocks and is untouched; missing/expired/wrong-payload approval sends nothing; dispatch error then no evidence => `uncertain` and a second `sendOnce` with the same id sends nothing; port 9222 and non-loopback refused; HTTP surface cannot approve.
