use super::{ScoreSnapshot, Team};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SpawnResetState {
    pub red_count: u32,
    pub blue_count: u32,
    pub red_username: Option<String>,
    pub blue_username: Option<String>,
}

impl SpawnResetState {
    pub fn record_assignment(&mut self, username: &str, team: Team) {
        match team {
            Team::Red => {
                self.red_count += 1;
                self.red_username = Some(username.to_owned());
            }
            Team::Blue => {
                self.blue_count += 1;
                self.blue_username = Some(username.to_owned());
            }
        }
    }
}

pub struct SpawnResetContract {
    pub expected_red_count: u32,
    pub expected_blue_count: u32,
    pub expected_blue_username: &'static str,
    pub reset_score: ScoreSnapshot,
    pub slot36_resource: &'static str,
    pub red_slot37_resource: &'static str,
    pub blue_slot37_resource: &'static str,
    pub reset_slot37_resource: &'static str,
    pub reset_state: &'static str,
}

pub fn spawn_team_assignment_milestone(
    username: &str,
    team: Team,
    red_count: u32,
    blue_count: u32,
    slot36_resource: &str,
    slot37_resource: &str,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_spawn_team_assignment username={} team={} red_count={} \
         blue_count={} spawn={} slot36={} slot37={}",
        username,
        team.label(),
        red_count,
        blue_count,
        team.label().to_ascii_lowercase(),
        slot36_resource,
        slot37_resource
    )
}

pub fn spawn_team_balance_milestone(
    state: &SpawnResetState,
    contract: &SpawnResetContract,
) -> Option<String> {
    if state.red_count != contract.expected_red_count {
        return None;
    }
    if state.blue_count != contract.expected_blue_count {
        return None;
    }
    let red_username = state.red_username.as_deref()?;
    let blue_username = state.blue_username.as_deref()?;
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_spawn_team_balance red_count={} blue_count={} \
         selected_teams={}:Red,{}:Blue",
        state.red_count, state.blue_count, red_username, blue_username
    ))
}

pub fn spawn_resource_reset_state_milestone(
    state: &SpawnResetState,
    username: &str,
    team: Team,
    reset_flag: Team,
    score: ScoreSnapshot,
    contract: &SpawnResetContract,
) -> Option<String> {
    if state.red_count != contract.expected_red_count {
        return None;
    }
    if state.blue_count != contract.expected_blue_count {
        return None;
    }
    if state.blue_username.as_deref() != Some(contract.expected_blue_username) {
        return None;
    }
    if score != contract.reset_score {
        return None;
    }
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_spawn_resource_reset_state username={} team={} \
         reset_flag={} red_score={} blue_score={} slot36={} slot37={} reset_state={}",
        username,
        team.label(),
        reset_flag.label(),
        score.red,
        score.blue,
        contract.slot36_resource,
        contract.reset_slot37_resource,
        contract.reset_state
    ))
}

pub fn defer_spawn_assignment(username: &str, team: Team, expected_blue_username: &str) -> bool {
    username == expected_blue_username && team == Team::Red
}
