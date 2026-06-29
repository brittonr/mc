use super::probes;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SurvivalDimensionState<'a> {
    pub(crate) spawn_environment: &'a str,
    pub(crate) environment_identifier: &'a str,
    pub(crate) client_environment_update: &'a str,
    pub(crate) normalized_identifier: &'a str,
}

pub(crate) fn survival_dimension_state<'a>(
    dimension_type_name: &'a str,
    world_name: &'a str,
) -> SurvivalDimensionState<'a> {
    let normalized_identifier =
        probes::derive_survival_environment_id(dimension_type_name, world_name);
    SurvivalDimensionState {
        spawn_environment: normalized_identifier,
        environment_identifier: world_name,
        client_environment_update: dimension_type_name,
        normalized_identifier,
    }
}

pub(crate) fn has_dimension_codec_selection(dimension_type_name: &str, world_name: &str) -> bool {
    !dimension_type_name.is_empty() && !world_name.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    const OVERWORLD_DIMENSION_TYPE: &str = "minecraft:overworld";
    const NETHER_WORLD_NAME: &str = "minecraft:the_nether";
    const UNKNOWN_DIMENSION_TYPE: &str = "custom:missing_dimension";
    const EMPTY_WORLD_NAME: &str = "";

    #[test]
    fn dimension_state_prefers_world_identifier_for_spawn_environment() {
        let state = survival_dimension_state(OVERWORLD_DIMENSION_TYPE, NETHER_WORLD_NAME);

        assert_eq!(state.spawn_environment, probes::SURVIVAL_NETHER_ID);
        assert_eq!(state.environment_identifier, NETHER_WORLD_NAME);
        assert_eq!(state.client_environment_update, OVERWORLD_DIMENSION_TYPE);
        assert_eq!(state.normalized_identifier, probes::SURVIVAL_NETHER_ID);
    }

    #[test]
    fn dimension_state_rejects_missing_or_unknown_dimension_inputs() {
        let state = survival_dimension_state(UNKNOWN_DIMENSION_TYPE, UNKNOWN_DIMENSION_TYPE);

        assert_eq!(
            state.normalized_identifier,
            probes::SURVIVAL_UNKNOWN_ENVIRONMENT_ID
        );
        assert!(!has_dimension_codec_selection(
            UNKNOWN_DIMENSION_TYPE,
            EMPTY_WORLD_NAME
        ));
        assert!(!has_dimension_codec_selection(
            EMPTY_WORLD_NAME,
            NETHER_WORLD_NAME
        ));
    }
}
