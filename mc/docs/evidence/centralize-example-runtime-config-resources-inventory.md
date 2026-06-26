# Runtime config resource inventory: centralize-example-runtime-config-resources

## Scope and non-claims

Selected ownership for this change covers Valence example runtime inputs in `servers/valence/examples/ctf.rs` and `servers/valence/examples/survival_compat.rs`.

`servers/valence/examples/terrain.rs` seed derivation is not migrated by this change: it is clock-derived, has no env/CLI contract in the selected compatibility fixtures, and remains a non-claim.

This evidence supports r[valence_bevy_ecs.runtime_config_resources.inventory] and preserves the existing non-claims: no broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, vanilla parity, full CTF correctness, or full survival correctness claim is added.

## CTF runtime config ownership

All CTF selected env inputs are parsed into `CtfRuntimeConfig` from explicit `CtfRuntimeConfigInputs`. The pure parser preserves legacy flag semantics: absent and `0` are disabled for most probe flags; any other present value is enabled. `MC_COMPAT_CTF_SPAWN_TEAM_RESET_PROBE` keeps its legacy presence-only behavior, so even `0` means enabled. The Bevy resource is inserted by `CtfGameplayPlugin` and refreshed by one shell system per schedule so selected systems consume resource fields rather than reading process env directly.

| Input | Type | Default | Reload/runtime expectation | Milestone impact |
| --- | --- | --- | --- | --- |
| `MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables inventory split/merge milestones |
| `MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables inventory drag milestones |
| `MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables vanilla combat reference row milestones |
| `MC_COMPAT_VANILLA_COMBAT_ARMOR_REFERENCE_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig`; also implies vanilla combat reference | switches row to `vanilla-combat-armor-reference-parity` and emits armor reference milestones |
| `MC_COMPAT_STEEL_CONFIG` | env path string | absent | refreshed through `ArrowPolicyRuntimeConfig`; loaded by startup/reload shell only | can publish/reject Steel arrow policy milestones |
| `MC_COMPAT_STEEL_RELOAD_REQUEST` | env request token | absent | refreshed through `ArrowPolicyRuntimeConfig`; stale same-token requests are ignored by `ArrowPolicyState` | controls explicit arrow policy reload attempts |
| `MC_COMPAT_CTF_SCORE_LIMIT_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables score-limit pre/final/win/duplicate milestones |
| `MC_COMPAT_CTF_RACE_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables race accept/reject/final milestones |
| `MC_COMPAT_CTF_SPAWN_TEAM_RESET_PROBE` | env presence flag | disabled | refreshed through `CtfRuntimeConfig` | enables spawn reset/team balance/resource reset milestones |
| `MC_COMPAT_CTF_INVALID_RETURN_DROP_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | selects invalid return/drop rejection milestone |
| `MC_COMPAT_CTF_INVALID_OPPONENT_BASE_RETURN_DROP_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | selects opponent-base return/drop rejection milestone |
| `MC_COMPAT_PROJECTILE_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables projectile loadout/use/hit milestones |
| `MC_COMPAT_ARMOR_MITIGATION_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables armor mitigation milestones |
| `MC_COMPAT_EQUIPMENT_UPDATE_PROBE` | env flag, non-`0` true | disabled | refreshed through `CtfRuntimeConfig` | enables equipment update milestones |

## Survival runtime config ownership

All selected survival fixture env inputs are parsed into `SurvivalRuntimeConfig` from explicit `SurvivalRuntimeConfigInputs`. The pure parser preserves legacy exact-`1` semantics for fixture flags: absent, `0`, `true`, and other malformed values are disabled. Path defaults are explicit parser inputs rooted at `std::env::temp_dir()` only in the shell constructor.

| Input | Type | Default | Reload/runtime expectation | Milestone impact |
| --- | --- | --- | --- | --- |
| `MC_COMPAT_SURVIVAL_CHEST_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables chest fixture blocks/resources and chest milestones |
| `MC_COMPAT_SURVIVAL_CRAFTING_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables crafting fixture and crafting milestones |
| `MC_COMPAT_SURVIVAL_CRAFTING_BREADTH_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables crafting breadth milestone |
| `MC_COMPAT_SURVIVAL_FURNACE_FIXTURE` | env flag, exactly `1` true | disabled unless smelting breadth fixture is enabled | startup/client config resource | enables furnace fixture and furnace milestones |
| `MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource; also enables furnace fixture | enables smelting breadth/invalid fuel milestones |
| `MC_COMPAT_SURVIVAL_HUNGER_FOOD_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | selects hunger food profile unless health profile is also enabled |
| `MC_COMPAT_SURVIVAL_HUNGER_HEALTH_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | health profile wins when both hunger flags are enabled, preserving existing precedence |
| `MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables mob drop fixture and milestones |
| `MC_COMPAT_SURVIVAL_MOB_AI_LOOT_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables synthetic mob AI loot breadth milestones |
| `MC_COMPAT_SURVIVAL_REDSTONE_TOGGLE_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables redstone toggle arena and milestones |
| `MC_COMPAT_SURVIVAL_REDSTONE_CIRCUIT_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables synthetic redstone circuit milestones |
| `MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables world persistence arena/resources and milestones |
| `MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_DIR` | env path | temp dir + `mc-compat-world-persistence` | startup path resource | controls world persistence marker path |
| `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_FIXTURE` | env flag, exactly `1` true | disabled | startup/client config resource | enables block entity arena/resources and milestones |
| `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_DIR` | env path | temp dir + `mc-compat-block-entity-persistence` | startup path resource | controls block entity marker path |
| `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_PHASE` | env phase string | pre-restart behavior | startup/client config resource | `post_restart` selects post-restart observation; stale phase without fixture is diagnostic-only in tests |
| `MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables synthetic multichunk milestones |
| `MC_COMPAT_SURVIVAL_WORLD_MULTICHUNK_PHASE` | env phase string | pre-restart behavior | client config resource | `post_restart` selects post-restart synthetic multichunk milestones; stale phase without fixture is diagnostic-only in tests |
| `MC_COMPAT_SURVIVAL_CONTAINER_BLOCK_ENTITY_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables synthetic container block entity milestones |
| `MC_COMPAT_SURVIVAL_SIGN_EDITING_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables synthetic sign editing milestones |
| `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables biome/dimension state milestones |
| `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_TRAVEL_FIXTURE` | env flag, exactly `1` true | disabled | client config resource | enables synthetic biome/dimension travel milestones |

## Verification hooks

Parser tests cover valid resource values, malformed disabled values, missing/default paths, stale reload/phase requests, conflict precedence, and disabled-plugin resource absence. Focused example test logs are promoted under `docs/evidence/run-logs/2026-06-25/centralize-example-runtime-config-resources-*.run.log`.
