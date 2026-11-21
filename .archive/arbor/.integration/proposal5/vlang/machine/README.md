# Machine-readable VLang planning artifacts

The JSON here is a **manually compiled, source-linked planning snapshot**, not generated runtime state, an installed registry, authoritative HVI result or accepted security gate.

- `program-status.json`: intent, location, pinned report base and aggregate era counts.
- `component-register.json`: one row per assessed component, with four requested era buckets, evidence state, path, narrative and explicit test truth.
- `gate-register.json`: unresolved acceptance conditions and owners, not GO assertions.
- `source-pins.json`: direct observations vs earlier audit/historical anchors; all SHA values immutable but branch tips may move.
- `schema/program-status.schema.json`: JSON Schema of the main status manifest.

Allowed era categories: `implemented`, `predev_prepped`, `discussed_not_implemented`, `vision`. `implemented` includes existing source and explicitly marked research-only source; it never implies production acceptance. Numeric row counts are not weighted completion scores. `tests_executed_this_sitrep: false` means no executable test was performed here. Before runtime automation, create an actual evidence adapter with observed test command/exits and an independent source lock; never silently treat this snapshot as runtime telemetry.

- `vec-product-vision.json`: proposed low-LLM natural-language programming engine and separation of user programs from future development source mutation. Vision only; no runtime/acceptance claims.
