# VEC — Natural-language programming engine for VIVIM
**Program state:** VISION_AND_ARCHITECTURE_PROPOSED; NOT_IMPLEMENTED, NOT_APPROVED_RUNTIME. Date: 2026-11-12.

## Product thesis
VEC is primarily the **user-programming substrate** of VIVIM, not merely source-file Rust comments, an agent refactoring tool, or code-maintenance infrastructure. A user describes a desired change to how VIVIM behaves in everyday language. VIVIM deterministically recognizes a known typed intent, composes existing SDK capabilities into a versioned user program, previews it, obtains any necessary Host approvals and installs/runs it locally. An LLM is a fallback for novel/complex/unsupported intent, not the default runtime processor.

## Canonical relationship
- VLang = umbrella semantic program/authoring framework; SCF = source/capability metadata, compiler + passport; SV = independent evidence checker.
- VEC = proposed executable capsule and program graph engine. Rust code embedded in first-party comments is one **developer packaging/discovery source** for versioned compiled SDK capsules; it is not a general right for comments to execute.
- User phrases -> canonical typed intents -> compiled declarative **VEC Program IR** -> existing Host/SDK program executor with precompiled Rust capabilities. Standard requests should not require generating, compiling or executing novel Rust source per utterance.
- Source mutation is a separate advanced permission class; never inferred from a request to customize personal UI or automation.
- Use canonical Host journal/grant/recovery, HQ semantic ID registry for *development source* concepts, HQKG algorithms, Studio signed-in app control and TierFlow publisher. No duplicate source-of-truth, program action executor, personal-data store, scheduler or UI publisher.

## Read in order
1. [Product vision](00-NL-PROGRAMMING-VISION.md)
2. [Deterministic interpretation and execution](01-INTENT-COMPILER.md)
3. [SDK and IR contracts](02-SDK-AND-PROGRAM-CONTRACTS.md)
4. [LLM escalation and amortization](03-LLM-ESCALATION.md)
5. [Permissions, lifecycle and source-editing distinction](04-PROGRAM-LIFECYCLE-AND-AUTHORITY.md)
6. [HQ × Rust × TierFlow convergence research](system-convergence/README.md): inspected sources, architecture, opportunities, test and security gates.
7. [Machine program specification](../machine/vec-product-vision.json)

No implementation, test or beta-device acceptance is claimed. No release workflows or active user code paths are changed by these proposals.
