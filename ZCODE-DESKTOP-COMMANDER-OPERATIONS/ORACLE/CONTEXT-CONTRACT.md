# Cross-session context contract
Each handoff uses a stable OW ID, receiving role, prepared-at time, repo and actual main base SHA, owned paths, dependencies, acceptance gates, evidence links, blockers, next action and acknowledgment. Only source-backed claims are promoted.

Three role views share ONE append-only work registry. Oracle audits, DevOps/Planning prepares, ChatGPT Dev implements. Sessions cannot silently see one another's live internal state; shared files require explicit refresh. Context should be incremental (role AGENTS + role SITREP + relevant task branch). Secrets and raw browser/session data remain excluded.

Cloud-first: ChatGPT develops in its cloud workspace and pushes to GitHub main; Desktop Commander is restricted to local read-only context or separately authorized post-push physical device steps. Rust first, TypeScript 7+ when sensible.