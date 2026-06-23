# `valence_server`

Defines the "core" of the Valence server that plugins depend on. If a plugin module here is large enough, it may be split off into its own crate to reduce compile times.

The contents of `valence_server` are re-exported from the main `valence` crate, so end users should not interact with this crate directly.

## Optional anti-cheat statistics

`valence_server::anticheat::AnticheatStatisticsPlugin` is an optional advisory plugin. It is not included in Valence's default plugins. When added explicitly, it samples Valence event streams and emits `AnticheatStatisticsEvent` observations for packet cadence, movement delta, and rotation delta metrics.

The retained data is in-memory only: a bounded rolling window plus lifetime sample counters per player entity, metric, and running plugin instance. The plugin does not persist raw packet payloads, IP addresses, account identifiers, or cross-session telemetry.

These metrics are signals, not enforcement. They can be affected by latency, server tick timing, teleports, gameplay context, and client behavior. The plugin does not kick, ban, disconnect, teleport, mutate inventory, or change gameplay by default. Any policy that consumes observations must be implemented and validated separately.

Non-claims: this is not a complete anti-cheat, not a vanilla movement legality checker, not Hyperion compatibility, not public-server safety evidence, not adversarial security evidence, and not production readiness evidence.

## Packet compose API

`packet_compose` is an opt-in API for building ordered packet bundles, resolving unicast/global/local/group route intents, and flushing a pure delivery plan through Valence clients. Planning is deterministic over explicit client snapshots, groups, exclusions, and bundle metadata; it does not read ECS state or write sockets. The direct-mode flush helper is a thin shell that writes planned bundle bytes to `Client` components and reports closed-client or backend-write failures without changing default packet-write behavior for systems that do not call compose.

Prefer direct `Client::write_packet` calls when a system already targets one client, when per-client encoding state is easiest to reason about at the write site, or when normal end-of-tick flushing should stay implicit. Prefer compose when broadcast route selection, exclusions, or future proxy-compatible route intents need separate tests. Compose does not claim proxy mode, production-scale performance, broad Minecraft compatibility, or Hyperion compatibility.

## Optional cached chunk egress

`ChunkLayer::enable_cached_chunk_egress()` opts a layer into keyed caching for chunk initialization packets. The default path remains uncached. Cache keys cover the chunk position, protocol version, dimension name/height/min-y, biome registry size, compression threshold, explicit light-input fingerprint, and a BLAKE3 content fingerprint.

Storage and network writes stay in the existing layer/client shells; the cache renderer is deterministic over explicit chunk snapshots. Missing light inputs or changed render settings fail closed by bypassing stale cached bytes. This is a repeated-egress optimization only, not a world-generation, Hyperion map-loader parity, broad chunk correctness, or production-readiness claim.
