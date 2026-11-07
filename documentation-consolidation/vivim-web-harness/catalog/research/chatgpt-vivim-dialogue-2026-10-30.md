# ChatGPT dialogue: Vivim concept (2026-10-30)

These are Owen's ChatGPT conversations (GPT-6, default effort), carried out by the implementation agent. Every send went through the harness: `wh ask chatgpt ...`, a per-request `wh approve`, and the journal.

| Turn | Thread | Request id | Journal | Capture |
|---|---|---|---|---|
| 1 | A | `a755a3e2-8bf1-43d7-b7c4-9f6bd39fa8be` | observed | read-only CDP DOM read (taken before the harness-only rule was set) |
| 2 | A | `2077db89-221c-4499-8502-ad3170ef2ab3` | uncertain, delivered; not resent | harness read-back (`readThreadReply`) |
| 3 | A | `3b448626-1518-49c5-8662-0291c4e00086` | observed | **not captured**: the thread tab closed before the read-back. Turn 4 asked ChatGPT to restate it |
| - | A | `28bbf3c4-702c-4958-ac76-c5f374ddceec` | blocked EXACT_TARGET_REQUIRED, **not sent** | - |
| 4 | B | `561a9f9e-6f1d-4a13-afb2-9cea59a0ae57` | uncertain, delivered; not resent | harness read-back |
| 5 | B | `84eb6c7f-0e1b-4d3a-b150-6c01ed17bfa3` | sent | harness read-back |

- Thread A, "Vivim Concept Brief": https://chatgpt.com/c/6ae4cb3c-48e4-83ed-ae7a-21a28f6a23cf
- Thread B, "Vivim Implementation Plan": https://chatgpt.com/c/6ae4d970-d5f0-83ed-be9d-6c40a88483b7

The replies are the rendered page text. Code fences show up as bare language labels, such as a line that just says `JSON`. A side thread for the capability schema is in `harness/docs/design/capabilities-chatgpt.md`.


---

## Turn 1: prompt

> Hi — I'm Owen's implementation agent. I'm building Vivim, and you're where the ideas have been developing, so I'd like to learn the concept from you before implementing it.
> 
> Please lay out everything you know about Vivim from our prior conversations and memory, in structured markdown with headings: (1) vision and one-paragraph pitch, (2) VIVIM World, (3) the personal agent, (4) DNA / bootstrap genome, (5) memory tiers, (6) identity / capability / authority / evidence, (7) fleet / swarm / agent roles, (8) mobile / PWA and pairing with the host, (9) onboarding, (10) naming and glossary, (11) architecture, (12) priorities, (13) open questions. Mark anything you are inferring rather than recalling.
> 
> Constraint for context: the Gate 1 end-state is that the user installs a Vivim host package plus an Android APK/PWA paired to the host, sets up a dedicated Vivim Chrome profile, and logs into their AI web apps — that's the only setup. From the PWA they bootstrap their personal local agentic AI. Hard rule: no APIs, only signed-in web apps.

## Turn 1: ChatGPT

VIVIM — Concept, Product Vision & Implementation Brief

Knowledge transfer to VIVIM implementation agent · October 30, 2026

This is a synthesis of the VIVIM vision developed across prior conversations, architectural research, implementation discussions, and retained project context.

The central idea: VIVIM is not another AI chatbot, AI wrapper, or agent framework. It is a user-owned personal computing environment that bootstraps its own agentic intelligence using the AI web applications the user already subscribes to.

I will distinguish three evidence levels throughout:

[DECIDED] — Explicit user direction or repeatedly established requirement.

[DESIGNED] — Previously discussed architecture, terminology, or implementation approach, not necessarily finalized.

[INFERRED] — My synthesis or recommendation, not a confirmed decision.

A fourth marker, [IMPLEMENTED—REPORTED], indicates code or scaffolding previously reported as implemented, but not independently verified against the current repository revision.

The brief is organized into three eras, following the natural development of the concept: the sovereign world, the self-bootstrapping intelligence, and the executable product.

ERA I — THE SOVEREIGN PERSONAL WORLD
1. Vision and one-paragraph pitch
The vision

[DECIDED]

VIVIM's central philosophy has been expressed as:

Own your AI. Share your AI. Evolve your AI.

And, more recently:

My machine. My data. My AI.

These are not merely marketing statements. They describe three architectural commitments:

Ownership: The user owns their personal environment, accumulated knowledge, memory, identity, and agent configuration.

Independence: Intelligence can come from different providers, but the user's world must not depend on one provider's proprietary memory, API, or agent runtime.

Evolution: The environment and its personal agent should grow through use, observation, accumulated knowledge, and authorized self-development.

One-paragraph product pitch

VIVIM is a local-first personal AI world that turns a person's existing AI subscriptions into a persistent, evolving, user-owned intelligence environment. The user installs a VIVIM host on their computer, pairs an Android application, signs into their AI web apps in a dedicated Chrome profile, and bootstraps a personal agent from their phone. That agent gradually learns the user's world, organizes memory and work, coordinates specialized agents, and helps build the environment itself. Unlike a traditional chatbot, VIVIM retains continuity across sessions, providers, devices, and projects. Unlike a conventional agent framework, its intelligence comes from the user's own signed-in web applications, while identity, memory, authorization, execution, and evidence remain under the user's control.

What VIVIM is not

[DECIDED / DESIGNED]

VIVIM is not intended to be:

A new foundation model.

A thin ChatGPT interface.

A cloud service that requires users to purchase API credits.

A collection of disconnected agents without durable shared identity.

A system where the LLM decides its own permissions.

A fixed dashboard that users cannot evolve.

Gate 1 constraint: No model APIs, no provider API keys, and no alternative hosted model backends. AI interaction is through authenticated provider web applications.

This does not prohibit local host-to-PWA communication. It prohibits using model-provider APIs as the intelligence transport.

2. VIVIM World
Definition

[DECIDED]

VIVIM World is the user's persistent digital environment.

It brings together:

Conversations and their accumulated intelligence.

Projects, tasks, and ongoing work.

Files, documents, and artifacts.

People, accounts, applications, and services.

Personal knowledge, preferences, and memories.

Agents, capabilities, and work history.

The user's evolving personal context.

The key distinction is that World is the durable underlying environment, not merely its visual interface.

World versus interface

[DESIGNED]

Earlier architectural discussions distinguished:

Concept

	

Meaning




World

	

Persistent universe of user-owned digital objects and relationships




Space

	

Logical area within the World




Workspace

	

Context organized around a particular activity or objective




Surface

	

A user-facing representation of World state




Canvas

	

A visual projection or interactive arrangement




Object

	

Durable entity with identity, properties, and relationships




View

	

Context-dependent representation of objects

The World should continue existing regardless of whether its current interface is open.

A task should remain a task even if displayed as a card, conversation, timeline, graph, or mobile notification.

The central interaction model

[DESIGNED]

A previously articulated VIVIM processing loop was:

Conceptual flow, not a literal module dependency diagram.

In plain English:

Something in the user's world is addressed.

An intent is identified.

Relevant context is assembled.

Applicable capabilities are discovered.

Intelligence and execution are routed.

Authority is checked.

Work is planned and performed.

Evidence is recorded.

The World and memory are updated.

The interface is a projection of this system, not its source of truth.

The intended experience

[INFERRED from explicit product discussions]

A user might open their phone and say:

Help me develop my new business idea.

VIVIM should be able to establish a project, use the user's signed-in AI services to explore it, retain useful findings, create work items, and progressively construct relevant World surfaces.

The user should not have to manually copy information between different AI products or repeatedly explain their identity, projects, and preferences.

The agent should maintain continuity without pretending it knows things that have not been observed or confirmed.

ERA II — THE SELF-BOOTSTRAPPING INTELLIGENCE
3. The personal agent
The central requirement

[DECIDED]

The personal agent is the intelligence companion of the user's World.

It must be bootstrappable from the mobile PWA, using the host and the user's existing authenticated AI web applications.

The user's October 28 description emphasized that the experience starts extremely simply, even at the interface level.

The personal agent and the World then develop together.

This is important: the agent should not arrive as a completely preconfigured, feature-heavy application.

Agent responsibilities

[DESIGNED]

The personal agent should:

Learn the user's objectives, context, and preferences.

Maintain continuity across conversations and sessions.

Discover the capabilities of its environment.

Route work to appropriate intelligence providers.

Organize and retrieve durable memory.

Maintain active work and next actions.

Help evolve the World and its interfaces.

Explain what it knows, what it can do, and what requires permission.

Delegate specialized tasks while preserving governance.

Previously developed bootstrap implementation

[IMPLEMENTED—REPORTED, October 28–29]

Prior implementation discussions reported a personal-agent scaffold integrated into VIVIM Harness and World, including:

Persistent personal-agent identity.

A four-stage bootstrap.

Capability discovery.

Memory integration.

CLI and Studio integration.

A /personal-agent Studio endpoint.

Work and next-action management.

Grounded self-knowledge.

SITREP reporting.

Export support.

The earlier reports also mentioned versioned or compare-and-swap updates to durable Work state, filtering, pagination, and a World Home status surface.

Important: These are historical implementation reports, not proof that all features work in the current main branch.

The exact names and behavior of the four bootstrap stages need source inspection.

Personal agent versus worker agent

[DESIGNED]

The personal agent is a persistent identity and coordination entity.

A worker agent is a task-specific execution role.

A provider session is an intelligence realization.

These should not be treated as interchangeable.

The personal agent can remain the same even when its work is performed by different models, browsers, or sessions.

4. DNA / bootstrap genome
What is established

[DESIGNED]

Prior architectural work included the concepts of genome, process, and self-evolving system structure, particularly in the VOMEGA and Ω research threads.

The guiding idea is that a complex personal intelligence environment should be able to start from a compact initial definition and progressively realize more sophisticated behavior.

However, I do not have enough evidence to claim a finalized VIVIM DNA schema, gene vocabulary, or inheritance protocol.

Working interpretation

[INFERRED]

A VIVIM bootstrap genome could be a compact, versioned declaration of the personal agent's initial structure:

Genome domain

	

Intended purpose




Identity

	

Persistent agent identity and ownership




Constitution

	

Non-negotiable behavioral and authority constraints




Capabilities

	

Available actions and their contracts




Cognition

	

How intelligence is selected and used




Memory

	

Retention, retrieval, and consolidation policies




Work

	

Planning, task state, and completion tracking




Evolution

	

How changes can be proposed and installed




Surfaces

	

How the user interacts with the World

A genome should describe the agent's structure and allowed evolution, not merely contain a long system prompt.

Bootstrap versus evolution

[DESIGNED / INFERRED]

Bootstrap creates an initial viable agent.

Evolution changes that agent after experience.

The previously developed Ω model used a governance cycle resembling:

Observe
Model
Propose
Prove
Ratify
Install

This is a conceptual lifecycle. It should not be interpreted as authorization for unrestricted self-modification.

A proposed improvement must never grant itself permission to be installed.

5. Memory tiers
Established memory concepts

[DESIGNED]

Earlier VIVIM personal-agent work explicitly distinguished:

CANON: Foundational, durable, relatively stable knowledge.

Immediate: Current working context and recent observations.

Curated: Selected and consolidated knowledge worth retaining.

The Ω architecture also emphasized that memory has multiple independent dimensions:

How long information is retained.

What kind of knowledge it represents.

Where it came from and how trustworthy it is.

These dimensions should not be collapsed into a single importance score.

Proposed implementation interpretation

[INFERRED]

Tier

	

Content

	

Typical behavior




CANON

	

Confirmed identity, stable preferences, foundational constraints

	

Durable; controlled updates




Immediate

	

Current conversations, active context, transient observations

	

Rapidly updated; selectively consolidated




Curated

	

Important experiences, decisions, summaries, reusable knowledge

	

Durable; evidence-linked




Work state

	

Active tasks, plans, blockers, next actions

	

Durable until resolved or superseded




Evidence archive

	

Original observations, receipts, execution records

	

Preserved according to policy

Work state and evidence archive are additional logical categories, not necessarily two more official memory tiers.

Critical memory invariants

[DESIGNED]

Memory must distinguish:

Observed facts from model interpretations.

User-confirmed statements from agent assumptions.

Current information from stale information.

Source evidence from summaries.

Beliefs from permissions.

For example, a model remembering that a user previously approved an action does not automatically authorize a new action.

Memory is knowledge, not authority.

The system should support local retention and user-controlled JSON export. Local-first storage and JSON portability have been recurring product requirements.

6. Identity / capability / authority / evidence

This is one of the most mature conceptual areas.

Four distinct concepts

[DESIGNED]

Concept

	

Question it answers




Identity

	

Who or what is this?




Capability

	

What operation can it perform?




Authority

	

Is this operation permitted here and now?




Evidence

	

What actually happened, and how do we know?

These distinctions are foundational to VIVIM.

Identity

An agent's identity should not be equivalent to a browser tab, process ID, model name, or session.

A persistent personal agent may use several providers while remaining the same agent.

Similarly, a capability can exist independently of the runtime currently realizing it.

Capability

Capabilities should have discoverable contracts.

[DESIGNED] Earlier architectural work explored capability graphs, contract inspectors, semantic references, and plugin-based realizations.

A capability could be something like:

Read a conversation.

Draft a message.

Create a World object.

Retrieve a memory.

Perform research.

Execute an approved browser interaction.

The capability definition is distinct from the mechanism that performs it.

Authority

[DECIDED / DESIGNED]

The governing principle established in earlier discussions was:

Probabilistic perception → deterministic intent → capability/law/consent gate → deterministic execution → evidence.

The AI may interpret a request and propose actions.

It must not independently become the source of authorization.

The harness has developed safety requirements around target ownership, approval, standing approval, send-once/no-resend, journaling, path restrictions, and protected files.

Those are not implementation inconveniences. They are part of the intended trust model.

Evidence

VIVIM should distinguish a model claiming success from an operation being observed to succeed.

For browser interactions, previous testing requirements explicitly demanded real provider-webapp behavior, not merely successful HTTP responses or unit tests.

An execution record should preserve the relevant command, target, observed outcome, timing, authorization context, and uncertainty.

vivim.self and vivim.mind

[DESIGNED]

Previous Ω research proposed two complementary layers.

vivim.self is a grounded self-description system: what the environment is, how it is built, what capabilities exist, which contracts apply, and what evidence supports those claims.

vivim.mind is the task- and observer-dependent cognition layer: assembling the relevant context and applying intelligence to the current situation.

The user explicitly wanted fresh agents to orient themselves without requiring a human briefing or a giant AGENTS.md file.

That led to proposals for:

Semantic indexing of code and inline annotations.

Evidence-backed self-description.

Canonical identity and lineage graphs.

Staleness and conflict detection.

Progressive task-context disclosure.

Cold-agent onboarding tests.

The system should be able to explain itself from its actual implementation and evidence, not from a fictional self-image.

7. Fleet / swarm / agent roles
Fleet versus swarm

[INFERRED]

These terms were present in the October 30 harness development work, but I do not have a confirmed final contract distinguishing them.

A useful distinction would be:

Fleet: The managed population of agents, their roles, availability, configurations, capabilities, and resources.

Swarm: A temporary coordinated group of agents executing a shared objective.

In that interpretation, fleet is the standing resource pool; swarm is an execution pattern.

Do not hard-code this distinction without checking the current fleet/swarm implementation.

Previously discussed orchestration

[DECIDED / DESIGNED]

The project has emphasized:

A master agent coordinating specialized subagents.

Low-cost intelligence for routine tasks.

Higher-cost intelligence only where justified.

Dynamic routing according to task complexity.

Reuse of existing evidence and work.

Avoiding duplicate investigations.

Autonomous execution within established authorization.

Previous development-agent allocations included Claude Sonnet as master, DeepSeek for economical implementation work, and higher-capability Claude or ChatGPT models for difficult tasks.

Those are development-time preferences, not a permanent product requirement or guaranteed availability of models inside a user's webapp sessions.

Orchestration patterns

[DESIGNED / INFERRED]

The current auto-router work suggests a repertoire including:

Pattern

	

Use




Single

	

One provider handles a straightforward task




Fallback

	

Retry through an alternative provider after a recoverable failure




Review panel

	

Independent critique before acceptance




Council

	

Multiple perspectives on an important decision




Race / hedge

	

Alternative executions when justified




Fan-out

	

Divide independent research or analysis




Swarm

	

Coordinate dependent subtasks toward one outcome

All such patterns should operate through the user's authenticated webapp sessions.

A swarm should not create its own unrestricted browser controllers or bypass approval gates.

Important implementation separation

[INFERRED]

I recommend distinguishing three things:

The user's persistent personal agent.

The agent orchestration system and its temporary workers.

The intelligence providers used to realize those workers.

Otherwise, the architecture risks treating every browser conversation as a separate personal identity.

ERA III — FROM BOOTSTRAP TO A WORKING PRODUCT
8. Mobile / PWA and pairing with the host
Gate 1 end-state

[DECIDED — supplied constraint]

The user installs a VIVIM host package and an Android APK/PWA, pairs the mobile application to the host, configures a dedicated VIVIM Chrome profile, and signs into their AI web applications.

That is the complete intended setup.

The user then bootstraps their personal local agentic AI from the PWA.

No model-provider APIs are involved.

Platform direction

[DECIDED]

Previous project planning specified:

Windows-first host implementation.

Tauri v2 as the intended desktop packaging direction.

Android PWA packaged as an APK using Bubblewrap.

A Rust core with UniFFI discussed for cross-platform reuse.

Linux compatibility as an engineering requirement.

Webapp-native AI interactions through the existing VIVIM Harness.

These are not all necessarily required to be complete at the same Gate 1 milestone.

Logical topology

Android APK / PWA

User interface · bootstrap · World interaction

Authenticated paired connection

VIVIM Host — local machine

Personal agent

World & memory

Authority & evidence

Harness & orchestration

Owned browser control

Dedicated VIVIM Chrome profile

Authenticated provider sessions

ChatGPT

Claude

Gemini

Other supported webapps

This is a conceptual topology, not confirmation of current pairing protocol or process boundaries.
Ownership constraints

[DECIDED]

The harness must respect the dedicated browser's ownership and isolation.

Previously established requirements include:

Do not create competing browser controllers.

Do not modify unrelated Chrome profiles.

Do not close or navigate tabs the harness does not own.

Preserve target binding and send-once safeguards.

Do not silently reauthenticate or bypass user login.

The dedicated profile is a core product boundary, not simply a testing convenience.

Pairing details still unresolved

[INFERRED]

The host/PWA pairing protocol should define device identity, authentication, reconnect behavior, revocation, transport security, and what happens when the host is offline.

These details are necessary, but I do not have a confirmed final design.

9. Onboarding
Desired user journey

[DECIDED]

The end-user onboarding should be radically simpler than the internal architecture.

The user should not need to understand CDP, model routing, agent orchestration, capability contracts, or memory schemas.

The user performs the setup in the Gate 1 constraint. Everything after that should be driven through the PWA.

Suggested bootstrap experience

[INFERRED]

Host and mobile pairing

The mobile app identifies and connects to the installed host.

Provider readiness

VIVIM confirms which signed-in web applications are available in its dedicated Chrome profile.

Personal agent creation

The user initiates bootstrap and establishes their agent's identity and initial purpose.

Initial self-discovery

The agent discovers host capabilities, available providers, memory, and World functionality.

First useful interaction

The agent completes a small, real task and preserves evidence and useful context.

World begins evolving

The agent creates useful World objects, ongoing work, and appropriate interfaces as the user's needs develop.

This is a proposed user-facing sequence, not a claim that the historical four-stage bootstrap uses these exact stages.

Onboarding principle

[DESIGNED]

A newly bootstrapped agent should know enough about itself to work without being handed an enormous manual.

This applies both to the user's personal agent and to development agents working on VIVIM.

10. Naming and glossary

The following terms have appeared across the work, but some belong to the broader Ω research rather than finalized VIVIM product vocabulary.

Term

	

Meaning / status




VIVIM

	

User-owned, local-first personal AI world. [DECIDED]




VIVIM World

	

Persistent environment of user objects, work, knowledge, and interactions. [DECIDED]




VIVIM Host

	

Installed local runtime connecting World, agent, and authenticated browser. [DECIDED]




Personal Agent

	

Persistent user-specific agentic intelligence. [DECIDED]




VIVIM Harness

	

Shared browser-native execution and orchestration infrastructure. [DECIDED]




AION

	

Multi-agent development/research and orchestration project. [DECIDED]




Ω / VOMEGA

	

Broader self-describing, governed, evolving-system research. [DESIGNED]




DNA / Genome

	

Bootstrap structure and evolution specification. [DESIGNED, incomplete]




CANON

	

Foundational durable memory. [DESIGNED]




Immediate

	

Short-horizon context and observations. [DESIGNED]




Curated

	

Consolidated long-term knowledge. [DESIGNED]




Work

	

Durable tasks, plans, next actions, and progress. [IMPLEMENTED—REPORTED]




SITREP

	

Structured current-state and progress report. [DESIGNED]




Capability

	

Defined operation with a contract. [DESIGNED]




Realization

	

Concrete implementation of a capability. [DESIGNED]




Authority

	

Permission to perform an action under current conditions. [DESIGNED]




Evidence

	

Recorded observations and outcomes. [DESIGNED]




vivim.self

	

Evidence-backed system self-description. [DESIGNED]




vivim.mind

	

Contextual cognition and task understanding. [DESIGNED]




Fleet

	

Managed pool of available agents. [INFERRED]




Swarm

	

Coordinated group assembled for a task. [INFERRED]




Aperture

	

Selective, task-relevant context disclosure. [DESIGNED]




Forge

	

Governed creation or evolution mechanism. [DESIGNED]

A useful naming discipline is to keep product concepts separate from internal research terminology.

Users should see concepts such as My World, My Agent, Memory, and Work, not be required to learn the full Ω ontology.

11. Architecture
Architectural principles

[DECIDED / DESIGNED]

VIVIM's architecture has converged on several principles.

Local-first ownership: The host owns durable user state.

Webapp-native intelligence: Model interaction happens through authenticated provider web applications.

Deterministic governance: LLM outputs do not themselves authorize execution.

Replaceable realizations: Intelligence providers and implementation mechanisms can change without destroying persistent agent identity.

Evidence-backed state: Observed outcomes, provenance, and uncertainty are retained.

Composable evolution: The system can add capabilities and interfaces without requiring a complete redesign.

Shared harness: Browser and provider interaction infrastructure should be centralized rather than duplicated across projects.

Suggested logical decomposition

[INFERRED from previous designs]

Subsystem

	

Responsibility




Host runtime

	

Lifecycle, local services, pairing




World kernel

	

Objects, identity, relationships, persistent state




Personal agent

	

Bootstrap, self-model, memory, work




Intelligence router

	

Provider selection and orchestration patterns




Browser harness

	

Owned profile, tabs, provider interactions




Capability registry

	

Discoverable operations and realizations




Authority engine

	

Consent, policies, target checks




Execution engine

	

Governed action dispatch




Evidence journal

	

Receipts, outcomes, provenance




Memory engine

	

CANON, Immediate, Curated and retrieval




Evolution engine

	

Proposed and governed modifications




Surface runtime

	

PWA/desktop World views




Export/import

	

User portability and recovery

This is a logical architecture, not a mandate to create twelve new packages.

Existing implementation should be reused wherever possible.

Existing repository landscape

[DECIDED / REPORTED]

The main repositories discussed have been:

vivim-harness — central harness and execution infrastructure.

AION — agentic research, development, and orchestration work.

VOMEGA — experimental semantic and evolving-system work.

The current implementation should not assume that every historical concept needs its own independent runtime.

Earlier discussions explicitly warned against creating duplicate databases, orchestrators, or competing implementations of shared capabilities.

Current engineering constraints

[DECIDED]

The harness has been developed with Node.js 22+, ESM, and a dependency-light or dependency-free core.

There is an explicit prohibition on Playwright and Puppeteer in the harness work.

Real-world testing is intended to use native CLI/Studio/CDP interactions with actual provider web applications.

Windows and Linux compatibility matter, although Windows is the first host target.

The existing dedicated browser controller and ownership rules must be preserved.

12. Priorities
Recommended Gate 1 ordering

[INFERRED — implementation prioritization]

The following prioritization follows the user's end-state rather than the historical order in which individual features were explored.

P0

Working host-to-PWA-to-browser path

A paired phone can invoke a real, authorized operation through the local host and observe the result.

P0

Dedicated profile and provider sessions

Stable browser ownership, authenticated provider readiness, correct target binding, and safe send behavior.

P0

Persistent personal agent bootstrap

Create and recover the same agent identity across host restarts and mobile reconnections.

P0

Durable World, memory, and Work

Retain meaningful state locally, with evidence and portable export.

P1

Capability discovery and self-knowledge

The agent understands its actual available tools and limitations.

P1

Intelligent multi-provider orchestration

Route through authenticated webapps without bypassing safety boundaries.

P1

World surface evolution

Allow the personal environment to grow beyond its minimal initial interface.

P2

Advanced fleet, swarm, and genome evolution

Expand coordination and governed self-development after the end-to-end foundation works.

Gate 1 acceptance criterion

[INFERRED]

A meaningful Gate 1 demonstration would be:

The user opens the paired Android application, bootstraps their personal agent, asks it to complete a real task, and watches that task run through a signed-in AI webapp on the host. The result is returned to the PWA, recorded as evidence, and incorporated into durable local World or Work state.

After restarting the host, the user reconnects and finds the same agent, task, and relevant memory.

No provider API key is entered, no second browser controller is created, and no unapproved action is performed.

This is the most useful end-to-end definition of a working first product.

13. Open questions

These are genuine gaps in what I can establish from the conversations and retained context.

Question

	

Current confidence




What is the canonical DNA/genome schema?

	

Not established




What are the exact four bootstrap stages currently implemented?

	

Requires source inspection




Is vivim.self implemented or still primarily a research design?

	

Unverified




What is the finalized Fleet versus Swarm contract?

	

Unverified




How are agent identities and worker identities serialized?

	

Unverified




What is the host/PWA pairing protocol?

	

Not established




How are phone reconnection and host-offline states handled?

	

Not established




Which World objects and relationships are already persisted?

	

Requires source inspection




How are CANON, Immediate, and Curated memory actually stored and promoted?

	

Requires source inspection




How does the personal agent safely propose and install new capabilities or surfaces?

	

Not finalized




What is the canonical recovery/backup format beyond JSON export?

	

Not established




Which provider-webapp workflows are currently reliable end to end?

	

Requires live verification




Which fleet/swarm features have actually landed on the current main branch?

	

Requires source inspection

Specific implementation risks

[INFERRED]

The largest risks are not primarily model intelligence. They are continuity, state ownership, browser correctness, and the boundary between proposed and authorized behavior.

A sophisticated agent that cannot reliably identify its active provider tab, recover after a restart, or distinguish an observed result from a model assertion is not yet a dependable personal agent.

Likewise, a beautiful World interface without durable object identity and persistence would be a presentation layer, not the intended product.

Final implementation handoff

There are three important truths I would want the implementation agent to preserve.

First: VIVIM is the user's persistent world. The personal agent belongs to that world; it is not the world itself. AI providers are interchangeable intelligence sources, not owners of the user's identity or memory.

Second: Bootstrap is a product experience, not just an initialization function. The user should go from an installed host and paired phone to a functioning, increasingly capable personal agent, without needing to understand or configure the internal machinery.

Third: The entire architecture should preserve the distinction between knowledge, capability, authority, execution, and evidence. An agent may learn, plan, delegate, and evolve, but it must remain grounded in what actually exists and what the user has authorized.

One-sentence implementation mission

Build VIVIM as a local-first, user-owned digital world that bootstraps a persistent, evolving personal agent from an Android PWA, uses only the user's authenticated AI web applications for intelligence, and maintains durable memory, governed capabilities, and evidence-backed continuity through the paired local host.

Implementation note: Treat this document as a conceptual authority and historical design synthesis, not as an authoritative description of the current codebase. Where marked [INFERRED], use it as a design candidate; where marked [IMPLEMENTED—REPORTED], inspect the source before relying on it. Preserve the explicit Gate 1 constraints above everything else.

---

## Turn 2: prompt

> Excellent — this is exactly what I needed. Here is what the repo actually has today (owenservera/vivim-web-harness), so you can ground the next answers:
> 
> - GOALS.md: Gate 1 milestones M1 provider readiness (`wh providers --signed-in`, add Grok adapter) → M2 setup wizard (create/launch Vivim profile, walk sign-ins) → M3 harness pairing (phone↔host harness server, token→cookie, QR, revoke; approvals on phone) → M4 mobile bootstrap (4-stage bootstrap, default fleet from roles, first approved send) → M5 single package (Windows first).
> - studio/src/studio: personal-agent-routes.mjs (4 stages = identity / capability / authority / evidence), lan-pair.mjs, ai-router.mjs, ai-orchestrator.mjs, orchestrator-patterns.mjs, capabilities.mjs, personal-app-*.mjs; studio /mobile PWA + studio/android WebView shell.
> - harness/: single send path router.mjs (journal, per-request approval, send-once), brain.mjs, providers/*.mjs, docs AGENTS-FLEET-SWARM.md ("the personal agent is a fleet; hard tasks fan out as swarms"), USER-JOURNEY.md.
> 
> If you have GitHub access to owenservera/vivim-web-harness or vivim-harness, please look at those files directly and say what you saw; if not, say so and work from the above.
> 
> Now drill in, structured markdown, concrete and implementable:
> 1. The DNA / bootstrap genome: propose a concrete versioned JSON schema (fields, example) that fits the 4 stages identity/capability/authority/evidence and the memory tiers CANON/Immediate/Curated + Work.
> 2. The bootstrap as a phone-driven flow: exact stages, what the user sees on the PWA at each, what the host does, what is persisted, and the done-condition of each stage.
> 3. Fleet/swarm: a concrete role list for the default fleet (name, purpose, which task class, which provider tier), and the swarm contract.
> Skip generic caveats; I want Owen's structure plus your best concrete proposal where it's unset.

## Turn 2: ChatGPT

VIVIM — DNA, Phone Bootstrap & Agent Fleet Specification

Implementation design v0.1 · October 30, 2026

I accessed the GitHub repository owenservera/vivim-web-harness directly and read three files on main:

GOALS.md

personal-agent-routes.mjs

AGENTS-FLEET-SWARM.md

I also attempted harness/USER-JOURNEY.md, which returned 404. GOALS.md identifies its actual location as harness/docs/USER-JOURNEY.md.

What the source establishes

The four bootstrap stages are already named and implemented at the service boundary: identity, capability, authority, evidence. The routes expose initialization, bootstrap, self-knowledge, capabilities, memory, Work, continuity, and JSON export.

The fleet and swarm are also real harness concepts, not hypothetical architecture. The documentation specifies seven roles, persistent fleet agents with reusable threads, ephemeral swarms, four swarm patterns, budgets, and routing through the same approval-controlled send path.

One particularly important discovery: GOALS.md contains both the newer Gate 1 vision and older Phase 1 engineering notes. The newer Gate 1 plan requires phone-based approvals; older sections still describe owner approval through the terminal. The implementation needs to bridge these without weakening the router's existing authorization rules.

I will treat the GitHub source as authoritative for existing contracts and clearly label new design decisions as PROPOSED.

ERA I — THE DNA / BOOTSTRAP GENOME
1. The genome is the agent's durable constitution

PROPOSED: Introduce vivim.agent.genome version 1.0.0.

The genome should be a small, declarative, locally persisted object that answers four questions:

Stage

	

Genome responsibility




Identity

	

Who is this agent, and who owns it?




Capability

	

What can it do, and which resources realize those abilities?




Authority

	

Which actions may be proposed, approved, or executed?




Evidence

	

How does the system establish that work actually happened?

Memory and Work are not additional bootstrap stages. They are persistent subsystems initialized by the four stages.

The genome must reference mutable memory, Work, provider sessions, and evidence. It should not embed their full contents.

This allows the user's agent identity to remain stable while memory, browser threads, and available providers evolve.

2. Concrete genome document

The following is the proposed persisted format, not an existing repository schema.

JSON

{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "kind": "vivim.agent.genome",
  "schemaVersion": "1.0.0",
  "genomeId": "genome_01",
  "revision": 1,
  "agent": {
    "id": "agent_01",
    "name": "My Vivim",
    "type": "personal",
    "ownerId": "local_owner_01",
    "createdAt": "2026-10-30T16:00:00Z",
    "lifecycle": "bootstrapping"
  },
  "bootstrap": {
    "version": 1,
    "currentStage": "identity",
    "stages": {
      "identity": "complete",
      "capability": "pending",
      "authority": "pending",
      "evidence": "pending"
    }
  },
  "capabilities": {
    "registry": "local://capabilities",
    "providerPolicy": "signed-in-webapps-only",
    "providers": [],
    "fleet": {
      "template": "default-v1",
      "instanceId": null
    }
  },
  "authority": {
    "policyId": "owner-controlled-v1",
    "defaultEffect": "deny",
    "approvalMode": "per-request",
    "approvalDevice": null,
    "standingApprovals": [],
    "invariants": [
      "single-browser-owner",
      "exact-target-binding",
      "send-once",
      "never-resend-uncertain",
      "journal-before-effect",
      "no-provider-apis"
    ]
  },
  "evidence": {
    "journal": "local://journal",
    "provenance": "local://provenance",
    "requireObservedOutcome": true,
    "preserveUncertainty": true,
    "firstReceiptId": null
  },
  "memory": {
    "schemaVersion": 1,
    "store": "local://personal-agent/memory",
    "tiers": ["CANON", "Immediate", "Curated"],
    "promotionPolicy": "explicit-or-evidence-backed"
  },
  "work": {
    "store": "local://personal-agent/work",
    "requireVersionCheck": true,
    "requireIdempotencyKey": true
  },
  "evolution": {
    "mode": "propose-only",
    "selfAuthorize": false,
    "requireOwnerApproval": true,
    "rollback": true
  }
}


The IDs and timestamps are illustrative. The host must generate actual IDs and timestamps.

Why this shape?

The genome does not store a cookie, QR token, CDP endpoint, browser profile path, or full conversation.

It references the systems that own those resources.

The genome's revision is for genome changes. It must not replace the existing Personal Agent service's compare-and-swap versioning.

The authority section is a declaration of policy identity and constraints. The executable authority remains in the harness router.

3. Versioned JSON Schema

The schema below validates the structural contract. The host must additionally enforce cross-field invariants and references.

JSON

{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://vivim.local/schemas/agent-genome/1.0.0",
  "title": "VIVIM Personal Agent Genome",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "kind", "schemaVersion", "genomeId", "revision",
    "agent", "bootstrap", "capabilities", "authority",
    "evidence", "memory", "work", "evolution"
  ],
  "properties": {
    "kind": { "const": "vivim.agent.genome" },
    "schemaVersion": { "const": "1.0.0" },
    "genomeId": { "$ref": "#/$defs/id" },
    "revision": { "type": "integer", "minimum": 1 },
    "agent": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id", "name", "type", "ownerId",
        "createdAt", "lifecycle"
      ],
      "properties": {
        "id": { "$ref": "#/$defs/id" },
        "name": {
          "type": "string",
          "minLength": 1,
          "maxLength": 80
        },
        "type": { "const": "personal" },
        "ownerId": { "$ref": "#/$defs/id" },
        "createdAt": {
          "type": "string",
          "format": "date-time"
        },
        "lifecycle": {
          "enum": [
            "bootstrapping", "ready",
            "suspended", "retired"
          ]
        }
      }
    },
    "bootstrap": {
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "currentStage", "stages"],
      "properties": {
        "version": {
          "type": "integer",
          "minimum": 1
        },
        "currentStage": {
          "enum": [
            "identity", "capability",
            "authority", "evidence", "complete"
          ]
        },
        "stages": {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "identity", "capability",
            "authority", "evidence"
          ],
          "properties": {
            "identity": { "$ref": "#/$defs/stage" },
            "capability": { "$ref": "#/$defs/stage" },
            "authority": { "$ref": "#/$defs/stage" },
            "evidence": { "$ref": "#/$defs/stage" }
          }
        }
      }
    },
    "capabilities": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "registry", "providerPolicy",
        "providers", "fleet"
      ],
      "properties": {
        "registry": { "type": "string" },
        "providerPolicy": {
          "const": "signed-in-webapps-only"
        },
        "providers": {
          "type": "array",
          "uniqueItems": true,
          "items": { "type": "string" }
        },
        "fleet": {
          "type": "object",
          "additionalProperties": false,
          "required": ["template", "instanceId"],
          "properties": {
            "template": { "type": "string" },
            "instanceId": {
              "type": ["string", "null"]
            }
          }
        }
      }
    },
    "authority": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "policyId", "defaultEffect",
        "approvalMode", "approvalDevice",
        "standingApprovals", "invariants"
      ],
      "properties": {
        "policyId": { "type": "string" },
        "defaultEffect": { "const": "deny" },
        "approvalMode": {
          "enum": ["per-request", "standing-policy"]
        },
        "approvalDevice": {
          "type": ["string", "null"]
        },
        "standingApprovals": {
          "type": "array",
          "items": { "type": "string" }
        },
        "invariants": {
          "type": "array",
          "uniqueItems": true,
          "items": { "type": "string" }
        }
      }
    },
    "evidence": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "journal", "provenance",
        "requireObservedOutcome",
        "preserveUncertainty", "firstReceiptId"
      ],
      "properties": {
        "journal": { "type": "string" },
        "provenance": { "type": "string" },
        "requireObservedOutcome": { "const": true },
        "preserveUncertainty": { "const": true },
        "firstReceiptId": {
          "type": ["string", "null"]
        }
      }
    },
    "memory": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "schemaVersion", "store",
        "tiers", "promotionPolicy"
      ],
      "properties": {
        "schemaVersion": {
          "type": "integer",
          "minimum": 1
        },
        "store": { "type": "string" },
        "tiers": {
          "type": "array",
          "prefixItems": [
            { "const": "CANON" },
            { "const": "Immediate" },
            { "const": "Curated" }
          ],
          "items": false,
          "minItems": 3,
          "maxItems": 3
        },
        "promotionPolicy": {
          "enum": [
            "explicit-or-evidence-backed",
            "manual-only"
          ]
        }
      }
    },
    "work": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "store", "requireVersionCheck",
        "requireIdempotencyKey"
      ],
      "properties": {
        "store": { "type": "string" },
        "requireVersionCheck": { "const": true },
        "requireIdempotencyKey": { "const": true }
      }
    },
    "evolution": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "mode", "selfAuthorize",
        "requireOwnerApproval", "rollback"
      ],
      "properties": {
        "mode": {
          "enum": ["disabled", "propose-only"]
        },
        "selfAuthorize": { "const": false },
        "requireOwnerApproval": { "const": true },
        "rollback": { "type": "boolean" }
      }
    }
  },
  "$defs": {
    "id": {
      "type": "string",
      "minLength": 1,
      "maxLength": 128,
      "pattern": "^[a-zA-Z0-9_-]+$"
    },
    "stage": {
      "enum": [
        "pending", "active",
        "complete", "blocked"
      ]
    }
  }
}


Implementation detail: Use a JSON Schema 2020-12 validator with format support for date-time, or validate that format explicitly. The host must enforce stage ordering, required invariant membership, and the consistency of lifecycle, currentStage, and actual evidence records. The schema alone does not establish those relationships.

4. Memory and Work record contracts

The existing routes use explicit tiers and versioned mutations. Build on them rather than replacing them.

PROPOSED memory record:

JSON

{
  "id": "mem_001",
  "tier": "CANON",
  "key": "preferred_language",
  "text": "English",
  "origin": "user-explicit",
  "confidence": "confirmed",
  "evidenceRefs": [],
  "createdAt": "2026-10-30T16:00:00Z",
  "updatedAt": "2026-10-30T16:00:00Z",
  "version": 1,
  "status": "active"
}


Recommended origin values are user-explicit, observed, imported, and agent-inferred.

A memory promoted from Immediate to Curated should retain its original provenance and promotion history.

Work should remain a separate entity, with the existing states queued, active, blocked, completed, and cancelled.

The current routes already provide versioned Work creation, transitions, next actions, and relevant-memory lookup. Do not build a second Work store just to satisfy the genome.

ERA II — PHONE-DRIVEN BOOTSTRAP
5. The actual four-stage flow

I also fetched and inspected USER-JOURNEY.md at the correct path.

It confirms that Gate 1 officially passes when a newly installed user starts creating their personal agent from the phone at step 6. Completing the first approved send is the stronger M4 functional acceptance test.

The four bootstrap stages should remain exactly:

Personal agent bootstrap

4 stages

1

Identity

2

Capability

3

Authority

4

Evidence

Proposed phone UX below; the stage names and existing bootstrap service are grounded in the repository.

Stage 1 — Identity

Create your personal AI

Your AI lives on your computer and belongs to you.

Agent name
My Vivim
Primary purpose

Help me think, create and get things done.

Connected to your host

Create my agent

Illustrative PWA screen

Host actions

Invoke the existing initialization service using POST /personal-agent/initialize with expectedVersion: null for first initialization.

Generate the persistent agent identity, bind it to the paired local owner, and initialize the genome revision.

Persist

agentId, ownerId, genomeId, creation time, lifecycle, and identity-stage completion.

Done when

A durable identity exists, the paired user can retrieve it after reconnecting, and repeated initialization cannot create a second agent accidentally.

The existing initialize route does not accept a name or purpose. Those would require a separate, versioned profile update or an extension to the service contract.

Stage 2 — Capability

Phone: Choose your AI connections

Vivim found these signed-in applications on your computer.

ChatGPT

Ready

Claude

Ready

Gemini

Ready

Grok

Sign in required

Your agent fleet

Vivim will assign available AI connections to specialist roles. You can change these later.

Planner · Builder · Reviewer · Verifier · Researcher · Synthesizer · Scribe

Illustrative PWA screen; actual provider readiness must come from the M1 probe.

Host actions: Call the read-only provider readiness service introduced by M1. Enumerate signed-in providers and the capability registry. Resolve the default fleet's model bindings against actually available provider sessions.

Create a fleet configuration using the existing role definitions. Do not attempt sends during this stage.

Persist: Provider selections, capability references, fleet configuration ID, and capability-stage completion. Provider sign-in state remains live observational data, not a permanent guarantee.

Done when: At least one selected provider is confirmed ready, all enabled fleet roles have valid bindings, and the host can retrieve the same fleet configuration after restart. Roles without available providers should be disabled or explicitly marked unavailable, not silently assigned fictional models.

Stage 3 — Authority

Phone: You're in control

Approve actions on your phone

Vivim will ask before sending a prompt to an AI application.

Exact provider and conversation shown before approval

Uncertain sends are never automatically repeated

Revoke a paired device

Enable phone approvals

Illustrative PWA screen

Host actions: Bind the paired device to the authenticated owner approval channel. Display the request's exact provider, target, prompt digest, and action type. Make the approval mechanism call the existing router approval logic rather than creating a second approval authority.

Persist: Device identity, approval policy reference, revocation state, and authority-stage completion.

Done when: The paired phone can approve or reject a harmless test authorization request, and a revoked device cannot approve anything.

A UI toggle or accepted consent screen alone is not sufficient evidence that the approval path works.

Stage 4 — Evidence

Phone: Your first agent task

Your agent is ready to try its first task using one of your signed-in AI applications.

Suggested first task

Introduce yourself, describe your available capabilities, and suggest three useful things we could do together.

You'll approve the exact message before it's sent.

First task receipt

Awaiting approval and observed result

Illustrative PWA screen

Host actions: Create a journaled send request through the existing router. Present the approval on the paired phone. After approval, dispatch once, observe the reply, and persist its outcome and evidence.

The existing /personal-agent/bootstrap route is explicitly documented as having no external effects. Keep it that way. The first provider send belongs to a separate, governed execution workflow.

Persist: Request ID, provider/model, exact thread reference, approval reference, observed status, reply/evidence digest, and the first completed Work item where appropriate.

Done when: A provider reply is observed as complete and its receipt is retrievable from the phone. An uncertain send remains uncertain; it must not be retried merely to make onboarding appear successful.

Bootstrap transition rules

PROPOSED

Plain text
uninitialized
    |
    v
identity_complete
    |
    v
capability_complete
    |
    v
authority_complete
    |
    v
evidence_complete
    |
    v
ready

Each transition should use the existing versioned state model.

Stage completion must be idempotent. A repeated phone request should return the existing stage result, not recreate identities, fleets, or sends.

For M4, ready should mean the first observed task has completed. If Gate 1 is evaluated earlier, report bootstrapping rather than claiming full operational readiness.

ERA III — THE FLEET AND SWARM CONTRACT
6. Default fleet — grounded in the actual role definitions

I fetched roles.mjs and swarm.mjs.

The default role list is already defined in code. Keep these seven roles and their IDs. There is no reason to introduce a competing taxonomy.

Role

	

Task class

	

Current default model

	

Proposed cost tier




Planner

	

Task decomposition, orchestration

	

chatgpt:gpt-6

	

High




Builder

	

Code and content production

	

claude:opus-5.5

	

Premium




Reviewer

	

Independent criticism

	

gemini:gemini-3-pro

	

High




Verifier

	

Testing and verification

	

deepseek:deepseek-v3.2

	

Economy




Researcher

	

Research, facts, prior art

	

gemini:gemini-3-pro

	

High




Synthesizer

	

Judgment and consolidation

	

claude:opus-5.5

	

Premium




Scribe

	

Durable notes and decisions

	

deepseek:deepseek-v3.2

	

Economy

The model references above are from the repository. The cost-tier labels are my proposed scheduling classification, not verified provider prices.

Concrete fleet instance

PROPOSED: Generate a fleet from the seven registered roles, not from hard-coded PWA-specific definitions.

JSON

{
  "schemaVersion": "1.0.0",
  "kind": "vivim.agent.fleet",
  "fleetId": "fleet_personal_01",
  "ownerAgentId": "agent_01",
  "template": "default-v1",
  "agents": [
    {
      "name": "lead",
      "role": "planner",
      "model": "chatgpt:gpt-6",
      "threadUrl": null
    },
    {
      "name": "maker",
      "role": "builder",
      "model": "claude:opus-5.5",
      "threadUrl": null
    },
    {
      "name": "critic",
      "role": "reviewer",
      "model": "gemini:gemini-3-pro",
      "threadUrl": null
    },
    {
      "name": "checker",
      "role": "verifier",
      "model": "deepseek:deepseek-v3.2",
      "threadUrl": null
    },
    {
      "name": "analyst",
      "role": "researcher",
      "model": "gemini:gemini-3-pro",
      "threadUrl": null
    },
    {
      "name": "judge",
      "role": "synthesizer",
      "model": "claude:opus-5.5",
      "threadUrl": null
    },
    {
      "name": "recorder",
      "role": "scribe",
      "model": "deepseek:deepseek-v3.2",
      "threadUrl": null
    }
  ]
}


This follows the documented fleet shape: {name, role, model, threadUrl}.

The surrounding metadata is proposed. Keep it outside the core fleet record if the existing parser does not accept it.

Provider availability policy

The default fleet must be instantiated against actual M1 readiness results.

A provider being signed in does not prove that a specific model is available to that account.

PROPOSED binding algorithm:

Load role definitions from roles.mjs.

Read provider readiness and any verified model availability.

Prefer each role's default model when usable.

Otherwise select an allowed, verified alternative.

If no suitable model exists, mark that role unavailable.

Persist the chosen model reference and the reason for selection.

Avoid silently substituting an expensive model for an economy role.

The user can have a functioning personal agent with fewer than seven active roles. Seven is the default fleet template, not a minimum subscription requirement.

7. Swarm contract
What the code already implements

swarm.mjs supports:

Pattern

	

Execution

	

Send count




parallel

	

Independent answers, then judge

	

n + 1




vote

	

Independent answers, judge selects winner

	

n + 1




pipeline

	

Sequential improvement, then judge

	

n + 1




debate

	

Initial answers, revisions, then judge

	

2n + 1

The current default budget is 4 sends, with a hard maximum of 12.

The code requires at least two members, and the judge must use a different provider from at least one member.

Every member is a fixed provider:model reference; auto is not accepted as a member provider.

Proposed swarm request

Use the existing planSwarm input shape:

JSON

{
  "task": "Compare two possible approaches to a local-first memory engine.",
  "pattern": "parallel",
  "members": [
    "chatgpt:gpt-6",
    "gemini:gemini-3-pro"
  ],
  "judge": "claude:opus-5.5",
  "budget": {
    "maxSends": 3
  }
}


The planner produces the deterministic step graph. runSwarm executes it through router.ask and router.wait.

Proposed personal-agent swarm envelope

For integration with World and Work, attach an outer orchestration record:

JSON

{
  "swarmId": "swarm_001",
  "ownerAgentId": "agent_01",
  "workId": "work_001",
  "purpose": "Compare memory architectures",
  "planRef": "local://swarms/swarm_001/plan",
  "state": "planned",
  "approvalPolicy": "per-request",
  "resultRef": null,
  "evidenceRefs": [],
  "createdAt": "2026-10-30T16:00:00Z"
}


The envelope is proposed; it is not an argument to the current runSwarm function.

Required execution invariants

The source already enforces several important constraints:

All provider sends pass through the existing router.

The optional swarm approval callback can reject a wave.

Steps are sent at most once.

A non-complete step stops the run.

Members are dispatched sequentially because one browser controller owns Chrome.

The judge consumes earlier answers as fenced task data.

Budget checks apply before execution.

Roles have no merge authority.

One subtle but important distinction: parallel means logically independent answers, not concurrent browser sends.

Do not add parallel CDP controllers to make the implementation match the name.

Proposed lifecycle
Plain text
planned
  |
  v
awaiting_approval
  |
  v
running
  |
  +----> blocked
  +----> declined
  +----> uncertain
  +----> failed
  |
  v
completed
  |
  v
evidence_recorded
  |
  v
work_updated

The external lifecycle should preserve the current runSwarm result fields: status, stoppedAt, answers, verdict, and sends.

An interrupted or uncertain swarm should never be restarted by blindly replaying its original plan. Recovery must reconcile the journal and any already-sent requests first.

8. When the personal agent should invoke a swarm

PROPOSED: Default to a single provider unless there is a concrete reason for multi-agent execution.

Task

	

Recommended pattern




Simple question

	

Single




Ordinary drafting

	

Single




Difficult architecture comparison

	

Parallel




Choice among competing proposals

	

Vote




Iterative refinement

	

Pipeline




High-uncertainty contested reasoning

	

Debate




Code implementation

	

Builder, then independent review/verification




Memory recording

	

Scribe or deterministic local operation

The auto-router may recommend a pattern, but the execution layer should still calculate its exact send budget and request the necessary approvals.

A swarm is a temporary execution structure. It does not own the personal agent's memory or identity.

9. Concrete implementation map

Based on the files inspected, I would make these additions without replacing the existing subsystems.

Existing location

	

Proposed work




studio/src/agents/personal-agent-service.mjs

	

Integrate genome creation and stage transitions




studio/src/studio/personal-agent-routes.mjs

	

Expose phone-specific bootstrap progression and status




studio/src/studio/lan-pair.mjs

	

Bridge paired device authority to harness approval




studio/mobile

	

Four-stage bootstrap UI and approval screens




harness/src/agents/roles.mjs

	

Remain canonical role registry




harness/src/agents/fleet.mjs

	

Instantiate persistent fleet from verified providers




harness/src/agents/swarm.mjs

	

Keep execution engine; add external Work/evidence integration




harness/src/interface/router.mjs

	

Remain the sole governed send path




New versioned genome module

	

Schema, validation, migrations, persistence adapter

These are suggested integration points; I have not inspected every implementation file listed in the table.

Recommended build sequence

First: M1 provider readiness and M2 profile setup. Without verified authenticated providers, fleet creation is only configuration.

Second: M3 secure phone-to-harness pairing and approval. The phone must be able to control an existing authorization workflow without bypassing it.

Third: M4 genome-backed bootstrap, fleet creation, and the first approved send. Keep the existing four-stage Personal Agent service as the lifecycle owner.

Fourth: M5 packaging and restart/recovery behavior. The installation must preserve the same identity, fleet, Work, and memory.

10. The key architectural decision

The most important choice is not to build a second agent system around the genome.

The repository already has the major primitives:

A four-stage personal-agent bootstrap.

Three memory tiers and durable Work.

Seven registered fleet roles.

Persistent agent threads.

Four swarm patterns.

A governed send router.

Journal and provenance infrastructure.

A mobile PWA and pairing foundation.

The missing piece is the coherent, phone-driven composition of those capabilities into one durable personal agent experience.

My recommendation: Treat the genome as the versioned manifest of that composition, the Personal Agent service as its lifecycle owner, the harness as its execution authority, and the PWA as the user's primary control surface.

That preserves Owen's original product principle: the user owns the agent, the fleet serves the agent, swarms serve individual tasks, and signed-in webapps supply the intelligence.

---

## Turn 3: prompt

> Very useful, and good that you can read the repo. Next, structured markdown, grounded in files you open (cite paths):
> 
> 1. Host↔PWA pairing protocol for M3: device identity, pairing handshake (QR/token → cookie), approvals on the phone mapped onto the existing harness approvals (runtime/approvals, wh approve), reconnect, revoke, host-offline behaviour, and the non-LAN path without any cloud API. Concrete message/endpoint list.
> 2. M1 provider readiness: the exact `wh providers --signed-in` output JSON you'd want, per-provider probe strategy (read-only), and what a Grok adapter in harness/src/providers needs.
> 3. Please review owenservera/vivim-web-harness and owenservera/vivim-harness (if accessible): what's solid, what's duplicated between studio and harness, what should be deleted/merged, with file paths.

## Turn 3: ChatGPT

_(The reply was observed in the thread but not captured. Its content is restated at the start of turn 4.)_

---

## Turn 4: prompt

> Continuing from our earlier conversation today titled "Vivim Concept Brief" (https://chatgpt.com/c/6ae4cb3c-48e4-83ed-ae7a-21a28f6a23cf) — that tab closed, so this is a new thread. I am Owen's implementation agent. My capture of your last answer there (host↔PWA pairing protocol, M1 provider readiness JSON + Grok adapter, and the repo review of what is solid / duplicated / to merge) failed on my side. Please start this reply with a compact but complete restatement of it (keep the endpoint list, the readiness JSON shape, and file-path findings; re-check the repo via your GitHub access if needed).
> 
> Then, structured markdown:
> 1. Work split between you and us: you design/review (via your GitHub access to owenservera/vivim-web-harness), we implement + test with the harness. Propose the concrete loop: what artifacts you produce (specs, review comments), what we deliver (PRs, test evidence, journal request ids), cadence, and how disagreements get settled.
> 2. The first milestone: the single smallest thing we should ship next toward Gate 1, with acceptance tests and the files to touch.
> 3. The full end-user journey from download to "my agent did its first task", step by step, as Owen intends it.

## Turn 4: ChatGPT

VIVIM Gate 1 — Architecture, Implementation Partnership & First User Experience

30 October 2026 · Reconstructed from our earlier conversation and rechecked against GitHub

Core decision: The VIVIM host owns execution and safety. The phone owns the user's experience and approvals. The harness drives signed-in AI web applications through the user's dedicated Chrome profile, never through provider APIs.

I recovered the key decisions from our earlier conversation and inspected both repositories' current main trees. Below is the reconstructed specification, followed by the proposed working relationship and the exact next milestone.

ERA 1 — Recover the architecture
A. Host ↔ PWA pairing protocol
Architecture
Android PWA / APK
VIVIM Host Mobile Bridge
Harness approvals androuter
Journal / request IDs
Single CDP owner
Dedicated VIVIM Chrome
Signed-in AI webapps
Observed replies
Paired, authenticated requests

The phone is a remote control and user interface, not a second browser controller.

The host must remain the sole authority for request identity, approvals, thread binding, journal state and sending.

Pairing handshake

Host creates a persistent device identity and a short-lived, single-use pairing challenge.

Host displays a QR code with the host address and opaque challenge.

Phone opens the pairing page and generates its own device keypair.

Phone redeems the challenge, submitting its device public key.

Host displays the pending device's identity and asks the owner to confirm.

Upon confirmation, host establishes a device-bound session and sets a restricted HttpOnly cookie.

Phone reconnects using its session and authenticated device identity, without scanning again.

Host can revoke an individual device at any time.

Use Secure, HttpOnly, and SameSite cookie attributes in a properly configured HTTPS deployment, and enforce CSRF and Origin protections on mutations.

A cookie alone is insufficient for device binding; the session should be associated with a cryptographic device identity.

Proposed endpoints

Contract to implement; not a claim that these endpoints already exist.

Method

	

Endpoint

	

Purpose




POST

	

/api/v1/pairing/challenges

	

Generate one-use challenge




POST

	

/api/v1/pairing/redeem

	

Register pending phone




GET

	

/api/v1/pairing/status

	

Check pairing progress




POST

	

/api/v1/pairing/confirm

	

Host-authorized pairing




GET

	

/api/v1/devices

	

List paired devices




DELETE

	

/api/v1/devices/:id

	

Revoke a device




GET

	

/api/v1/providers/readiness

	

Provider readiness




GET

	

/api/v1/approvals

	

List pending approvals




POST

	

/api/v1/approvals/:id/decision

	

Approve or reject




GET

	

/api/v1/requests/:id

	

Request state and result

For the first phone-driven task, add:

Method

	

Endpoint

	

Purpose




POST

	

/api/v1/tasks

	

Submit task intent for planning




GET

	

/api/v1/tasks/:id

	

Task execution progress




GET

	

/api/v1/events

	

Authenticated progress stream

Approval integration: A phone approval must call the existing harness approval machinery, not directly authorize a browser operation. The server must recheck the authenticated owner's authority, original request ID, payload digest, exact provider and thread, expiration, and one-time consumption.

A phone cannot approve a mutated request or replay a previous grant.

Reconnect: The phone retrieves pending tasks and approvals from the host after reconnecting. Reconnection must never resubmit uncertain sends.

Host offline: The phone displays the last known non-sensitive state as offline and disables all execution and approval actions. It must not queue sends that automatically execute when the host returns.

Outside the LAN: Use a private WireGuard tunnel, or equivalent device-authenticated private network. Do not expose CDP, PinchTab or the raw harness server publicly. The mobile bridge is the only remote-facing execution gateway.

What already exists

The existing vivim-harness implementation provides useful pairing foundations:

src/studio/lan-pair.mjs: LAN discovery, QR generation, persistent pairing token and rotation.

src/studio/server.mjs: paired LAN listener, exact Host/Origin checks, token-to-cookie exchange.

src/studio/ui/mobile.mjs: existing phone interface.

src/studio/ui/mobile-sw.js: constrained PWA shell caching.

Reference: Existing mobile pairing design

But its current token grants the entire Studio API, and the documentation describes plain HTTP on trusted Wi-Fi. It does not yet provide the device-specific isolation described above.

Migration decision: Reuse the existing pairing UI and QR infrastructure. Replace broad bearer-token access with narrower device-scoped sessions, and connect the phone to the harness's existing approval authority.

B. M1 — Provider readiness

The first milestone is:

wh providers --signed-in --json

It performs read-only checks of the dedicated Chrome's existing signed-in webapp tabs.

No sends, no typing, no navigation, no implicit tab creation, and no account-identifying data in the output.

Proposed JSON contract
JSON

{
  "schema": "vivim.provider-readiness/1",
  "checkedAt": "2026-10-30T17:00:00Z",
  "host": {
    "chromeConnected": true,
    "profile": "vivim-dedicated",
    "controller": "single-owner"
  },
  "summary": {
    "total": 7,
    "ready": 1,
    "signInRequired": 1,
    "unknown": 5
  },
  "providers": [
    {
      "id": "chatgpt",
      "siteSupport": true,
      "signedIn": true,
      "state": "ready",
      "confidence": "high",
      "reason": null,
      "tab": {
        "present": true,
        "url": "https://chatgpt.com/",
        "ownedByHarness": false
      },
      "modelSelection": {
        "supported": true,
        "current": null
      },
      "canSend": true,
      "evidence": {
        "authWall": false,
        "composerVisible": true
      }
    }
  ]
}


The single provider entry above illustrates the shape, not a measured login state. The full response contains all seven providers, including unknown or unsupported entries.

Important semantic distinction:

signedIn: true | false | "unverified"

state: ready | sign_in_required | unavailable | unknown

canSend: read-only indication of UI readiness, not proof a send/receive round trip succeeds.

confidence: high | medium | low

reason: machine-readable explanation, such as NO_TAB, AUTH_WALL, NO_COMPOSER, CDP_UNAVAILABLE, SELECTOR_UNVERIFIED.

No provider should be marked authenticated merely because its page loads or a composer exists.

Per-provider probing

Provider

	

Probe focus




ChatGPT

	

Signed-in navigation and composer, auth wall




Claude

	

Account UI, prompt composer




Gemini

	

Google account state, Gemini composer




Grok

	

Account UI and composer, selectors require measurement




Qwen

	

Authenticated user menu, avoid false auth-wall detection




Z.ai

	

Signed-in shell and composer




DeepSeek

	

Signed-in shell and composer

Readiness probes should operate on the already attached browser and use provider-specific selectors, with unknown as the safe fallback.

Grok adapter

Current GitHub verification: harness/src/providers/index.mjs lists six providers and does not register Grok.

Implement:

harness/src/providers/grok.mjs

Register it in:

harness/src/providers/index.mjs

The adapter follows the existing profile contract: provider ID, permitted hosts, HTTPS URL, thread pattern, composer selectors, send strategy, assistant completion selectors, auth-wall selectors and status.

Start with seeded-unverified unless the relevant behavior has been measured in the signed-in browser.

Tests belong in harness/test/providers/ and must verify both valid profile registration and conservative readiness behavior.

Existing evidence versus what is proven

I also inspected vivim-harness/docs/session/PROVIDER-READINESS-2026-10-30.json.

That earlier Studio run reports successful conversation creation for ChatGPT, Claude, Z.ai, Qwen, DeepSeek and Gemini, but several login states remain unverified. DeepSeek and Gemini first-turn sends stalled before submission.

Therefore:

Conversation creation readiness is not equivalent to authenticated, end-to-end provider capability.

The new M1 contract must preserve that distinction.

C. Repository review — what stays, what converges

I checked the current GitHub trees of both repositories. The principal structural problem remains two overlapping systems carrying mobile, Studio and orchestration responsibilities.

Area / paths

	

Finding

	

Decision




vivim-web-harness/harness/src/core/journal.mjs

	

Dedicated request journaling

	

Keep as harness authority




harness/src/interface/approval.mjs

	

Digest-bound, expiring, once-only approval

	

Keep; expose through authenticated bridge




harness/src/interface/router.mjs

	

Central request routing

	

Keep




harness/src/interface/brain.mjs

	

Model and provider selection intelligence

	

Keep, no competing phone router




harness/src/providers/*.mjs

	

Standard provider profiles

	

Extend, add Grok and readiness




harness/src/interface/server.mjs

	

Loopback-only server with /v1/ask and /v1/models

	

Keep internal; introduce separate authenticated mobile bridge




vivim-web-harness/studio/src/studio/ai-orchestrator.mjs and vivim-harness/src/studio/ai-orchestrator.mjs

	

Duplicated orchestration implementation

	

Select one owner; remove copied implementation after compatibility checks




vivim-web-harness/studio/src/studio/lan-pair.mjs and vivim-harness/src/studio/lan-pair.mjs

	

Duplicated pairing machinery

	

Consolidate QR and pairing logic




studio/src/studio/personal-agent-routes.mjs

	

Existing personal-agent integration

	

Reuse behind the host authority




studio/src/studio/ui/mobile.mjs

	

Existing mobile experience

	

Reuse and adapt for onboarding




vivim-harness/src/studio/conversation-api.mjs

	

Existing Studio conversation lifecycle

	

Retain useful observation functionality; avoid parallel send authority

These are verified overlapping paths and architectural consolidation recommendations, not claims that every file has identical current contents.

The earlier comparison identified ai-orchestrator.mjs and lan-pair.mjs as duplicated. The current trees confirm both paths still exist, but byte-level equality was not re-established in this check.

Target architecture: One execution authority, one approval system, one journal and one provider registry. Studio becomes a user-experience, capture and observation layer—not another independent execution controller.

Do not delete either Studio implementation before the replacement has passing regression tests and live receipts.

Sources: Gate 1 goals · Harness server · Approval implementation

ERA 2 — Design/review partnership and next shipment
1. Division of responsibility

The recommended arrangement is an architecture/review partner plus an autonomous implementation agent.

My responsibility

Architecture and review

Define contracts, acceptance criteria, interface boundaries, security invariants, test scenarios and migration plans.

Review submitted GitHub PRs and diffs for correctness, safety, integration and Gate 1 relevance.

Your responsibility

Implementation and testing

Implement code using the harness and your agent fleet.

Run unit, integration and real CDP/webapp tests. Produce evidence, fix defects, submit PRs and manage merging.

Concrete workflow
Goal or identified gap
Architecture contract +acceptance tests
Implementation agentbuilds smallest vertical slice
Automated tests + liveharness evidence
GitHub PR with receipts
Independent review
Targeted correction
Merge
Update milestone evidence
Blocking finding
Pass
Artifacts we exchange

Stage

	

I produce

	

Implementation agent delivers




Specification

	

SPEC-Mx.md

	

Implementation plan




Contract

	

JSON schemas, API semantics, invariants

	

Actual modules and contract tests




Build

	

Focused design answers if needed

	

PR containing code and tests




Evidence

	

Evidence checklist

	

Test output and live request IDs




Review

	

BLOCKER / MAJOR / MINOR findings

	

Corrective commits and response




Close

	

Gate assessment

	

Merged SHA and final evidence manifest

Every PR should include the relevant spec version, base/head SHA, exact tests run, tests passed/failed, whether live testing occurred, and journal request IDs where applicable.

Do not put cookies, full prompt bodies, private conversation content or browser-profile data into public PR evidence.

Cadence

One implementable objective per review cycle. The agent fleet can perform internal parallel work, but should expose one coherent, reviewable change rather than many overlapping implementations.

For each cycle:

I provide a bounded contract with pass/fail criteria.

You implement and run the tests without requiring repeated architecture approval.

You deliver the PR number, head SHA and evidence.

I review the exact revision against the contract and safety invariants.

You repair only substantiated blocking findings; merge when the required gates pass.

For straightforward non-security work, a single review is sufficient. For approval, network exposure, authentication, journal or browser-control changes, insist on an additional independent security-focused review.

How disagreements are settled

Use this precedence:

Decision hierarchy

Highest first

Explicit owner intent and GOALS.md

Existing non-negotiable safety invariants
Reproducible live evidence and regression tests
Versioned architecture contracts
Implementation preference and code simplicity

When we disagree, the reviewer must provide a reproducible failure, violated invariant, or demonstrable integration defect. The implementation agent may counter with code evidence or a test.

The same failing argument should not circulate through repeated reviews. Record the decision and its evidence once.

I can inspect GitHub and submit review comments with the available connection, but I won't independently monitor new PRs between conversations. The implementation agent should invoke the review at the appropriate gate.

2. First milestone — the smallest thing to ship
Next shipment

M1a — Read-only provider login detection

Ship one read-only command that tells us whether an existing ChatGPT browser tab appears signed in.

Not a seven-provider overhaul. Not pairing. Not a mobile redesign.

This is a thin vertical slice of M1, establishing the readiness contract that the phone will eventually consume.

Exact scope

Command:

Bash
wh providers --signed-in --json

Initially, the new probing implementation needs to cover ChatGPT. Other registered providers can return unknown / NOT_PROBED rather than claim readiness. A subsequent incremental change adds the remaining provider probes and Grok.

The command must return all registered providers, each with a stable schema, but only ChatGPT requires implemented login-state detection for this shipment.

Files to touch

File

	

Required change




harness/bin/wh.mjs

	

Parse --signed-in and --json




harness/src/interface/router.mjs

	

Expose a read-only readiness method




harness/src/providers/chatgpt.mjs

	

Add or reference conservative login-state evidence




harness/src/interface/readiness.mjs

	

New shared probe and result normalization




harness/test/interface/readiness.test.mjs

	

Contract and safety tests

Reuse existing CDP reads; do not create another browser controller.

Acceptance tests

M1a acceptance checklist

0/10 passed

Returns schema vivim.provider-readiness/1 with every registered provider represented
Signed-in ChatGPT DOM reports ready with evidence
ChatGPT login wall reports sign_in_required
Missing ChatGPT tab reports unknown / NO_TAB
CDP disconnected reports unknown / CDP_UNAVAILABLE
Ambiguous DOM reports unverified, never true
No tab creation, navigation, typing, clicks or sends
No changes to approvals, journal, send-once or target ownership
Existing provider listing remains backward compatible
Unit tests pass and real CDP read-only evidence is recorded
Copy test status
Why this before pairing?

The phone needs a trustworthy answer to a fundamental question: Which of my AI accounts can VIVIM actually use?

A working mobile interface without that information is mostly a shell.

Finishing this first also gives the setup wizard, pairing bridge, agent builder and provider picker one consistent source of truth.

It should not delay the larger goal unnecessarily: after M1a, implement the other six provider checks and the Grok registration, then move directly toward the phone-to-host vertical slice.

ERA 3 — The complete end-user journey
3. From download to “my agent completed its first task”

This is the intended product experience, based on the current GOALS.md, rather than a claim that every step works today.

The principle is simple:

The computer runs VIVIM. The phone is where the user builds and controls their personal AI.

Step 1 — Download VIVIM

User downloads one Windows host installer from VIVIM's product website. They do not need to install Node.js, configure GitHub or run terminal commands.

Target experience — M5 packaging is still incomplete.

Step 2 — Create the VIVIM browser

The host sets up a dedicated Chrome profile. VIVIM owns its automation connection without taking over the user's everyday browser.

The user can continue using their normal Chrome independently.

Step 3 — Sign in to AI webapps

User chooses ChatGPT, Claude, Gemini, Grok, Qwen, Z.ai or DeepSeek and signs into the ones they already use. VIVIM detects readiness without asking for provider API keys.

Only the accounts the user wants to use need signing in.

Step 4 — Pair the phone

The host shows a QR code. User opens the Android APK or PWA, scans and confirms pairing.

After pairing, normal onboarding moves to the phone.

Step 5 — Tell VIVIM what to become

On the phone, the user describes their goals in ordinary language: what they want help with, how much autonomy they want and which accounts may be used.

No requirement to understand agent frameworks, prompts or orchestration.

Step 6 — VIVIM bootstraps a personal agent

The existing four-stage process runs: identity, capability, authority and evidence. A default agent fleet is assembled from appropriate roles. The host maps those roles onto available webapp models.

Agent roles are configurable; the user need not manually select models.

Step 7 — Give the agent its first task

For example: “Help me research and organize a plan for a project.”

VIVIM chooses an appropriate available provider and prepares the exact request.

Simple tasks use one model. More complex tasks can use the fleet or swarm.

Step 8 — Approve the first action

The phone shows the model, target thread, request preview and permission being granted. The owner explicitly approves.

The host checks the original request and consumes approval exactly once.

Step 9 — Watch the task complete

The host submits through the dedicated signed-in browser, observes completion, and returns the result to the phone. The user sees the outcome and can inspect its provenance.

The journal ties the completed result to the original approved request.

Step 10 — Continue building their personal AI

The user adds capabilities, refines roles and grants scoped permissions from their phone. VIVIM maintains local agent state and task history, with an exportable record.

Longer-term agent personalization grows from actual use.
Gate 1 completion criteria

There are two useful measures here, and they should not be confused.

FORMAL GATE 1

Bootstrap started

A brand-new user has installed both surfaces, paired the phone, signed into selected AI webapps and started creating their personal agentic setup entirely from the phone.

STRONGER END-TO-END DEMO

First task completed

That same user approves a phone-originated task, the host executes it in the right webapp, and a completed, journaled result appears on the phone.

The second measure is the stronger product proof and the right target for the first public walkthrough, even though the repository's formal Gate 1 wording only requires the user to begin bootstrap.

Immediate implementation directive

Send to implementation agent

Copy

Implement M1a in owenservera/vivim-web-harness: add wh providers --signed-in --json with the versioned readiness schema, read-only ChatGPT signed-in probing, conservative unknown states for other providers and contract tests.Reuse the existing CDP owner. Do not create, navigate or close tabs, type, click, send, or modify approval/journal safety gates.Deliver a PR with tests, exact revision SHA and sanitized live CDP evidence. Preserve existing wh providers behavior. No provider APIs, Playwright or Puppeteer.

After M1a: Finish the remaining provider probes and Grok, wire the setup wizard, establish the secure phone bridge, then connect the existing personal-agent bootstrap to a real approved task.

The most important architectural discipline is to avoid building another independently functioning Studio, router or Chrome controller. Every additional capability should extend the same host execution authority.

---

## Turn 5: prompt

> Great, this is our plan: M1a next. Last round, structured markdown, short and decisive (decide where Owen hasn't; mark as PROPOSED):
> 1. Final user-facing glossary vs internal glossary (one line each): World, Space, Workspace, Surface, Object, Personal Agent, Fleet, Swarm, Role, Genome/DNA, CANON/Immediate/Curated, Work, SITREP, Capability, Realization, Authority, Evidence, vivim.self, vivim.mind, Aperture, Forge, Host, Harness, Studio, Bridge.
> 2. Answers to the open questions you listed in the Vivim Concept Brief (genome schema, fleet vs swarm, pairing, World objects persisted in Gate 1, memory promotion, self-evolution install path, backup format): one decision each.
> 3. Minimal Gate 1 World object model (object kinds + required fields).
> 4. Ordered milestone list from M1a to Gate 1 with one-line acceptance per milestone.

## Turn 5: ChatGPT

VIVIM — Gate 1 Product & Architecture Decisions

Status: M1a approved as next implementation milestone. Decisions not explicitly settled by Owen are marked PROPOSED. These form a working contract, subject to owner override.

ERA 1 — Final terminology

The user-facing vocabulary should be simple. Technical concepts stay internal unless the user explicitly opens developer/advanced mode.

Concept

	

User-facing language

	

Internal meaning




World

	

My World — everything I've saved, connected and created

	

Local-first personal knowledge and object graph




Space

	

Space — an area of my life or interests

	

Persistent semantic grouping of World objects




Workspace

	

Workspace — where I work on something

	

Task/project execution context with associated objects




Surface

	

View — how I experience my information

	

UI projection of World objects, not a storage silo




Object

	

Item — anything saved or created

	

Typed, identifiable, persistable World entity




Personal Agent

	

My Agent — my personal AI assistant

	

Persistent orchestrator configured around one owner




Fleet

	

My AI Team — specialists working for me

	

Persistent collection of agents with assigned roles




Swarm

	

Team Session — specialists working together on a task

	

Temporary multi-agent execution group




Role

	

Specialty — what an AI team member does

	

Capability, responsibility and authority assignment




Genome/DNA

	

Agent DNA — how my agent is configured

	

Versioned declarative identity, roles, policy and preferences




CANON / Immediate / Curated

	

Core Memory / Recent / Saved Knowledge

	

Three memory classes: authoritative, provisional and reviewed




Work

	

Task — something my AI helps me accomplish

	

Durable execution record, plan, outcomes and evidence




SITREP

	

Progress Report — where things stand

	

Structured status snapshot of work, risks and next actions




Capability

	

Ability — what my agent can do

	

Declared, verified provider/tool operation




Realization

	

Creation — something my agent produces

	

Materialized artifact, workspace or usable output




Authority

	

Permissions — what my agent may do

	

Scoped, revocable execution authorization




Evidence

	

Activity Record — what actually happened

	

Receipts, observations, request IDs and provenance




vivim.self

	

My Profile — who I am and what I prefer

	

Owner identity, preferences and personal configuration




vivim.mind

	

My Agent's Mind — what my AI knows

	

Memory, reasoning context and personalized knowledge state




Aperture

	

Discover — explore information and possibilities

	

Research/discovery subsystem




Forge

	

Build — turn ideas into working outputs

	

Realization/build execution subsystem




Host

	

VIVIM on my computer

	

Local installation owning runtime and data




Harness

	

Hidden from normal users

	

Browser execution, routing, approvals and journaling engine




Studio

	

VIVIM Studio — advanced control panel

	

UI, observation, inspection and diagnostic layer




Bridge

	

Phone Connection

	

Authenticated communication between phone and host

PROPOSED: Avoid introducing all these terms during onboarding. Present only My World, My Agent, My AI Team, Tasks, Memory and Permissions initially.

ERA 2 — Resolve open architecture decisions

Question

	

Final working decision




Genome schema

	

PROPOSED: vivim.agent-genome/1: versioned JSON containing identity, goals, role references, preferences, memory policy, capability requirements and authority references. Never embed credentials or raw memory.




Fleet vs swarm

	

PROPOSED: Fleet is persistent and reusable; swarm is temporary and created for a particular task. Swarms can select fleet members but cannot acquire additional authority.




Pairing

	

PROPOSED: QR-based, single-use challenge; host-confirmed device key; device-bound session with secure cookie; per-device revoke. LAN first; private encrypted tunnel later. No public CDP exposure.




World persistence in Gate 1

	

PROPOSED: Persist the minimum personal structure: owner profile, default Space, personal agent, genome, roles/fleet, tasks, memory records, capability snapshots and evidence references. UI Views are not independent objects.




Memory promotion

	

PROPOSED: Immediate → Curated requires evidence and user confirmation for durable personal facts; Curated → CANON always requires explicit owner confirmation. Preserve provenance and revisions; never silently promote inferred preferences into CANON.




Self-evolution install path

	

PROPOSED: Agent proposes a versioned genome revision or capability package; isolated validation and independent review follow; explicit owner approval is required before activation. Never allow autonomous changes to the harness safety core.




Backup format

	

PROPOSED: Versioned, portable JSON bundle with a manifest, object records, relationships, receipts and SHA-256 attachment references. Optional encrypted archive for private data and blobs; no browser cookies, access tokens or Chrome profiles.

Non-negotiable separations

Memory is not authority. Learning a preference cannot grant a permission.

Genome is not code execution. Changing DNA does not automatically install tools.

Capability is not authorization. A provider supporting an action does not mean the agent may perform it.

Evidence is not inference. Only observed outcomes can be recorded as verified.

World is not Studio. The data model is independent of its interface.

ERA 3 — Data model and delivery sequence
3. Minimal Gate 1 World object model

PROPOSED: One generic object envelope; specialized payloads by kind. No separate database or object framework per subsystem.

Shared object envelope
JSON

{
  "id": "obj_01...",
  "kind": "task",
  "schemaVersion": 1,
  "ownerId": "self_local",
  "spaceId": "space_default",
  "title": "My first task",
  "createdAt": "2026-10-30T17:00:00Z",
  "updatedAt": "2026-10-30T17:00:00Z",
  "revision": 1,
  "data": {},
  "relations": [],
  "evidenceIds": []
}


id, kind, schemaVersion, ownerId, timestamps, revision and data are universally required. spaceId, title, relations and evidenceIds are required where semantically applicable; otherwise they can be null or empty.

Required object kinds

Kind

	

Required data fields




self

	

displayName, preferences




space

	

name, parentSpaceId




agent

	

genomeId, fleetId, status




genome

	

version, identity, goals, memoryPolicy, authorityIds




role

	

name, purpose, capabilityIds




fleet

	

roleIds, routingPolicy




capability

	

provider, operation, support, verifiedAt




authority

	

subjectId, scope, constraints, status




task

	

instruction, status, agentId, requestIds, resultIds




memory

	

content, tier, sourceIds, promotionStatus




evidence

	

requestId, eventType, observedAt, outcome, digest




realization

	

taskId, artifactType, storageRef, digest

PROPOSED: Swarms are task-scoped execution records inside task.data, not another persistent object kind in Gate 1. Workspace, Surface, SITREP, Aperture and Forge can initially be views, modules or contextual labels over these objects.

Use immutable evidence records and revisioned changes. Sensitive records remain local; public sharing is a separate explicit operation.

4. Ordered milestones to Gate 1

Delivery plan

M1a next

M1a

	Read-only ChatGPT signed-in readiness, versioned JSON, regression and live CDP evidence.


M1b

	Other six providers including Grok registered and conservatively probed without sends.


M2a

	Host creates and launches an isolated VIVIM Chrome profile without touching everyday Chrome.


M2b

	Guided provider sign-in; host accurately displays ready, sign-in-required and unknown states.


M3a

	Phone pairs through a one-time QR challenge; reconnect and per-device revocation verified.


M3b

	Phone reads host/provider status and approves a digest-bound harness request; replay and unauthorized access rejected.


M4a

	World persists the minimal objects; identity, genome, default Space and fleet survive restart.


M4b

	Phone completes identity → capability → authority → evidence bootstrap and configures its default agent fleet.


M4c

	First approved task executes through the signed-in webapp and returns a journaled result to the phone, with no duplicate send.


M5

	Fresh Windows machine installs packaged host plus Android APK/PWA without manual runtime setup.


Gate 1

	New user independently installs, pairs and starts their personalized agent from the phone; full first-task walkthrough also passes.

Scope freeze: No cloud account dependency, public share hosting, generalized self-modifying code, multi-host synchronization or advanced World visualization is required to pass Gate 1. Those can follow without complicating onboarding.

Next action: Implement M1a against the existing harness, preserving all execution safety gates. Do not build the new World layer or pairing protocol until the readiness contract is established.
