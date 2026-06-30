# Hyperion gameplay composition inventory

Date: 2026-06-30
Owner: Cairn drain for Hyperion game-mode composability changes

## Scope

This inventory covers the Bedwars event crate composition seam introduced for the active Hyperion game-mode Cairns. The implemented shared boundary is a module boundary in `hyperion/events/bedwars/src/gameplay.rs`, backed by pure composition validation in `hyperion/crates/hyperion-game-modes/src/composition.rs`.

The boundary is intentionally not a separate gameplay crate yet. Several feature modules still live in the Bedwars event crate and are exposed through compatibility re-exports because some mechanics still reference Bedwars-local types such as `Team` or command handlers. Moving those modules to a separate crate remains a non-claim until those dependencies are split.

## Default gameplay inventory

| Feature | Public path | Classification | Dependencies / ownership notes |
| --- | --- | --- | --- |
| attack | `bedwars::gameplay::features::AttackPlugin` | common default, Bedwars-local implementation | Requires `HyperionCore`; installs attack observers and melee systems; consumes inventory, combat stats, packet, spatial, health, `Compose`, and optional `Team` state. |
| block | `bedwars::gameplay::features::BlockPlugin` | common default | Requires `HyperionCore`; handles block interaction systems around ingress decode and block storage. |
| bow | `bedwars::gameplay::features::BowPlugin` | common default | Requires `HyperionCore`; owns `LastFireTime` and `BowCharging` player components. |
| chat | `bedwars::gameplay::features::ChatPlugin` | common default, Bedwars-local optional team formatting | Requires `HyperionCore`; owns `ChatCooldown`; uses optional `Team` for display color. |
| damage | `bedwars::gameplay::features::DamagePlugin` | common default | Requires `HyperionCore`; consumes `HitGroundEvent`, `Health`, `Position`, `ConnectionId`, and `Compose`. |
| regeneration | `bedwars::gameplay::features::RegenerationPlugin` | common default | Requires `HyperionCore`; owns `LastDamaged` player component. |
| skin | `bedwars::gameplay::features::SkinPlugin` | common default | Requires `HyperionCore`; propagates skin/profile update state. |
| spawn | `bedwars::gameplay::features::SpawnPlugin` | common default | Requires `HyperionCore`; owns spawn placement helpers and observers. |
| stats | `bedwars::gameplay::features::StatsPlugin` | common default | Requires `HyperionCore`; displays tick/player stats. |
| vanish | `bedwars::gameplay::features::VanishPlugin` | common default | Requires `HyperionCore`; owns `Vanished` state used by commands. |
| clap-command | `bedwars::gameplay::features::ClapCommandPlugin` | shared infrastructure | Requires Hyperion command root from `HyperionCore`; installs command registry handling and permission command registration. |
| genmap | `bedwars::gameplay::features::GenMapPlugin` | shared infrastructure | Installs default map generation resources/systems from `hyperion-genmap`; runtime resource needs are owned by that plugin. |
| item | `bedwars::gameplay::features::ItemPlugin` | shared infrastructure | Requires `HyperionCore`; installs item behavior from `hyperion-item`. |
| permission | `bedwars::gameplay::features::PermissionPlugin` | shared infrastructure | Requires `HyperionCore` and `LocalDb`; owns permission storage and group initialization. |
| proxy | `bedwars::gameplay::features::HyperionProxyPlugin` | shared infrastructure | Requires `HyperionCore` for live proxy operation; owns proxy event/observer setup. |
| command-registration | `bedwars::gameplay::features::CommandRegistrationPlugin` | command wiring | Requires `HyperionCore` and `ClapCommandPlugin`; registers Bedwars commands after command registry resources exist. |

The machine-readable mirror for this table is `DEFAULT_GAMEPLAY_INVENTORY` in `hyperion/events/bedwars/src/gameplay.rs`; tests assert that it stays in the same order as `hyperion_game_modes::composition::DEFAULT_GAMEPLAY_FEATURES`.

## Mode-only versus preset responsibilities

Mode plugins now own only exclusive mode identity and mode-local player initialization:

| Plugin | Mode identity | Player marker/setup | Shared gameplay ownership |
| --- | --- | --- | --- |
| `BedwarsPlugin` | `ActiveGameType(GameType::Bedwars)` | inserts `Spatial`, `Team::Red`, and `BedwarsPlayer` on play-state add | none |
| `DayzPlugin` | `ActiveGameType(GameType::Dayz)` | inserts `Spatial` and `DayzSurvivor` | none |
| `HardcoreFactionsPlugin` | `ActiveGameType(GameType::HardcoreFactions)` | inserts `Spatial` and `HardcoreFactionsPlayer` | none |

`GamePreset` and the existing app builders own full default app construction: proxy/crypto resources, `HyperionCore`, optional `DefaultGameplayPlugins`, and exactly one selected mode plugin. Existing `build_game_app_with_type_and_proxy` and default wrappers call the preset shell so Bedwars, Dayz, and HardcoreFactions keep the same default behavior unless a caller explicitly disables default gameplay.

## Pure composition diagnostics

The pure core in `hyperion-game-modes::composition` validates these inputs before Bevy app mutation:

- exactly one exclusive mode (`Bedwars`, `Dayz`, or `HardcoreFactions`);
- duplicate mode and multi-mode conflicts;
- duplicate disabled/replacement feature requests;
- feature toggles without default gameplay;
- replacement requests for infrastructure features such as proxy, permission, map, or command registration;
- command registration dependency on `ClapCommandPlugin`;
- duplicate custom plugin intents.

The Bevy shell maps a validated `GamePreset` into app mutation. Replacement feature intents currently disable the original default feature so callers can add their replacement plugin through normal Bevy APIs; type-erased custom plugin names remain diagnostics/intents, not runtime plugin instances.

## Exclusive-mode and marker behavior

Directly adding multiple mode plugins now fails before ambiguous app state is produced. `insert_active_game_type` rejects a second exclusive mode with a deterministic panic containing `exclusive Hyperion game mode conflict`.

Mode marker/run-condition surfaces added in `lib.rs`:

- `BedwarsPlayer`, `DayzSurvivor`, `HardcoreFactionsPlayer` marker components;
- `active_game_type_matches` pure helper;
- `bedwars_mode_active`, `dayz_mode_active`, and `hardcore_factions_mode_active` Bevy run-condition helpers.

No scoped temporary mode state needed cleanup in this pass. The cleanup decision is a non-action because the touched mode-local setup only inserts marker components at play-state initialization and does not allocate arena/session state.

## Plugin `Component` derive inventory

Before this pass, `BedwarsPlugin`, `DayzPlugin`, and `HardcoreFactionsPlugin` derived `Component` even though they were only Bevy plugin types and were not inserted or queried as ECS entity state. Those derives were removed. Real ECS state remains on marker/state components such as `BedwarsPlayer`, `DayzSurvivor`, `HardcoreFactionsPlayer`, `Team`, `ChatCooldown`, `BowCharging`, `LastFireTime`, `ImmuneUntil`, `CombatStats`, `LastDamaged`, and `Vanished`.

## Test matrix

| Coverage | Positive checks | Negative checks |
| --- | --- | --- |
| Pure preset core | one mode plus disabled/replacement/custom intents builds a `PresetPlan` | missing mode, duplicate mode, multiple modes, duplicate features, duplicate custom plugin, missing dependency, unsupported replacement |
| Public gameplay group | default group installs public feature handles and Bedwars commands | missing `ClapCommandPlugin`, duplicate public feature without disable, disabled Bow feature absent |
| Mode-only plugins | direct `BedwarsPlugin` sets only active mode and no shared gameplay | adding `DayzPlugin` after `BedwarsPlugin` rejects exclusive-mode conflict |
| Preset shell | default builders still install default gameplay and selected mode | mode-only preset disables shared gameplay; invalid preset is rejected before app mutation |
| Public API | integration test imports `bedwars::gameplay::features::*` from outside the crate | private `bedwars::plugin::*` path is documented as `compile_fail`; disabled feature path removes Bow |

## Non-claims and follow-up boundaries

This pass does not claim a separate shared gameplay crate, full extraction of Bedwars-local mechanics, multi-world/multi-mode stacking, broad Minecraft compatibility, vanilla parity, production readiness, or live gameplay correctness. It establishes the composition seam, pure diagnostics, focused tests, and reviewable inventories needed for later extraction once Bedwars-specific coupling is split.
