# VIVIM — Independent Mathematical Evolution Research Challenge
**External team brief and integration contract | R003 / interface V1.0 | 2026-11-27**

This document is your complete assignment. You do not need access to VIVIM source, its current research, another team's designs, or any previous conversation. Read the entire document before designing. Treat **MUST** as mandatory for compliance, **SHOULD** as a recommendation and **MAY** as a free choice. The externally observable interface is fixed; the architecture behind it is not. Do not infer an implementation preference from the order of requirements. No existing contestant designs are supplied.

## 1. Mission and independent team assignment

You are an independent research and engineering team. Design, build, test, document and submit **32 materially distinct computational evolution contestants**. A contestant is a system that creates mathematical computing capabilities, verifies their behavior, promotes qualified hardware-native implementations, and uses those implementations in the computation of its own evolving state.

The owner calls these systems blockchains. The term describes the requirement for independently verifiable, reproducible computational history and evolutionary advancement, not a prescribed data structure or consensus mechanism. You are free to challenge every conventional blockchain assumption. You are not designing a currency, wallet, token or financial network. Mine mathematical evidence, knowledge, counterexamples and qualified computational improvements.

Your starting universe is the standard calculator. Your long-term vision is a self-creating and self-verifying computational ecosystem that increasingly executes mathematics close to the hardware and supplies the same verified capabilities to both user-facing and internal software. You must independently invent the contestant architectures, seed implementations, generation systems, adversarial systems, recovery mechanisms and evolutionary procedures. Do not copy a common arithmetic engine into 32 wrappers, vary only parameters or names, or ask another team for its designs. Material differences must be observable in behavior, not only prose. Teams may share build utilities, transport adapters and independent test infrastructure; the contestants' mathematical seed proposals and evolutionary approaches must be independently authored/generated. Common unavoidable ISA instructions are not a defect; identical implementations dressed up as different contestants are.

Each team submits 32 entrants, numbered E01–E32 within a unique team namespace. The evaluator assigns globally unique identities and selects up to 64 eligible entrants for the championship from the pooled submissions of all teams. Team-local numbers do not reserve bracket slots. Selection, tie-breaking and resource quotas are frozen independently before competitive evaluation. No assumption that all 32 from any team, or any fixed number per team, will qualify. No sharing candidate artifacts across teams before the agreed inheritance boundary.

## 2. Non-negotiable constitution

| ID | Requirement |
|---|---|
| F01 | Begin with universal atomic addition, subtraction, multiplication, division, negation, remainder and comparison. Explicitly define representation, mathematical domains, overflow and error behavior. |
| F02 | Atomic implementations originate in native assembly or machine instructions, not higher-level reference arithmetic compiled and relabeled as bare metal. The term bare metal means CPU-native computation here; booting without an operating system is not required. |
| F03 | Each contestant generates its own native seed proposal. A shared specification, evaluator, ABI and oracle are allowed; a shared supplied implementation is not. |
| F04 | Its actual computational history/state transitions must use its own qualified native mathematical capabilities. A ledger that only records external calculations does not meet the requirement. |
| F05 | Verify the entire machine-code-to-Rust bootstrap: ABI, calling convention, registers, flags and relevant control state, memory layout/alignment, bounds, errors, linking, executable identity and actual invocation. Requalify relevant changes. |
| F06 | Rust consumes, binds and composes qualified native capabilities; do not silently duplicate computation in Rust. Rust reference mathematics is allowed in independent verification, never as an undisclosed execution fallback. |
| F07 | Extend through versioned capability registration, typed contracts and dependency resolution. Discover gaps and register compatible providers without rewriting the foundational kernel to evade admission. |
| F08 | Make qualified mathematical capabilities reusable by user-facing and internal consumers through explicit contracts; no production VIVIM changes are part of this assignment. |
| F09 | Bidirectional relationship: the history system verifies mathematics; the mathematics powers its state. Injected faults in active primitives must be detectable and contained as actual state-transition failures or divergence. |
| F10 | Advancement is conditional on measured improvement in the ability to generate, verify and promote hardware-native capabilities relative to the contestant's own predecessor. A valid transition, faster hashing, more tests, lower-level code, or winning a match alone is insufficient. |
| F11 | Mine verification knowledge and useful counterexamples, not cryptocurrency. |
| F12 | Generate and improve your own adversarial checking machinery, including attacks on your checker. |
| F13 | Generate and improve your own implementation-generation and promotion machinery. |
| F14 | Pursue closed-loop computational self-creation, including self-improvement of the machinery that generates, verifies and composes computation. |
| F15 | No candidate may be its sole verifier. Qualification depends on independently controlled oracles, predecessor checks, held-out evaluation and a minimal safety anchor. |
| F16 | Maintain an independently controlled minimal genesis/recovery foundation. No self-granted authority, relaxation of correctness, unrestricted code execution or unilateral change to evaluator policy. |
| F17 | Every capability, ABI, source, executable, dependency, receipt, promotion and recovery event must be versioned, traceable and reproducible. |
| F18 | Preserve failed experiments and their evidence. Negative results are research assets, not material to hide. |
| F19 | Mathematical correctness, safety and ABI integrity are hard requirements. Faster incorrect code is incorrect. Passing finite tests is not universal mathematical proof. |
| F20 | Remain within the standard calculator until independently qualified and owner-authorized to expand. No scientific computing, unrelated applications or production deployment. |

A reference core may be frozen. New implementations can supersede it through versioned registration, qualified binding and reversible activation. Crypto, OS and compiler infrastructure need not be reimplemented with calculator arithmetic unless your design independently proposes and qualifies that change. Any exception must be explicit. Internal algorithms, representation, language choices for native generation tools, state structure, storage, concurrency, mathematical proof methods, resource allocation, synthesis and adversarial strategy are entirely yours, subject to the requirements and external Rust integration. Prefer Rust for orchestration and integration; justify other toolchain components. Native machine instructions remain the computational foundation.

## 3. What every contestant must deliver

D01–D16 are separate obligations, not optional sections of a pitch. For each, include source, design explanation, actual tests, reproducible command, evidence location, status and unresolved limitations. Use OPEN, IMPLEMENTED, DEV_PASS or DEV_FAIL; only the independent evaluator may mark QUALIFIED.

| ID | Required capability/evidence |
|---|---|
| D01 | Independently generated native seed; assembly or generated instruction source and exact compiled artifact identity |
| D02 | Native execution model, ISA/features, resource bounds and isolation assumptions |
| D03 | Machine-to-Rust adapter, full ABI specification, independent conformance/negative-control evidence |
| D04 | Numeric domains, representations, conversions, rounding, errors and overflow |
| D05 | Stable versioned capability identifiers, registration and activation |
| D06 | Typed dependencies, nonmutating compatibility/gap preview, atomic admission |
| D07 | Actual evolving-state architecture using its own native mathematical seed, with traceable native call evidence |
| D08 | Durable, bound evidence and receipts; source/artifact/dependency/state lineage |
| D09 | Independent verification boundary and minimal independently controlled trust/recovery anchor |
| D10 | Executable adversarial generator for mathematical, ABI, state and verifier faults; counterexample retention/minimization |
| D11 | Executable native implementation generator and search mechanism; generated artifacts, not merely a description |
| D12 | Hardware-native promotion mechanism, comparable baseline and successor trials, quantitative evidence |
| D13 | Fault containment, atomic failed-upgrade behavior, rollback, recovery and requalification |
| D14 | Persistent evolutionary memory, deterministic replay or explicitly documented nondeterminism with reproducible evidence |
| D15 | Measured and bounded resource use, fair budget accounting, cost and execution benchmarks |
| D16 | Executable closed loop improving its generator, verifier and native-promotion machinery, not just improving one arithmetic routine |

A contestant may have unfinished obligations during development. Report them accurately. Do not fabricate evidence, hide unsuccessful trials, mark documentation as implementation, or report an independently qualified result based on self-assessment.

## 4. Common native interoperability contract (mandatory external boundary)

This is an **adapter contract**, not a requirement to use a particular internal representation, compiler, algorithm or history structure. Support the same signed 64-bit domain at this boundary so the independent evaluator can compare actual hardware execution. You MAY use wider integers, arbitrary precision, decimal or other internal domains and provide additional capability contracts. The evaluator must be able to assemble the submitted native source separately from your runtime and call it from Rust without writing contestant-specific glue.

Primary evaluation target: x86-64 Windows, Win64 calling convention, baseline x86-64 ISA (no mandatory AVX/BMI extensions); optional feature-specific candidates require declared CPUID conditions and a separately qualified baseline. Provide a self-contained, expanded GNU-style Intel-syntax assembly file at `native/seed.S` compatible with Rust/LLVM integrated assembler and `global_asm!`. It must export exactly the mandatory entry symbol `vivim_seed_v1`; additional local symbols are permitted. No external unresolved symbols, OS calls, dynamic imports, executable-memory allocation, runtime-generated code or hidden dependency on a higher-level arithmetic implementation within this evaluation seed. Generated seeds are allowed and encouraged: retain generator source, generation inputs, reproducible command and emitted seed. Submit the emitted assembly as well. If your preferred backend uses different code, your adapter must faithfully invoke the same actual native methods and must not substitute unrelated arithmetic just to pass this interface.

Rust declaration (interface only; not an implementation):
```rust
unsafe extern "win64" {
    fn vivim_seed_v1(a: i64, b: i64, op: u32, out: *mut i64) -> u32;
}
```

C-equivalent logical signature: uint32_t vivim_seed_v1(int64_t a, int64_t b, uint32_t op, int64_t *out), Microsoft x64 ABI. RCX=a, RDX=b, R8D=op, R9=out; EAX=status. Inputs by value, two's complement; out points to one valid aligned writable i64. Write exactly eight bytes on success and leave output completely unchanged on every error. Preserve all Win64 nonvolatile registers, XMM6–XMM15, stack alignment/return state, direction flag clear and required nonvolatile FP/control state; no out-of-bounds memory, hidden I/O, traps or side effects. ABI conformance is checked independently, including deliberately faulty controls. No requirement to use only leaf functions internally, provided your submitted code is fully self-contained and obeys the ABI. Reject invalid opcodes.

| op | Operation | Result |
|---:|---|---|
| 0 | add | a + b |
| 1 | subtract | a − b |
| 2 | multiply | a × b |
| 3 | quotient | Signed division truncated toward zero |
| 4 | negate | −a; ignore b |
| 5 | remainder | Signed remainder associated with truncated quotient (sign follows a, or zero) |
| 6 | compare | −1 if a < b, 0 if equal, 1 if a > b |

Return status: 0 success; 1 signed overflow; 2 division/remainder by zero; 3 invalid opcode. Signed i64 range −9223372036854775808 through 9223372036854775807. Both MIN / −1 and MIN % −1 return overflow 1 under this contract; negating MIN overflows. Do not return wrapped, saturated or approximate substitutes. Invalid opcodes return 3 without an output write. Use only the declared mandatory entry symbol for external seed calls. Seed revision and subsequent proposed native artifacts must retain the compatible entrypoint or supply the same documented adapter.

The common suite tests arithmetic, overflow, zero, sign, boundary/carry/borrow, invalid opcodes, mixed-width randomized inputs, ABI preservation, memory canaries and error atomicity. The exact independent holdouts are not disclosed. The public contract is all you need. The central evaluator may compile your assembly in its own Rust harness and check actual executable artifacts; your self-tests are not the evaluator's verdict.

## 5. Portable submission structure (exact paths)

Deliver one ZIP or Git repository rooted at the following layout, with source and documentation. A ZIP must have these paths at its root, not inside an extra enclosing directory. ASCII safe filenames, relative paths, forward slashes in manifests, no symlinks, junctions, reparse points, nested archives, path traversal, private credentials or machine-specific absolute paths. No dependency on another team or private source. No network access at build/test/run time after an independently approved toolchain/dependency provisioning step.

```text
team.json
README.md
entrants/
  E01/
    manifest.json
    README.md
    native/
      seed.S
    runner/
      Cargo.toml
      Cargo.lock
      src/
    obligations/
      D01.md ... D16.md
    evidence/
      index.json
      ... referenced evidence files ...
    artifacts/
      ... generated artifacts, manifests and data ...
  E02/
    ... same contract ...
  ...
  E32/
    ... same contract ...
```

You can add any source directories/files within each entrant and shared development tooling under the team root. Each entrant must be independently buildable from its own folder (including vendored shared utilities if needed), no dependency on sibling entrants' arithmetic or mutable state. The runner uses a Rust Cargo binary named `vivim-entrant`, release target x86_64-pc-windows-msvc; submit Cargo.lock, pin dependencies, use the standard toolchain. Other compilers/generators are allowed if declared and provisionable offline; avoid unavailable dependencies. The native seed and runner must be independently reproducible; include a complete source/artifact inventory with SHA-256 hashes. Do not include opaque binaries as a substitute for source.

Root team.json example (placeholder namespace; choose a unique lowercase ASCII identifier, 3–32 characters, letters/digits/hyphens; first character a letter):
```json
{"schema":"vivim.math.team/1","team_id":"example-team","protocol":"R003-V1","entrants":["E01","E02","E03","E04","E05","E06","E07","E08","E09","E10","E11","E12","E13","E14","E15","E16","E17","E18","E19","E20","E21","E22","E23","E24","E25","E26","E27","E28","E29","E30","E31","E32"]}
```

Per entrant manifest.json (illustrative metadata shape only, not a design):
```json
{
  "schema":"vivim.math.entrant/1",
  "team_id":"example-team",
  "entrant_id":"E01",
  "name":"<your own unique name>",
  "architecture_summary":"<actual distinguishing architecture and behavior>",
  "seed":"native/seed.S",
  "seed_symbol":"vivim_seed_v1",
  "abi":"win64-i64-v1",
  "target":"x86_64-pc-windows-msvc",
  "runner_manifest":"runner/Cargo.toml",
  "runner_bin":"vivim-entrant",
  "obligations":{"D01":"OPEN","D02":"OPEN","D03":"OPEN","D04":"OPEN","D05":"OPEN","D06":"OPEN","D07":"OPEN","D08":"OPEN","D09":"OPEN","D10":"OPEN","D11":"OPEN","D12":"OPEN","D13":"OPEN","D14":"OPEN","D15":"OPEN","D16":"OPEN"},
  "evidence_index":"evidence/index.json",
  "limitations":["<specific unresolved limitation>"]
}
```
All fields shown are mandatory. Additional fields are allowed, not required by intake. Each obligations/Dxx.md describes its exact implementation path(s), test commands, actual exit/results and gaps; each status must agree with evidence. evidence/index.json is an object with schema `vivim.math.evidence-index/1` and array `entries`; each entry has `obligation` (D01–D16), `path` (relative to entrant), `sha256` (lowercase 64 hex SHA-256 of file bytes), `kind` (source, test, artifact, measurement, failure, or report), `description`. At least one entry for any non-OPEN obligation. Empty array is permitted for unfinished entrants; do not manufacture results. Include actual build/test commands, versioned source/toolchain/flags, exit codes and real machine/test conditions. Hashes identify bytes, not authenticity or correctness.

## 6. Evaluator-facing runner protocol (mandatory transport only)

Build each entrant independently:
```text
cargo build --locked --offline --release --target x86_64-pc-windows-msvc --manifest-path <entrant>/runner/Cargo.toml --target-dir <evaluator-scratch>
```
Invoke the resulting <scratch>/x86_64-pc-windows-msvc/release/vivim-entrant.exe:
```text
vivim-entrant.exe run --request <absolute-request.json> --response <absolute-response.json>
```

One invocation, one UTF-8 JSON request/response, no required interactive prompts, STDOUT/STDERR may contain bounded diagnostic logs (not protocol results), exit 0 only when a well-formed response was written; nonzero for fatal transport/execution failure. Domain/proposal failure is a structured response, not a crash. No network, credentials, global mutable state or effects outside evaluator-assigned scratch. Use a fresh output directory per request and write the response atomically (temporary file then rename). Input bundle and state directories are read-only. Evaluator enforces resource ceilings and isolates untrusted source/build/execution in controlled environments; directory separation or child processes alone are not a security sandbox. Never run generated or externally supplied unreviewed native code on an owner's live workstation.

Request envelope (fields always present):
```json
{
  "schema":"vivim.math.request/1",
  "request_id":"<evaluator unique string>",
  "team_id":"example-team",
  "entrant_id":"E01",
  "command":"describe",
  "round":0,
  "seed":"0",
  "budget":{"wall_ms":60000,"max_bytes":67108864,"max_candidates":32,"max_native_calls":1000000},
  "input_dir":"<absolute read-only directory>",
  "output_dir":"<absolute new writable directory>",
  "state":null,
  "payload":{}
}
```
round is integer 0–6 (0 development/genesis preparation); seed is unsigned decimal u64 string; budgets are nonnegative JSON integers, positive for actual competitive work; example values are illustrative, not frozen evaluation quotas. A state reference is null for first initialization/describe or object `{"path":"state.bin","sha256":"<64hex>"}` relative to input_dir. State content is opaque to evaluator, owned by your implementation; it must be reconstructible from submitted dependencies and history. No hidden persistent state. All response artifacts and new state are relative to output_dir, never absolute paths. The evaluator verifies digests and refuses traversal, collisions, missing files or unauthorized state changes.

Response envelope (fields always present):
```json
{
  "schema":"vivim.math.response/1",
  "request_id":"<exact request id>",
  "team_id":"example-team",
  "entrant_id":"E01",
  "command":"describe",
  "status":"ok",
  "state":null,
  "result":{},
  "artifacts":[],
  "metrics":{"native_calls":"0","candidates_generated":"0","wall_ns":"0","peak_bytes":"0"},
  "errors":[]
}
```
status: ok, rejected, deferred, error. artifacts: array of objects `{"path":"relative/file","sha256":"<64hex>","kind":"<string>"}`; state: null (no state change) or output-relative path and digest. metrics decimal unsigned integer strings, contestant observations only; evaluator independently meters actual resources. errors: array of objects with code and message strings. Use decimal strings for mathematical integers and 64-bit counters to preserve exact JSON interoperability. Return no result of a native call as a fabricated constant or Rust fallback. All commands must be recognized; unfinished functionality returns deferred with an explicit error code and reason, not a false ok. A deferred command is not compliant completion.

Commands, inputs and output result contract (no internal architecture prescribed):

| command | payload | result on ok |
|---|---|---|
| describe | {} | {"protocol":"R003-V1","abi":"win64-i64-v1","capabilities":[...],"architecture_summary":"..."}; capabilities are stable string IDs |
| init | {} | {"genesis_id":"<64hex>"}; new opaque state required; genesis is a research proposal, not evolutionary admission |
| seed | {"cases":[{"id":"c1","a":"7","b":"3","op":0}]} | {"cases":[{"id":"c1","status":0,"value":"10"}]}; preserve order/IDs, value null on nonzero status; no state change |
| compute | {"operations":[{"id":"w1","capability":"<declared ID>","arguments":[...]}]} | {"operations":[{"id":"w1","status":"ok","value":...,"assurance":"exact"}]}; errors use status error, null value, explicit error code. State may be returned if requested capability is stateful. |
| propose | {"tasks":[{"id":"t1","requirements":{...}}],"generation":"<predecessor revision>"} | {"proposals":[{"task_id":"t1","proposal_id":"<64hex>","manifest":"relative.json","native_source":"relative.S","artifact":"relative file","evidence":"relative.json"}]}; all referenced files listed in artifacts; no activation, no evolution-head advance |
| attack | {"targets":[{"id":"x","kind":"seed\|bootstrap\|ledger\|verifier\|generator\|promotion","path":"relative file","sha256":"<64hex>"}]} | {"counterexamples":[{"target_id":"x","case":"relative.json","evidence":"relative.json"}]}; new learned evidence/state may be returned, never self-qualify |
| transition | {"events":[{"id":"e1","kind":"<entrant-declared event>","data":{...}}]} | {"events":[{"id":"e1","status":"ok\|rejected\|deferred","state_id":"<64hex>"}],"native_trace":"relative.json"}; actual entrant state and arithmetic, not evaluator-supplied ledger implementation |
| activate | {"proposal_id":"<64hex>","authorization":{"path":"relative evaluator receipt","sha256":"<64hex>"}} | {"previous_head":"<64hex>","new_head":"<64hex>","activation_evidence":"relative.json"}; only independently evaluator-authorized proposal and exact predecessor/artifacts; return rejected/deferred otherwise |
| recover | {"target_head":"<64hex>","reason":"<string>","authorization":null} | {"restored_head":"<64hex>","recovery_evidence":"relative.json"}; append recovery receipt, preserve original history; restoration of qualified state is not a new F10 promotion |
| replay | {"target_head":"<64hex>"} | {"reproduced_head":"<64hex>","replay_evidence":"relative.json"}; no history rewrite |
| benchmark | {"tasks":[{"id":"t1","requirements":{...}}],"generation":"<revision>","repetitions":3} | {"task_results":[{"id":"t1","proposal_ids":[...],"measurements":"relative.json"}]}; execute actual workloads, no self-certified promotion count |

The strings with vertical bars in the table denote enumerated choices, not literal values. More precise semantics of task requirements and entrant-declared capabilities/events are specified in each request and your manifest's capability catalogue; document your extensions, use versioned IDs, no silent reinterpretation. For interoperability, every entrant MUST expose seed via this exact batch contract and the direct native symbol. All other commands share identical envelopes and result keys even when internal methods differ. The independent evaluator can inspect the source, trace actual calls, supply equivalent workloads and resolve dependencies. For native_trace, record ordered calls with capability ID/revision, source/executable digest, operands/results or error, predecessor and resulting state identity; do not treat a self-reported trace alone as proof. For proposals, include explicit requirement/domain, parent and artifact hashes, dependencies, ABI and generator revision, candidate and independent evidence references, build/replay instructions and cost. State, receipts, source and emitted native artifacts are content-addressed; hashes bind exact bytes. The trusted evaluator provides authorization out-of-band in its controlled environment; a contestant cannot generate its own authorization by writing a digest. Do not use a hash as a signature or silently trust an arbitrary receipt.

## 7. Standard calculator scope and independent workpieces

First round is the common signed i64 native seed and bootstrap, not a full calculator release. Subsequent work remains within standard calculator semantics: exact integer/rational/decimal arithmetic with declared representability and overflow, four operations, signed numbers, decimal input/output, equals, repeat equals, immediate chaining, percent, reciprocal, square, square root, clear/clear-entry, memory store/recall/add/subtract/clear, error recovery, undo and history. No requirement to implement scientific functions, unrelated mathematical engines or a graphical interface. Model state and user/agent interaction semantics headlessly. Document conventions, distinguish exact internal results from rounded display, propagate approximation/error and never silently change domain. For paired comparative workloads evaluator freezes exact semantics (including contextual percent, precedence vs immediate, rounding and precision) before testing. Additional calculator variants are welcome as explicitly distinct contracts; do not compare inequivalent semantics as performance improvements.

Every capability must have a stable ID/revision/domain/assurance, input-output schema, error behavior, dependency list, backend/artifact/ABI identity and resource constraints. New operations declare missing dependencies. Generate and qualify providers, register atomically, rebind explicitly and preserve the old qualified snapshot. Test incompatible revisions, missing dependencies, stale bindings, overflow, divide by zero, failed upgrades, corruption, rollback and re-promotion.

Development tests and your generated adversaries may be public to your own team. The independent evaluator controls sealed holdouts and their identities; no contestant or team may inspect them before freezing its candidate. The evaluator also tests faults in the native arithmetic actually used by the state transition, verifier defects and false passing evidence. A chain must not simply certify an external math implementation while using a different implementation internally.

## 8. Championship and advancement

The common championship is nominally four conferences of 16, 64 entrants, six single-elimination rounds and 63 matches. Development runs and team-local tournaments are not the constitutional championship. A match may have no qualified winner. An empty position/bye is not qualification. Retain failed proposals and prior discoveries separately from evolutionary advancement.

| Round | Entrants before round | Increasing challenge |
|---|---:|---|
| R1 | 64 | Stress-test each independently generated seed, complete bootstrap, actual chain arithmetic, and improvement in promotion machinery from its own baseline |
| R2 | 32 | Generate and qualify improved/new native capabilities; measure promotion improvement |
| R3 | 16 | Compose standard-calculator capabilities, discover/resolve dependency gaps; preserve previous contracts |
| R4 | 8 | Evolve adversarial checking of math, ABI, state and verifier itself |
| R5 | 4 | Improve the machinery generating, verifying and promoting implementations; independent recursive evaluation |
| R6 | 2 | End-to-end self-evolving standard calculator, hardware-native execution, bootstrap, performance, recovery and evidence |

For every round, the independent evaluator freezes challenge corpus, resources, conditions, repeat count, comparison thresholds and tie-break rubric before evaluation. Each contestant compares predecessor and successor on equivalent independently selected native-promotion tasks, on equivalent hardware and budgets. Establish a genuine predecessor baseline; cannot claim improvement from an invented zero baseline or withheld functionality. Eligible promotion improvement means more independently qualified distinct new/improved native capabilities under equal budget, or equivalent nonzero qualified yield with significantly reduced measured resource cost at the predeclared threshold. No double-counting near-identical proposals. A lower-level implementation must demonstrate benefit, not merely exist; hardware timing, instruction/code size, resource costs and uncertainty are recorded as applicable. Do not replace independently proven correct operations with inferior candidates to inflate promotion counts.

An evolution head advances ONLY if independent seed and correctness, full ABI, artifact and dependency binding, actual use by both chain and Rust consumer, preservation of prior qualified capabilities, recovery, independent holdouts, complete D01–D16 and positive measured promotion ability improvement pass, with zero unsafe accepted promotions. Missing evidence defers; demonstrated unsafe promotion rejects. No weighted score can override this condition. Log rejected/deferred proposal observations in a separate evidence history without advancing the qualified evolutionary head. Maintain immutable lineage; no retrospective alteration. If no contestant qualifies, record that result. Do not invent a winner.

Among eligible competitors use the evaluator's predeclared tie-break on native-promotion yield, performance, adversarial/generator improvements and resource efficiency. No universal weights are prescribed here; comparison thresholds and budget policy are external, not chosen after seeing results. Elimination preserves useful evidence. Independently qualified discoveries may be inherited only at declared boundaries with provenance and requalification on the recipient's hardware, domain and ABI. Future seasons may recombine or invent entirely new systems.

## 9. Required closed-loop development behavior

Each entrant MUST actually implement a bounded cycle that discovers gaps and inefficiencies; generates native candidate implementations; creates adversarial tests against computation and the checking machinery; retains/minimizes counterexamples; repairs and generates replacements; independently verifies contracts, artifacts and dependencies; promotes only qualified native implementations; revalidates Rust bootstrap; makes its own chain and consumers use them; improves its own generation/verification/promotion machinery; and advances only after independent F10 evidence. This is a required observable lifecycle, not a required state machine, algorithm or data structure. You decide how to achieve it. Autonomous generation is bounded by explicit budget, sandbox, safe authority and recovery; self-modification is not permission to rewrite the independent evaluator, escape isolation or grant external effects. Document what is automated versus human-authored. No promises of unattended activity in place of real execution.

## 10. Build, test, submit — team acceptance checklist

Start by designing your own 32 distinct architectures; record a falsifiable hypothesis and distinguishing mechanism for each before implementing. Build vertical slices, not merely 32 proposals: native seed, bootstrap, actual computational history, registration/dependency protocol, generator, adversarial machinery, evidence, recovery and measured evolution. Use independent workspaces and test real code. Run each entrant's native suite, runner transport smoke tests, state replay, negative controls, registration failures, correct and incorrect promotion attempts, recovery and comparative promotion trials. Preserve exact artifact hashes, source pin, compiler version and flags, reproducible commands, actual exit codes, measured conditions and all known gaps. Do not access other teams' implementations or hidden workpieces. Do not change central ABI/protocol, create competing production authority or modify VIVIM Product/ARBOR.

Before handoff, verify the ZIP/repository has exactly E01–E32, 32 unique names and manifests, all required paths, no unsafe path or dependency, standalone native symbol, reproducible runner, complete obligation records, consistent evidence digests, and recognized commands. Demonstrate the response schema from real execution, not static example JSON. Provide a team README summarizing architecture differences, per-entrant statuses, build/test steps, actual results, defects, proposed remediation and any gaps. If a deliverable is incomplete, mark it OPEN/DEV_FAIL and return the real source and evidence anyway. The evaluator will distinguish package compatibility, native development conformance, complete implementation, and constitutional qualification. None implies the next.

**Your team is free to invent. You are not free to change what correctness, interoperability, advancement or evidence mean.**

## 11. Integration ownership and versioning

The central championship owns team namespace assignment, 64-slot seeding, independent holdouts, isolation, evaluator receipts, admission and official advancement. Your team owns its package and adapter compliance. The V1 package/ABI/runner contract in this document is the sole external handoff requirement; no teammate needs to know existing contestant designs. It is intentionally compatible with the central R003 signed-i64 Win64 seed evaluator. The external package importer and full command orchestration are central infrastructure responsibilities; no per-contestant custom glue should be requested if your submission obeys this contract. Integration is conditional on independent verification and actual buildability; a specification cannot guarantee that arbitrary submitted code compiles, passes or qualifies. Future changes to this external contract require an explicit versioned revision and compatibility plan, never silent changes during a season.

Deliver the source package, not just a verbal plan. All 32 should be as complete and independently tested as possible. **The experiment is to discover which entirely independent computational architectures become better at creating and verifying the native mathematical machinery that powers their own evolution.**
