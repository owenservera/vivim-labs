# vivim-labs

Working record for the VIVIM Labs initiative. **This repository is intended for the local ZCode team to design and operate.** The initial documentation captures our discussion; it does **not** establish a finalized architecture, source layout, orchestration workflow, operating policy or implementation.

## Intent discussed

Labs is a high-volume engineering, testing, evaluation, tooling and **process-oriented** work area supporting VIVIM. Its role is to help expand both **breadth** (what we have explored, specified or can attempt) and **depth** (what we have actually executed and verified), without multiplying competing production authorities.

The user has already run at least three parallel local ZCode threads with eight workers each (24+ demonstrated concurrent workers). The local PC has access to configured OpenRouter and NVIDIA free-tier models. ZCode is intended to centrally manage fleets/swarms. Actual effective throughput, provider/model limits and hardware constraints are for local measurement.

## What Labs may cover

See [discussion record](DISCUSSION-RECORD.md): natural-language command expansion and tests; a catalogue of catalogues; reusable assets; integration research; security, compatibility, performance and failure testing; production-environment scaffolding; documentation and process work; and evidence of what truly works.

See [agentic_excelence](agentic_excelence/README.md) for discussion topics about agent tooling, skills, agents, evaluation and operational excellence. The folder name intentionally follows the requested spelling.

## Legacy recovery intake

See [Past orphaned work](past-orphaned-work/README.md): a documentation-only source-linked audit split into [Past Implemented](past-orphaned-work/past-implemented/README.md) and [Past Researched](past-orphaned-work/past-researched/README.md), each organized into ten high-level platform domains. Initial scope covers DCB/context, memory, capability intelligence, parsing, automation, self-healing and optimization. This folder neither executes legacy algorithms nor grants them production authority.

## Relationship to existing repos

- [tierflow](https://github.com/owenservera/tierflow): the canonical visual/command runtime work.
- [repos-hq](https://github.com/owenservera/repos-hq): knowledge, semantics and related intelligence work.
- [vivim-web-harness](https://github.com/owenservera/vivim-web-harness): existing VIVIM application, Host and device work.

Labs is **not** being designated another KB, UI publisher, Host effect authority, command registry or product runtime. The exact promotion, integration, evidence and source-ownership workflows remain for the local team to design and validate.

## Status and next work

**Notes only.** Local ZCode workers should review these records, inspect the real existing tools and repositories, propose the lab's structure and workflows, then substantiate proposals through tests and operational evidence. No new workflow or executable system is implied by this seed.
