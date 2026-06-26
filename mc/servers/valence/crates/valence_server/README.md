# `valence_server`

Defines the "core" of the Valence server that plugins depend on. If a plugin module here is large enough, it may be split off into its own crate to reduce compile times.

The contents of `valence_server` are re-exported from the main `valence` crate, so end users should not interact with this crate directly.

## Optional anti-cheat statistics

`valence_server::anticheat::AnticheatStatisticsPlugin` is an optional advisory plugin. It is not included in Valence's default plugins. When added explicitly, it samples Valence event streams and emits `AnticheatStatisticsEvent` observations for packet cadence, movement delta, and rotation delta metrics.

The retained data is in-memory only: a bounded rolling window plus lifetime sample counters per live client entity, metric, and running plugin instance. Per-client windows are stored on `PlayerAnticheatStatistics` components attached to live `Client` entities; `AnticheatStatisticsState` retains only plugin-global tick state. Query the component directly instead of reading a resource-owned entity map. The plugin does not persist raw packet payloads, IP addresses, account identifiers, or cross-session telemetry.

These metrics are signals, not enforcement. They can be affected by latency, server tick timing, teleports, gameplay context, and client behavior. The plugin does not kick, ban, disconnect, teleport, mutate inventory, or change gameplay by default. Any policy that consumes observations must be implemented and validated separately.

Non-claims: this is not a complete anti-cheat, not a vanilla movement legality checker, not Hyperion compatibility, not public-server safety evidence, not adversarial security evidence, and not production readiness evidence.

## Optional observability hooks

`valence_server::observability::ObservabilityPlugin` is an optional plugin for bounded tick-phase and serverbound packet observations. It is not included in Valence default plugins. Add it explicitly when a server wants `ObservabilityEvent` records.

The run-condition inventory for this plugin is:

| Targeted system | Runtime config | Event reader | Disabled contract | Runtime toggle behavior |
| --- | --- | --- | --- | --- |
| `emit_pre_update_phase`, `emit_event_loop_pre_update_phase`, `emit_event_loop_update_phase`, `emit_event_loop_post_update_phase`, `emit_post_update_phase` | `ObservabilityConfig::enabled && ObservabilityConfig::emit_tick_phases` | None | Skip with Bevy `run_if`; no system body runs while disabled. | Re-enabling resumes future tick-phase records only. |
| `emit_network_packet_records` | `ObservabilityConfig::enabled && ObservabilityConfig::emit_network_packets` | `PacketEvent` | Explicit in-system guard and drain; disabled updates clear the reader and emit no observability records. | Re-enabling observes new packets only; disabled-period packets are not replayed. |

`AnticheatStatisticsPlugin` was inspected as an optional event-reading plugin, but it has no runtime enabled switch: plugin absence is the disabled mode, and adding a runtime run condition is outside this observability change. Cached chunk egress was also inspected and is not targeted because its enable flag is per-layer packet rendering state, not a Bevy system with an event reader.

Non-claims: observability records are local advisory hooks. They are not gameplay correctness evidence, broad Minecraft compatibility evidence, public-server safety evidence, or production readiness evidence.

## Packet compose API

`packet_compose` is an opt-in API for building ordered packet bundles, resolving unicast/global/local/group route intents, and flushing a pure delivery plan through Valence clients. Planning is deterministic over explicit client snapshots, groups, exclusions, and bundle metadata; it does not read ECS state or write sockets. The direct-mode flush helper is a thin shell that writes planned bundle bytes to `Client` components and reports closed-client or backend-write failures without changing default packet-write behavior for systems that do not call compose.

Prefer direct `Client::write_packet` calls when a system already targets one client, when per-client encoding state is easiest to reason about at the write site, or when normal end-of-tick flushing should stay implicit. Prefer compose when broadcast route selection, exclusions, or future proxy-compatible route intents need separate tests. Compose does not claim proxy mode, production-scale performance, broad Minecraft compatibility, or Hyperion compatibility.

## Optional tick scheduler

`tick_scheduler::TickScheduler<K, V>` is a deterministic utility for work keyed by explicit ticks or other ordered keys. The core is independent of Bevy, wall-clock time, async tasks, I/O, and global server state: callers schedule values, inspect the earliest item, drain due work up to an explicit key, cancel by handle, or clear the queue. Equal-key work drains in stable insertion order.

`tick_scheduler::TickSchedulerPlugin<E>` is an opt-in shell for Valence apps that want scheduled values emitted as Bevy events. It registers `tick_scheduler::ServerTickScheduler<E>` and drains it against `Server::current_tick()` only when the plugin is added by user code. It is not part of `DefaultPlugins`, so existing gameplay has no timer behavior by default.

Typical uses are gameplay policy owned by the caller: cooldown expiry events, delayed despawn requests, or temporary block restoration events. The scheduler does not choose durations, mutate entities, restore blocks, retry failed work, run async jobs, or claim vanilla timing parity.

## Optional cached chunk egress

`ChunkLayer::enable_cached_chunk_egress()` opts a layer into keyed caching for chunk initialization packets. The default path remains uncached. Cache keys cover the chunk position, protocol version, dimension name/height/min-y, biome registry size, compression threshold, explicit light-input fingerprint, and a BLAKE3 content fingerprint.

Storage and network writes stay in the existing layer/client shells; the cache renderer is deterministic over explicit chunk snapshots. Missing light inputs or changed render settings fail closed by bypassing stale cached bytes. This is a repeated-egress optimization only, not a world-generation, Hyperion map-loader parity, broad chunk correctness, or production-readiness claim.

## Typed packet-derived gameplay events

`action::PlayerActionEvent` is the selected typed event promoted from a raw `PacketEvent` boundary in this pass.
The adapter is intentionally narrow: raw `PacketEvent` remains public for low-level protocol users and unsupported packet
semantics.

### Inventory

| Selected semantic | Previous raw reader | Packet type | Event-loop phase | Mutation target | Emitted gameplay semantic | Previous malformed/stale behavior |
| --- | --- | --- | --- | --- | --- | --- |
| Player block/action requests | `action::handle_player_action` | `PlayerActionC2s` | `EventLoopPreUpdate` | `ActionSequence` | `DiggingEvent` for start/abort/stop destroy actions | Wrong IDs, decode errors, and partial decodes returned no decoded packet through `PacketEvent::decode`; missing `ActionSequence` skipped sequence tracking but could still emit a digging event. |

### Contract

`action::emit_player_action_events` owns decoding `PlayerActionC2s` during `EventLoopPreUpdate` and emits exactly one
`PlayerActionEvent` for each valid packet from a live client with an `ActionSequence` component. The event carries the
source client entity, packet arrival `Instant`, decoded action, block position, direction, and synchronization sequence.
Wrong packet IDs, decode failures, partial decodes, malformed action payloads, and stale or non-client entities emit no
`PlayerActionEvent`, no `DiggingEvent`, and no typed rejection event; decode diagnostics remain the existing
`PacketEvent::decode` tracing warnings when decoding is attempted. `action::handle_player_action` consumes the typed
event, updates `ActionSequence`, and maps block-destroy actions to `DiggingEvent`, so downstream gameplay no longer
needs to decode the raw packet body for this semantic.

Non-claims: this adapter does not promote every serverbound gameplay packet, does not remove direct packet access, and
does not claim broad Minecraft compatibility, vanilla semantic equivalence, public-server safety, or production
readiness.
