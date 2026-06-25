# Valence gameplay example plugin organization inventory

## Question

What Bevy app wiring in `servers/valence/examples/{ctf.rs,survival_compat.rs,terrain.rs}` must move behind opt-in example plugins for Cairn change `organize-valence-gameplay-examples-as-bevy-plugins`?

## Requirement IDs and dependency chain

- r[valence_bevy_ecs.gameplay_plugins.inventory] records current systems, schedules, resources, events, env toggles, compatibility milestones, and non-goals before refactoring.
- r[valence_bevy_ecs.gameplay_plugins.contract] defines named input, rule-evaluation, world-mutation, presentation, and cleanup `SystemSet` contracts.
- r[valence_bevy_ecs.gameplay_plugins.wiring] moves example wiring into opt-in plugins while pure decisions stay outside ECS shell code.
- r[valence_bevy_ecs.gameplay_plugins.compatibility] preserves commands, env vars, milestone text, selected scenario behavior, and non-claim boundaries.
- r[valence_bevy_ecs.gameplay_plugins.tests] adds positive plugin/schedule smoke tests and negative disabled-plugin or ordering regression tests.
- r[valence_bevy_ecs.gameplay_plugins.validation] records focused checks, Cairn gates, Cairn validation, and task-evidence validation.

Dependency chain: inventory -> contract -> wiring -> compatibility/tests -> validation.

Owner subtree: `servers/valence/examples/` plus review evidence under `docs/evidence/`. No Hyperion code or concepts were adopted, ported, referenced, or rejected for this change.

## Acceptance criteria

- `ctf`, `survival_compat`, and `terrain` keep their existing example command names and remain opt-in binaries.
- Example `main` functions delegate gameplay/fixture wiring to example-local plugins; `DefaultPlugins` and Valence core plugin groups remain unchanged.
- Plugins expose named Bevy sets for input, rule evaluation, world mutation, presentation, and cleanup ordering.
- Fixture decisions already in `fixture_core::{ctf,survival}` and terrain pure generation helpers stay pure; plugin code remains ECS orchestration.
- Env var names and `MC-COMPAT-MILESTONE` text remain unchanged.
- Positive tests prove plugin contracts/resources/schedules install; negative tests prove disabled plugins do not install plugin-owned resources and ordering regressions are rejected.
- Non-claims remain explicit: no broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, terrain production worldgen, or vanilla parity.

## Current CTF app wiring

| Class | Current items | Refactor boundary |
| --- | --- | --- |
| Command | `cargo run --example ctf` / existing mc-compat Valence backend command shape. | Preserve command; `main` still owns pre-`DefaultPlugins` network settings and then adds `CtfGameplayPlugin`. |
| Schedules | `Startup`: `setup`; `EventLoopUpdate`: `handle_combat_events`, `handle_projectile_events`; `Update`: `init_clients`, `despawn_disconnected_ctf_clients`, `digging`, `place_blocks`, `do_team_selector_portals`, inventory loggers, flag visuals/capturing, clone update, OOB teleport, necromancy, scoreboard update. | Plugin registers systems into named CTF gameplay phase sets. |
| Resources inserted by app/setup | `NetworkSettings` offline, `ArrowPolicyState`, `CtfGlobals`, `FlagManager`, `CtfLayers`, `Score`, `WinConditionState`, `ReconnectJoinCounts`, `CtfRaceProbeState`, `CtfSpawnTeamResetProbeState`, `Portals`. | `NetworkSettings` remains pre-default shell because `NetworkPlugin` consumes it during build; plugin owns `ArrowPolicyState`, phase contract, and system wiring; setup still creates world resources. |
| Events/readers | `DiggingEvent`, `InteractBlockEvent`, `UpdateSelectedSlotEvent`, `DropItemStackEvent`, `ClickSlotEvent`, `HandSwingEvent`, `InteractItemEvent`, `RequestRespawnEvent`, removed `Client` components. | Event readers remain thin ECS adapters. |
| Env toggles | `MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE`, `MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE`, `MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE`, `MC_COMPAT_VANILLA_COMBAT_ARMOR_REFERENCE_PROBE`, `MC_COMPAT_STEEL_CONFIG`, `MC_COMPAT_STEEL_RELOAD_REQUEST`, `MC_COMPAT_CTF_SCORE_LIMIT_PROBE`, `MC_COMPAT_CTF_RACE_PROBE`, `MC_COMPAT_CTF_SPAWN_TEAM_RESET_PROBE`, `MC_COMPAT_CTF_INVALID_RETURN_DROP_PROBE`, `MC_COMPAT_CTF_INVALID_OPPONENT_BASE_RETURN_DROP_PROBE`, projectile/equipment/armor probes already used by helpers. | Preserve names and enabled semantics. |
| Milestone emitters | Existing `MC-COMPAT-MILESTONE` strings for flags, score limit, race, spawn reset, inventory, combat, projectile, armor, equipment, reconnect, and Steel policy reload. | Do not change text. |
| Pure decisions | `fixture_core::ctf` helpers plus arrow policy validation/evaluation helpers and terrain-independent predicates. | Stay outside Bevy queries/commands. |
| Non-goals | Full CTF correctness, broad combat/vanilla parity, production readiness, and public-server safety. | Preserve as non-claims. |

## Current survival compatibility app wiring

| Class | Current items | Refactor boundary |
| --- | --- | --- |
| Command | `cargo run --example survival_compat` and mc-compat survival fixture backend command shape. | Preserve command; `main` still owns pre-`DefaultPlugins` offline network settings and then adds `SurvivalCompatibilityPlugin`. |
| Schedules | `Startup`: `setup`; `EventLoopPreUpdate`: `handle_survival_chest_close`; `Update`: `init_clients`, `despawn_disconnected_clients`, block break/place, redstone, persistence, chest/crafting/furnace open/store/click, hunger, mob-drop attack and pickup advancement. | Plugin registers systems into named survival gameplay phase sets. |
| Resources inserted by setup | Optional fixture resources for chest, crafting, crafting breadth, furnace, hunger, mob drop, redstone, world persistence, and block entity fixtures. | Setup still owns conditional fixture resource creation from env toggles; plugin owns phase contract and wiring. |
| Events/readers | `PacketEvent`, `DiggingEvent`, `InteractBlockEvent`, `InteractItemEvent`, `InteractEntityEvent`, `ClickSlotEvent`. | Event readers remain adapters around pure fixture predicates. |
| Env toggles | `MC_COMPAT_SURVIVAL_CHEST_FIXTURE`, `MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE`, `MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_FIXTURE`, `MC_COMPAT_SURVIVAL_FURNACE_FIXTURE`, `MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE`, `MC_COMPAT_SURVIVAL_HUNGER_FOOD_FIXTURE`, `MC_COMPAT_SURVIVAL_HUNGER_HEALTH_FIXTURE`, `MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE`, `MC_COMPAT_SURVIVAL_MOB_AI_LOOT_FIXTURE`, `MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE`, `MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_FIXTURE`, `MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_FIXTURE`, `MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_DIR`, `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_FIXTURE`, `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_DIR`, `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PHASE`, `MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_FIXTURE`, `MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PHASE`, `MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE`, `MC_COMPAT_SURVIVAL_SIGN_EDITING_FIXTURE`, `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE`, `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE`. | Preserve names and enabled semantics. |
| Milestone emitters | Existing survival join, break/place, chest, crafting, furnace, hunger, mob drop, redstone, persistence, block entity, breadth, biome/dimension milestones. | Do not change text. |
| Pure decisions | `fixture_core::survival` helpers plus local pure predicates for slots, windows, block positions, stacks, hunger profiles, environment IDs, and marker state. | Stay outside Bevy queries/commands/file I/O. |
| Non-goals | Full survival correctness, full recipe/container/redstone/mob AI/world persistence, semantic equivalence, production readiness. | Preserve as non-claims. |

## Current terrain app wiring

| Class | Current items | Refactor boundary |
| --- | --- | --- |
| Command | `cargo run --example terrain`. | Preserve command; `main` adds `DefaultPlugins` then `TerrainGameplayPlugin`. |
| Schedules | `Startup`: `setup`; `Update`: chained `init_clients`, `remove_unviewed_chunks`, `update_client_views`, `run_chunk_tasks`, plus `despawn_disconnected_clients`. | Plugin registers systems into named terrain gameplay phase sets and preserves the existing chained terrain request/task order. |
| Resources inserted by setup | `GameState { pending, noise, generation_settings }`; spawned overworld layer. | Setup remains ECS shell; generation settings and pending state stay explicit resources. |
| Events/readers | No custom event readers; systems react to client/view state and task completion. | No new events. |
| Env toggles/milestones | None in current terrain example. | No new compatibility evidence claim. |
| Pure decisions | `generate_chunk`, `validate_generation_settings`, request planning, completion decision, noise/terrain block helpers. | Stay pure; task spawning/polling/logging/world insertion stay shell code. |
| Non-goals | Production worldgen, persistence, universal runtime behavior, vanilla terrain parity. | Preserve as non-claims. |

## Planned phase contract

Each selected example plugin exposes the same phase intent with example-local `SystemSet` names:

1. Input: client joins, packet/event observation, and view/request collection.
2. Rule evaluation: pure predicate/contract decisions and adapter classification.
3. World mutation: ECS resources, chunks, entities, inventories, packets, or task state mutation.
4. Presentation: scoreboard, visible state, and receipt/milestone presentation boundaries where separately scheduled.
5. Cleanup: disconnected-client or stale-work cleanup.

The phase names are schedule-review boundaries only. They do not promote broad gameplay, vanilla parity, production-readiness, or public-server safety claims.

## Baseline checks run before code movement

- Cairn proposal gate: `nix run .#cairn -- gate proposal organize-valence-gameplay-examples-as-bevy-plugins --root .` passed before implementation.
- Cairn design gate: `nix run .#cairn -- gate design organize-valence-gameplay-examples-as-bevy-plugins --root .` passed before implementation.
- Cairn tasks gate: `nix run .#cairn -- gate tasks organize-valence-gameplay-examples-as-bevy-plugins --root .` passed before implementation.
- Cairn validation: `nix run .#cairn -- validate --root .` passed before implementation with three active changes.
- Focused Valence baseline: `cargo test --example ctf`, `cargo test --example survival_compat`, and `cargo test --example terrain` passed through the mc devshell before code movement.
