# ChatGPT Dev SITREP — 2026-11-21
This is a role snapshot, not an active coding session.
Active user-visible priorities: (1) user types and AutoWiki appears, (2) agent types and AutoWiki appears. Relevant owner request identifiers include OW-007, OW-009, OW-014 and OW-017; always check latest updates.
Historical OW-014 deployment reports ARBOR 95089a3 and 37 focused tests passing, but real Android proof UNRUN. Current GitHub main and live Host are not verified by this file.
Existing prep sources are ../DESIGN/SOURCE-DISPOSITION, ../DESIGN/PRE-CODE, ../code/agent-native-composer and Planning's bundle pointers. Next Dev action is an OW-scoped cloud-source reconciliation before implementation.

## Session-learned reliability update — 2026-11-21
The [source-to-live playbook](RELEASE-AND-INTEGRATION-PLAYBOOK.md) captures concrete engineering traps from a ChatGPT-authored ARBOR build/deployment attempt: dirty 134-behind local checkout, ZCode 429, native binary placement yielding HTTP 503, synchronous throw misasserted as async, Origin-less health probe causing rollback, and working HTTPS with unresolved Windows MagicDNS. **These are historical incidents, not a current machine check**. Future Dev sessions should use the playbook after refreshing both GitHub main refs, avoid duplicate activity/agent-composer implementations, and demand real phone evidence for the owner’s two finish lines. Owner-WORK local-only receipts are not remotely verified by this role folder.
