# Anticheat statistics component-state inventory

## Question

Which anticheat statistics state is entity-owned and should migrate from the plugin resource into Bevy components?

## Inspected evidence

- `servers/valence/crates/valence_server/src/anticheat.rs` before migration stored `AnticheatStatisticsState { clients: HashMap<Entity, PlayerAnticheatStatistics>, current_tick: u64 }` and sampled `PacketEvent` / `MovementEvent` readers into the entity-keyed map.
- `servers/valence/crates/valence_server/src/anticheat.rs` after migration stores per-client windows on `PlayerAnticheatStatistics` components and keeps `AnticheatStatisticsState` as plugin-global tick state only.
- Baseline and focused checks are captured in `docs/evidence/run-logs/2026-06-26/migrate-anticheat-statistics-to-components-baseline.run.log` and `docs/evidence/run-logs/2026-06-26/migrate-anticheat-statistics-to-components-focused-anticheat-first.run.log`.

## Inventory and classification

| State or boundary | Previous owner / key space | Lifecycle and cleanup risk | Consumers / accessor impact | Classification | Decision |
| --- | --- | --- | --- | --- | --- |
| `AnticheatStatisticsConfig::sample_window` | Bevy resource shared by the optional plugin | Plugin-global config, no per-entity cleanup | Read by sampling system | Global resource state | Remains a resource. |
| `AnticheatStatisticsState::current_tick` | Bevy resource owned by the optional plugin | Plugin-global tick, advanced once per sampling pass | Read by sampling system and public `current_tick()` | Global resource state | Remains a resource. |
| `AnticheatStatisticsState::clients` | Resource `HashMap<Entity, PlayerAnticheatStatistics>` keyed by client entity | Required explicit stale-key hygiene; despawn/disconnect could leave retained state | Mutated by packet/movement sampling; public `state.client(entity)` accessor exposed map reads | Entity-owned component data | Removed as retained state. Per-client windows now live on `PlayerAnticheatStatistics` components. |
| `PlayerAnticheatStatistics::{packet_cadence,movement_delta,rotation_delta}` | Values inside the entity-keyed resource map | Belongs to one live client and should disappear with that client or owning `Client` role | Read directly from component queries after migration | Entity-owned component data | Derives `Component`; initialized for added `Client` entities and removed when `Client` is removed. |
| Packet/movement event readers | Event-loop shell over `PacketEvent` and `MovementEvent` | Can observe stale entity IDs after despawn/disconnect | Emits unchanged `AnticheatStatisticsEvent` observations | Imperative shell boundary | Samples only entities with both `Client` and `PlayerAnticheatStatistics`; stale or missing ownership is ignored. |
| `AnticheatStatisticsEvent` | Bevy event emitted by optional plugin | Event buffer only, no retained state | Downstream advisory consumers | Compatibility surface | Event shape and advisory-only semantics are unchanged. |
| Disabled plugin behavior | No plugin registration | No resources, no events, no statistics components | User opt-in boundary | Compatibility surface | Plugin absence remains the disabled mode. |

## Decision

Adopt the component migration: per-client anticheat metric windows are entity-owned and now live in `PlayerAnticheatStatistics` components. Keep only plugin-global tick/config in resources. Preserve the advisory event shape and explicit opt-in behavior. This does not add enforcement, public-server safety evidence, production cheat detection, default plugin membership, broad Minecraft compatibility, or full gameplay correctness claims.

## Owner

Valence `valence_server::anticheat` optional plugin.

## Next action

Close out the Cairn change with focused anticheat tests, schedule hygiene (because plugin system wiring changed), Cairn gates, validation, task-evidence checks, and evidence manifests before archive.
