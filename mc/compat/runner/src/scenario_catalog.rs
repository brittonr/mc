//! Pure scenario catalog constants shared by the runner shell and scenario core.
//!
//! This module is deliberately data-only: no filesystem, process, network, clock,
//! environment, stdout/stderr, or exit-code behavior belongs here.

pub(crate) const SUPPORTED_SCENARIO_USAGE: &str = "smoke|valence-compat-bot-probe|flag-score-repeat|blue-flag-score|inventory-interaction|inventory-stack-split-merge|inventory-drag-transactions|survival-break-place-pickup|survival-chest-persistence|survival-crafting-table|survival-crafting-recipe-breadth|survival-furnace-persistence|survival-furnace-smelting-breadth|survival-hunger-food|survival-hunger-health-cycle|survival-mob-drop|survival-mob-ai-loot-breadth|survival-redstone-toggle|survival-redstone-circuit-breadth|survival-world-persistence-restart|survival-world-multichunk-durability|survival-crash-recovery-parity|survival-block-entity-persistence-parity|survival-container-block-entity-breadth|survival-biome-dimension-state|survival-biome-dimension-travel|survival-sign-editing-live|mcp-controlled-smoke|combat-damage|combat-knockback|vanilla-combat-reference-parity|vanilla-combat-armor-reference-parity|armor-equipment-mitigation|armor-loadout-enchantment-status-matrix|equipment-update-observation|equipment-slot-item-matrix-expansion|projectile-hit|projectile-damage-attribution|flag-carrier-death-return|reconnect-flag-state|reconnect-flag-score|multi-client-load-score|negative-inventory-stale-state|negative-inventory-invalid-click|negative-custom-payload|negative-reconnect-race|negative-ctf-wrong-score|ctf-invalid-pickup-ownership|ctf-invalid-return-drop|ctf-invalid-opponent-base-return-drop|ctf-score-limit-win-condition|ctf-simultaneous-pickup-capture-race|ctf-spawn-team-balance-reset";

pub(crate) const INVENTORY_STACK_CLIENT_INITIAL_NEEDLE: &str =
    "inventory_stack_initial_slot window=0 state_id=";
pub(crate) const INVENTORY_STACK_CLIENT_SPLIT_PICKUP_NEEDLE: &str =
    "inventory_stack_split_pickup_sent";
pub(crate) const INVENTORY_STACK_CLIENT_SPLIT_SOURCE_NEEDLE: &str =
    "inventory_stack_split_source_seen";
pub(crate) const INVENTORY_STACK_CLIENT_SPLIT_PLACE_NEEDLE: &str =
    "inventory_stack_split_place_sent";
pub(crate) const INVENTORY_STACK_CLIENT_DESTINATION_NEEDLE: &str =
    "inventory_stack_split_destination_seen";
pub(crate) const INVENTORY_STACK_CLIENT_MERGE_PICKUP_NEEDLE: &str =
    "inventory_stack_merge_pickup_sent";
pub(crate) const INVENTORY_STACK_CLIENT_MERGE_EMPTY_NEEDLE: &str =
    "inventory_stack_merge_destination_empty_seen";
pub(crate) const INVENTORY_STACK_CLIENT_MERGE_PLACE_NEEDLE: &str =
    "inventory_stack_merge_place_sent";
pub(crate) const INVENTORY_STACK_CLIENT_FINAL_NEEDLE: &str = "inventory_stack_final_source_seen";
pub(crate) const INVENTORY_STACK_SERVER_SPLIT_PICKUP_NEEDLE: &str =
    "inventory_stack_server_split_pickup username=compatbot";
pub(crate) const INVENTORY_STACK_SERVER_SPLIT_NEEDLE: &str =
    "inventory_stack_server_split username=compatbot";
pub(crate) const INVENTORY_STACK_SERVER_MERGE_PICKUP_NEEDLE: &str =
    "inventory_stack_server_merge_pickup username=compatbot";
pub(crate) const INVENTORY_STACK_SERVER_MERGE_NEEDLE: &str =
    "inventory_stack_server_merge username=compatbot";
pub(crate) const INVENTORY_DRAG_CLIENT_INITIAL_NEEDLE: &str =
    "inventory_drag_initial_slot window=0 state_id=";
pub(crate) const INVENTORY_DRAG_CLIENT_PICKUP_NEEDLE: &str = "inventory_drag_pickup_sent";
pub(crate) const INVENTORY_DRAG_CLIENT_SOURCE_EMPTY_NEEDLE: &str =
    "inventory_drag_source_empty_seen";
pub(crate) const INVENTORY_DRAG_CLIENT_START_NEEDLE: &str = "inventory_drag_start_sent";
pub(crate) const INVENTORY_DRAG_CLIENT_TARGET_A_NEEDLE: &str = "inventory_drag_target_a_sent";
pub(crate) const INVENTORY_DRAG_CLIENT_TARGET_B_NEEDLE: &str = "inventory_drag_target_b_sent";
pub(crate) const INVENTORY_DRAG_CLIENT_END_NEEDLE: &str = "inventory_drag_end_sent";
pub(crate) const INVENTORY_DRAG_CLIENT_FINAL_NEEDLE: &str =
    "inventory_drag_final_distribution_seen";
pub(crate) const INVENTORY_DRAG_SERVER_PICKUP_NEEDLE: &str =
    "inventory_drag_server_pickup username=compatbot";
pub(crate) const INVENTORY_DRAG_SERVER_START_NEEDLE: &str =
    "inventory_drag_server_start username=compatbot";
pub(crate) const INVENTORY_DRAG_SERVER_TARGET_A_NEEDLE: &str =
    "inventory_drag_server_target_a username=compatbot";
pub(crate) const INVENTORY_DRAG_SERVER_TARGET_B_NEEDLE: &str =
    "inventory_drag_server_target_b username=compatbot";
pub(crate) const INVENTORY_DRAG_SERVER_END_NEEDLE: &str =
    "inventory_drag_server_end username=compatbot";
pub(crate) const SURVIVAL_CHEST_CLIENT_OPEN_NEEDLE: &str =
    "survival_chest_open_seen window=1 position=8,64,0";
pub(crate) const SURVIVAL_CHEST_CLIENT_STORE_NEEDLE: &str =
    "survival_chest_store_sent window=1 slot=0 item=Dirt count=1";
pub(crate) const SURVIVAL_CHEST_CLIENT_CLOSE_NEEDLE: &str = "survival_chest_close_sent window=1";
pub(crate) const SURVIVAL_CHEST_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_chest_reconnect_sent session=1";
pub(crate) const SURVIVAL_CHEST_CLIENT_REOPEN_NEEDLE: &str =
    "survival_chest_reopen_seen window=1 position=8,64,0";
pub(crate) const SURVIVAL_CHEST_CLIENT_PERSISTED_NEEDLE: &str =
    "survival_chest_persisted_seen window=1 slot=0 item=Dirt count=1";
pub(crate) const SURVIVAL_CHEST_SERVER_OPEN_NEEDLE: &str =
    "survival_chest_open username=compatbot position=8,64,0 window=1";
pub(crate) const SURVIVAL_CHEST_SERVER_STORE_NEEDLE: &str =
    "survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1";
pub(crate) const SURVIVAL_CHEST_SERVER_CLOSE_NEEDLE: &str =
    "survival_chest_close username=compatbot window=1";
pub(crate) const SURVIVAL_CHEST_SERVER_REOPEN_NEEDLE: &str =
    "survival_chest_reopen username=compatbot position=8,64,0 window=1";
pub(crate) const SURVIVAL_CHEST_SERVER_PERSISTED_NEEDLE: &str =
    "survival_chest_persisted username=compatbot slot=0 item=Dirt count=1";
pub(crate) const SURVIVAL_CRAFTING_CLIENT_OPEN_NEEDLE: &str =
    "survival_crafting_table_open_seen window=1 position=4,64,0";
pub(crate) const SURVIVAL_CRAFTING_CLIENT_INPUT_A_NEEDLE: &str =
    "survival_crafting_input_a_sent window=1 slot=1 item=OakPlanks count=1";
pub(crate) const SURVIVAL_CRAFTING_CLIENT_INPUT_B_NEEDLE: &str =
    "survival_crafting_input_b_sent window=1 slot=4 item=OakPlanks count=1";
pub(crate) const SURVIVAL_CRAFTING_CLIENT_RESULT_NEEDLE: &str =
    "survival_crafting_result_seen window=1 slot=0 item=Stick count=4 recipe=minecraft:stick";
pub(crate) const SURVIVAL_CRAFTING_CLIENT_COLLECT_NEEDLE: &str =
    "survival_crafting_result_collected window=1 slot=0 item=Stick count=4";
pub(crate) const SURVIVAL_CRAFTING_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_crafting_inventory_updated slot=36 item=Stick count=4";
pub(crate) const SURVIVAL_CRAFTING_SERVER_OPEN_NEEDLE: &str =
    "survival_crafting_table_open username=compatbot position=4,64,0 window=1";
pub(crate) const SURVIVAL_CRAFTING_SERVER_INPUT_A_NEEDLE: &str =
    "survival_crafting_input_a username=compatbot window=1 slot=1 item=OakPlanks count=1";
pub(crate) const SURVIVAL_CRAFTING_SERVER_INPUT_B_NEEDLE: &str =
    "survival_crafting_input_b username=compatbot window=1 slot=4 item=OakPlanks count=1";
pub(crate) const SURVIVAL_CRAFTING_SERVER_RESULT_NEEDLE: &str =
    "survival_crafting_result username=compatbot window=1 slot=0 item=Stick count=4 recipe=minecraft:stick";
pub(crate) const SURVIVAL_CRAFTING_SERVER_COLLECT_NEEDLE: &str =
    "survival_crafting_collect username=compatbot window=1 slot=0 item=Stick count=4 inventory_slot=36";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_CLIENT_SHAPED_NEEDLE: &str =
    "survival_crafting_breadth_shaped_seen window=1 recipe=minecraft:chest input=oak_planksx8 result=Chest count=1";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_CLIENT_SHAPELESS_NEEDLE: &str =
    "survival_crafting_breadth_shapeless_seen window=1 recipe=minecraft:oak_planks input=oak_logx1 result=OakPlanks count=4";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_CLIENT_CLEAR_NEEDLE: &str =
    "survival_crafting_breadth_grid_clear_seen window=1 occupied_slots=0";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_CLIENT_INVALID_NEEDLE: &str =
    "survival_crafting_breadth_invalid_seen window=1 recipe=minecraft:stick_insufficient_input_rejection input=single_oak_plank outcome=no_result";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_crafting_breadth_inventory_updated slot=36 item=Chest count=1 slot=37 item=OakPlanks count=4";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_SERVER_SHAPED_NEEDLE: &str =
    "survival_crafting_breadth_shaped username=compatbot recipe=minecraft:chest input=oak_planksx8 result=Chest count=1";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_SERVER_SHAPELESS_NEEDLE: &str =
    "survival_crafting_breadth_shapeless username=compatbot recipe=minecraft:oak_planks input=oak_logx1 result=OakPlanks count=4";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_SERVER_CLEAR_NEEDLE: &str =
    "survival_crafting_breadth_grid_clear username=compatbot window=1 occupied_slots=0";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_SERVER_INVALID_NEEDLE: &str =
    "survival_crafting_breadth_invalid_rejected username=compatbot recipe=minecraft:stick_insufficient_input_rejection input=single_oak_plank outcome=no_result";
pub(crate) const SURVIVAL_CRAFTING_BREADTH_SERVER_STATE_NEEDLE: &str =
    "survival_crafting_breadth_state username=compatbot shaped=true shapeless=true invalid_rejected=true extra_outputs=false";
pub(crate) const SURVIVAL_FURNACE_CLIENT_OPEN_NEEDLE: &str =
    "survival_furnace_open_seen window=1 position=12,64,0";
pub(crate) const SURVIVAL_FURNACE_CLIENT_INPUT_NEEDLE: &str =
    "survival_furnace_input_sent window=1 slot=0 item=RawIron count=1";
pub(crate) const SURVIVAL_FURNACE_CLIENT_FUEL_NEEDLE: &str =
    "survival_furnace_fuel_sent window=1 slot=1 item=Coal count=1";
pub(crate) const SURVIVAL_FURNACE_CLIENT_BURN_NEEDLE: &str =
    "survival_furnace_burn_progress_seen window=1 progress=started";
pub(crate) const SURVIVAL_FURNACE_CLIENT_OUTPUT_NEEDLE: &str =
    "survival_furnace_output_seen window=1 slot=2 item=IronIngot count=1";
pub(crate) const SURVIVAL_FURNACE_CLIENT_COLLECT_NEEDLE: &str =
    "survival_furnace_output_collected window=1 slot=2 item=IronIngot count=1";
pub(crate) const SURVIVAL_FURNACE_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_furnace_inventory_updated slot=36 item=IronIngot count=1";
pub(crate) const SURVIVAL_FURNACE_CLIENT_INVALID_FUEL_NEEDLE: &str =
    "survival_furnace_invalid_fuel_sent window=1 slot=1 item=RawIron outcome=no_burn";
pub(crate) const SURVIVAL_FURNACE_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_furnace_reconnect_sent session=1";
pub(crate) const SURVIVAL_FURNACE_CLIENT_REOPEN_NEEDLE: &str =
    "survival_furnace_reopen_seen window=1 position=12,64,0";
pub(crate) const SURVIVAL_FURNACE_SERVER_OPEN_NEEDLE: &str =
    "survival_furnace_open username=compatbot position=12,64,0 window=1";
pub(crate) const SURVIVAL_FURNACE_SERVER_INPUT_NEEDLE: &str =
    "survival_furnace_input_insert username=compatbot window=1 slot=0 item=RawIron count=1";
pub(crate) const SURVIVAL_FURNACE_SERVER_FUEL_NEEDLE: &str =
    "survival_furnace_fuel_insert username=compatbot window=1 slot=1 item=Coal count=1";
pub(crate) const SURVIVAL_FURNACE_SERVER_BURN_NEEDLE: &str =
    "survival_furnace_burn_progress username=compatbot window=1 progress=started";
pub(crate) const SURVIVAL_FURNACE_SERVER_OUTPUT_NEEDLE: &str =
    "survival_furnace_output_available username=compatbot window=1 slot=2 item=IronIngot count=1";
pub(crate) const SURVIVAL_FURNACE_SERVER_COLLECT_NEEDLE: &str =
    "survival_furnace_output_collect username=compatbot window=1 slot=2 item=IronIngot count=1 inventory_slot=36";
pub(crate) const SURVIVAL_FURNACE_SERVER_INVALID_FUEL_NEEDLE: &str =
    "survival_furnace_invalid_fuel_rejected username=compatbot window=1 slot=1 item=RawIron outcome=no_burn";
pub(crate) const SURVIVAL_FURNACE_SERVER_BREADTH_STATE_NEEDLE: &str =
    "survival_furnace_breadth_state username=compatbot recipe=minecraft:iron_ingot input=RawIron fuel=Coal output=IronIngot count=1 invalid_fuel=RawIron invalid_fuel_outcome=no_burn broad_all_furnaces=false";
pub(crate) const SURVIVAL_FURNACE_SERVER_REOPEN_NEEDLE: &str =
    "survival_furnace_reconnect_reopen username=compatbot position=12,64,0 window=1";
pub(crate) const SURVIVAL_FURNACE_SERVER_STATE_NEEDLE: &str =
    "survival_furnace_server_state username=compatbot position=12,64,0 input=RawIron fuel=Coal output=empty collected=true session_persistent=true";
pub(crate) const SURVIVAL_HUNGER_FOOD_CLIENT_ITEM_NEEDLE: &str =
    "survival_hunger_food_item_seen slot=36 item=Bread count=1";
pub(crate) const SURVIVAL_HUNGER_FOOD_CLIENT_PRE_NEEDLE: &str =
    "survival_hunger_food_pre_seen health=20.0 food=15 saturation=0.0";
pub(crate) const SURVIVAL_HUNGER_FOOD_CLIENT_USE_NEEDLE: &str =
    "survival_hunger_food_use_sent slot=36 item=Bread count=1 hand=main sequence=810";
pub(crate) const SURVIVAL_HUNGER_FOOD_CLIENT_POST_NEEDLE: &str =
    "survival_hunger_food_post_seen health=20.0 food=20 saturation=6.0";
pub(crate) const SURVIVAL_HUNGER_FOOD_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_hunger_food_inventory_updated slot=36 item=Bread count=0";
pub(crate) const SURVIVAL_HUNGER_FOOD_SERVER_PRE_NEEDLE: &str =
    "survival_hunger_food_pre username=compatbot health=20.0 food=15 saturation=0.0 item=Bread count=1 slot=36";
pub(crate) const SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_START_NEEDLE: &str =
    "survival_hunger_food_consume_start username=compatbot item=Bread slot=36 food_before=15 saturation_before=0.0";
pub(crate) const SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_FINISH_NEEDLE: &str =
    "survival_hunger_food_consume_finish username=compatbot item=Bread slot=36 food_after=20 saturation_after=6.0";
pub(crate) const SURVIVAL_HUNGER_FOOD_SERVER_INVENTORY_NEEDLE: &str =
    "survival_hunger_food_inventory username=compatbot slot=36 item=Bread count_before=1 count_after=0";
pub(crate) const SURVIVAL_HUNGER_FOOD_SERVER_STATE_NEEDLE: &str =
    "survival_hunger_food_state username=compatbot health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false";
pub(crate) const SURVIVAL_HUNGER_HEALTH_CLIENT_ITEM_NEEDLE: &str =
    "survival_hunger_health_item_seen slot=36 item=Bread count=1";
pub(crate) const SURVIVAL_HUNGER_HEALTH_CLIENT_PRE_NEEDLE: &str =
    "survival_hunger_health_pre_seen health=18.0 food=15 saturation=0.0";
pub(crate) const SURVIVAL_HUNGER_HEALTH_CLIENT_USE_NEEDLE: &str =
    "survival_hunger_health_consume_sent slot=36 item=Bread count=1 hand=main sequence=810";
pub(crate) const SURVIVAL_HUNGER_HEALTH_CLIENT_POST_NEEDLE: &str =
    "survival_hunger_health_recovery_seen health=20.0 food=20 saturation=6.0";
pub(crate) const SURVIVAL_HUNGER_HEALTH_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_hunger_health_inventory_updated slot=36 item=Bread count=0";
pub(crate) const SURVIVAL_HUNGER_HEALTH_SERVER_PRE_NEEDLE: &str =
    "survival_hunger_health_pre username=compatbot health=18.0 food=15 saturation=0.0 item=Bread count=1 slot=36";
pub(crate) const SURVIVAL_HUNGER_HEALTH_SERVER_CONSUME_START_NEEDLE: &str =
    "survival_hunger_health_consume_start username=compatbot item=Bread slot=36 food_before=15 saturation_before=0.0";
pub(crate) const SURVIVAL_HUNGER_HEALTH_SERVER_CONSUME_FINISH_NEEDLE: &str =
    "survival_hunger_health_consume_finish username=compatbot item=Bread slot=36 food_after=20 saturation_after=6.0";
pub(crate) const SURVIVAL_HUNGER_HEALTH_SERVER_INVENTORY_NEEDLE: &str =
    "survival_hunger_health_inventory username=compatbot slot=36 item=Bread count_before=1 count_after=0";
pub(crate) const SURVIVAL_HUNGER_HEALTH_SERVER_STATE_NEEDLE: &str =
    "survival_hunger_health_state username=compatbot pre_health=18.0 post_health=20.0 food_before=15 food_after=20 saturation_before=0.0 saturation_after=6.0 unexpected_damage=false death=false";
pub(crate) const SURVIVAL_MOB_DROP_CLIENT_MOB_NEEDLE: &str =
    "survival_mob_drop_mob_seen mob=IronGolem position=16.5,65.0,2.5";
pub(crate) const SURVIVAL_MOB_DROP_CLIENT_ATTACK_NEEDLE: &str =
    "survival_mob_drop_attack_sent mob=IronGolem target_id=";
pub(crate) const SURVIVAL_MOB_DROP_CLIENT_DEATH_NEEDLE: &str =
    "survival_mob_drop_death_seen mob=IronGolem target_id=";
pub(crate) const SURVIVAL_MOB_DROP_CLIENT_DROP_NEEDLE: &str =
    "survival_mob_drop_drop_seen item=IronIngot count=1";
pub(crate) const SURVIVAL_MOB_DROP_CLIENT_PICKUP_NEEDLE: &str =
    "survival_mob_drop_pickup_seen item=IronIngot count=1";
pub(crate) const SURVIVAL_MOB_DROP_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_mob_drop_inventory_updated slot=36 item=IronIngot count=1";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_SPAWN_NEEDLE: &str =
    "survival_mob_drop_spawn username=compatbot mob=IronGolem position=16.5,65.0,2.5";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_ATTACK_NEEDLE: &str =
    "survival_mob_drop_attack username=compatbot mob=IronGolem damage=20.0";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_DEATH_NEEDLE: &str =
    "survival_mob_drop_death username=compatbot mob=IronGolem";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_DROP_NEEDLE: &str =
    "survival_mob_drop_drop_spawn username=compatbot item=IronIngot count=1";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_PICKUP_NEEDLE: &str =
    "survival_mob_drop_pickup username=compatbot item=IronIngot count=1";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_INVENTORY_NEEDLE: &str =
    "survival_mob_drop_inventory username=compatbot slot=36 item=IronIngot count=1";
pub(crate) const SURVIVAL_MOB_DROP_SERVER_STATE_NEEDLE: &str =
    "survival_mob_drop_state username=compatbot mob=IronGolem drop=IronIngot count=1 extra_drops=false";
pub(crate) const SURVIVAL_MOB_AI_LOOT_CLIENT_MOB_NEEDLE: &str =
    "survival_mob_ai_loot_mob_seen mob=Zombie position=16.5,65.0,4.5 ai_checkpoint=approach_player";
pub(crate) const SURVIVAL_MOB_AI_LOOT_CLIENT_ATTACK_NEEDLE: &str =
    "survival_mob_ai_loot_attack_sent mob=Zombie kill_method=player_attack";
pub(crate) const SURVIVAL_MOB_AI_LOOT_CLIENT_DEATH_NEEDLE: &str =
    "survival_mob_ai_loot_death_seen mob=Zombie";
pub(crate) const SURVIVAL_MOB_AI_LOOT_CLIENT_DROP_NEEDLE: &str =
    "survival_mob_ai_loot_drop_seen item=RottenFlesh count=1";
pub(crate) const SURVIVAL_MOB_AI_LOOT_CLIENT_PICKUP_NEEDLE: &str =
    "survival_mob_ai_loot_pickup_seen item=RottenFlesh count=1";
pub(crate) const SURVIVAL_MOB_AI_LOOT_CLIENT_INVENTORY_NEEDLE: &str =
    "survival_mob_ai_loot_inventory_updated slot=36 item=RottenFlesh count=1";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_SPAWN_NEEDLE: &str =
    "survival_mob_ai_loot_spawn username=compatbot mob=Zombie position=16.5,65.0,4.5";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_AI_NEEDLE: &str =
    "survival_mob_ai_loot_ai_checkpoint username=compatbot mob=Zombie checkpoint=approach_player target=compatbot";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_ATTACK_NEEDLE: &str =
    "survival_mob_ai_loot_attack username=compatbot mob=Zombie kill_method=player_attack";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_DEATH_NEEDLE: &str =
    "survival_mob_ai_loot_death username=compatbot mob=Zombie";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_DROP_NEEDLE: &str =
    "survival_mob_ai_loot_drop_spawn username=compatbot item=RottenFlesh count=1";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_PICKUP_NEEDLE: &str =
    "survival_mob_ai_loot_pickup username=compatbot item=RottenFlesh count=1";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_INVENTORY_NEEDLE: &str =
    "survival_mob_ai_loot_inventory username=compatbot slot=36 item=RottenFlesh count=1";
pub(crate) const SURVIVAL_MOB_AI_LOOT_SERVER_STATE_NEEDLE: &str =
    "survival_mob_ai_loot_state username=compatbot mob=Zombie ai_checkpoint=approach_player kill_method=player_attack drop=RottenFlesh count=1 pickup=observed inventory_increment=1 extra_mobs=false";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_ON_NEEDLE: &str =
    "survival_redstone_toggle_input_sent control=Lever position=20,64,0 powered_before=false powered_after=true";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_ON_NEEDLE: &str =
    "survival_redstone_toggle_output_update output=RedstoneLamp position=21,64,0 powered=true";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_OFF_NEEDLE: &str =
    "survival_redstone_toggle_return_input_sent control=Lever position=20,64,0 powered_before=true powered_after=false";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_OFF_NEEDLE: &str =
    "survival_redstone_toggle_return_update output=RedstoneLamp position=21,64,0 powered=false";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_SERVER_INPUT_NEEDLE: &str =
    "survival_redstone_toggle_input username=compatbot control=Lever position=20,64,0 powered_before=false powered_after=true";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_SERVER_ON_NEEDLE: &str =
    "survival_redstone_toggle_powered_on username=compatbot output=RedstoneLamp position=21,64,0 powered=true";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_SERVER_OFF_NEEDLE: &str =
    "survival_redstone_toggle_powered_off username=compatbot output=RedstoneLamp position=21,64,0 powered=false";
pub(crate) const SURVIVAL_REDSTONE_TOGGLE_SERVER_STATE_NEEDLE: &str =
    "survival_redstone_toggle_state username=compatbot control=Lever output=RedstoneLamp on_seen=true off_seen=true unintended_outputs=false";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_INITIAL_NEEDLE: &str =
    "survival_redstone_circuit_initial_state circuit=lever_lamp_repeater tick=0 powered=false";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_INPUT_NEEDLE: &str =
    "survival_redstone_circuit_input_sent control=Lever position=20,64,0 tick=2 powered_after=true";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_OUTPUT_ON_NEEDLE: &str =
    "survival_redstone_circuit_output_update output=RedstoneLamp repeater=Repeater position=21,64,0 tick=2 powered=true";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_RETURN_NEEDLE: &str =
    "survival_redstone_circuit_return_input_sent control=Lever position=20,64,0 tick=4 powered_after=false";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_CLIENT_OUTPUT_OFF_NEEDLE: &str =
    "survival_redstone_circuit_return_update output=RedstoneLamp repeater=Repeater position=21,64,0 tick=4 powered=false";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_SERVER_INITIAL_NEEDLE: &str =
    "survival_redstone_circuit_initial username=compatbot circuit=lever_lamp_repeater powered=false tick=0";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_SERVER_INPUT_NEEDLE: &str =
    "survival_redstone_circuit_input username=compatbot control=Lever position=20,64,0 tick=2 powered_after=true";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_SERVER_ON_NEEDLE: &str =
    "survival_redstone_circuit_powered_on username=compatbot output=RedstoneLamp repeater=Repeater tick=2 powered=true";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_SERVER_OFF_NEEDLE: &str =
    "survival_redstone_circuit_powered_off username=compatbot output=RedstoneLamp repeater=Repeater tick=4 powered=false";
pub(crate) const SURVIVAL_REDSTONE_CIRCUIT_SERVER_STATE_NEEDLE: &str =
    "survival_redstone_circuit_state username=compatbot circuit=lever_lamp_repeater initial=false after_input=true after_return=false tick_sequence=0:false,2:true,4:false unintended_outputs=false";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_CLIENT_MUTATION_NEEDLE: &str =
    "survival_world_persistence_mutation_sent block=Dirt position=24,64,0 slot=36";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_CLIENT_PRE_RESTART_NEEDLE: &str =
    "survival_world_persistence_pre_restart_update block=Dirt position=24,64,0";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_world_persistence_reconnect_sent session=restart";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_CLIENT_POST_RESTART_NEEDLE: &str =
    "survival_world_persistence_post_restart_update block=Dirt position=24,64,0";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SERVER_MUTATION_NEEDLE: &str =
    "survival_world_persistence_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SERVER_CLEAN_NEEDLE: &str =
    "survival_world_persistence_clean_shutdown username=compatbot storage=isolated shutdown=graceful";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SERVER_RESTART_NEEDLE: &str =
    "survival_world_persistence_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SERVER_POST_NEEDLE: &str =
    "survival_world_persistence_post_restart_observe username=compatbot block=Dirt position=24,64,0 persisted=true";
pub(crate) const SURVIVAL_WORLD_PERSISTENCE_SERVER_STATE_NEEDLE: &str =
    "survival_world_persistence_state username=compatbot block=Dirt position=24,64,0 pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false";
pub(crate) const SURVIVAL_BLOCK_ENTITY_CLIENT_PRE_RESTART_NEEDLE: &str =
    "survival_block_entity_pre_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist";
pub(crate) const SURVIVAL_BLOCK_ENTITY_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_block_entity_reconnect_sent session=restart";
pub(crate) const SURVIVAL_BLOCK_ENTITY_CLIENT_POST_RESTART_NEEDLE: &str =
    "survival_block_entity_post_restart_update kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist";
pub(crate) const SURVIVAL_BLOCK_ENTITY_SERVER_MUTATION_NEEDLE: &str =
    "survival_block_entity_persistence_mutation username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted_before=false persisted_after=true";
pub(crate) const SURVIVAL_BLOCK_ENTITY_SERVER_CLEAN_NEEDLE: &str =
    "survival_block_entity_persistence_clean_shutdown username=compatbot storage=isolated shutdown=graceful";
pub(crate) const SURVIVAL_BLOCK_ENTITY_SERVER_RESTART_NEEDLE: &str =
    "survival_block_entity_persistence_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true";
pub(crate) const SURVIVAL_BLOCK_ENTITY_SERVER_POST_NEEDLE: &str =
    "survival_block_entity_persistence_post_restart_observe username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist persisted=true";
pub(crate) const SURVIVAL_BLOCK_ENTITY_SERVER_STATE_NEEDLE: &str =
    "survival_block_entity_persistence_state username=compatbot kind=Sign position=28,64,0 text=MC|Compat|Sign|Persist pre_mutation=true clean_shutdown=true backend_restart=true post_observed=true dirty_reuse=false";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_CLIENT_MUTATION_NEEDLE: &str =
    "survival_world_multichunk_mutation_sent primary=0,64,0:Dirt secondary=32,64,0:OakPlanks chunks=0,0;2,0";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_CLIENT_PRE_RESTART_NEEDLE: &str =
    "survival_world_multichunk_pre_restart_update primary=present secondary=present auxiliary_marker_only=false";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_world_multichunk_reconnect_sent session=restart";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_CLIENT_POST_RESTART_NEEDLE: &str =
    "survival_world_multichunk_post_restart_update primary=present secondary=present";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_SERVER_MUTATION_NEEDLE: &str =
    "survival_world_multichunk_mutation username=compatbot chunks=0,0;2,0 primary=0,64,0:Dirt secondary=32,64,0:OakPlanks persisted_before=false persisted_after=true";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_SERVER_CLEAN_NEEDLE: &str =
    "survival_world_multichunk_clean_shutdown username=compatbot storage=isolated shutdown=graceful";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_SERVER_RESTART_NEEDLE: &str =
    "survival_world_multichunk_backend_restart username=compatbot method=controlled_reload storage=isolated restart_confirmed=true";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_SERVER_POST_NEEDLE: &str =
    "survival_world_multichunk_post_restart_observe username=compatbot primary=present secondary=present auxiliary_marker_only=false";
pub(crate) const SURVIVAL_WORLD_MULTICHUNK_SERVER_STATE_NEEDLE: &str =
    "survival_world_multichunk_state username=compatbot chunks=0,0;2,0 primary=present secondary=present controlled_reload=true post_observed=true auxiliary_marker_only=false dirty_reuse=false";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_OPEN_NEEDLE: &str =
    "survival_container_block_entity_open_seen window=1 kind=Barrel position=34,64,0";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_TRANSFER_NEEDLE: &str =
    "survival_container_block_entity_transfer_sent window=1 slot=0 item=Dirt count=1";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_PAYLOAD_NEEDLE: &str =
    "survival_container_block_entity_payload_seen summary=slot0:Dirt:1";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_METADATA_NEEDLE: &str =
    "survival_container_block_entity_metadata_seen summary=custom_name:MC Compat Barrel";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_REOPEN_NEEDLE: &str =
    "survival_container_block_entity_reopen_seen window=1 kind=Barrel position=34,64,0 payload=slot0:Dirt:1";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_OPEN_NEEDLE: &str =
    "survival_container_block_entity_open username=compatbot window=1 kind=Barrel position=34,64,0";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_TRANSFER_NEEDLE: &str =
    "survival_container_block_entity_transfer username=compatbot window=1 slot=0 item=Dirt count=1";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_PAYLOAD_NEEDLE: &str =
    "survival_container_block_entity_payload username=compatbot summary=slot0:Dirt:1";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_METADATA_NEEDLE: &str =
    "survival_container_block_entity_metadata username=compatbot summary=custom_name:MC Compat Barrel";
pub(crate) const SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_STATE_NEEDLE: &str =
    "survival_container_block_entity_state username=compatbot kind=Barrel position=34,64,0 transfer=Dirt:1 payload=slot0:Dirt:1 metadata=custom_name:MC Compat Barrel reopen=payload_present arbitrary_nbt=false";
pub(crate) const SURVIVAL_SIGN_EDITING_CLIENT_OPEN_NEEDLE: &str =
    "survival_sign_editing_open_seen position=28,64,0 side=front milestone=sign_editor_open_observed";
pub(crate) const SURVIVAL_SIGN_EDITING_CLIENT_UPDATE_NEEDLE: &str =
    "survival_sign_editing_update_sent position=28,64,0 side=front payload=MC|Compat|Sign|Edit milestone=sign_update_sent";
pub(crate) const SURVIVAL_SIGN_EDITING_CLIENT_POST_NEEDLE: &str =
    "survival_sign_editing_post_update_seen position=28,64,0 side=front text=MC|Compat|Sign|Edit observation=text_visible";
pub(crate) const SURVIVAL_SIGN_EDITING_SERVER_OPEN_NEEDLE: &str =
    "survival_sign_editing_open username=compatbot position=28,64,0 side=front milestone=sign_editor_open_observed";
pub(crate) const SURVIVAL_SIGN_EDITING_SERVER_UPDATE_NEEDLE: &str =
    "survival_sign_editing_update_accepted username=compatbot position=28,64,0 side=front payload=MC|Compat|Sign|Edit milestone=sign_update_accepted_observed";
pub(crate) const SURVIVAL_SIGN_EDITING_SERVER_STATE_NEEDLE: &str =
    "survival_sign_editing_state username=compatbot position=28,64,0 side=front payload=MC|Compat|Sign|Edit post_update=text_visible arbitrary_sign_ui=false";
pub(crate) const SURVIVAL_CRASH_RECOVERY_CLIENT_MUTATION_NEEDLE: &str =
    "survival_crash_recovery_mutation_sent block=Dirt position=24,64,0 slot=36";
pub(crate) const SURVIVAL_CRASH_RECOVERY_CLIENT_PRE_CRASH_NEEDLE: &str =
    "survival_crash_recovery_pre_crash_update block=Dirt position=24,64,0";
pub(crate) const SURVIVAL_CRASH_RECOVERY_CLIENT_RECONNECT_NEEDLE: &str =
    "survival_crash_recovery_reconnect_sent session=crash_recovery";
pub(crate) const SURVIVAL_CRASH_RECOVERY_CLIENT_POST_CRASH_NEEDLE: &str =
    "survival_crash_recovery_post_crash_update block=Dirt position=24,64,0";
pub(crate) const SURVIVAL_CRASH_RECOVERY_SERVER_MUTATION_NEEDLE: &str =
    "survival_crash_recovery_mutation username=compatbot block=Dirt position=24,64,0 persisted_before=false persisted_after=true";
pub(crate) const SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE: &str =
    "survival_crash_recovery_forced_stop username=compatbot method=forced_stop storage=isolated graceful=false";
pub(crate) const SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE: &str =
    "survival_crash_recovery_backend_restart username=compatbot method=crash_recovery storage=isolated restart_confirmed=true";
pub(crate) const SURVIVAL_CRASH_RECOVERY_SERVER_POST_NEEDLE: &str =
    "survival_crash_recovery_post_crash_observe username=compatbot block=Dirt position=24,64,0 persisted=true";
pub(crate) const SURVIVAL_CRASH_RECOVERY_SERVER_STATE_NEEDLE: &str =
    "survival_crash_recovery_state username=compatbot block=Dirt position=24,64,0 pre_mutation=true crash_stop=true backend_restart=true post_observed=true dirty_reuse=false";
pub(crate) const SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE: &str =
    "survival_biome_dimension_state spawn_environment=minecraft:overworld environment_identifier=minecraft:overworld client_environment_update=minecraft:overworld normalized_identifier=minecraft:overworld";
pub(crate) const SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE: &str =
    "survival_biome_dimension_state username=compatbot spawn_environment=minecraft:overworld environment_identifier=minecraft:overworld server_environment_state=minecraft:overworld normalized_identifier=minecraft:overworld";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_ORIGIN_NEEDLE: &str =
    "survival_biome_dimension_travel_origin dimension=minecraft:overworld biome=minecraft:plains";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_TRANSITION_NEEDLE: &str =
    "survival_biome_dimension_travel_transition_sent kind=nether_portal destination=minecraft:the_nether";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_DESTINATION_NEEDLE: &str =
    "survival_biome_dimension_travel_destination_seen dimension=minecraft:the_nether biome=minecraft:nether_wastes checkpoint=dimension_changed";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_ORIGIN_NEEDLE: &str =
    "survival_biome_dimension_travel_origin username=compatbot dimension=minecraft:overworld biome=minecraft:plains";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_TRANSITION_NEEDLE: &str =
    "survival_biome_dimension_travel_transition username=compatbot kind=nether_portal from=minecraft:overworld to=minecraft:the_nether";
pub(crate) const SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_STATE_NEEDLE: &str =
    "survival_biome_dimension_travel_state username=compatbot origin_dimension=minecraft:overworld origin_biome=minecraft:plains destination_dimension=minecraft:the_nether destination_biome=minecraft:nether_wastes transition=nether_portal server_checkpoint=environment_changed";
pub(crate) const MCP_CONTROLLED_SMOKE_SCENARIO: &str = "mcp-controlled-smoke";
pub(crate) const CTF_OPPONENT_RETURN_DROP_CLIENT_ATTEMPT_NEEDLE: &str =
    "ctf_invalid_opponent_base_return_drop_attempted";
pub(crate) const CTF_OPPONENT_RETURN_DROP_CLIENT_CONTAINED_NEEDLE: &str =
    "ctf_invalid_opponent_base_return_drop_contained";
pub(crate) const VANILLA_COMBAT_REFERENCE_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_vanilla_combat_reference_client_count=2";
pub(crate) const VANILLA_COMBAT_REFERENCE_DAMAGE_NEEDLE: &str = "vanilla_combat_reference_damage";
pub(crate) const VANILLA_COMBAT_REFERENCE_KNOCKBACK_NEEDLE: &str =
    "vanilla_combat_reference_knockback";
pub(crate) const VANILLA_COMBAT_ARMOR_REFERENCE_HEALTH_NEEDLE: &str = "update_health health=15.3";
pub(crate) const CTF_SCORE_LIMIT_CLIENT_WIN_NEEDLE: &str = "ctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false";
pub(crate) const CTF_SCORE_LIMIT_SERVER_PRE_STATE_NEEDLE: &str = "score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win";
pub(crate) const CTF_SCORE_LIMIT_SERVER_FINAL_CAPTURE_NEEDLE: &str = "score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0";
pub(crate) const CTF_SCORE_LIMIT_SERVER_WIN_NEEDLE: &str = "score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0";
pub(crate) const CTF_RACE_CLIENT_COUNT_NEEDLE: &str = "mc_compat_ctf_race_client_count=2";
pub(crate) const CTF_RACE_ACCEPTED_SERVER_NEEDLE: &str = "ctf_race_accepted_transition username=compatbotb player_team=Red flag_team=Blue transition=pickup";
pub(crate) const CTF_RACE_REJECTED_SERVER_NEEDLE: &str = "ctf_race_rejected_transition username=compatbota player_team=Red flag_team=Blue transition=duplicate_pickup";
pub(crate) const CTF_RACE_FINAL_SERVER_NEEDLE: &str = "ctf_race_final_state capture_username=compatbotb accepted_username=compatbotb rejected_username=compatbota capture_team=Red carried_flag=Blue final_blue_flag_state=at_base red_score=1 blue_score=0";
pub(crate) const CTF_SPAWN_TEAM_RESET_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_ctf_spawn_team_reset_client_count=2";
pub(crate) const CTF_SPAWN_TEAM_RED_ASSIGNMENT_NEEDLE: &str = "ctf_spawn_team_assignment username=compatbota team=Red red_count=1 blue_count=0 spawn_x=-40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=RedWool:64";
pub(crate) const CTF_SPAWN_TEAM_BLUE_ASSIGNMENT_NEEDLE: &str = "ctf_spawn_team_assignment username=compatbotb team=Blue red_count=1 blue_count=1 spawn_x=40.0 spawn_y=65.0 spawn_z=0.0 slot36=WoodenSword:1 slot37=BlueWool:64";
pub(crate) const CTF_SPAWN_TEAM_BALANCE_NEEDLE: &str = "ctf_spawn_team_balance red_count=1 blue_count=1 selected_teams=compatbota:Red,compatbotb:Blue outcome=balanced";
pub(crate) const CTF_SPAWN_RESOURCE_RESET_NEEDLE: &str = "ctf_spawn_resource_reset_state trigger=score capture_username=compatbota capture_team=Red carried_flag=Blue red_count=1 blue_count=1 red_score=1 blue_score=0 red_spawn=-40.0,65.0,0.0 blue_spawn=40.0,65.0,0.0 slot36=WoodenSword:1 slot37=TeamWool:64 reset_state=scoreboard_flags_and_resources_coherent";
