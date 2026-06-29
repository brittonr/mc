pub fn mob_ai_loot_milestones(username: &str) -> Vec<String> {
    vec![
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_spawn username={} mob=Zombie position=16.5,65.0,4.5",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_ai_checkpoint username={} mob=Zombie checkpoint=approach_player target=compatbot",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_attack username={} mob=Zombie kill_method=player_attack",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_death username={} mob=Zombie",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_drop_spawn username={} item=RottenFlesh count=1",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_pickup username={} item=RottenFlesh count=1",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_inventory username={} slot=36 item=RottenFlesh count=1",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_mob_ai_loot_state username={} mob=Zombie ai_checkpoint=approach_player kill_method=player_attack drop=RottenFlesh count=1 pickup=observed inventory_increment=1 extra_mobs=false",
            username,
        ),
    ]
}

pub fn redstone_circuit_milestones(username: &str) -> Vec<String> {
    vec![
        format!(
            "MC-COMPAT-MILESTONE survival_redstone_circuit_initial username={} circuit=lever_lamp_repeater powered=false tick=0",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_redstone_circuit_input username={} control=Lever position=20,64,0 tick=2 powered_after=true",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_redstone_circuit_powered_on username={} output=RedstoneLamp repeater=Repeater tick=2 powered=true",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_redstone_circuit_powered_off username={} output=RedstoneLamp repeater=Repeater tick=4 powered=false",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_redstone_circuit_state username={} circuit=lever_lamp_repeater initial=false after_input=true after_return=false tick_sequence=0:false,2:true,4:false unintended_outputs=false",
            username,
        ),
    ]
}

pub fn world_multichunk_milestones(username: &str, post_restart: bool) -> Vec<String> {
    if post_restart {
        return vec![
            format!(
                "MC-COMPAT-MILESTONE survival_world_multichunk_post_restart_observe username={} primary=present secondary=present auxiliary_marker_only=false",
                username,
            ),
            format!(
                "MC-COMPAT-MILESTONE survival_world_multichunk_state username={} chunks=0,0;2,0 primary=present secondary=present controlled_reload=true post_observed=true auxiliary_marker_only=false dirty_reuse=false",
                username,
            ),
        ];
    }
    vec![format!(
        "MC-COMPAT-MILESTONE survival_world_multichunk_mutation username={} chunks=0,0;2,0 primary=0,64,0:Dirt secondary=32,64,0:OakPlanks persisted_before=false persisted_after=true",
        username,
    )]
}

pub fn container_block_entity_milestones(username: &str) -> Vec<String> {
    vec![
        format!(
            "MC-COMPAT-MILESTONE survival_container_block_entity_open username={} window=1 kind=Barrel position=34,64,0",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_container_block_entity_transfer username={} window=1 slot=0 item=Dirt count=1",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_container_block_entity_payload username={} summary=slot0:Dirt:1",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_container_block_entity_metadata username={} summary=custom_name:MC Compat Barrel",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_container_block_entity_state username={} kind=Barrel position=34,64,0 transfer=Dirt:1 payload=slot0:Dirt:1 metadata=custom_name:MC Compat Barrel reopen=payload_present arbitrary_nbt=false",
            username,
        ),
    ]
}

pub fn biome_dimension_travel_milestones(username: &str) -> Vec<String> {
    vec![
        format!(
            "MC-COMPAT-MILESTONE survival_biome_dimension_travel_origin username={} dimension=minecraft:overworld biome=minecraft:plains",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_biome_dimension_travel_transition username={} kind=nether_portal from=minecraft:overworld to=minecraft:the_nether",
            username,
        ),
        format!(
            "MC-COMPAT-MILESTONE survival_biome_dimension_travel_state username={} origin_dimension=minecraft:overworld origin_biome=minecraft:plains destination_dimension=minecraft:the_nether destination_biome=minecraft:nether_wastes transition=nether_portal server_checkpoint=environment_changed",
            username,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_USERNAME: &str = "compatbot";

    #[test]
    fn breadth_milestones_cover_positive_fixture_rows() {
        assert!(mob_ai_loot_milestones(TEST_USERNAME)
            .iter()
            .any(|milestone| milestone.contains("survival_mob_ai_loot_state")));
        assert!(redstone_circuit_milestones(TEST_USERNAME)
            .iter()
            .any(|milestone| milestone.contains("unintended_outputs=false")));
        assert!(world_multichunk_milestones(TEST_USERNAME, false)
            .iter()
            .any(|milestone| milestone.contains("survival_world_multichunk_mutation")));
        assert!(container_block_entity_milestones(TEST_USERNAME)
            .iter()
            .any(|milestone| milestone.contains("arbitrary_nbt=false")));
        assert!(biome_dimension_travel_milestones(TEST_USERNAME)
            .iter()
            .any(|milestone| milestone.contains("server_checkpoint=environment_changed")));
    }

    #[test]
    fn world_multichunk_post_restart_uses_observe_path_not_mutation_path() {
        let milestones = world_multichunk_milestones(TEST_USERNAME, true);
        assert!(milestones
            .iter()
            .any(|milestone| milestone.contains("post_restart_observe")));
        assert!(!milestones
            .iter()
            .any(|milestone| milestone.contains("survival_world_multichunk_mutation")));
    }
}
