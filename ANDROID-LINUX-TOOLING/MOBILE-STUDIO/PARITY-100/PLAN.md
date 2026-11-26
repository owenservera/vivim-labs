# Android ARBOR parity — 100 atomic research tasks

Owner objective: mobile ARBOR/Studio with as much Windows functional parity as Android supports; first app WhatsApp, first acceptance bounded recent conversation read before send. This is Labs research planning, not a replacement Product task catalogue. Work sequentially along the first app-launch/capture/read critical path; later groups may proceed only with prerequisites and ownership. A verified item means its specific acceptance evidence is available, never that an entire subsystem has parity. No root, bypass, silent capture, protected app data access or background surveillance. Android OS constraints are legitimate outcomes. No llama.cpp.

Windows references: `tools/mcp/vivim-arbor/HANDOFF.md` blob `13da650699beaccba1957813d8658303fb50bad8`; `NATIVE-WORKSTATION-PARITY.md` blob `4cccc73a852e8933b3ae18649374e883118a22ce`; Android bridge `android/vivim-android-runtime.cjs` blob `204cf46a3b8d8a405d2bda6325bef675bd30a29b` in ARBOR main. Live version 0.3.0 observed; source-to-deployed equivalence not proven. Existing Product task catalogue and Host authority remain canonical. User-reported app launch failure overrides command exit 0. See R001-EXECUTION.md for evidence.

## 01 — Bootstrap and foreground app control
Windows mapping: Windows 01–09, 27–28

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-001 | Pin Windows baseline | Record Windows parity/handoff and Android runtime blob SHAs | verified |
| AND-002 | Verify live Android transport | Health ready; read current grant without publishing secrets | verified |
| AND-003 | Inventory Android platform | API, native ABI, storage and existing toolchain evidence | verified |
| AND-004 | Discover application launch targets | Same-user package inventory and WhatsApp launcher/share/deep-link resolution | verified |
| AND-005 | Reconcile failed launch | Record rejected shell call and zero exit with owner-confirmed no foreground launch | verified |
| AND-006 | Assess APK toolchain | Identify native aapt/aapt2/d8/signing/JDK candidates and missing platform jar | verified |
| AND-007 | Implement launch receipt semantics | Native Rust tests reject command-only, stale and mismatched foreground evidence | open |
| AND-008 | Prepare reproducible APK build tools | Validate signed upstream tools, platform API jar and native compatibility; no root | open |
| AND-009 | Build foreground companion fixture | Debug APK with user-tapped launch action, scoped package queries and distinct dispatch state | open |
| AND-010 | Install and verify foreground launch | Owner installs candidate and confirms actual WhatsApp foreground; no forced app stop | open |

## 02 — Screen observation and app lifecycle
Windows mapping: Windows 03, 26, 30–31

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-011 | Request screen consent | User selects test app through Android MediaProjection dialog | open |
| AND-012 | Start capture foreground service | Correct API 35 service type and visible notification | open |
| AND-013 | Acquire bounded frame | One frame from owned fixture; dimensions and digest recorded | open |
| AND-014 | Enforce capture session identity | Reject reused, expired and unrelated capture tokens | open |
| AND-015 | Stop capture | User stop/revocation ends frame production and deletes transient frames | open |
| AND-016 | Test secure-window boundary | Protected fixture yields no readable protected pixels; no workaround | open |
| AND-017 | Verify foreground observation | Distinguish app dispatch from visible target evidence; uncertainty explicit | open |
| AND-018 | Handle orientation and resize | Reconfigure buffers without reusing consent token for a new display | open |
| AND-019 | Define app switch/home/back | Use supported user-mediated actions; confirm visible transition | open |
| AND-020 | Define close semantics | Return/hide own activity or user removes task; do not force-stop third-party apps | open |

## 03 — WhatsApp first end-to-end integration
Windows mapping: Windows 30–31, Studio adapter semantics

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-021 | Register WhatsApp read scenario | Bound three recent conversations and three messages per chat | verified |
| AND-022 | Open WhatsApp conversation list | Owner confirms launch or selected app capture; no private database access | open |
| AND-023 | Read list rows | Extract visible conversation names, preview timestamps and pinned indicators | open |
| AND-024 | Rank recent activity | Determine recency from visible timestamps; report ambiguity/archived gaps | open |
| AND-025 | Confirm read-state effects | Owner acknowledges opening unread chats may mark them read | open |
| AND-026 | Observe three selected chats | User navigates selected chats; capture only requested visible excerpts | open |
| AND-027 | Extract bounded message fields | At most nine messages, visible sender/time/text; uncertainty explicit | open |
| AND-028 | Return and purge evidence | Present requested results privately; remove transient captures; no Labs content | open |
| AND-029 | Test self-chat draft | Resolve recipient and exact text; owner reviews before send | open |
| AND-030 | Verify self-chat send | Owner confirms send; observe actual result; distinguish dispatch and delivery | open |

## 04 — App interoperability and semantic Studio
Windows mapping: Windows 21–25, 30–31

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-031 | Discover launcher activities | Native companion enumerates permitted launchable apps with actual visibility limits | open |
| AND-032 | Resolve app metadata | Map labels, icons, version and package identity; never assume complete inventory | open |
| AND-033 | Track package changes | Reconcile install/update/remove events through public APIs | open |
| AND-034 | Resolve supported intents | Query VIEW/SEND/OPEN_DOCUMENT candidates without effects | open |
| AND-035 | Model deep links | Allow only validated app links; handle missing targets and chooser | open |
| AND-036 | Implement outbound share | Share selected text/file via explicit target or chooser with URI scope | open |
| AND-037 | Implement inbound share | Receive user-shared fixture content with MIME/size validation | open |
| AND-038 | Project semantic controls | Model app-provided nodes or user-approved visible observations with revision identity | open |
| AND-039 | Test user-directed UI interaction | Use owned UI Automator fixture; evaluate permitted interfaces, no general autonomous accessibility agent | open |
| AND-040 | Build app profile projection | Graph app contracts, evidence, unknowns and revocation; reuse Product descriptors | open |

## 05 — Native files and documents
Windows mapping: Windows 14–15, 21–25

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-041 | Verify scoped workspace listing | Actual listing and denial for outside scope without exposing private paths | open |
| AND-042 | Verify bounded read | Exact bytes, offsets, truncation and digest on owned fixture | open |
| AND-043 | Verify conditional file write | Create/replace with prior digest and collision handling | open |
| AND-044 | Implement binary file transfer | Bounded chunks, digest, atomic finalize and cancellation | open |
| AND-045 | Implement recursive search | Paged search with exclusion, byte limits and cancel | open |
| AND-046 | Implement metadata batch | Typed size/time/type plus per-file errors | open |
| AND-047 | Implement atomic rename/move | Collision/stale refusal; recovery on owned fixture | open |
| AND-048 | Implement patch/diff | Unique match, preview, CAS and reversible receipt | open |
| AND-049 | Integrate SAF selection | User grants file/tree URI; test revoke and unavailable roots | open |
| AND-050 | Validate document tooling | Owned PDF/DOCX/XLSX round trips; fidelity and resource budget | open |

## 06 — Terminal, process and local development
Windows mapping: Windows 09–20, 26, A+01–06

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-051 | Verify native Rust execution | Exact physical device smoke test with exit and output | verified |
| AND-052 | Verify persistent session readback | Reconnect and resume owned stdout/stderr cursors | open |
| AND-053 | Verify stdin | Bounded interactive child echo and EOF | open |
| AND-054 | Verify timeout and cancellation | Owned process group termination; unrelated fixture unaffected | open |
| AND-055 | Verify idempotent requests | Same ID/same digest returns receipt; changed digest rejected | open |
| AND-056 | Implement PTY fidelity | PTY if supported; resize, Unicode, Ctrl-C and backpressure; otherwise report limitation | open |
| AND-057 | Verify Git synchronization | Exact pin fetch and clean non-destructive checkout; no overwrite | open |
| AND-058 | Verify Product workspace build | Locked native Cargo check/test actual exits and artifacts | open |
| AND-059 | Measure Rust cache | Cold/warm builds, cache hits and disk/RAM ceilings | open |
| AND-060 | Verify bounded agent task | Approved model and task, actual tool use, budget, cancellation and receipt | open |

## 07 — Native device capabilities and data
Windows mapping: Windows 27, 30–31; Android-specific extensions

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-061 | Expose battery and charging | Public BatteryManager state and actual device readback | open |
| AND-062 | Expose network status | ConnectivityManager status; no silent configuration | open |
| AND-063 | Discover sensors | Feature list and bounded public sample where permitted | open |
| AND-064 | Expose haptics | User-requested vibration fixture and visible stop | open |
| AND-065 | Expose camera capture | User-mediated camera intent and selected output | open |
| AND-066 | Expose speech and audio | Explicit recording consent, stop and indicator; no background recording | open |
| AND-067 | Expose media selection | Photo Picker fixture; URI grant and revocation | open |
| AND-068 | Expose media playback controls | Owned or specifically authorized media session; verify result | open |
| AND-069 | Expose selected contact/calendar | User-selected record or explicit scoped grant; no bulk extraction | open |
| AND-070 | Evaluate notification integration | Opt-in test notification and reply affordance; no history reconstruction or unattended monitoring | open |

## 08 — Host authority, security and multi-device routing
Windows mapping: Windows 01, 03–08, 27–28, L+03/07

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-071 | Specify device identities | Stable pairing identity; Android and Windows cannot be confused | open |
| AND-072 | Define capability leases | Device/app/action scoped grant with expiry and revocation | open |
| AND-073 | Bind request and approval | Exact action/target/parameters, user-visible consent when required | open |
| AND-074 | Bind IPC caller identity | Authenticate local companion; no exported generic shell | open |
| AND-075 | Implement replay protection | Nonce/request digest and prior receipt reconciliation | open |
| AND-076 | Protect private storage | Android private storage, encrypted sensitive records where appropriate; no secret logging | open |
| AND-077 | Implement local stop control | User stops session and revokes capture/effect authority immediately | open |
| AND-078 | Test network reconnection | Mobile data disconnect/reconnect; no duplicate effects | open |
| AND-079 | Verify multi-device routing | Route harmless fixture to intended device; deny cross-device reuse | open |
| AND-080 | Expose scoped MCP catalogue | Typed discovery, current grant and actual new client call; no overclaim | open |

## 09 — Product SDK, graph and one surface
Windows mapping: Windows 29–31, A+02/07, L+08

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-081 | Map to canonical SDK tasks | Link research IDs to existing Product catalogue; no competing scheduler | open |
| AND-082 | Define Android capability ABI | Typed Rust IR and versioned adapter descriptors | open |
| AND-083 | Implement native JNI boundary | Minimal Java/Kotlin framework calls, Rust core; owned fixture | open |
| AND-084 | Integrate Host receipts | Correlate device, request, observation and result; unknown never success | open |
| AND-085 | Project into VLANG | Discover and compile typed device/app operations using existing evaluator | open |
| AND-086 | Project into HQKG | Graph app identities, relationships, capability evidence and change revisions | open |
| AND-087 | Project into AutoWiki | Revision-aware readback of verified capability state; no private message content by default | open |
| AND-088 | Integrate Oracle | Use graph context for scoped user requests; no duplicate reasoning engine | open |
| AND-089 | Integrate GPUI surface | Expose same Product controls and status, mobile viewport invariant; native OS consent remains native | open |
| AND-090 | Verify cross-device workflow | Same typed command across Windows and Android with capability-specific effects | open |

## 10 — Lifecycle, release, recovery and parity acceptance
Windows mapping: Windows 18–20, 32, A+04/08, L+01–08

| ID | Atomic task | Acceptance | State at plan creation |
|---|---|---|---|
| AND-091 | Define release seal | Exact source, APK/native artifact digests and test receipts | open |
| AND-092 | Stage update | Build immutable candidate without replacing current bridge | open |
| AND-093 | Drain owned work | Reject new effects during handover; preserve settled output | open |
| AND-094 | Promote authorized candidate | Owner-approved install/update and live readiness; no privilege expansion | open |
| AND-095 | Test rollback | Restore previous known-good candidate through supported mechanism | open |
| AND-096 | Recover interrupted sessions | Checkpoint and reconcile after Android process kill/restart | open |
| AND-097 | Test resource policy | RAM, battery, thermal, storage and one-job constraints on actual phone | open |
| AND-098 | Complete two development cycles | Cloud source -> Android build/test -> evidence twice, one adapter update | open |
| AND-099 | Compare Windows parity | Map each reference deliverable to verified/equivalent/partial/unavailable; explicit OS limits | open |
| AND-100 | Accept integrated Mobile Studio | Physical-device end-to-end app launch, observation, WhatsApp bounded read and confirmed send, SDK receipt, stop/revoke and rollback; owner acceptance | open |

## Dependencies and acceptance
AND-001–010 foreground companion first; 011–020 capture and lifecycle next; 021–028 bounded WhatsApp read before 029–030 send. 031–050 extend app and file surface; 051–060 compute; 061–070 OS capabilities; 071–090 cross-cutting authority and SDK integration; 091–100 release and acceptance. Security and permission checks apply from the first experiment, not deferred to their implementation group. Each task closes only on source-linked actual evidence, explicit scope, negative test where applicable, and outcome. A blocked task remains open; no synthetic acceptance. Use existing Product ownership and tracker when integrating production.
