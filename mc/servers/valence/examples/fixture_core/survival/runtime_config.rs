use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuntimeFixtureFlags {
    pub hunger_food: bool,
    pub hunger_health: bool,
    pub block_entity: bool,
    pub block_entity_post_restart: bool,
    pub world_multichunk: bool,
    pub world_multichunk_post_restart: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeConfigIssue {
    ConflictingHungerFixtures,
    StaleBlockEntityPhase,
    StaleWorldMultichunkPhase,
}

pub fn enabled_flag(value: Option<&str>, enabled_value: &str) -> bool {
    matches!(value, Some(observed) if observed == enabled_value)
}

pub fn post_restart_phase(value: Option<&str>, post_restart_value: &str) -> bool {
    matches!(value, Some(observed) if observed == post_restart_value)
}

pub fn marker_path(
    configured_dir: Option<&str>,
    temp_dir: &Path,
    default_dir_name: &str,
    marker_file: &str,
) -> PathBuf {
    configured_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| temp_dir.join(default_dir_name))
        .join(marker_file)
}

pub fn runtime_config_issues(flags: RuntimeFixtureFlags) -> Vec<RuntimeConfigIssue> {
    let mut issues = Vec::new();
    if flags.hunger_food && flags.hunger_health {
        issues.push(RuntimeConfigIssue::ConflictingHungerFixtures);
    }
    if flags.block_entity_post_restart && !flags.block_entity {
        issues.push(RuntimeConfigIssue::StaleBlockEntityPhase);
    }
    if flags.world_multichunk_post_restart && !flags.world_multichunk {
        issues.push(RuntimeConfigIssue::StaleWorldMultichunkPhase);
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENABLED_VALUE: &str = "1";
    const POST_RESTART: &str = "post_restart";
    const OTHER_VALUE: &str = "true";
    const TEMP_DIR: &str = "/tmp/mc-compat";
    const CONFIGURED_DIR: &str = "/tmp/configured";
    const DEFAULT_DIR: &str = "default-survival";
    const MARKER_FILE: &str = "persisted.marker";

    #[test]
    fn flags_accept_exact_enabled_and_post_restart_values() {
        assert!(enabled_flag(Some(ENABLED_VALUE), ENABLED_VALUE));
        assert!(!enabled_flag(Some(OTHER_VALUE), ENABLED_VALUE));
        assert!(!enabled_flag(None, ENABLED_VALUE));
        assert!(post_restart_phase(Some(POST_RESTART), POST_RESTART));
        assert!(!post_restart_phase(Some(OTHER_VALUE), POST_RESTART));
    }

    #[test]
    fn marker_path_uses_configured_directory_or_temp_default() {
        assert_eq!(
            marker_path(
                Some(CONFIGURED_DIR),
                Path::new(TEMP_DIR),
                DEFAULT_DIR,
                MARKER_FILE,
            ),
            PathBuf::from(CONFIGURED_DIR).join(MARKER_FILE),
        );
        assert_eq!(
            marker_path(None, Path::new(TEMP_DIR), DEFAULT_DIR, MARKER_FILE),
            PathBuf::from(TEMP_DIR).join(DEFAULT_DIR).join(MARKER_FILE),
        );
    }

    #[test]
    fn invalid_runtime_flags_report_conflicts_and_stale_phases() {
        assert_eq!(
            runtime_config_issues(RuntimeFixtureFlags {
                hunger_food: true,
                hunger_health: true,
                block_entity_post_restart: true,
                world_multichunk_post_restart: true,
                ..RuntimeFixtureFlags::default()
            }),
            vec![
                RuntimeConfigIssue::ConflictingHungerFixtures,
                RuntimeConfigIssue::StaleBlockEntityPhase,
                RuntimeConfigIssue::StaleWorldMultichunkPhase,
            ],
        );
        assert!(runtime_config_issues(RuntimeFixtureFlags {
            hunger_food: true,
            block_entity: true,
            block_entity_post_restart: true,
            ..RuntimeFixtureFlags::default()
        })
        .is_empty());
    }
}
