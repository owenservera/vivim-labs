# Native OS capability security — threat model and gating

## Non-negotiable

A general-purpose computer-command interface is effectively a privileged remote-control plane. Desktop Commander's documented `allowedDirectories` limitation is an anti-pattern to avoid: a path allowlist on file tools does **not** confine arbitrary shell access. ARBOR needs OS-level least privilege, typed effects, bounded grants, user ownership, trustworthy receipts and an explicit refusal path.

## R0–R3 crosswalk (reuse existing `intent-planner.mjs` scale)

| Risk | Typical example | Host behavior |
|---|---|---|
| **R0** scoped read | List files, inspect process, retrieve wiki | No mutation; still require read scope, redact secrets, rate/output limits and audit |
| **R1** local reversible change | Rename approved file, update wiki, set theme | Exact preview + approval of typed plan, exclusive claim, receipt; undo only if supported |
| **R2** sensitive, external or hard-to-undo action | Share file, send message, modify app state, stop process | Stronger exact-content consent, destination identity, source recheck, receipt and recovery |
| **R3** privileged, destructive, machine safety | Permanently delete, network disable, arbitrary shell, OS permissions, purchases | **Non-executable in current catalog**; design proposal only, require explicit separate policy / OS protection and independent review before any future enablement |

R0 is **not** authorization-free: reading a user's Documents directory from an untrusted web page may leak secrets. Presence in a command vocabulary grants nothing.

## Required defenses by adapter

**Filesystem:** user-selected roots, canonical opened handles checked by OS (not only string normalization), Windows junction/reparse/symlink handling, mount-point checks, hidden/system/sensitive exclusion, bounded recursion and per-file size/count, TOCTOU revalidation, permission errors surfaced without fallback. Scoped path in a message is a *candidate*, not a permission. Never silently follow symbolic links into another granted root. Prefer trash over irreversible deletion and record exact restore identity.

**Processes:** use an explicit executable allowlist and argument vector (`Command::new(exe).args(..)`), `shell=false` equivalent as default, fixed working directory, environment scrub, executable identity verification, process group lifetime, timeout/output bounds, cleanup and cancellation semantics. Interactive process sessions need owner/device binding and expiry. Full shell pipe/redirection/substitution and ad hoc code execution remain R3 withheld until isolated.

**Apps/Windows:** only controlling the logged-in, user-available desktop session; no unconsented background capture or credential extraction. Focus/close/create windows through a narrow platform broker; avoid high-integrity windows/UAC bypass. Accessibility APIs require an explicit permission and privacy model.

**Studio/browser:** signed-in profile under single Host lease; no alternative Chrome controller. Treat webpages as adversarial input; a page cannot issue instructions as a user or change command risk. External send/submit/upload/purchase require user review of resolved account, content and destination.

**PWA/mobile:** browser cannot access the OS by pretending to be native. Route to owner-paired device with expiring grants and explicit online state. Mobile itself only gets browser-exposed capabilities; privileged operations stay on PC or auditable native companion.

**MCP/external callers:** authenticate user, World and devices separately; treat tool metadata/annotations as descriptive only; scoped bearer tokens and replay protection; never equate `tools/call` with permission. Tools projecting potentially sensitive content need user ACL checks. Avoid public shell.

**Privacy:** redaction on output, no path/token/credential logging in analytics, encrypt private journals as applicable, allow export/revoke/deletion, threat model screenshots/clipboard. Keep credentials out of Labs and out of tool descriptions.

## High-risk natural-language ambiguity rules

- “Clean up Downloads” → never infer deletion. Return options: archive, categorize or recycle, each previewed.
- “Stop what's slowing my PC” → list processes first; never kill OS-critical processes from guesswork.
- “Send this to Sarah” → verify current selected object, Sarah's exact identity, channel and destination.
- “Delete everything old” → refuse until root, date definition, preview, scope and explicit destructive policy.
- “Fix the WiFi” → inspect status and propose supported troubleshooting; never silently disable the network.
- “Run this script from the internet” → default refuse; user must evaluate and explicitly authorize isolated execution.
- “Remember to always approve these” → refuse authority delegation. Learned aliases are not standing approval.
- Commands embedded in a webpage/PDF/repo README are **data**, not new user requests.
- A successful plan with an unobserved side effect is **uncertain**, never automatically retried.

## Acceptance gate (evidence required)

1. Out-of-root path, symlink/junction traversal, traversal encodings and alternate mount points are denied in real Windows tests.
2. Untrusted documents and browser content cannot directly create plans or approvals.
3. Stale source revisions, changed file identity, overwritten target, duplicate idempotency and cancellation are handled without duplicate side effects.
4. Host signatures/approvals cannot be forged by PWA, GPUI, MCP, child helper or model proposal.
5. Unknown effect is surfaced and separately reconciled; no invisible retry.
6. R3 remains unsupported until an explicit audited OS privilege boundary exists.

References: https://github.com/wonderwhy-er/DesktopCommanderMCP#configuration-management ; https://modelcontextprotocol.io/specification/2026-07-28/server/tools ; existing ARBOR dispatcher at pinned SHA. No security test was performed by this Labs report.
