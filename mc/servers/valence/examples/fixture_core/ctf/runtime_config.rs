pub const ENV_FLAG_DISABLED_VALUE: &str = "0";
pub const ENV_FLAG_ENABLED_VALUE: &str = "1";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub probes: ProbeConfig,
    pub arrow_policy: ArrowPolicyConfig,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProbeConfig {
    pub inventory_stack_split_merge: bool,
    pub inventory_drag_transactions: bool,
    pub vanilla_combat_reference: bool,
    pub vanilla_combat_armor_reference: bool,
    pub score_limit_win: bool,
    pub race: bool,
    pub spawn_team_reset: bool,
    pub invalid_return_drop: bool,
    pub invalid_opponent_base_return_drop: bool,
    pub projectile: bool,
    pub armor_mitigation: bool,
    pub equipment_update: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArrowPolicyConfig {
    pub config_path: Option<String>,
    pub reload_request: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeConfigInputs {
    pub inventory_stack_split_merge_probe: Option<String>,
    pub inventory_drag_transactions_probe: Option<String>,
    pub vanilla_combat_reference_probe: Option<String>,
    pub vanilla_combat_armor_reference_probe: Option<String>,
    pub arrow_policy_config: Option<String>,
    pub arrow_policy_reload_request: Option<String>,
    pub ctf_score_limit_win_probe: Option<String>,
    pub ctf_race_probe: Option<String>,
    pub ctf_spawn_team_reset_probe: Option<String>,
    pub ctf_invalid_return_drop_probe: Option<String>,
    pub ctf_invalid_opponent_base_return_drop_probe: Option<String>,
    pub projectile_probe: Option<String>,
    pub armor_mitigation_probe: Option<String>,
    pub equipment_update_probe: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeConfigIssue {
    MissingReloadPolicyPath,
    StaleReloadRequest,
    DisabledProjectilePolicy,
}

pub fn parse_runtime_config(inputs: &RuntimeConfigInputs) -> RuntimeConfig {
    let vanilla_combat_armor_reference =
        parse_nonzero_env_flag(inputs.vanilla_combat_armor_reference_probe.as_deref());
    RuntimeConfig {
        probes: ProbeConfig {
            inventory_stack_split_merge: parse_nonzero_env_flag(
                inputs.inventory_stack_split_merge_probe.as_deref(),
            ),
            inventory_drag_transactions: parse_nonzero_env_flag(
                inputs.inventory_drag_transactions_probe.as_deref(),
            ),
            vanilla_combat_reference: parse_nonzero_env_flag(
                inputs.vanilla_combat_reference_probe.as_deref(),
            ) || vanilla_combat_armor_reference,
            vanilla_combat_armor_reference,
            score_limit_win: parse_nonzero_env_flag(inputs.ctf_score_limit_win_probe.as_deref()),
            race: parse_nonzero_env_flag(inputs.ctf_race_probe.as_deref()),
            spawn_team_reset: parse_present_env_flag(inputs.ctf_spawn_team_reset_probe.as_deref()),
            invalid_return_drop: parse_nonzero_env_flag(
                inputs.ctf_invalid_return_drop_probe.as_deref(),
            ),
            invalid_opponent_base_return_drop: parse_nonzero_env_flag(
                inputs
                    .ctf_invalid_opponent_base_return_drop_probe
                    .as_deref(),
            ),
            projectile: parse_nonzero_env_flag(inputs.projectile_probe.as_deref()),
            armor_mitigation: parse_nonzero_env_flag(inputs.armor_mitigation_probe.as_deref()),
            equipment_update: parse_nonzero_env_flag(inputs.equipment_update_probe.as_deref()),
        },
        arrow_policy: ArrowPolicyConfig {
            config_path: inputs.arrow_policy_config.clone(),
            reload_request: inputs.arrow_policy_reload_request.clone(),
        },
    }
}

pub fn runtime_config_issues(
    previous: Option<&RuntimeConfig>,
    config: &RuntimeConfig,
) -> Vec<RuntimeConfigIssue> {
    let mut issues = Vec::new();
    if config.arrow_policy.reload_request.is_some() && config.arrow_policy.config_path.is_none() {
        issues.push(RuntimeConfigIssue::MissingReloadPolicyPath);
    }
    if config.arrow_policy.config_path.is_some() && !config.probes.projectile {
        issues.push(RuntimeConfigIssue::DisabledProjectilePolicy);
    }
    if previous
        .and_then(|previous| previous.arrow_policy.reload_request.as_ref())
        .is_some_and(|previous| Some(previous) == config.arrow_policy.reload_request.as_ref())
    {
        issues.push(RuntimeConfigIssue::StaleReloadRequest);
    }
    issues
}

pub fn parse_nonzero_env_flag(value: Option<&str>) -> bool {
    match value {
        Some(ENV_FLAG_DISABLED_VALUE) | None => false,
        Some(ENV_FLAG_ENABLED_VALUE) => true,
        Some(_) => true,
    }
}

pub fn parse_present_env_flag(value: Option<&str>) -> bool {
    value.is_some()
}
