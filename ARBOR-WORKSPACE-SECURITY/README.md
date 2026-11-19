# ARBOR Workspace Security

VIVIM Labs research and implementation design. 2026-11-19.

**My machine. My data. My AI. My authority.**

Secure the actual local-first, Windows-first, PWA-first VIVIM workspace, not merely its chat surface. Host remains the sole logical effect authority; Rust supplies native enforcement at existing boundaries. No new central router, second journal, graph, or blanket approval loop.

## Package
- [Architecture](ARCHITECTURE.md)
- [Threat model](THREAT-MODEL.md)
- [Source baseline and dispositions](SOURCE-BASELINE.md)
- [Capability contract](CAPABILITY-CONTRACT.md)
- [Implementation lanes](LANES.md)
- [Atomic topology audit protocol](TOPOLOGY-AUDIT.md)
- [Acceptance matrix](ACCEPTANCE.md)
- [Master implementation instructions](MASTER-UPGRADE.md)
- [Machine-readable manifest](manifest.json)

Design baseline: ARBOR `e07e61385a53f30890b4a7f7441fe25d22a79b1c`. This is a proposal, not a penetration test or assertion of implementation. No live credentials, personal data, machine paths, or runtime attack artifacts belong in this research package. Keep any future sensitive findings local or in a restricted repository.
