# `valence_server`

Defines the "core" of the Valence server that plugins depend on. If a plugin module here is large enough, it may be split off into its own crate to reduce compile times.

The contents of `valence_server` are re-exported from the main `valence` crate, so end users should not interact with this crate directly.

## Optional anti-cheat statistics

`valence_server::anticheat::AnticheatStatisticsPlugin` is an optional advisory plugin. It is not included in Valence's default plugins. When added explicitly, it samples Valence event streams and emits `AnticheatStatisticsEvent` observations for packet cadence, movement delta, and rotation delta metrics.

The retained data is in-memory only: a bounded rolling window plus lifetime sample counters per player entity, metric, and running plugin instance. The plugin does not persist raw packet payloads, IP addresses, account identifiers, or cross-session telemetry.

These metrics are signals, not enforcement. They can be affected by latency, server tick timing, teleports, gameplay context, and client behavior. The plugin does not kick, ban, disconnect, teleport, mutate inventory, or change gameplay by default. Any policy that consumes observations must be implemented and validated separately.

Non-claims: this is not a complete anti-cheat, not a vanilla movement legality checker, not Hyperion compatibility, not public-server safety evidence, not adversarial security evidence, and not production readiness evidence.
