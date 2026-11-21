# ChatGPT plugin ecosystem — Desktop Commander case study

**Date:** 2026-11-21 (Europe/Madrid)  
**Status:** externally researched; Context7 queried; no live desktop or ChatGPT plugin use executed in this study.  
**Location:** VIVIM Labs → vivim → chatgpt-plugin-ecosystem  
**Ownership:** Labs research only; ARBOR owns any production Rust/MCP/Host implementation.  
**Security:** Public-safe documentation only. Never place tokens, local user data, installed profiles or tool outputs here.

## Executive decision

Desktop Commander Remote demonstrates a viable way for ChatGPT web sessions to operate on a user's computer through a paired native agent and a hosted OAuth-protected MCP relay. The free/open-source local Desktop Commander server is a different deployment mode; the hosted remote service implementation is not open source.

For VIVIM/ARBOR: **use Desktop Commander Remote as a manually approved research and operational tool, not as the product's source of effects or permissions.** ARBOR should expose a narrow, first-party, Rust-hosted, typed MCP facade that delegates all effectful operations to existing Host approval, provenance, journal, recovery and receipt contracts. Do not fork a new authority or assume filesystem directories isolate shell execution.

## Research documents

**Current ARBOR-native direction:** [Native command-space architecture and 279-intent research registry](native-command-space/README.md). This is the main design for fully integrating Desktop Commander-class capability **within ARBOR**, not embedding or depending on Desktop Commander. Includes 23-domain taxonomy, 322 seed cases, Host authority model, Rust implementation waves and 90% weighted-completion benchmark.

1. [01-DESKTOP-COMMANDER.md](01-DESKTOP-COMMANDER.md) — actual local/remote architecture, features, install, pricing, privacy and threat model.
2. [02-CONTEXT7-PULLS.md](02-CONTEXT7-PULLS.md) — 3 Context7 library resolutions + 3 documentation pulls, source pins, implications and evidence limits.
3. [03-CHATGPT-PLUGIN-PUBLISHING.md](03-CHATGPT-PLUGIN-PUBLISHING.md) — private vs workspace vs public, package layout, review requirements, desktop-only restriction, complexity and cost drivers.
4. [04-ARBOR-STRATEGY.md](04-ARBOR-STRATEGY.md) — Rust-first end-state architecture, trust boundaries, contract proposals and phased recommendation.
5. [05-VALIDATION-PLAYBOOK.md](05-VALIDATION-PLAYBOOK.md) — owner-local setup, safe probe plan, review and publishing readiness gates.

## Three eras

- **Era 1 / Understand:** verify Desktop Commander Remote behavior with read-only prompts; distinguish remote relay and local stdio.
- **Era 2 / Build:** make a bounded ARBOR Rust MCP service with read-only capabilities and typed, approval-gated writes.
- **Era 3 / Publish:** private trial first; public remote HTTPS MCP plugin only after security, OAuth, reviewer account, annotations, identity and directory review. Desktop-only local publishing requires separate partner approval.

## References (official primary sources)

- https://github.com/desktop-commander/remote-desktop-commander
- https://github.com/wonderwhy-er/DesktopCommanderMCP
- https://desktopcommander.app/help/getting-started/what-is-desktop-commander
- https://desktopcommander.app/help/security-privacy/use-safely-limit-access
- https://desktopcommander.app/help/security-privacy/data-privacy
- https://desktopcommander.app/pricing/
- https://developers.openai.com/plugins
- https://developers.openai.com/plugins/build/plugins
- https://developers.openai.com/plugins/deploy/submission
- https://developers.openai.com/plugins/deploy/submission-errors
- https://developers.openai.com/plugins/deploy/desktop-only
- https://developers.openai.com/plugins/plugin-guidelines

See [existing VIVIM plugin architecture](../PLUGIN-SYSTEM-ARCHITECTURE.md). This notebook does not supersede that proposal or assert implementation in ARBOR.
