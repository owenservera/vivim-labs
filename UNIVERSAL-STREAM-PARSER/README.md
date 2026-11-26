# Universal Stream Parser — R001
Isolated native Rust Labs experiment. No ARBOR/Product modifications.

## Objective
Given an authorized stream of bytes or browser events and a description of the desired information, discover the framing, normalize records, locate matching fields and report the source, exact JSON pointer, generalized schema pointer, byte span, match evidence and value. Repeated matches accumulate a bounded per-session source/schema index.

## Run
- cargo test --offline
- cargo run -- stdin "answer text" (pipe input into stdin)
- cargo run -- tcp "answer text" 127.0.0.1:9000 (authorized TCP endpoint)
- cargo run -- cdp "answer text" (pipe newline-delimited CDP events from the existing authorized capture transport)

The CLI emits JSON hits, plus a learned source/path summary on stderr. The library exposes explicit format selection, per-stream incremental parsing, and the browser-event adapter. JSON/concatenated JSON, SSE, text lines and conservative big-endian 32-bit length framing are supported; unknown binary can be retained as base64 at EOF. Format inference is heuristic, not proof. Per-parser input buffer is bounded; the CLI limits concurrent CDP stream identities. TCP is application-payload access, not packet sniffing.

## Architecture
Capture adapters -> source demultiplexing -> framing -> typed record -> goal-driven field scoring -> learned source/schema locations. Keep capture separate from the Rust core. Deterministic extraction can eventually be compiled into a persistent extraction recipe, with schema-drift detection and fallback to discovery. R001 uses lexical/contextual scoring and a small synonym vocabulary, not an LLM or general semantic model. Scores are ranking signals, not calibrated probabilities.

Chrome integration reference: https://chromedevtools.github.io/devtools-protocol/tot/Network/
Existing Host/Studio should supply authorized CDP events. The adapter consumes Network.dataReceived (when data is included), Network.webSocketFrameReceived and Network.eventSourceMessageReceived. For live HTTP streaming, capture must enable Network.streamResourceContent and deliver bufferedData before subsequent dataReceived chunks; choose exactly one SSE representation to avoid duplicates. This CLI does not open a Chrome debugging session. No provider credentials, browser profile data, or private captured streams are stored here.

## Evidence and limits
Tests exercise fragmented JSON and SSE, length-prefixed frames, goal matching, browser event decoding, incomplete/bounded inputs, and a real local TCP socket. Test fixtures are controlled public-safe inputs, not proof of live provider compatibility. Message-boundary offsets are relative to the message, not absolute wire positions. SSE coverage is a subset (LF/CRLF, basic fields; no reconnection/id inheritance). Opaque data is not automatically decrypted or reverse engineered. Unknown compression, HTTP/2/3, protobuf schema discovery, arbitrary framing grammars, multi-layer transform discovery, semantic model inference, persistent recipes and drift recovery remain future research. No universal parser can uniquely infer every encrypted or arbitrary binary protocol from observations alone.

Next: connect the authorized Studio event transport and test actual browser streams; add ranked competing framing hypotheses with evidence, content-type/context metadata, schema inference, drift-aware extraction plans, benchmarks and fuzz tests. Do not promote into Product without verified real-stream evidence.

