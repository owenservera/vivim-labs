# Atomic host transition measurement and provider accounting

Baseline 0a3b982e4e3b26f78504e0b99f513bbd7ab22f7a. E42 source closure.

A whitelisted host command can commit its result and pre-persistence transition wall-time receipt in one indexed revision. Journal record anchors the receipt; readback verifies identity and response digest against the world. Replay returns the original result without remeasurement or extra revision. Legacy metering remains readable; transition-* operation labels distinguish the new scope. Failed/uncertain commands and lock acquisition, store load and persistence latency are not measured by this path.

Resource report distinguishes native-attempt counter coverage from routed attempts. Population report no longer substitutes logical timestamps for missing measured elapsed. Provider accounting separates confirmed real and synthetic raw usage, attributable verified yield, missing raw receipt IDs and charge-only receipts. Lifetime actual and live holds reported by each original pool; overlapping pools must never be added. Checked arithmetic returns null on overflow. No conversion between currencies or token types.

Authored replay/negative fixture. No compilation, test or provider execution. E42 remains PARTIAL_SOURCE. Linux OS counters and total end-to-end/failed-attempt accounting remain, as does empirical qualification. CLI: kernel <store> provider-accounting.
