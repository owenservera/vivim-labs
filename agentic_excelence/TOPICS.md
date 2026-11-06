# Agentic excellence — topics to investigate locally

These are exploratory questions from the discussion, not a fixed design or mandated implementation.

## Tooling and environment
- What tools and model endpoints are already configured and functional in the local ZCode environment?
- How do OpenRouter and NVIDIA limits, retries, rate controls and model quality behave under demonstrated concurrency?
- What existing HQ/OpenCode/ZCode agent registries, prompts, skills and execution recipes can be reused?
- Which builds, tests and device scenarios really run locally; which require a different machine or explicit credentials/permission?

## Agents, skills and reusable assets
- Which recurring tasks benefit from specialized workers versus reusable deterministic scripts?
- What skills, adapters, templates, fixtures, tool wrappers and operational instructions are worth recording?
- What coverage, quality and reliability evidence would show that an agent/skill is genuinely useful?
- How do we capture failures, reproducible inputs, source SHAs and lessons learned without exposing private user data?

## Process and evidence
- Which local procedures are already effective for many parallel workers?
- Where do work collisions, duplicate feature implementations or conflicting reports arise?
- How can test-running, security evaluation, compatibility experiments, release checks and production-scaffold validation become regular process work?
- How should Labs communicate verified findings and candidate reusable artifacts to the authoritative product repos?

## Scope boundary
Lab work can explore and report broadly. It does not by itself authorize live provider sends, Host effects, private KB changes, Android/Windows release, production cutover or uncontrolled replacement of canonical systems.

The local team is expected to **design, experiment, verify and document** resulting workflows after inspecting the actual environment. This file deliberately does not prescribe a fleet topology, directory schema or rollout process.
