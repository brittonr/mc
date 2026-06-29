use super::{ScoreSnapshot, Team};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagPresence {
    AtBase,
    Held,
}

impl FlagPresence {
    pub fn label(self) -> &'static str {
        match self {
            Self::AtBase => "at_base",
            Self::Held => "held",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlagSnapshot {
    pub red: FlagPresence,
    pub blue: FlagPresence,
}

impl FlagSnapshot {
    pub fn for_team(self, team: Team) -> FlagPresence {
        match team {
            Team::Red => self.red,
            Team::Blue => self.blue,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagPickupDecision {
    Accept,
    RejectOwnFlag,
    RejectAlreadyHeld,
}

pub fn evaluate_flag_pickup(
    player_team: Team,
    flag_team: Team,
    flag_presence: FlagPresence,
) -> FlagPickupDecision {
    if player_team == flag_team {
        return FlagPickupDecision::RejectOwnFlag;
    }
    if flag_presence == FlagPresence::Held {
        return FlagPickupDecision::RejectAlreadyHeld;
    }
    FlagPickupDecision::Accept
}

pub fn race_duplicate_pickup_blocked(accepted_username_present: bool) -> bool {
    accepted_username_present
}

pub fn race_accepted_transition_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    accepted_transition: &str,
    race_window_ticks: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_race_accepted_transition username={} player_team={} \
         flag_team={} transition={} race_window_ticks={}",
        username,
        player_team.label(),
        flag_team.label(),
        accepted_transition,
        race_window_ticks
    )
}

pub fn race_rejected_transition_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    rejected_transition: &str,
    race_window_ticks: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_race_rejected_transition username={} player_team={} \
         flag_team={} transition={} reason=flag_already_held race_window_ticks={}",
        username,
        player_team.label(),
        flag_team.label(),
        rejected_transition,
        race_window_ticks
    )
}

pub struct RaceFinalContract<'a> {
    pub expected_capture_team: Team,
    pub expected_carried_flag: Team,
    pub expected_score: ScoreSnapshot,
    pub expected_flag_state: FlagPresence,
    pub flag_state_label: &'a str,
    pub race_window_ticks: u32,
    pub accepted_transition: &'a str,
    pub rejected_transition: &'a str,
}

pub fn race_final_state_milestone(
    accepted_username: &str,
    rejected_username: &str,
    capture_username: &str,
    capture_team: Team,
    carried_flag: Team,
    score: ScoreSnapshot,
    flags: FlagSnapshot,
    contract: RaceFinalContract<'_>,
) -> Option<String> {
    if capture_team != contract.expected_capture_team {
        return None;
    }
    if carried_flag != contract.expected_carried_flag {
        return None;
    }
    if score != contract.expected_score {
        return None;
    }
    if flags.for_team(carried_flag) != contract.expected_flag_state {
        return None;
    }
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_race_final_state capture_username={} \
         accepted_username={} rejected_username={} capture_team={} carried_flag={} \
         final_blue_flag_state={} red_score={} blue_score={} race_window_ticks={} \
         accepted_transition={} rejected_transition={}",
        capture_username,
        accepted_username,
        rejected_username,
        capture_team.label(),
        carried_flag.label(),
        contract.flag_state_label,
        score.red,
        score.blue,
        contract.race_window_ticks,
        contract.accepted_transition,
        contract.rejected_transition
    ))
}

pub fn invalid_flag_pickup_rejection_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    pre_owner: &str,
    post_owner: &str,
    score: ScoreSnapshot,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE invalid_flag_pickup_rejected username={} player_team={} \
         flag_team={} pre_owner={} post_owner={} red_score={} blue_score={} \
         outcome=no_owner_transfer_no_score",
        username,
        player_team.label(),
        flag_team.label(),
        pre_owner,
        post_owner,
        score.red,
        score.blue
    )
}

pub fn invalid_return_drop_rejection_milestone(
    milestone: &str,
    username: &str,
    actor_team: Team,
    flag_team: Team,
    pre_state: &str,
    post_state: &str,
    score: ScoreSnapshot,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE {} username={} actor_team={} flag_team={} pre_state={} \
         post_state={} red_score={} blue_score={} outcome=no_flag_state_mutation_no_score",
        milestone,
        username,
        actor_team.label(),
        flag_team.label(),
        pre_state,
        post_state,
        score.red,
        score.blue
    )
}
