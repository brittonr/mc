pub fn biome_dimension_state_milestone(
    username: &str,
    spawn_environment: &str,
    environment_identifier: &str,
    derived_environment: &str,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE survival_biome_dimension_state username={} spawn_environment={} \
         environment_identifier={} derived_environment={}",
        username, spawn_environment, environment_identifier, derived_environment,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_USERNAME: &str = "compatbot";
    const NETHER: &str = "minecraft:the_nether";
    const OVERWORLD: &str = "minecraft:overworld";

    #[test]
    fn biome_dimension_milestone_preserves_vocabulary() {
        let milestone =
            biome_dimension_state_milestone(TEST_USERNAME, NETHER, OVERWORLD, OVERWORLD);
        assert!(
            milestone.contains("survival_biome_dimension_state"),
            "{milestone}",
        );
        assert!(
            milestone.contains("derived_environment=minecraft:overworld"),
            "{milestone}",
        );
    }
}
