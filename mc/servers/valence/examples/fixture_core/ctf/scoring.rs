use super::Team;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScoreSnapshot {
    pub red: u32,
    pub blue: u32,
}

impl ScoreSnapshot {
    pub fn for_team(self, team: Team) -> u32 {
        match team {
            Team::Red => self.red,
            Team::Blue => self.blue,
        }
    }
}

pub fn score_limit_pre_state_milestone(score: ScoreSnapshot, score_limit: u32) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_pre_state score_limit={} red_score={} blue_score={}",
        score_limit, score.red, score.blue
    )
}

pub fn score_limit_final_capture_milestone(
    username: &str,
    capture_team: Team,
    carried_flag: Team,
    before: ScoreSnapshot,
    after: ScoreSnapshot,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_final_capture username={} capture_team={} \
         carried_flag={} red_score_before={} blue_score_before={} red_score_after={} \
         blue_score_after={}",
        username,
        capture_team.label(),
        carried_flag.label(),
        before.red,
        before.blue,
        after.red,
        after.blue
    )
}

pub fn score_limit_win_condition_milestone(
    username: &str,
    winning_team: Team,
    score: ScoreSnapshot,
    win_emissions: u32,
    score_limit: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_win_condition username={} winning_team={} \
         red_score={} blue_score={} score_limit={} end_state=winner_declared win_emissions={} \
         duplicate_win=false post_win_score_delta=0",
        username,
        winning_team.label(),
        score.red,
        score.blue,
        score_limit,
        win_emissions
    )
}

pub fn score_limit_duplicate_win_milestone(username: &str, winning_team: Team) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_duplicate_win username={} winning_team={} \
         outcome=forbidden_duplicate_win",
        username,
        winning_team.label()
    )
}

pub fn score_limit_post_win_score_mutation_milestone(username: &str, winning_team: Team) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_post_win_score_mutation username={} winning_team={} \
         outcome=forbidden_score_after_win",
        username,
        winning_team.label()
    )
}
