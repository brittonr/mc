# Add core plugin SystemSet contracts inventory

## Scope

This evidence note supports `add-core-plugin-system-set-contracts` and the requirements `r[valence_bevy_ecs.core_plugin_sets.inventory]`, `r[valence_bevy_ecs.core_plugin_sets.contract]`, `r[valence_bevy_ecs.core_plugin_sets.wiring]`, `r[valence_bevy_ecs.core_plugin_sets.compatibility]`, `r[valence_bevy_ecs.core_plugin_sets.tests]`, and `r[valence_bevy_ecs.core_plugin_sets.validation]`.

Selected plugins are `valence_command`, `valence_equipment`, `valence_advancement`, `valence_scoreboard`, `valence_weather`, `valence_world_border`, and `valence_boss_bar`. All selected plugins are optional Valence features that are enabled by the root `valence` default feature list and added by `DefaultPlugins` behind matching `#[cfg(feature = "...")]` gates. This change does not add, remove, or reorder default plugin membership.

## Selected plugin inventory

| Plugin | Feature/default membership | Schedule systems and promoted sets | Resources and events | Downstream ordering assumptions |
| --- | --- | --- | --- | --- |
| `valence_command::manager::CommandPlugin` | `command` default feature; `DefaultPlugins` adds `CommandPlugin` behind `#[cfg(feature = "command")]`. | `PreUpdate`: `insert_scope_component.after(SpawnClientsSet)` remains a private initialization point. `EventLoopPreUpdate`: `update_command_tree` and `command_tree_update_with_client` are in `CommandTreeSet`; `read_incoming_packets.before(CommandSystemSet)` remains packet-input plumbing; `parse_incoming_commands` is in `CommandSystemSet`. `CommandHandlerPlugin<T>` keeps `PostStartup` graph assembly and `EventLoopPreUpdate` handler dispatch after `CommandSystemSet`. | Inserts `CommandRegistry`; `CommandScopePlugin` initializes `CommandScopeRegistry`; adds `CommandExecutionEvent` and `CommandProcessedEvent`; each command handler adds `CommandResultEvent<T>` and `CommandResource<T>`. | `examples/command.rs` and downstream command handlers can order around command tree synchronization or parse/dispatch without relying on anonymous tuple membership. Packet decode internals and handler-specific result events remain private to command plumbing. |
| `valence_advancement::AdvancementPlugin` | `advancement` default feature; `DefaultPlugins` adds `AdvancementPlugin` behind `#[cfg(feature = "advancement")]`. | `PreUpdate`: `add_advancement_update_component_to_new_clients.after(SpawnClientsSet)` is in `InitAdvancementClientsSet`; `event::handle_advancement_tab_change` is in `ReadAdvancementTabsSet`. `PostUpdate`: `systems::update_advancement_cached_bytes` is in `WriteAdvancementToCacheSet`; `systems::send_advancement_update_packet` is in `WriteAdvancementPacketToClientsSet`; cache writes remain before packet writes and packet writes remain before `FlushPacketsSet`. | Adds `AdvancementTabChangeEvent`; uses advancement components and `AdvancementClientUpdate`; no plugin-owned resource is inserted. | Advancement examples and plugins can order before or after client initialization, tab input, cache rebuild, or packet-write phases. Byte encoding internals remain private. |
| `valence_equipment::EquipmentPlugin` | `equipment` default feature; `DefaultPlugins` adds `EquipmentPlugin` behind `#[cfg(feature = "equipment")]`. | `PreUpdate`: `on_entity_init` is in `EquipmentInitSet`; interaction start/stop and `inventory_sync::held_item_from_client` are in `EquipmentInputSet`; `inventory_sync::on_attach` and `inventory_sync::run` are in `EquipmentSyncSet`. `PostUpdate`: `update_equipment` and `on_entity_load` are in `EquipmentBroadcastSet` before `FlushPacketsSet`. | Adds `EquipmentChangeEvent`; uses `Equipment`, `EquipmentInventorySync`, inventory components/events, living entity flags, load events, layer/entity IDs, and packet writers; no plugin-owned resource is inserted. | `examples/equipment.rs` and downstream equipment integrations can order around initialization, input, inventory sync, and visible broadcast phases. Existing same-tick equipment-vs-inventory priority stays in the pure sync helper logic. |
| `valence_scoreboard::ScoreboardPlugin` | `scoreboard` default feature; `DefaultPlugins` adds `ScoreboardPlugin` behind `#[cfg(feature = "scoreboard")]`. | `PostUpdate`: objective create/update, display, despawn cleanup, new-client visibility, and score updates remain in `ScoreboardSet` before `UpdateLayersPreClientSet`. Existing local `after` edges keep display and score updates behind their required producers. | Uses scoreboard components and layer/client packet writers; no plugin-owned event or resource is inserted. | Scoreboard users can order before or after the scoreboard output phase. Internal objective/display/score sequencing remains system-local instead of promoted as public sets. |
| `valence_weather::WeatherPlugin` | `weather` default feature; `DefaultPlugins` adds `WeatherPlugin` behind `#[cfg(feature = "weather")]`. | `PostUpdate`: client direct writes (`init_weather_on_layer_join`, `change_client_rain_level`, `change_client_thunder_level`) are in `WeatherClientUpdateSet` before `FlushPacketsSet`; layer writes (`change_layer_rain_level`, `change_layer_thunder_level`) are in `WeatherLayerUpdateSet` before `UpdateClientsSet`. | Uses `Rain`, `Thunder`, `WeatherBundle`, client/layer visibility components, and packet writers; no plugin-owned event or resource is inserted. | Weather users can order around client-direct and layer-broadcast weather output phases without assuming an order between those two independent paths beyond the existing client/layer constraints. |
| `valence_world_border::WorldBorderPlugin` | `world_border` default feature; `DefaultPlugins` adds `WorldBorderPlugin` behind `#[cfg(feature = "world_border")]`. | `PostUpdate`: new-client initialization, lerp ticking, center/warning/portal-boundary writes remain in `UpdateWorldBorderSet` before `UpdateClientsSet`. | Uses world-border components plus `Server` tick-rate resource, chunk layer, visible layer, and packet writers; no plugin-owned event or resource is inserted. | `examples/world_border.rs` and downstream world-border users can order around the world-border update phase. Packet-specific write helpers remain private. |
| `valence_boss_bar::BossBarPlugin` | `boss_bar` default feature; `DefaultPlugins` adds `BossBarPlugin` behind `#[cfg(feature = "boss_bar")]`. | `PostUpdate`: component change writes, layer-view updates, chunk-view updates, and despawn removal are in `BossBarUpdateSet` before `UpdateLayersPreClientSet`. | Uses boss-bar components, layer/client visibility components, entity layer IDs, positions, and packet writers; no plugin-owned event or resource is inserted. | `examples/boss_bar.rs` and downstream boss-bar users can order before or after the boss-bar update phase. Per-component packet action mapping remains private. |

## Contract classification

Promoted contracts are phase-level sets only. The change intentionally does not expose one set per system, and it keeps local `before`/`after` constraints private when they are only needed to preserve internal producer/consumer order.

| Contract | Schedule | Visibility | Reason |
| --- | --- | --- | --- |
| `CommandTreeSet` | `EventLoopPreUpdate` | Public | Stable boundary for command tree refresh before clients rely on command graph/scope visibility. |
| `CommandSystemSet` | `EventLoopPreUpdate` | Existing public | Existing parse/dispatch boundary kept compatible for command handlers. |
| `InitAdvancementClientsSet` | `PreUpdate` | Public | Stable point after client spawn where advancement client state is attached. |
| `ReadAdvancementTabsSet` | `PreUpdate` | Public | Stable point for translating advancement tab packets into typed events. |
| `WriteAdvancementToCacheSet` | `PostUpdate` | Existing public | Existing cache rebuild phase retained before packet writes. |
| `WriteAdvancementPacketToClientsSet` | `PostUpdate` | Existing public | Existing packet-write phase retained before flush. |
| `EquipmentInitSet` | `PreUpdate` | Public | Stable point for inserting default equipment on living entities. |
| `EquipmentInputSet` | `PreUpdate` | Public | Stable point for equipment-owned packet/event input handling. |
| `EquipmentSyncSet` | `PreUpdate` | Public | Stable point for inventory/equipment synchronization. |
| `EquipmentBroadcastSet` | `PostUpdate` | Public | Stable point for visible equipment packet/event broadcast before packet flush. |
| `ScoreboardSet` | `PostUpdate` | Existing public | Single scoreboard output phase is sufficient; finer objective/display/score internals remain private. |
| `WeatherClientUpdateSet` | `PostUpdate` | Public | Stable point for client-direct weather packets before packet flush. |
| `WeatherLayerUpdateSet` | `PostUpdate` | Public | Stable point for layer weather packets before client layer updates. |
| `UpdateWorldBorderSet` | `PostUpdate` | Existing public | Existing world-border update phase retained. |
| `BossBarUpdateSet` | `PostUpdate` | Public | Stable point for boss-bar packet output before layer-to-client updates. |

## Preserved behavior and non-claims

- Event names, resource names, component names, feature gates, and `DefaultPlugins` membership are unchanged.
- Existing packet writers, entity/layer queries, command parsing, advancement cache encoding, scoreboard diffing, weather values, world-border lerp math, and boss-bar packet action helpers are not behavior changes.
- Anonymous or private internal ordering remains intentionally private where a downstream phase-level contract is enough: command packet decode before parse, command handler dispatch after parse, scoreboard objective/display/score local edges, advancement byte encoding helpers, equipment sync helper priority, weather packet helper details, world-border packet helper details, and boss-bar component-to-packet mappings.
- No broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claim is added.

## Verification hooks

- `servers/valence/src/tests/core_plugin_sets.rs` has a positive default-without-network schedule smoke test and negative disabled-plugin tests for all selected plugins.
- `tools/check_valence_schedule_hygiene.rs --root .` now validates the promoted set tokens in source and the maintained schedule inventory.
- Focused logs for baseline, wiring, schedule tests, schedule hygiene, Cairn gates, Cairn validation, and task evidence are promoted under `docs/evidence/`.
